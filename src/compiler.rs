//! Compiler — translates metaritual IR JSON into Ableton Live actions.
//!
//! Reads an IrPatch (from JSON), creates tracks, loads instruments/effects,
//! writes clips with notes, and applies automation. Pure Rust, no Python needed.

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crate::clip::Note;
use crate::error::Result;
use crate::session::Session;

// ---------------------------------------------------------------------------
// IR types (deserialized from JSON)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct IrBreakpoint {
    pub time: f64,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct IrAutomation {
    pub param_name: String,
    pub breakpoints: Vec<IrBreakpoint>,
}

#[derive(Debug, Clone)]
pub struct IrParam {
    pub name: String,
    pub value: f64,
    pub range: (f64, f64),
}

#[derive(Debug, Clone)]
pub struct IrEvent {
    pub value: f64,
    pub start: f64,
    pub duration: f64,
}

#[derive(Debug, Clone)]
pub struct IrClip {
    pub events: Vec<IrEvent>,
    pub length: f64,
    pub bpm: f64,
}

#[derive(Debug, Clone)]
pub struct IrEffect {
    pub name: String,
    pub params: Vec<IrParam>,
    pub automation: Vec<IrAutomation>,
}

#[derive(Debug, Clone)]
pub enum IrSourceKind {
    Sample { path: String, label: String },
    Synth { preset: String, params: Vec<(String, f64)>, label: String },
    LiveInput { channel: i32, label: String },
    Resampled { patch_id: String, label: String },
}

impl IrSourceKind {
    pub fn label(&self) -> &str {
        match self {
            IrSourceKind::Sample { label, .. } => label,
            IrSourceKind::Synth { label, .. } => label,
            IrSourceKind::LiveInput { label, .. } => label,
            IrSourceKind::Resampled { label, .. } => label,
        }
    }

    pub fn is_live_input(&self) -> bool {
        matches!(self, IrSourceKind::LiveInput { .. })
    }
}

#[derive(Debug, Clone)]
pub enum IrNodeKind {
    Source(IrSourceKind),
    Pattern(IrClip),
    Effect(IrEffect),
    Chain(Vec<IrEffect>),
    Mixer { channels: i32 },
    Split { outputs: i32 },
    Merge { inputs: i32 },
}

#[derive(Debug, Clone)]
pub struct IrNode {
    pub id: i32,
    pub label: String,
    pub kind: IrNodeKind,
}

#[derive(Debug, Clone)]
pub struct IrEdge {
    pub from_node: i32,
    pub from_port: i32,
    pub to_node: i32,
    pub to_port: i32,
}

#[derive(Debug, Clone)]
pub struct IrSpace {
    pub pan: IrAutomation,
    pub width: IrAutomation,
    pub depth: IrAutomation,
}

#[derive(Debug, Clone)]
pub struct IrExposedParam {
    pub name: String,
    pub range: (f64, f64),
    pub default: f64,
    pub automation: IrAutomation,
}

#[derive(Debug, Clone)]
pub struct IrPatch {
    pub label: String,
    pub bpm: f64,
    pub nodes: Vec<IrNode>,
    pub edges: Vec<IrEdge>,
    pub space: IrSpace,
    pub exposed_params: Vec<IrExposedParam>,
}

// ---------------------------------------------------------------------------
// JSON parsing
// ---------------------------------------------------------------------------

use serde_json::Value;

fn parse_breakpoint(v: &Value) -> IrBreakpoint {
    IrBreakpoint {
        time: v["time"].as_f64().unwrap_or(0.0),
        value: v["value"].as_f64().unwrap_or(0.0),
    }
}

fn parse_automation(v: &Value) -> IrAutomation {
    IrAutomation {
        param_name: v["param_name"].as_str().unwrap_or("").to_string(),
        breakpoints: v["breakpoints"]
            .as_array()
            .map(|arr| arr.iter().map(parse_breakpoint).collect())
            .unwrap_or_default(),
    }
}

fn parse_param(v: &Value) -> IrParam {
    let range = v["range"].as_array().map(|a| {
        (a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(1.0))
    }).unwrap_or((0.0, 1.0));
    IrParam {
        name: v["name"].as_str().unwrap_or("").to_string(),
        value: v["value"].as_f64().unwrap_or(0.0),
        range,
    }
}

fn parse_event(v: &Value) -> IrEvent {
    IrEvent {
        value: v["value"].as_f64().unwrap_or(0.0),
        start: v["start"].as_f64().unwrap_or(0.0),
        duration: v["duration"].as_f64().unwrap_or(0.0),
    }
}

