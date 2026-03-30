//! Live session with incremental update support.
//!
//! Tracks the current patch state and Ableton track mapping so that
//! pattern-only changes (the common case during live coding) can be
//! applied without recreating tracks, instruments, or effects.

use std::collections::HashMap;

use crate::clip::Note;
use crate::compiler::{
    find_pattern_nodes, find_source_nodes, CompileResult, IrNodeKind, IrPatch, IrSourceKind,
};
use crate::error::Result;
use crate::session::Session;

/// What kind of update was performed.
#[derive(Debug, Clone, Copy)]
pub enum UpdateKind {
    /// First compile — full setup.
    Initial,
    /// Only clip notes/tempo changed — fast path.
    ClipsOnly,
    /// Only tempo changed — instant.
    TempoOnly,
    /// Structure changed — full teardown and rebuild.
    Full,
    /// Nothing changed.
    NoOp,
}

/// Result of a live update.
#[derive(Debug)]
pub struct UpdateResult {
    pub kind: UpdateKind,
    pub compile_result: CompileResult,
}

impl UpdateResult {
    pub fn summary(&self) -> String {
        let kind_str = match self.kind {
            UpdateKind::Initial => "initial",
            UpdateKind::ClipsOnly => "clips-only",
            UpdateKind::TempoOnly => "tempo-only",
            UpdateKind::Full => "full rebuild",
            UpdateKind::NoOp => "no change",
        };
        match self.kind {
            UpdateKind::TempoOnly | UpdateKind::NoOp => format!("[{kind_str}]"),
            _ => format!("[{kind_str}] {}", self.compile_result.summary()),
        }
    }
}

/// A live session that supports incremental updates to Ableton.
pub struct LiveSession {
    session: Session,
    current_patch: Option<IrPatch>,
    /// Maps source node ID -> Ableton track index.
    track_map: HashMap<i32, i32>,
    /// Track indices we created (sorted), for cleanup.
    created_tracks: Vec<i32>,
}

