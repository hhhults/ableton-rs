use crate::error::{Error, Result};
use crate::osc::{Arg, OscClient};

/// A MIDI note. Includes expressive fields (probability, velocity deviation,
/// release velocity) that Live 11 surfaces per note; defaults keep older
/// callers working unchanged.
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub pitch: i32,
    pub start: f32,
    pub duration: f32,
    pub velocity: i32,
    pub mute: bool,
    /// 0.0..1.0 — likelihood the note fires each cycle (default 1.0 = always).
    pub probability: f32,
    /// -127..127 — per-note velocity jitter applied by Live (default 0.0).
    pub velocity_deviation: f32,
    /// 0..127 — release velocity (default 64).
    pub release_velocity: f32,
}

impl Note {
    pub fn new(pitch: i32, start: f32, duration: f32, velocity: i32) -> Self {
        Self {
            pitch,
            start,
            duration,
            velocity,
            mute: false,
            probability: 1.0,
            velocity_deviation: 0.0,
            release_velocity: 64.0,
        }
    }

    pub fn with_probability(mut self, p: f32) -> Self {
        self.probability = p;
        self
    }

    pub fn with_velocity_deviation(mut self, v: f32) -> Self {
        self.velocity_deviation = v;
        self
    }

    pub fn with_release_velocity(mut self, v: f32) -> Self {
        self.release_velocity = v;
        self
    }
}

/// A note returned from `get_notes_ext`, carrying the Live-assigned `note_id`
/// required to modify it in place via `apply_note_mods`.
#[derive(Debug, Clone, PartialEq)]
pub struct NoteRef {
    pub note_id: i32,
    pub note: Note,
}

/// A clip in a Session View slot.
#[derive(Clone)]
pub struct Clip {
    pub(crate) osc: OscClient,
    pub track_idx: i32,
    pub clip_idx: i32,
}

impl Clip {
    pub(crate) fn new(osc: OscClient, track_idx: i32, clip_idx: i32) -> Self {
        Self { osc, track_idx, clip_idx }
    }

    fn prefix(&self) -> [Arg; 2] {
        [Arg::Int(self.track_idx), Arg::Int(self.clip_idx)]
    }

    fn send(&self, addr: &str, extra: &[Arg]) -> Result<()> {
        let mut args = self.prefix().to_vec();
        args.extend_from_slice(extra);
        self.osc.send(addr, &args)
    }

    fn query(&self, addr: &str, extra: &[Arg]) -> Result<Vec<Arg>> {
        let mut args = self.prefix().to_vec();
        args.extend_from_slice(extra);
        self.osc.query(addr, &args)
    }

    // -- Notes --

    /// Add MIDI notes to the clip. Sends the 8-field extended payload so
    /// expressive fields (probability, velocity deviation, release velocity)
    /// are written in a single round trip.
    pub fn add_notes(&self, notes: &[Note]) -> Result<()> {
        let mut flat = Vec::with_capacity(notes.len() * 8);
        for n in notes {
            flat.push(Arg::Int(n.pitch));
            flat.push(Arg::Float(n.start));
            flat.push(Arg::Float(n.duration));
            flat.push(Arg::Float(n.velocity as f32));
            flat.push(Arg::Int(n.mute as i32));
            flat.push(Arg::Float(n.probability));
            flat.push(Arg::Float(n.velocity_deviation));
            flat.push(Arg::Float(n.release_velocity));
        }
        self.send("/live/clip/add/notes_ext", &flat)
    }

    /// Get all MIDI notes (basic fields only — faster).
    pub fn get_notes(&self) -> Result<Vec<Note>> {
        let resp = self.query("/live/clip/get/notes", &[])?;
        let data = if resp.len() > 2 { &resp[2..] } else { &resp };
        let mut notes = Vec::new();
        let mut i = 0;
        while i + 4 < data.len() {
            notes.push(Note {
                pitch: data[i].as_i32().unwrap_or(0),
                start: data[i + 1].as_f32().unwrap_or(0.0),
                duration: data[i + 2].as_f32().unwrap_or(0.0),
                velocity: data[i + 3].as_i32().unwrap_or(100),
                mute: data[i + 4].as_i32().unwrap_or(0) != 0,
                probability: 1.0,
                velocity_deviation: 0.0,
                release_velocity: 64.0,
            });
            i += 5;
        }
        Ok(notes)
    }