fn parse_clip(v: &Value) -> IrClip {
    IrClip {
        events: v["events"].as_array()
            .map(|arr| arr.iter().map(parse_event).collect())
            .unwrap_or_default(),
        length: v["length"].as_f64().unwrap_or(4.0),
        bpm: v["bpm"].as_f64().unwrap_or(120.0),
    }
}

fn parse_effect(v: &Value) -> IrEffect {
    IrEffect {
        name: v["name"].as_str().unwrap_or("").to_string(),
        params: v["params"].as_array()
            .map(|arr| arr.iter().map(parse_param).collect())
            .unwrap_or_default(),
        automation: v["automation"].as_array()
            .map(|arr| arr.iter().map(parse_automation).collect())
            .unwrap_or_default(),
    }
}

fn parse_source(v: &Value) -> IrSourceKind {
    let typ = v["type"].as_str().unwrap_or("synth");
    let label = v["label"].as_str().unwrap_or("").to_string();
    match typ {
        "sample" => IrSourceKind::Sample {
            path: v["path"].as_str().unwrap_or("").to_string(),
            label,
        },
        "synth" => {
            let params = v["params"].as_array()
                .map(|arr| arr.iter().filter_map(|p| {
                    let a = p.as_array()?;
                    Some((a[0].as_str()?.to_string(), a[1].as_f64()?))
                }).collect())
                .unwrap_or_default();
            IrSourceKind::Synth {
                preset: v["preset"].as_str().unwrap_or("Analog").to_string(),
                params,
                label,
            }
        }
        "live_input" => IrSourceKind::LiveInput {
            channel: v["channel"].as_i64().unwrap_or(0) as i32,
            label,
        },
        "resampled" => IrSourceKind::Resampled {
            patch_id: v["patch_id"].as_str().unwrap_or("").to_string(),
            label,
        },
        _ => IrSourceKind::Synth {
            preset: "Analog".to_string(),
            params: vec![],
            label,
        },
    }
}

fn parse_node(v: &Value) -> IrNode {
    let nt = v["type"].as_str().unwrap_or("source");
    let id = v["id"].as_i64().unwrap_or(0) as i32;
    let label = v["label"].as_str().unwrap_or("").to_string();
    let kind = match nt {
        "source" => IrNodeKind::Source(parse_source(&v["source"])),
        "pattern" => IrNodeKind::Pattern(parse_clip(&v["clip"])),
        "effect" => IrNodeKind::Effect(parse_effect(&v["effect"])),
        "chain" => {
            let effects = v["effects"].as_array()
                .map(|arr| arr.iter().map(parse_effect).collect())
                .unwrap_or_default();
            IrNodeKind::Chain(effects)
        }
        "mixer" => IrNodeKind::Mixer {
            channels: v["channels"].as_i64().unwrap_or(2) as i32,
        },
        "split" => IrNodeKind::Split {
            outputs: v["outputs"].as_i64().unwrap_or(2) as i32,
        },
        "merge" => IrNodeKind::Merge {
            inputs: v["inputs"].as_i64().unwrap_or(2) as i32,
        },
        _ => IrNodeKind::Mixer { channels: 2 },
    };
    IrNode { id, label, kind }
}

impl IrPatch {
    /// Parse an IrPatch from a JSON string.
    pub fn from_json(json: &str) -> std::result::Result<Self, String> {
        let v: Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        Ok(Self {
            label: v["label"].as_str().unwrap_or("").to_string(),
            bpm: v["bpm"].as_f64().unwrap_or(120.0),
            nodes: v["nodes"].as_array()
                .map(|arr| arr.iter().map(parse_node).collect())
                .unwrap_or_default(),
            edges: v["edges"].as_array()
                .map(|arr| arr.iter().map(|e| IrEdge {
                    from_node: e["from_node"].as_i64().unwrap_or(0) as i32,
                    from_port: e["from_port"].as_i64().unwrap_or(0) as i32,
                    to_node: e["to_node"].as_i64().unwrap_or(0) as i32,
                    to_port: e["to_port"].as_i64().unwrap_or(0) as i32,
                }).collect())
                .unwrap_or_default(),
            space: IrSpace {
                pan: parse_automation(&v["space"]["pan"]),
                width: parse_automation(&v["space"]["width"]),
                depth: parse_automation(&v["space"]["depth"]),
            },
            exposed_params: v["exposed_params"].as_array()
                .map(|arr| arr.iter().map(|p| IrExposedParam {
                    name: p["name"].as_str().unwrap_or("").to_string(),
                    range: p["range"].as_array().map(|a| {
                        (a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(1.0))
                    }).unwrap_or((0.0, 1.0)),
                    default: p["default"].as_f64().unwrap_or(0.0),
                    automation: parse_automation(&p["automation"]),
                }).collect())
                .unwrap_or_default(),
        })
    }
}