impl LiveSession {
    pub fn new(session: Session) -> Self {
        Self {
            session,
            current_patch: None,
            track_map: HashMap::new(),
            created_tracks: Vec::new(),
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    /// Apply a patch, using incremental updates when possible.
    pub fn update(&mut self, patch: IrPatch) -> Result<UpdateResult> {
        let kind = match &self.current_patch {
            None => UpdateKind::Initial,
            Some(old) => categorize_change(old, &patch),
        };

        match kind {
            UpdateKind::Initial => {
                let result = crate::compiler::compile(&patch, &self.session)?;
                self.store_mapping(&result);
                self.current_patch = Some(patch);
                Ok(UpdateResult { kind, compile_result: result })
            }
            UpdateKind::Full => {
                self.teardown()?;
                let result = crate::compiler::compile(&patch, &self.session)?;
                self.store_mapping(&result);
                self.current_patch = Some(patch);
                Ok(UpdateResult { kind, compile_result: result })
            }
            UpdateKind::ClipsOnly => {
                let result = self.rewrite_clips(&patch)?;
                // Also update tempo if it changed
                if let Some(old) = &self.current_patch {
                    if (old.bpm - patch.bpm).abs() > 0.01 {
                        self.session.set_tempo(patch.bpm as f32)?;
                    }
                }
                self.current_patch = Some(patch);
                Ok(UpdateResult { kind, compile_result: result })
            }
            UpdateKind::TempoOnly => {
                self.session.set_tempo(patch.bpm as f32)?;
                self.current_patch = Some(patch);
                Ok(UpdateResult {
                    kind,
                    compile_result: CompileResult::default(),
                })
            }
            UpdateKind::NoOp => {
                Ok(UpdateResult {
                    kind,
                    compile_result: CompileResult::default(),
                })
            }
        }
    }

    fn store_mapping(&mut self, result: &CompileResult) {
        self.track_map = result.track_map.clone();
        self.created_tracks = result.track_map.values().copied().collect();
        self.created_tracks.sort();
    }

    fn teardown(&mut self) -> Result<()> {
        // Delete tracks in reverse order to avoid index shifting
        let mut indices = self.created_tracks.clone();
        indices.sort();
        indices.reverse();
        for idx in indices {
            self.session.delete_track(idx)?;
        }
        self.track_map.clear();
        self.created_tracks.clear();
        Ok(())
    }

    fn rewrite_clips(&self, patch: &IrPatch) -> Result<CompileResult> {
        let mut result = CompileResult::default();
        let source_nodes = find_source_nodes(patch);
        let pattern_nodes = find_pattern_nodes(patch);

        for (i, pnode) in pattern_nodes.iter().enumerate() {
            if let IrNodeKind::Pattern(ref clip_data) = pnode.kind {
                let source_idx = i % source_nodes.len();
                if let Some(&track_idx) = self.track_map.get(&source_nodes[source_idx].id) {
                    let track = self.session.track(track_idx);

                    // Check if clip length changed — if so, recreate it
                    let length_changed = if track.has_clip(0).unwrap_or(false) {
                        let clip = track.clip(0);
                        let old_len = clip.get_length().unwrap_or(0.0);
                        (old_len - clip_data.length as f32).abs() > 0.01
                    } else {
                        false
                    };

                    if length_changed {
                        track.delete_clip(0)?;
                    }

                    if !track.has_clip(0).unwrap_or(false) {
                        // Create fresh clip
                        let clip = track.create_clip(0, clip_data.length as f32)?;
                        clip.set_name(&pnode.label)?;
                        clip.set_looping(true)?;

                        let notes: Vec<Note> = clip_data
                            .events
                            .iter()
                            .map(|e| {
                                let pitch = (e.value.round() as i32).clamp(0, 127);
                                Note::new(pitch, e.start as f32, e.duration as f32, 100)
                            })
                            .collect();

                        if !notes.is_empty() {
                            clip.add_notes(&notes)?;
                        }
                        // Re-fire the clip so it starts playing
                        let _ = clip.fire();
                    } else {
                        // Same length — just rewrite notes
                        let clip = track.clip(0);
                        clip.clear_notes()?;

                        let notes: Vec<Note> = clip_data
                            .events
                            .iter()
                            .map(|e| {
                                let pitch = (e.value.round() as i32).clamp(0, 127);
                                Note::new(pitch, e.start as f32, e.duration as f32, 100)
                            })
                            .collect();

                        if !notes.is_empty() {
                            clip.add_notes(&notes)?;
                        }
                    }

                    result.clips_created += 1;
                }
            }
        }

        Ok(result)
    }

    /// Fire all clips in slot 0 and start playback.
    pub fn play(&self) -> Result<()> {
        if let Ok(tracks) = self.session.tracks() {
            for track in &tracks {
                if track.has_clip(0).unwrap_or(false) {
                    let clip = track.clip(0);
                    let _ = clip.fire();
                }
            }
        }
        self.session.play()?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Change detection
// ---------------------------------------------------------------------------

fn categorize_change(old: &IrPatch, new: &IrPatch) -> UpdateKind {
    let old_sources = find_source_nodes(old);
    let new_sources = find_source_nodes(new);

    // Different number of sources -> full rebuild
    if old_sources.len() != new_sources.len() {
        return UpdateKind::Full;
    }

    // Check if source types/presets match
    for (o, n) in old_sources.iter().zip(new_sources.iter()) {
        if !source_kinds_match(&o.kind, &n.kind) {
            return UpdateKind::Full;
        }
    }

    // Check if effect chains match (count and names)
    let old_effects: Vec<_> = old
        .nodes
        .iter()
        .filter(|n| matches!(n.kind, IrNodeKind::Effect(_) | IrNodeKind::Chain(_)))
        .collect();
    let new_effects: Vec<_> = new
        .nodes
        .iter()
        .filter(|n| matches!(n.kind, IrNodeKind::Effect(_) | IrNodeKind::Chain(_)))
        .collect();

    if old_effects.len() != new_effects.len() {
        return UpdateKind::Full;
    }
    for (o, n) in old_effects.iter().zip(new_effects.iter()) {
        if !effect_nodes_match(&o.kind, &n.kind) {
            return UpdateKind::Full;
        }
    }

    // Check if edges match
    if old.edges.len() != new.edges.len() {
        return UpdateKind::Full;
    }

    // Structure matches — check patterns and tempo
    let old_patterns = find_pattern_nodes(old);
    let new_patterns = find_pattern_nodes(new);

    let patterns_changed = old_patterns.len() != new_patterns.len()
        || old_patterns
            .iter()
            .zip(new_patterns.iter())
            .any(|(o, n)| !clips_match(&o.kind, &n.kind));

    let tempo_changed = (old.bpm - new.bpm).abs() > 0.01;

    if patterns_changed {
        UpdateKind::ClipsOnly // will also handle tempo
    } else if tempo_changed {
        UpdateKind::TempoOnly
    } else {
        UpdateKind::NoOp
    }
}

fn source_kinds_match(a: &IrNodeKind, b: &IrNodeKind) -> bool {
    match (a, b) {
        (IrNodeKind::Source(sa), IrNodeKind::Source(sb)) => match (sa, sb) {
            (
                IrSourceKind::Synth { preset: pa, .. },
                IrSourceKind::Synth { preset: pb, .. },
            ) => pa == pb,
            (
                IrSourceKind::Sample { path: pa, .. },
                IrSourceKind::Sample { path: pb, .. },
            ) => pa == pb,
            (IrSourceKind::LiveInput { .. }, IrSourceKind::LiveInput { .. }) => true,
            (IrSourceKind::Resampled { .. }, IrSourceKind::Resampled { .. }) => true,
            _ => false,
        },
        _ => false,
    }
}

fn effect_nodes_match(a: &IrNodeKind, b: &IrNodeKind) -> bool {
    match (a, b) {
        (IrNodeKind::Effect(ea), IrNodeKind::Effect(eb)) => ea.name == eb.name,
        (IrNodeKind::Chain(ca), IrNodeKind::Chain(cb)) => {
            ca.len() == cb.len() && ca.iter().zip(cb.iter()).all(|(a, b)| a.name == b.name)
        }
        _ => false,
    }
}

fn clips_match(a: &IrNodeKind, b: &IrNodeKind) -> bool {
    match (a, b) {
        (IrNodeKind::Pattern(ca), IrNodeKind::Pattern(cb)) => {
            ca.events.len() == cb.events.len()
                && (ca.length - cb.length).abs() < 0.001
                && ca
                    .events
                    .iter()
                    .zip(cb.events.iter())
                    .all(|(ea, eb)| {
                        (ea.value - eb.value).abs() < 0.001
                            && (ea.start - eb.start).abs() < 0.001
                            && (ea.duration - eb.duration).abs() < 0.001
                    })
        }
        _ => false,
    }
}