    /// Get all MIDI notes with note IDs and full expressive fields.
    /// Use the returned IDs with `apply_note_mods` to modify in place.
    pub fn get_notes_ext(&self) -> Result<Vec<NoteRef>> {
        let resp = self.query("/live/clip/get/notes_ext", &[])?;
        let data = if resp.len() > 2 { &resp[2..] } else { &resp };
        let mut out = Vec::new();
        let mut i = 0;
        while i + 8 < data.len() {
            out.push(NoteRef {
                note_id: data[i].as_i32().unwrap_or(0),
                note: Note {
                    pitch: data[i + 1].as_i32().unwrap_or(0),
                    start: data[i + 2].as_f32().unwrap_or(0.0),
                    duration: data[i + 3].as_f32().unwrap_or(0.0),
                    velocity: data[i + 4].as_f32().unwrap_or(100.0) as i32,
                    mute: data[i + 5].as_bool().unwrap_or(false),
                    probability: data[i + 6].as_f32().unwrap_or(1.0),
                    velocity_deviation: data[i + 7].as_f32().unwrap_or(0.0),
                    release_velocity: data[i + 8].as_f32().unwrap_or(64.0),
                },
            });
            i += 9;
        }
        Ok(out)
    }

    /// Apply modifications to notes referenced by note_id. The full 9-field
    /// payload is sent per note; unchanged fields are simply re-set to their
    /// current values.
    pub fn apply_note_mods(&self, refs: &[NoteRef]) -> Result<()> {
        let mut flat = Vec::with_capacity(refs.len() * 9);
        for r in refs {
            flat.push(Arg::Int(r.note_id));
            flat.push(Arg::Int(r.note.pitch));
            flat.push(Arg::Float(r.note.start));
            flat.push(Arg::Float(r.note.duration));
            flat.push(Arg::Float(r.note.velocity as f32));
            flat.push(Arg::Int(r.note.mute as i32));
            flat.push(Arg::Float(r.note.probability));
            flat.push(Arg::Float(r.note.velocity_deviation));
            flat.push(Arg::Float(r.note.release_velocity));
        }
        self.send("/live/clip/apply_note_mods", &flat)
    }

    /// Remove all notes.
    pub fn clear_notes(&self) -> Result<()> {
        self.send("/live/clip/remove/notes", &[])
    }

    // -- Automation --

    /// Write automation steps: `(start_time, duration, value)`.
    pub fn automate(
        &self,
        device_idx: i32,
        param_idx: i32,
        steps: &[(f32, f32, f32)],
    ) -> Result<()> {
        const BATCH: usize = 60;
        for chunk in steps.chunks(BATCH) {
            let mut flat = vec![
                Arg::Int(self.track_idx),
                Arg::Int(self.clip_idx),
                Arg::Int(device_idx),
                Arg::Int(param_idx),
            ];
            for &(start, dur, val) in chunk {
                flat.push(Arg::Float(start));
                flat.push(Arg::Float(dur));
                flat.push(Arg::Float(val));
            }
            self.osc.send("/live/clip/insert_automation_steps", &flat)?;
        }
        Ok(())
    }

    /// Smooth automation by linearly interpolating between `(time, value)` points.
    pub fn automate_smooth(
        &self,
        device_idx: i32,
        param_idx: i32,
        points: &[(f32, f32)],
        resolution: f32,
    ) -> Result<()> {
        if points.len() < 2 {
            return Ok(());
        }
        let mut steps = Vec::new();
        for w in points.windows(2) {
            let (t0, v0) = w[0];
            let (t1, v1) = w[1];
            let mut t = t0;
            while t < t1 {
                let frac = if (t1 - t0).abs() > f32::EPSILON {
                    (t - t0) / (t1 - t0)
                } else {
                    0.0
                };
                let val = v0 + frac * (v1 - v0);
                let dur = (t1 - t).min(resolution);
                steps.push((t, dur, val));
                t += resolution;
            }
        }
        self.automate(device_idx, param_idx, &steps)
    }

    /// Clear automation for one parameter.
    pub fn clear_automation(&self, device_idx: i32, param_idx: i32) -> Result<()> {
        self.osc.send(
            "/live/clip/clear_automation",
            &[
                Arg::Int(self.track_idx),
                Arg::Int(self.clip_idx),
                Arg::Int(device_idx),
                Arg::Int(param_idx),
            ],
        )
    }

    /// Clear all automation on this clip.
    pub fn clear_all_automation(&self) -> Result<()> {
        self.send("/live/clip/clear_all_automation", &[])
    }

    /// Insert a single automation step. Value is normalized 0.0-1.0.
    pub fn insert_automation_step(
        &self,
        device_idx: i32,
        param_idx: i32,
        time: f32,
        duration: f32,
        value: f32,
    ) -> Result<()> {
        self.osc.send(
            "/live/clip/insert_automation_step",
            &[
                Arg::Int(self.track_idx),
                Arg::Int(self.clip_idx),
                Arg::Int(device_idx),
                Arg::Int(param_idx),
                Arg::Float(time),
                Arg::Float(duration),
                Arg::Float(value),
            ],
        )
    }