// ---------------------------------------------------------------------------
// Effect name mapping: IR name → Ableton browser name
// ---------------------------------------------------------------------------

fn effect_name(ir_name: &str) -> &str {
    match ir_name {
        "Reverb" => "Reverb",
        "Delay" => "Delay",
        "LowPass" | "HighPass" => "Auto Filter",
        "Compressor" => "Compressor",
        "EQ3" => "EQ Three",
        "Saturator" => "Saturator",
        _ => ir_name,
    }
}

// ---------------------------------------------------------------------------
// Compile result
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct CompileResult {
    pub tracks_created: Vec<String>,
    pub clips_created: usize,
    pub effects_loaded: Vec<String>,
    pub errors: Vec<String>,
    /// Maps source node ID → Ableton track index (for incremental updates).
    pub track_map: HashMap<i32, i32>,
}

impl CompileResult {
    pub fn summary(&self) -> String {
        let mut lines = vec![format!(
            "Compiled: {} tracks, {} clips, {} effects",
            self.tracks_created.len(),
            self.clips_created,
            self.effects_loaded.len(),
        )];
        for t in &self.tracks_created {
            lines.push(format!("  track: {t}"));
        }
        for e in &self.errors {
            lines.push(format!("  error: {e}"));
        }
        lines.join("\n")
    }
}

// ---------------------------------------------------------------------------
// Graph helpers
// ---------------------------------------------------------------------------

fn build_adjacency(patch: &IrPatch) -> HashMap<i32, Vec<i32>> {
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
    for node in &patch.nodes {
        adj.entry(node.id).or_default();
    }
    for edge in &patch.edges {
        adj.entry(edge.from_node).or_default().push(edge.to_node);
    }
    adj
}