    /// Read the automation value at a given time. Returns normalized 0.0-1.0.
    pub fn automation_value_at(
        &self,
        device_idx: i32,
        param_idx: i32,
        time: f32,
    ) -> Result<f32> {
        let resp = self.osc.query(
            "/live/clip/get_automation_value",
            &[
                Arg::Int(self.track_idx),
                Arg::Int(self.clip_idx),
                Arg::Int(device_idx),
                Arg::Int(param_idx),
                Arg::Float(time),
            ],
        )?;
        resp.into_iter()
            .filter_map(|a| a.as_f32())
            .last()
            .ok_or_else(|| Error::Ableton("no response from get_automation_value".into()))
    }

    // -- Transport --

    pub fn fire(&self) -> Result<()> {
        self.send("/live/clip/fire", &[])
    }

    pub fn stop(&self) -> Result<()> {
        self.send("/live/clip/stop", &[])
    }

    // -- Properties --

    fn get_prop(&self, prop: &str) -> Result<Arg> {
        let resp = self.query(&format!("/live/clip/get/{prop}"), &[])?;
        resp.into_iter().nth(2).ok_or_else(|| Error::BadResponse {
            address: format!("/live/clip/get/{prop}"),
            expected: 3,
            got: 0,
        })
    }

    fn set_prop(&self, prop: &str, value: Arg) -> Result<()> {
        self.send(&format!("/live/clip/set/{prop}"), &[value])
    }

    pub fn get_name(&self) -> Result<String> {
        self.get_prop("name").map(|a| a.as_str().unwrap_or("").to_string())
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        self.set_prop("name", Arg::from(name))
    }

    pub fn get_length(&self) -> Result<f32> {
        self.get_prop("length").map(|a| a.as_f32().unwrap_or(4.0))
    }

    pub fn get_looping(&self) -> Result<bool> {
        self.get_prop("looping").map(|a| a.as_bool().unwrap_or(false))
    }

    pub fn set_looping(&self, on: bool) -> Result<()> {
        self.set_prop("looping", Arg::from(on))
    }

    pub fn get_loop_start(&self) -> Result<f32> {
        self.get_prop("loop_start").map(|a| a.as_f32().unwrap_or(0.0))
    }

    pub fn set_loop_start(&self, beats: f32) -> Result<()> {
        self.set_prop("loop_start", Arg::Float(beats))
    }

    pub fn get_loop_end(&self) -> Result<f32> {
        self.get_prop("loop_end").map(|a| a.as_f32().unwrap_or(4.0))
    }

    pub fn set_loop_end(&self, beats: f32) -> Result<()> {
        self.set_prop("loop_end", Arg::Float(beats))
    }

    pub fn get_color(&self) -> Result<i32> {
        self.get_prop("color").map(|a| a.as_i32().unwrap_or(0))
    }

    pub fn set_color(&self, color: i32) -> Result<()> {
        self.set_prop("color", Arg::Int(color))
    }

    pub fn get_muted(&self) -> Result<bool> {
        self.get_prop("muted").map(|a| a.as_bool().unwrap_or(false))
    }

    pub fn set_muted(&self, muted: bool) -> Result<()> {
        self.set_prop("muted", Arg::from(muted))
    }

    pub fn get_gain(&self) -> Result<f32> {
        self.get_prop("gain").map(|a| a.as_f32().unwrap_or(1.0))
    }

    pub fn set_gain(&self, gain: f32) -> Result<()> {
        self.set_prop("gain", Arg::Float(gain))
    }

    pub fn get_pitch_coarse(&self) -> Result<i32> {
        self.get_prop("pitch_coarse").map(|a| a.as_i32().unwrap_or(0))
    }

    pub fn set_pitch_coarse(&self, semitones: i32) -> Result<()> {
        self.set_prop("pitch_coarse", Arg::Int(semitones))
    }

    pub fn get_warp_mode(&self) -> Result<i32> {
        self.get_prop("warp_mode").map(|a| a.as_i32().unwrap_or(0))
    }

    pub fn set_warp_mode(&self, mode: i32) -> Result<()> {
        self.set_prop("warp_mode", Arg::Int(mode))
    }

    pub fn get_warping(&self) -> Result<bool> {
        self.get_prop("warping").map(|a| a.as_bool().unwrap_or(false))
    }

    pub fn set_warping(&self, on: bool) -> Result<()> {
        self.set_prop("warping", Arg::from(on))
    }
}