fn find_effect_chain<'a>(patch: &'a IrPatch, source_id: i32) -> Vec<&'a IrNode> {
    let adj = build_adjacency(patch);
    let nodes_by_id: HashMap<i32, &IrNode> = patch.nodes.iter().map(|n| (n.id, n)).collect();
    let mut chain = Vec::new();
    let mut current = source_id;

    loop {
        let downstream = adj.get(&current).cloned().unwrap_or_default();
        let mut next: Option<&IrNode> = None;
        for nid in &downstream {
            if let Some(node) = nodes_by_id.get(nid) {
                match &node.kind {
                    IrNodeKind::Effect(_) | IrNodeKind::Chain(_) => {
                        next = Some(node);
                        break;
                    }
                    IrNodeKind::Mixer { .. } => {
                        // Walk through mixer to find effects after it
                        let mixer_downstream = adj.get(&node.id).cloned().unwrap_or_default();
                        for mid in &mixer_downstream {
                            if let Some(mnode) = nodes_by_id.get(mid) {
                                match &mnode.kind {
                                    IrNodeKind::Effect(_) | IrNodeKind::Chain(_) => {
                                        next = Some(mnode);
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        if next.is_some() {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
        match next {
            None => break,
            Some(n) => {
                chain.push(n);
                current = n.id;
            }
        }
    }
    chain
}

pub fn find_source_nodes(patch: &IrPatch) -> Vec<&IrNode> {
    patch.nodes.iter().filter(|n| matches!(n.kind, IrNodeKind::Source(_))).collect()
}

pub fn find_pattern_nodes(patch: &IrPatch) -> Vec<&IrNode> {
    patch.nodes.iter().filter(|n| matches!(n.kind, IrNodeKind::Pattern(_))).collect()
}

// ---------------------------------------------------------------------------
// Core compiler
// ---------------------------------------------------------------------------

/// Compile an IrPatch into a live Ableton session.
pub fn compile(patch: &IrPatch, session: &Session) -> Result<CompileResult> {
    let mut result = CompileResult::default();

    // Set tempo
    session.set_tempo(patch.bpm as f32)?;

    let source_nodes = find_source_nodes(patch);
    let pattern_nodes = find_pattern_nodes(patch);

    // Create a track for each source
    let mut track_map: HashMap<i32, i32> = HashMap::new();

    for snode in &source_nodes {
        if let IrNodeKind::Source(ref source) = snode.kind {
            if source.is_live_input() {
                let track = session.create_audio_track(-1)?;
                track.set_name(&snode.label)?;
            } else {
                let track = session.create_midi_track(-1)?;
                track.set_name(&snode.label)?;
                let track_idx = track.track_idx;

                match source {
                    IrSourceKind::Sample { path, .. } => {
                        match track.load_sample(path) {
                            Ok(_) => {}
                            Err(e) => result.errors.push(format!("Failed to load sample {path}: {e}")),
                        }
                    }
                    IrSourceKind::Synth { preset, params, .. } => {
                        match session.load_instrument(track_idx, preset) {
                            Ok(_) => {
                                thread::sleep(Duration::from_millis(200));
                                // Apply synth parameters
                                if !params.is_empty() {
                                    let track = session.track(track_idx);
                                    let device = track.device(0);
                                    for (key, value) in params {
                                        // Keys that parse as integers → set by index
                                        // Otherwise → set by name
                                        if let Ok(idx) = key.parse::<i32>() {
                                            let _ = device.set_param(idx, *value as f32);
                                        } else {
                                            let _ = device.set_param_by_name(key, *value as f32);
                                        }
                                    }
                                }
                            }
                            Err(e) => result.errors.push(format!("Failed to load instrument {preset}: {e}")),
                        }
                    }
                    _ => {}
                }
            }

            result.tracks_created.push(snode.label.clone());
            let n = session.num_tracks()?;
            track_map.insert(snode.id, n - 1);
        }
    }

    result.track_map = track_map.clone();

    // Load effects onto each source's track
    for snode in &source_nodes {
        let track_idx = match track_map.get(&snode.id) {
            Some(&idx) => idx,
            None => continue,
        };

        let effects_chain = find_effect_chain(patch, snode.id);
        for enode in effects_chain {
            match &enode.kind {
                IrNodeKind::Effect(eff) => {
                    load_effect(session, track_idx, eff, &mut result);
                }
                IrNodeKind::Chain(effs) => {
                    for eff in effs {
                        load_effect(session, track_idx, eff, &mut result);
                    }
                }
                _ => {}
            }
        }
    }

    // Create clips from pattern nodes
    if !pattern_nodes.is_empty() && !source_nodes.is_empty() {
        for (i, pnode) in pattern_nodes.iter().enumerate() {
            if let IrNodeKind::Pattern(ref clip_data) = pnode.kind {
                let source_idx = i % source_nodes.len();
                if let Some(&track_idx) = track_map.get(&source_nodes[source_idx].id) {
                    create_clip(session, track_idx, 0, &pnode.label, clip_data, &mut result)?;
                }
            }
        }
    }

    // Apply space (pan) to first track
    if !source_nodes.is_empty() {
        if let Some(&first_track_idx) = track_map.get(&source_nodes[0].id) {
            if let Some(bp) = patch.space.pan.breakpoints.first() {
                let track = session.track(first_track_idx);
                track.set_panning(bp.value as f32)?;
            }
        }
    }

    Ok(result)
}

fn load_effect(session: &Session, track_idx: i32, eff: &IrEffect, result: &mut CompileResult) {
    let ableton_name = effect_name(&eff.name);
    match session.load_effect(track_idx, ableton_name) {
        Ok(_) => {
            result.effects_loaded.push(eff.name.clone());
            thread::sleep(Duration::from_millis(200));

            // Set static parameters
            let track = session.track(track_idx);
            if let Ok(devices) = track.devices() {
                if let Some(device) = devices.last() {
                    for param in &eff.params {
                        let _ = device.set_param_by_name(&param.name, param.value as f32);
                    }
                }
            }
        }
        Err(e) => {
            result.errors.push(format!("Failed to load effect {} (as {}): {}", eff.name, ableton_name, e));
        }
    }
}

fn create_clip(
    session: &Session,
    track_idx: i32,
    slot_idx: i32,
    label: &str,
    clip_data: &IrClip,
    result: &mut CompileResult,
) -> Result<()> {
    if clip_data.events.is_empty() {
        return Ok(());
    }

    let track = session.track(track_idx);
    let clip = track.create_clip(slot_idx, clip_data.length as f32)?;
    clip.set_name(label)?;
    clip.set_looping(true)?;

    let notes: Vec<Note> = clip_data.events.iter().map(|e| {
        let pitch = (e.value.round() as i32).clamp(0, 127);
        Note::new(pitch, e.start as f32, e.duration as f32, 100)
    }).collect();

    if !notes.is_empty() {
        clip.add_notes(&notes)?;
    }

    result.clips_created += 1;
    Ok(())
}
