/// EMERGENCE v2 — a hand-synthesized song.
///
/// Every sound built from raw oscillators. No samples. No presets.
/// Intricate polyrhythmic patterns. Lush voicings. Subverted expectations.
///
/// 135 BPM, ~3 minutes, 7 tracks.

use ableton::{Note, Session};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn ms(millis: u64) { thread::sleep(Duration::from_millis(millis)); }

fn add_notes_batched(clip: &ableton::Clip, notes: &[Note]) -> Result<(), Box<dyn std::error::Error>> {
    for chunk in notes.chunks(64) {
        clip.add_notes(chunk)?;
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

struct Op {
    dev: ableton::Device,
    map: HashMap<String, usize>,
}

impl Op {
    fn new(dev: ableton::Device) -> Result<Self, Box<dyn std::error::Error>> {
        let params = dev.parameters()?;
        let map = params.iter().map(|p| (p.name.clone(), p.index)).collect();
        Ok(Self { dev, map })
    }
    fn set(&self, name: &str, val: f32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(&idx) = self.map.get(name) {
            self.dev.set_param(idx as i32, val)?;
        }
        Ok(())
    }
    fn idx(&self, name: &str) -> Option<i32> {
        self.map.get(name).map(|&i| i as i32)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::connect()?;
    println!("connected to Ableton {:?}\n", session.version()?);

    let n = session.num_tracks()?;
    for i in (0..n).rev() {
        session.delete_track(i)?;
        ms(100);
    }
    ms(300);

    session.set_tempo(135.0)?;
    let total_beats: f32 = 256.0;

    // ================================================================
    // TRACK 1: BASS — Liquid Metal (Operator)
    // ================================================================
    println!("building track 1: BASS (liquid metal)...");
    let bass_track = session.create_midi_track(-1)?;
    bass_track.set_name("bass")?;
    let bass_idx = bass_track.track_idx;

    session.load_instrument(bass_idx, "Operator")?;
    ms(500);

    let bass_op = Op::new(bass_track.device(0))?;
    bass_op.set("Algorithm", 0.1)?;
    bass_op.set("Osc-A Level", 1.0)?;
    bass_op.set("Osc-A Wave", 0.0)?;
    bass_op.set("A Coarse", 0.5)?;
    bass_op.set("Osc-B On", 1.0)?;
    bass_op.set("Osc-B Level", 0.55)?;
    bass_op.set("Osc-B Wave", 0.0)?;
    bass_op.set("B Coarse", 0.68)?;
    bass_op.set("Osc-C On", 1.0)?;
    bass_op.set("Osc-C Level", 0.4)?;
    bass_op.set("Osc-C Wave", 0.0)?;
    bass_op.set("C Coarse", 0.33)?;
    bass_op.set("Osc-D On", 1.0)?;
    bass_op.set("Osc-D Level", 0.5)?;
    bass_op.set("Osc-D Wave", 0.0)?;
    bass_op.set("D Coarse", 0.82)?;
    bass_op.set("Osc-D Feedback", 0.5)?;
    bass_op.set("Filter On", 1.0)?;
    bass_op.set("Filter Freq", 0.45)?;
    bass_op.set("Filter Res", 0.5)?;
    bass_op.set("Ae Attack", 0.05)?;
    bass_op.set("Ae Decay", 0.5)?;
    bass_op.set("Ae Sustain", 0.45)?;
    bass_op.set("Ae Release", 0.35)?;
    bass_op.set("Pe Init", 0.58)?;
    bass_op.set("Pe Peak", 0.53)?;
    bass_op.set("Pe Time", 0.15)?;

    let bass_clip = bass_track.create_clip(0, total_beats)?;
    bass_clip.set_name("bass")?;
    bass_clip.set_looping(false)?;

    let mut bass_notes = Vec::new();

    // === BASS TEASE (bar 13, beat 48) ===
    // One note appears. Listener expects bass to enter. It withdraws.
    bass_notes.push(Note::new(30, 48.0, 1.5, 80));
    bass_notes.push(Note::new(42, 49.75, 0.25, 55)); // octave hint
    // ...then silence until bar 17. The ghost of what's coming.

    // === PULSE (bars 17-32, beats 64-127) ===
    // Four different 4-bar phrases — no exact repetition

    // Phrase A (bars 17-20): syncopated, breathing
    // Anticipation notes land BEFORE the downbeat
    bass_notes.push(Note::new(30, 63.75, 2.25, 115));  // anticipates bar 17!
    bass_notes.push(Note::new(30, 66.5, 0.25, 70));    // ghost
    bass_notes.push(Note::new(42, 67.0, 0.5, 95));     // octave answer
    bass_notes.push(Note::new(30, 68.0, 1.5, 110));
    bass_notes.push(Note::new(30, 70.0, 0.25, 55));    // ghost triplet
    bass_notes.push(Note::new(30, 70.33, 0.25, 50));
    bass_notes.push(Note::new(30, 70.67, 0.25, 45));
    bass_notes.push(Note::new(30, 71.75, 2.25, 120));  // anticipates bar 19
    bass_notes.push(Note::new(35, 74.5, 0.5, 85));     // Eb answer
    bass_notes.push(Note::new(30, 75.5, 0.25, 60));    // ghost
    bass_notes.push(Note::new(30, 76.0, 1.5, 110));
    bass_notes.push(Note::new(42, 78.0, 0.25, 75));
    bass_notes.push(Note::new(30, 79.0, 0.5, 100));

    // Phrase B (bars 21-24): more driving, 16th note subdivisions
    bass_notes.push(Note::new(30, 80.0, 1.0, 120));
    bass_notes.push(Note::new(30, 81.25, 0.25, 65));   // offbeat ghost
    bass_notes.push(Note::new(30, 81.75, 0.25, 70));
    bass_notes.push(Note::new(42, 82.0, 1.0, 100));    // octave
    bass_notes.push(Note::new(30, 83.5, 0.5, 90));
    bass_notes.push(Note::new(30, 84.0, 0.75, 115));
    bass_notes.push(Note::new(30, 85.0, 0.25, 55));    // ghost
    bass_notes.push(Note::new(30, 85.5, 0.25, 50));
    bass_notes.push(Note::new(35, 86.0, 1.0, 105));    // Eb
    bass_notes.push(Note::new(30, 87.25, 0.75, 95));
    bass_notes.push(Note::new(30, 88.0, 1.5, 120));
    bass_notes.push(Note::new(42, 89.75, 0.25, 80));
    bass_notes.push(Note::new(30, 90.0, 0.5, 100));
    bass_notes.push(Note::new(30, 91.0, 0.25, 55));
    bass_notes.push(Note::new(30, 91.5, 0.25, 50));
    bass_notes.push(Note::new(30, 92.0, 1.0, 115));
    bass_notes.push(Note::new(35, 93.25, 0.75, 90));
    bass_notes.push(Note::new(30, 94.0, 1.0, 110));
    bass_notes.push(Note::new(30, 95.5, 0.25, 65));

    // === FALSE BREATHE (bars 25-26, beats 96-103) ===
    // Everything strips back. Just bass holding a long tone. Is it over?
    bass_notes.push(Note::new(30, 95.75, 7.0, 75));    // loooong sustained note

    // === SURGE BACK (bars 27-28, beats 104-111) ===
    // No — comes back harder than before
    bass_notes.push(Note::new(30, 103.75, 0.25, 125)); // ANTICIPATION — smashes back
    bass_notes.push(Note::new(30, 104.0, 1.0, 127));
    bass_notes.push(Note::new(42, 105.0, 0.5, 100));
    bass_notes.push(Note::new(30, 105.75, 0.25, 70));
    bass_notes.push(Note::new(30, 106.0, 0.75, 115));
    bass_notes.push(Note::new(30, 107.0, 0.25, 55));
    bass_notes.push(Note::new(30, 107.33, 0.25, 50));  // triplet
    bass_notes.push(Note::new(30, 107.67, 0.25, 60));
    bass_notes.push(Note::new(30, 108.0, 1.5, 120));
    bass_notes.push(Note::new(35, 109.75, 0.25, 85));
    bass_notes.push(Note::new(42, 110.0, 0.5, 95));
    bass_notes.push(Note::new(30, 111.0, 0.5, 105));

    // Phrase D (bars 29-32): building toward Breathe, accelerating subdivisions
    bass_notes.push(Note::new(30, 112.0, 1.0, 115));
    bass_notes.push(Note::new(30, 113.25, 0.5, 80));
    bass_notes.push(Note::new(42, 114.0, 0.5, 95));
    bass_notes.push(Note::new(30, 115.0, 0.25, 60));
    bass_notes.push(Note::new(30, 115.25, 0.25, 65));
    bass_notes.push(Note::new(30, 115.5, 0.25, 70));   // accelerating
    bass_notes.push(Note::new(30, 115.75, 0.25, 75));
    bass_notes.push(Note::new(30, 116.0, 1.5, 120));
    bass_notes.push(Note::new(35, 117.75, 0.25, 85));
    bass_notes.push(Note::new(30, 118.0, 0.5, 100));
    bass_notes.push(Note::new(30, 119.0, 0.25, 55));
    bass_notes.push(Note::new(30, 119.25, 0.25, 60));
    bass_notes.push(Note::new(30, 119.5, 0.25, 70));
    bass_notes.push(Note::new(30, 119.75, 0.25, 80));  // 16ths into breathe
    bass_notes.push(Note::new(30, 120.0, 2.0, 115));
    bass_notes.push(Note::new(42, 122.5, 0.5, 90));
    bass_notes.push(Note::new(30, 123.0, 0.5, 100));
    bass_notes.push(Note::new(30, 124.0, 1.0, 110));
    bass_notes.push(Note::new(30, 125.5, 0.5, 75));
    bass_notes.push(Note::new(30, 126.5, 0.25, 55));
    bass_notes.push(Note::new(30, 126.75, 0.25, 50));  // fading into breathe...
    bass_notes.push(Note::new(30, 127.0, 1.0, 40));    // last breath

    // Breathe (bars 33-40): bass is SILENT. The absence is the point.

    // === PRE-ERUPTION: beat 160-161.5 is SILENCE ===
    // Everything built up at end of Breathe... and then NOTHING.
    // 1.5 beats of held breath.

    // === ERUPTION (bars 41-56, beats 160-223) ===
    // Bass enters LATE — beat 161.5, not 160. Subverted expectation.
    bass_notes.push(Note::new(30, 161.5, 0.5, 127));   // SMASH — 1.5 beats late

    // Eruption phrase A (bars 41-44): heavy, new material
    bass_notes.push(Note::new(30, 162.0, 0.75, 120));
    bass_notes.push(Note::new(30, 162.75, 0.25, 60));  // ghost
    bass_notes.push(Note::new(42, 163.0, 0.5, 105));
    bass_notes.push(Note::new(30, 163.75, 0.25, 70));
    bass_notes.push(Note::new(30, 164.0, 1.0, 125));
    bass_notes.push(Note::new(30, 165.25, 0.25, 55));
    bass_notes.push(Note::new(30, 165.5, 0.25, 60));
    bass_notes.push(Note::new(30, 166.0, 0.5, 110));
    bass_notes.push(Note::new(42, 166.75, 0.25, 85));
    bass_notes.push(Note::new(30, 167.0, 0.5, 115));
    bass_notes.push(Note::new(30, 168.0, 1.5, 120));
    bass_notes.push(Note::new(35, 169.75, 0.25, 80));
    bass_notes.push(Note::new(30, 170.0, 0.75, 115));
    bass_notes.push(Note::new(30, 170.75, 0.25, 55));
    bass_notes.push(Note::new(42, 171.0, 0.5, 100));
    bass_notes.push(Note::new(30, 172.0, 1.0, 120));
    bass_notes.push(Note::new(30, 173.25, 0.25, 60));
    bass_notes.push(Note::new(30, 173.5, 0.25, 65));
    bass_notes.push(Note::new(30, 174.0, 1.0, 115));
    bass_notes.push(Note::new(35, 175.0, 0.5, 90));
    bass_notes.push(Note::new(30, 175.5, 0.25, 70));

    // Eruption phrase B (bars 45-47): intensifying
    bass_notes.push(Note::new(30, 176.0, 0.75, 125));
    bass_notes.push(Note::new(42, 176.75, 0.25, 90));
    bass_notes.push(Note::new(30, 177.0, 0.5, 110));
    bass_notes.push(Note::new(30, 177.75, 0.25, 65));
    bass_notes.push(Note::new(30, 178.0, 0.5, 120));
    bass_notes.push(Note::new(30, 178.5, 0.25, 55));
    bass_notes.push(Note::new(30, 178.75, 0.25, 60));
    bass_notes.push(Note::new(42, 179.0, 0.5, 100));
    bass_notes.push(Note::new(30, 179.75, 0.25, 70));
    bass_notes.push(Note::new(30, 180.0, 1.0, 125));
    bass_notes.push(Note::new(30, 181.25, 0.25, 55));
    bass_notes.push(Note::new(30, 181.5, 0.25, 60));
    bass_notes.push(Note::new(35, 182.0, 0.5, 105));
    bass_notes.push(Note::new(30, 182.75, 0.25, 70));
    bass_notes.push(Note::new(30, 183.0, 0.5, 115));
    bass_notes.push(Note::new(42, 183.5, 0.25, 85));
    bass_notes.push(Note::new(30, 184.0, 1.0, 120));
    bass_notes.push(Note::new(30, 185.25, 0.25, 60));
    bass_notes.push(Note::new(30, 185.5, 0.5, 110));
    bass_notes.push(Note::new(30, 186.0, 0.75, 125));
    bass_notes.push(Note::new(42, 186.75, 0.25, 90));
    bass_notes.push(Note::new(30, 187.0, 0.5, 100));
    bass_notes.push(Note::new(30, 187.5, 0.25, 55));

    // === FALSE ENDING (bar 48, beats 188-191): TOTAL SILENCE ===
    // Deep in eruption... everything cuts. Is it over?

    // === MAXIMUM INTENSITY (bars 49-54, beats 192-215) ===
    // NO. It comes back at peak intensity. Fastest, heaviest bass.
    bass_notes.push(Note::new(30, 192.0, 0.5, 127));
    bass_notes.push(Note::new(42, 192.5, 0.25, 95));
    bass_notes.push(Note::new(30, 192.75, 0.25, 70));
    bass_notes.push(Note::new(30, 193.0, 0.5, 120));
    bass_notes.push(Note::new(30, 193.5, 0.25, 55));
    bass_notes.push(Note::new(30, 193.75, 0.25, 60));
    bass_notes.push(Note::new(42, 194.0, 0.5, 105));
    bass_notes.push(Note::new(30, 194.5, 0.25, 65));
    bass_notes.push(Note::new(30, 195.0, 0.5, 125));
    bass_notes.push(Note::new(30, 195.75, 0.25, 70));
    // Continuous 8th-note drive
    for i in 0..16 {
        let t = 196.0 + i as f32 * 0.5;
        let v = if i % 4 == 0 { 120 } else if i % 2 == 0 { 95 } else { 60 };
        let p = if i % 8 == 4 { 42 } else { 30 };
        bass_notes.push(Note::new(p, t, 0.4, v));
    }
    // Peak phrase (bars 52-54)
    bass_notes.push(Note::new(30, 204.0, 0.75, 127));
    bass_notes.push(Note::new(42, 204.75, 0.25, 100));
    bass_notes.push(Note::new(30, 205.0, 0.5, 120));
    bass_notes.push(Note::new(30, 205.5, 0.25, 60));
    bass_notes.push(Note::new(30, 205.75, 0.25, 65));
    bass_notes.push(Note::new(30, 206.0, 0.5, 125));
    bass_notes.push(Note::new(35, 206.5, 0.5, 95));
    bass_notes.push(Note::new(30, 207.0, 0.5, 115));
    bass_notes.push(Note::new(42, 207.5, 0.25, 80));
    bass_notes.push(Note::new(30, 208.0, 1.0, 120));
    bass_notes.push(Note::new(30, 209.25, 0.5, 90));
    bass_notes.push(Note::new(42, 210.0, 0.5, 100));
    bass_notes.push(Note::new(30, 211.0, 0.75, 110));
    bass_notes.push(Note::new(30, 212.0, 1.5, 100));
    bass_notes.push(Note::new(30, 214.0, 2.0, 80));    // slowing...

    // Dissolve: bass is one of the LAST to leave (bar 59, beat 232)
    bass_notes.push(Note::new(30, 216.0, 3.0, 70));
    bass_notes.push(Note::new(30, 220.0, 4.0, 55));
    bass_notes.push(Note::new(30, 225.0, 5.0, 40));
    bass_notes.push(Note::new(30, 231.0, 3.0, 25));    // final whisper

    add_notes_batched(&bass_clip, &bass_notes)?;

    // Bass automation
    if let Some(b_idx) = bass_op.idx("Osc-B Level") {
        bass_clip.automate_smooth(0, b_idx, &[
            (0.0, 0.0),
            (48.0, 0.3),     // tease
            (50.0, 0.0),     // withdraws
            (64.0, 0.45),    // pulse entrance
            (96.0, 0.55),    // building
            (100.0, 0.25),   // false breathe — drops
            (104.0, 0.65),   // surge back — harder
            (120.0, 0.55),
            (128.0, 0.0),    // breathe — gone
            (160.0, 0.0),    // silence...
            (161.5, 0.7),    // ERUPTION — smash
            (188.0, 0.75),   // building...
            (188.5, 0.0),    // false ending — cut
            (192.0, 0.85),   // MAXIMUM — returns
            (204.0, 0.95),   // peak
            (216.0, 0.5),    // dissolve
            (232.0, 0.15),
            (256.0, 0.0),
        ], 0.5)?;
    }
    if let Some(f_idx) = bass_op.idx("Filter Freq") {
        bass_clip.automate_smooth(0, f_idx, &[
            (0.0, 0.2),
            (48.0, 0.35),
            (50.0, 0.2),
            (64.0, 0.3),
            (80.0, 0.5),
            (96.0, 0.55),
            (100.0, 0.25),   // false breathe
            (104.0, 0.65),   // surge — filter opens
            (120.0, 0.5),
            (128.0, 0.15),   // breathe
            (161.5, 0.4),    // eruption
            (176.0, 0.6),
            (188.0, 0.65),
            (188.5, 0.15),   // false ending
            (192.0, 0.5),    // returns
            (204.0, 0.85),   // PEAK — wide open
            (216.0, 0.35),
            (232.0, 0.15),
            (256.0, 0.1),
        ], 0.5)?;
    }
    if let Some(fb_idx) = bass_op.idx("Osc-D Feedback") {
        bass_clip.automate_smooth(0, fb_idx, &[
            (0.0, 0.2),
            (64.0, 0.35),
            (96.0, 0.45),
            (104.0, 0.55),
            (128.0, 0.2),
            (161.5, 0.5),
            (192.0, 0.7),
            (204.0, 0.9),    // peak chaos
            (216.0, 0.4),
            (256.0, 0.1),
        ], 0.5)?;
    }

    println!("  done");

    // ================================================================
    // TRACK 2: KICK (Analog)
    // ================================================================
    println!("building track 2: KICK (synthesized)...");
    let kick_track = session.create_midi_track(-1)?;
    kick_track.set_name("kick")?;
    let kick_idx = kick_track.track_idx;

    session.load_instrument(kick_idx, "Analog")?;
    ms(400);

    let kick_dev = kick_track.device(0);
    kick_dev.set_param(38, 1.0)?;   // OSC1 On
    kick_dev.set_param(39, 0.0)?;   // Sine
    kick_dev.set_param(40, -1.0)?;  // Octave down
    kick_dev.set_param(49, 0.75)?;  // PEG Amount
    kick_dev.set_param(43, 0.12)?;  // PEG Time
    kick_dev.set_param(105, 0.0)?;  // OSC2 Off
    kick_dev.set_param(34, 0.0)?;   // Noise Off
    kick_dev.set_param(53, 0.0)?;   // Filter Off
    kick_dev.set_param(89, 0.0)?;   // Attack = 0
    kick_dev.set_param(90, 0.28)?;  // Decay
    kick_dev.set_param(92, 0.0)?;   // Sustain = 0
    kick_dev.set_param(94, 0.12)?;  // Release

    let kick_clip = kick_track.create_clip(0, total_beats)?;
    kick_clip.set_name("kick")?;
    kick_clip.set_looping(false)?;

    let mut kick_notes = Vec::new();

    // === COALESCE (bars 9-16): kick emerges gradually ===
    // Bars 9-10: isolated, far apart
    kick_notes.push(Note::new(36, 32.0, 0.2, 75));
    kick_notes.push(Note::new(36, 36.0, 0.2, 80));
    // Bars 11-12: add ghost on the-and-of-2
    kick_notes.push(Note::new(36, 40.0, 0.2, 90));
    kick_notes.push(Note::new(36, 41.5, 0.15, 40));
    kick_notes.push(Note::new(36, 44.0, 0.2, 95));
    kick_notes.push(Note::new(36, 45.5, 0.15, 45));
    // Bars 13-14: more active, the pattern crystallizes
    kick_notes.push(Note::new(36, 48.0, 0.2, 100));
    kick_notes.push(Note::new(36, 49.5, 0.15, 50));
    kick_notes.push(Note::new(36, 50.0, 0.2, 85));
    kick_notes.push(Note::new(36, 52.0, 0.2, 105));
    kick_notes.push(Note::new(36, 53.25, 0.15, 45));
    kick_notes.push(Note::new(36, 54.0, 0.2, 90));
    kick_notes.push(Note::new(36, 55.5, 0.15, 50));
    // Bars 15-16: nearly full, with swing
    kick_notes.push(Note::new(36, 56.0, 0.2, 110));
    kick_notes.push(Note::new(36, 56.75, 0.15, 40));   // ghost
    kick_notes.push(Note::new(36, 57.5, 0.2, 80));
    kick_notes.push(Note::new(36, 58.0, 0.2, 100));
    kick_notes.push(Note::new(36, 59.5, 0.15, 45));
    kick_notes.push(Note::new(36, 60.0, 0.2, 115));
    kick_notes.push(Note::new(36, 60.75, 0.15, 40));
    kick_notes.push(Note::new(36, 62.0, 0.2, 95));
    kick_notes.push(Note::new(36, 63.0, 0.2, 105));
    kick_notes.push(Note::new(36, 63.5, 0.15, 50));

    // === PULSE (bars 17-32): NOT four-on-the-floor ===
    // Syncopated kick patterns — each 4-bar group different
    // Group A (bars 17-20): displaced groove
    for bar in 0..4 {
        let b = 64.0 + bar as f32 * 4.0;
        kick_notes.push(Note::new(36, b, 0.2, 120));           // 1
        kick_notes.push(Note::new(36, b + 0.75, 0.15, 45));    // ghost
        kick_notes.push(Note::new(36, b + 1.5, 0.2, 90));      // and-of-2
        kick_notes.push(Note::new(36, b + 2.5, 0.15, 50));     // ghost
        if bar % 2 == 1 {
            kick_notes.push(Note::new(36, b + 3.25, 0.15, 55)); // ghost fill
        }
    }
    // Group B (bars 21-24): heavier, dotted rhythm
    for bar in 0..4 {
        let b = 80.0 + bar as f32 * 4.0;
        kick_notes.push(Note::new(36, b, 0.2, 125));
        kick_notes.push(Note::new(36, b + 1.0, 0.15, 50));     // ghost on 2
        kick_notes.push(Note::new(36, b + 1.75, 0.2, 95));     // dotted
        kick_notes.push(Note::new(36, b + 2.75, 0.15, 45));    // ghost
        kick_notes.push(Note::new(36, b + 3.0, 0.2, 110));     // 4
    }

    // === FALSE BREATHE (bars 25-26, beats 96-103) ===
    // Just a lonely kick on 1. Everything else drops.
    kick_notes.push(Note::new(36, 96.0, 0.2, 80));
    kick_notes.push(Note::new(36, 100.0, 0.2, 75));

    // === SURGE BACK (bars 27-28): returns harder ===
    for bar in 0..2 {
        let b = 104.0 + bar as f32 * 4.0;
        kick_notes.push(Note::new(36, b, 0.2, 127));
        kick_notes.push(Note::new(36, b + 0.5, 0.15, 55));
        kick_notes.push(Note::new(36, b + 1.0, 0.2, 100));
        kick_notes.push(Note::new(36, b + 1.75, 0.15, 50));
        kick_notes.push(Note::new(36, b + 2.0, 0.2, 115));
        kick_notes.push(Note::new(36, b + 2.75, 0.15, 45));
        kick_notes.push(Note::new(36, b + 3.0, 0.2, 105));
        kick_notes.push(Note::new(36, b + 3.5, 0.15, 55));
    }

    // Group D (bars 29-32): building to Breathe with accelerating ghosts
    for bar in 0..4 {
        let b = 112.0 + bar as f32 * 4.0;
        kick_notes.push(Note::new(36, b, 0.2, 115));
        kick_notes.push(Note::new(36, b + 1.5, 0.2, 90));
        kick_notes.push(Note::new(36, b + 2.0, 0.2, 105));
        // Accelerating ghost notes — more per bar as we approach breathe
        let ghost_count = 1 + bar;
        for g in 0..ghost_count {
            let gt = b + 3.0 + g as f32 * (1.0 / ghost_count as f32);
            if gt < b + 4.0 {
                kick_notes.push(Note::new(36, gt, 0.12, 35 + g as i32 * 5));
            }
        }
    }

    // === BREATHE (bars 33-40): 3-against-4 polyrhythm ===
    // Kick every 3 beats instead of 4 — unsettling, floating feel
    let mut poly_t = 128.0;
    while poly_t < 152.0 {
        kick_notes.push(Note::new(36, poly_t, 0.2, 70));
        poly_t += 3.0;
    }
    // Bars 39-40: accelerating build — 8ths then 16ths
    for i in 0..8 {
        kick_notes.push(Note::new(36, 152.0 + i as f32 * 0.5, 0.15, 60 + i * 5));
    }
    for i in 0..8 {
        kick_notes.push(Note::new(36, 156.0 + i as f32 * 0.25, 0.12, 70 + i * 5));
    }
    // Bar 40 last beat: 32nd note ROLL leading to...
    for i in 0..8 {
        kick_notes.push(Note::new(36, 158.0 + i as f32 * 0.125, 0.1, 80 + i * 5));
    }
    // ...beat 159.875 = last 32nd. Then...

    // === SILENCE (beats 160-161.5): the held breath ===
    // NOTHING. After all that build. Dead air.

    // === ERUPTION (starts beat 161.5) ===
    // Explosive entrance, syncopated and heavy
    kick_notes.push(Note::new(36, 161.5, 0.2, 127));   // THE DROP — late!

    // Eruption Group A (bars 41-47): complex syncopated pattern
    for bar in 0..7 {
        let b = 162.0 + bar as f32 * 4.0;  // starts offset!
        // Core syncopated pattern
        kick_notes.push(Note::new(36, b, 0.2, 125));
        kick_notes.push(Note::new(36, b + 0.75, 0.15, 50));    // ghost
        kick_notes.push(Note::new(36, b + 1.25, 0.2, 100));    // offbeat
        kick_notes.push(Note::new(36, b + 2.0, 0.2, 120));
        kick_notes.push(Note::new(36, b + 2.5, 0.15, 45));     // ghost
        kick_notes.push(Note::new(36, b + 3.0, 0.2, 110));
        // Variation per bar
        if bar % 3 == 0 {
            kick_notes.push(Note::new(36, b + 3.5, 0.15, 55));
            kick_notes.push(Note::new(36, b + 3.75, 0.15, 60)); // double ghost
        }
        if bar % 2 == 1 {
            kick_notes.push(Note::new(36, b + 1.75, 0.12, 40)); // extra ghost
        }
    }

    // === FALSE ENDING (bar 48, beats 188-191): SILENCE ===
    // Everything stops for a full bar. Is the song over?

    // === MAXIMUM (bars 49-56, beats 192-223) ===
    // Comes back with double-time feel
    for bar in 0..8 {
        let b = 192.0 + bar as f32 * 4.0;
        // 8th note kicks with accent pattern
        for i in 0..8 {
            let t = b + i as f32 * 0.5;
            let v = match i {
                0 => 127, 4 => 115,            // strong beats
                2 | 6 => 90,                    // medium
                _ => 50 + (bar * 3) as i32,     // ghosts get louder over time
            };
            kick_notes.push(Note::new(36, t, 0.15, v));
        }
    }

    // === DISSOLVE (bars 57-64): decelerating ===
    // Kick thins out — gaps get longer
    kick_notes.push(Note::new(36, 224.0, 0.2, 100));
    kick_notes.push(Note::new(36, 225.5, 0.2, 85));
    kick_notes.push(Note::new(36, 228.0, 0.2, 75));
    kick_notes.push(Note::new(36, 231.0, 0.2, 60));    // last kick — bar 58
    // After bar 58: no more kick. Dissolve continues without it.

    add_notes_batched(&kick_clip, &kick_notes)?;
    println!("  done");

    // ================================================================
    // TRACK 3: SNARE/PERC (Analog)
    // ================================================================
    println!("building track 3: SNARE/PERC (polyrhythmic)...");
    let perc_track = session.create_midi_track(-1)?;
    perc_track.set_name("perc")?;
    let perc_idx = perc_track.track_idx;

    session.load_instrument(perc_idx, "Analog")?;
    ms(400);

    let perc_dev = perc_track.device(0);
    perc_dev.set_param(38, 1.0)?;
    perc_dev.set_param(39, 0.0)?;
    perc_dev.set_param(52, 0.3)?;
    perc_dev.set_param(34, 1.0)?;
    perc_dev.set_param(35, 0.55)?;
    perc_dev.set_param(37, 0.65)?;
    perc_dev.set_param(105, 0.0)?;
    perc_dev.set_param(53, 1.0)?;
    perc_dev.set_param(54, 0.0)?;
    perc_dev.set_param(57, 0.5)?;
    perc_dev.set_param(59, 0.45)?;
    perc_dev.set_param(62, 0.7)?;
    perc_dev.set_param(71, 0.12)?;
    perc_dev.set_param(73, 0.0)?;
    perc_dev.set_param(89, 0.0)?;
    perc_dev.set_param(90, 0.18)?;
    perc_dev.set_param(92, 0.0)?;
    perc_dev.set_param(94, 0.1)?;

    let perc_clip = perc_track.create_clip(0, total_beats)?;
    perc_clip.set_name("perc")?;
    perc_clip.set_looping(false)?;

    let mut perc_notes = Vec::new();

    // MIDI pitches: 60=snare, 72=hat, 66=rimshot/click (polyrhythm carrier)

    // === COALESCE (bars 13-16): isolated clicks hinting at rhythm ===
    perc_notes.push(Note::new(66, 49.5, 0.08, 55));
    perc_notes.push(Note::new(66, 52.0, 0.08, 50));
    perc_notes.push(Note::new(72, 54.5, 0.06, 40));
    perc_notes.push(Note::new(66, 55.5, 0.08, 55));
    perc_notes.push(Note::new(72, 57.0, 0.06, 45));
    perc_notes.push(Note::new(66, 58.5, 0.08, 60));
    perc_notes.push(Note::new(72, 60.0, 0.06, 50));
    perc_notes.push(Note::new(66, 61.5, 0.08, 55));
    perc_notes.push(Note::new(72, 62.5, 0.06, 50));
    perc_notes.push(Note::new(66, 63.0, 0.08, 60));

    // === PULSE (bars 17-32): layered polyrhythm ===
    for bar in 0..16 {
        let b = 64.0 + bar as f32 * 4.0;
        let bar_type = bar % 4;

        // SNARE: sometimes on 2, sometimes displaced to and-of-2
        if bar_type == 0 || bar_type == 2 {
            perc_notes.push(Note::new(60, b + 1.0, 0.15, 115));  // snare on 2
            perc_notes.push(Note::new(60, b + 3.0, 0.15, 120));  // snare on 4
        } else if bar_type == 1 {
            perc_notes.push(Note::new(60, b + 1.25, 0.15, 110)); // DISPLACED snare
            perc_notes.push(Note::new(60, b + 3.0, 0.15, 118));
        } else {
            // Flam (two hits close together)
            perc_notes.push(Note::new(60, b + 0.95, 0.08, 55));  // grace note
            perc_notes.push(Note::new(60, b + 1.0, 0.15, 120));  // main hit
            perc_notes.push(Note::new(60, b + 3.0, 0.15, 115));
            perc_notes.push(Note::new(60, b + 3.75, 0.1, 60));   // ghost
        }

        // HATS: different pattern per bar type
        match bar_type {
            0 => {
                // Straight 8ths
                for i in 0..8 {
                    let vel = if i % 2 == 0 { 75 } else { 55 };
                    perc_notes.push(Note::new(72, b + i as f32 * 0.5, 0.06, vel));
                }
            }
            1 => {
                // 16ths with accents creating 3-feel
                for i in 0..16 {
                    let vel = if i % 3 == 0 { 80 } else { 40 };
                    perc_notes.push(Note::new(72, b + i as f32 * 0.25, 0.04, vel));
                }
            }
            2 => {
                // Dotted 8ths (groups of 3 16ths) — creates lilt
                let mut t = b;
                while t < b + 4.0 {
                    perc_notes.push(Note::new(72, t, 0.06, 70));
                    t += 0.75;
                }
            }
            _ => {
                // Sparse — just offbeats, leaving space
                perc_notes.push(Note::new(72, b + 0.5, 0.06, 65));
                perc_notes.push(Note::new(72, b + 1.5, 0.06, 60));
                perc_notes.push(Note::new(72, b + 2.5, 0.06, 65));
                perc_notes.push(Note::new(72, b + 3.5, 0.06, 60));
            }
        }
    }

    // POLYRHYTHMIC CLICK: rimshot every 1.5 beats (3-against-4) through pulse
    {
        let mut t = 64.0;
        while t < 128.0 {
            perc_notes.push(Note::new(66, t, 0.08, 70));
            t += 1.5;
        }
    }

    // === FALSE BREATHE (bars 25-26): only the polyrhythmic click continues ===
    // (Already covered by the 1.5-beat loop above through beat 128)
    // Snare and hats are absent — but the click pattern persists.
    // This is the "skeleton" of the rhythm exposed.

    // === BREATHE (bars 33-40): the cross-rhythm alone ===
    // The 3-against-4 click becomes the ONLY rhythmic element (with kick's 3-beat poly)
    {
        let mut t = 128.0;
        while t < 152.0 {
            perc_notes.push(Note::new(66, t, 0.08, 60));
            t += 1.5;
        }
    }
    // Sparse hat whispers
    for bar in 0..8 {
        let b = 128.0 + bar as f32 * 4.0;
        perc_notes.push(Note::new(72, b + 1.0, 0.06, 35));
        if bar % 2 == 0 {
            perc_notes.push(Note::new(72, b + 3.0, 0.06, 30));
        }
    }

    // Build at end of breathe (bars 39-40): snare roll
    for i in 0..8 {
        perc_notes.push(Note::new(60, 156.0 + i as f32 * 0.25, 0.08, 50 + i * 8));
    }
    // 32nd note snare roll
    for i in 0..16 {
        perc_notes.push(Note::new(60, 158.0 + i as f32 * 0.125, 0.06, 60 + i * 3));
    }

    // === SILENCE (beats 160-161.5) ===

    // === ERUPTION (bars 41-56): full polyrhythmic fury ===
    for bar in 0..15 {
        // Skip bar 48 (false ending) — bar index 7 maps to beat 189.5 area
        // Actually bar 48 = beats 188-191 in absolute time
        // Our eruption starts at 162, so bar offset 6.5 = beat 188
        let b = 162.0 + bar as f32 * 4.0;
        if b >= 188.0 && b < 192.0 { continue; } // FALSE ENDING

        let bar_type = bar % 4;

        // Snare — displaced patterns
        match bar_type {
            0 => {
                perc_notes.push(Note::new(60, b + 1.0, 0.15, 125));
                perc_notes.push(Note::new(60, b + 3.0, 0.15, 127));
            }
            1 => {
                // Displaced
                perc_notes.push(Note::new(60, b + 1.25, 0.15, 120));
                perc_notes.push(Note::new(60, b + 2.75, 0.15, 115));
                perc_notes.push(Note::new(60, b + 3.5, 0.1, 65)); // ghost
            }
            2 => {
                // Double hit
                perc_notes.push(Note::new(60, b + 0.95, 0.06, 60));
                perc_notes.push(Note::new(60, b + 1.0, 0.15, 125));
                perc_notes.push(Note::new(60, b + 3.0, 0.15, 120));
                perc_notes.push(Note::new(60, b + 3.25, 0.08, 70));
            }
            _ => {
                // Fill bar
                perc_notes.push(Note::new(60, b + 1.0, 0.15, 120));
                perc_notes.push(Note::new(60, b + 2.5, 0.1, 90));
                perc_notes.push(Note::new(60, b + 3.0, 0.1, 95));
                perc_notes.push(Note::new(60, b + 3.25, 0.08, 100));
                perc_notes.push(Note::new(60, b + 3.5, 0.08, 105));
                perc_notes.push(Note::new(60, b + 3.75, 0.08, 110));
            }
        }

        // 16th note hats with varying accents
        for i in 0..16 {
            let t = b + i as f32 * 0.25;
            let vel = match (i + bar) % 5 {
                0 => 90, 1 => 50, 2 => 70, 3 => 45, _ => 60,
            };
            perc_notes.push(Note::new(72, t, 0.05, vel));
        }
    }

    // 3-against-4 click continues through eruption
    {
        let mut t = 162.0;
        while t < 188.0 {
            perc_notes.push(Note::new(66, t, 0.08, 80));
            t += 1.5;
        }
        // After false ending, click returns at higher intensity
        let mut t = 192.0;
        while t < 224.0 {
            perc_notes.push(Note::new(66, t, 0.08, 90));
            t += 1.5;
        }
    }

    // === DISSOLVE: perc drops at bar 58 ===
    perc_notes.push(Note::new(72, 224.0, 0.06, 50));
    perc_notes.push(Note::new(60, 225.0, 0.15, 70));
    perc_notes.push(Note::new(72, 226.5, 0.06, 40));
    perc_notes.push(Note::new(66, 228.0, 0.08, 50));
    perc_notes.push(Note::new(72, 230.0, 0.06, 30));
    // Last perc sound — a single lonely click
    perc_notes.push(Note::new(66, 232.0, 0.08, 35));

    add_notes_batched(&perc_clip, &perc_notes)?;
    println!("  done");

    // ================================================================
    // TRACK 4: LEAD (Operator — crystalline FM)
    // ================================================================
    println!("building track 4: LEAD (crystalline FM)...");
    let lead_track = session.create_midi_track(-1)?;
    lead_track.set_name("lead")?;
    let lead_idx = lead_track.track_idx;

    session.load_instrument(lead_idx, "Operator")?;
    ms(500);

    let lead_op = Op::new(lead_track.device(0))?;
    lead_op.set("Algorithm", 0.15)?;
    lead_op.set("Osc-A Level", 0.85)?;
    lead_op.set("Osc-A Wave", 0.0)?;
    lead_op.set("Osc-B On", 1.0)?;
    lead_op.set("Osc-B Level", 0.4)?;
    lead_op.set("Osc-B Wave", 0.0)?;
    lead_op.set("B Coarse", 0.75)?;
    lead_op.set("B Fine", 0.52)?;
    lead_op.set("Osc-C On", 1.0)?;
    lead_op.set("Osc-C Level", 0.25)?;
    lead_op.set("Osc-C Wave", 0.0)?;
    lead_op.set("C Coarse", 0.6)?;
    lead_op.set("Filter On", 1.0)?;
    lead_op.set("Filter Freq", 0.65)?;
    lead_op.set("Filter Res", 0.25)?;
    lead_op.set("Ae Attack", 0.08)?;
    lead_op.set("Ae Decay", 0.55)?;
    lead_op.set("Ae Sustain", 0.35)?;
    lead_op.set("Ae Release", 0.5)?;

    // Reverb — more lush settings
    session.load_effect(lead_idx, "Reverb")?;
    ms(300);
    let lead_rev = lead_track.device(1);
    let _ = lead_rev.set_param_by_name("Decay Time", 0.7);
    let _ = lead_rev.set_param_by_name("Dry/Wet", 0.4);

    let lead_clip = lead_track.create_clip(0, total_beats)?;
    lead_clip.set_name("lead")?;
    lead_clip.set_looping(false)?;

    let mut lead_notes = Vec::new();

    // Bb minor scale: Bb(58/70/82) C(60/72) Db(61/73/85) Eb(63/75) F(65/77) Gb(66/78) Ab(68/80)

    // === Single tentative note at bar 20 (beat 76) — a question ===
    lead_notes.push(Note::new(70, 76.0, 3.5, 65));  // Bb4 — hangs alone. What is this?

    // === Phrase 1 (bars 21-24): call and response ===
    // Call
    lead_notes.push(Note::new(73, 82.0, 0.75, 95));   // Db5
    lead_notes.push(Note::new(75, 83.0, 1.5, 100));   // Eb5
    // Space — let it breathe
    // Response
    lead_notes.push(Note::new(77, 85.5, 0.75, 90));   // F5
    lead_notes.push(Note::new(75, 86.5, 0.5, 85));    // Eb5
    lead_notes.push(Note::new(73, 87.5, 2.0, 100));   // Db5 — resolves down

    // === Phrase 2 (bars 25-28): over the false breathe ===
    // Lead is alone here (bass + kick drop out). Exposed.
    // Grace notes before main notes
    lead_notes.push(Note::new(73, 95.85, 0.1, 50));   // grace
    lead_notes.push(Note::new(75, 96.0, 2.5, 105));   // Eb5 — sustained over silence
    // The listener hears the instruments drop away beneath this note
    lead_notes.push(Note::new(78, 99.0, 0.75, 95));   // Gb5
    lead_notes.push(Note::new(77, 100.0, 1.5, 100));  // F5
    // Everything surges back at 104 and the lead continues unfazed
    lead_notes.push(Note::new(75, 102.0, 0.75, 90));  // Eb5
    lead_notes.push(Note::new(73, 103.0, 0.5, 85));   // Db5
    lead_notes.push(Note::new(70, 103.75, 2.0, 100)); // Bb4

    // === Phrase 3 (bars 29-32): ascending, building ===
    // Ornamental — grace notes, scalar runs
    lead_notes.push(Note::new(68, 111.85, 0.1, 45));  // grace (Ab4)
    lead_notes.push(Note::new(70, 112.0, 0.75, 95));  // Bb4
    lead_notes.push(Note::new(73, 113.0, 0.5, 100));  // Db5
    lead_notes.push(Note::new(73, 113.85, 0.1, 50));  // grace re-articulation
    lead_notes.push(Note::new(75, 114.0, 0.75, 105)); // Eb5
    lead_notes.push(Note::new(77, 115.0, 0.5, 110));  // F5
    // Rapid ascending run
    lead_notes.push(Note::new(75, 115.75, 0.15, 60)); // Eb5
    lead_notes.push(Note::new(77, 115.9, 0.1, 65));   // F5
    lead_notes.push(Note::new(78, 116.0, 1.0, 115));  // Gb5 — arrival
    lead_notes.push(Note::new(80, 117.5, 3.0, 105));  // Ab5 — sustain into breathe

    // === BREATHE: lead holds one ethereal note ===
    lead_notes.push(Note::new(82, 128.0, 8.0, 55));   // Bb5 — very high, very quiet
    // Faint echoes (grace notes from the reverb will carry these)
    lead_notes.push(Note::new(80, 138.0, 2.0, 40));   // Ab5 whisper
    lead_notes.push(Note::new(78, 142.0, 3.0, 35));   // Gb5 — fading

    // === Pre-eruption: ascending grace notes simulating a pitch rise ===
    lead_notes.push(Note::new(70, 155.0, 0.2, 50));
    lead_notes.push(Note::new(73, 155.5, 0.2, 55));
    lead_notes.push(Note::new(75, 156.0, 0.2, 60));
    lead_notes.push(Note::new(77, 156.5, 0.2, 65));
    lead_notes.push(Note::new(78, 157.0, 0.2, 70));
    lead_notes.push(Note::new(80, 157.5, 0.2, 75));
    lead_notes.push(Note::new(82, 158.0, 0.2, 80));
    lead_notes.push(Note::new(85, 158.5, 0.15, 85));
    lead_notes.push(Note::new(87, 159.0, 0.1, 90));
    lead_notes.push(Note::new(89, 159.5, 0.1, 95));
    // Building to a SCREAM and then... silence at 160.

    // === ERUPTION lead (starts beat 162) ===
    // More intense, wider intervals, rhythmic displacement
    lead_notes.push(Note::new(75, 162.0, 0.5, 115));  // Eb5 — immediate
    lead_notes.push(Note::new(80, 162.75, 0.25, 75)); // grace
    lead_notes.push(Note::new(82, 163.0, 2.0, 120));  // Bb5
    lead_notes.push(Note::new(78, 165.5, 0.75, 100)); // Gb5
    lead_notes.push(Note::new(77, 166.5, 1.5, 110));  // F5
    // Space
    lead_notes.push(Note::new(80, 169.0, 1.0, 115));  // Ab5
    lead_notes.push(Note::new(82, 170.5, 0.5, 105));  // Bb5
    lead_notes.push(Note::new(85, 171.0, 2.0, 120));  // Db6 — reaching
    lead_notes.push(Note::new(82, 173.5, 0.75, 100)); // Bb5

    // Second eruption phrase
    lead_notes.push(Note::new(77, 176.0, 0.75, 110)); // F5
    lead_notes.push(Note::new(78, 176.85, 0.1, 65));  // grace
    lead_notes.push(Note::new(80, 177.0, 1.5, 115));  // Ab5
    lead_notes.push(Note::new(82, 179.0, 0.75, 105)); // Bb5
    lead_notes.push(Note::new(85, 180.0, 2.0, 120));  // Db6
    lead_notes.push(Note::new(87, 182.5, 1.0, 110));  // Eb6 — highest!
    lead_notes.push(Note::new(85, 184.0, 1.5, 105));  // Db6
    lead_notes.push(Note::new(82, 186.0, 1.5, 100));  // Bb5

    // False ending — lead cuts too (bar 48)

    // After false ending: PEAK melody
    lead_notes.push(Note::new(75, 192.0, 0.75, 120)); // Eb5
    lead_notes.push(Note::new(77, 192.85, 0.1, 70));  // grace
    lead_notes.push(Note::new(78, 193.0, 0.5, 110));  // Gb5
    lead_notes.push(Note::new(80, 193.75, 0.25, 75)); // grace
    lead_notes.push(Note::new(82, 194.0, 1.5, 125));  // Bb5
    lead_notes.push(Note::new(85, 196.0, 1.0, 120));  // Db6
    lead_notes.push(Note::new(87, 197.5, 2.0, 127));  // Eb6 — THE PEAK NOTE
    lead_notes.push(Note::new(85, 200.0, 1.0, 110));  // descent begins
    lead_notes.push(Note::new(82, 201.5, 1.5, 105));
    lead_notes.push(Note::new(80, 203.5, 1.0, 100));
    lead_notes.push(Note::new(78, 205.0, 1.5, 95));
    lead_notes.push(Note::new(77, 207.0, 2.0, 90));
    lead_notes.push(Note::new(75, 209.5, 2.0, 85));
    lead_notes.push(Note::new(73, 212.0, 2.0, 75));
    lead_notes.push(Note::new(70, 214.5, 3.0, 65));   // Bb4 — settling
    // Lead drops out at bar 55 (beat ~218)

    lead_clip.add_notes(&lead_notes)?;

    if let Some(b_idx) = lead_op.idx("Osc-B Level") {
        lead_clip.automate_smooth(0, b_idx, &[
            (0.0, 0.3),
            (76.0, 0.32),
            (96.0, 0.4),     // false breathe — exposed shimmer
            (112.0, 0.5),
            (128.0, 0.35),   // breathe
            (155.0, 0.45),   // pre-eruption build
            (162.0, 0.55),
            (192.0, 0.7),    // peak shimmer
            (214.0, 0.3),
            (256.0, 0.2),
        ], 1.0)?;
    }

    println!("  done");

    // ================================================================
    // TRACK 5: TEXTURE (Wavetable — micro-notes become timbre)
    // ================================================================
    println!("building track 5: TEXTURE (subatomic wavetable)...");
    let tex_track = session.create_midi_track(-1)?;
    tex_track.set_name("texture")?;
    let tex_idx = tex_track.track_idx;

    session.load_instrument(tex_idx, "Wavetable")?;
    ms(500);

    let tex_wt = tex_track.device(0);
    tex_wt.set_param(4, 0.5)?;
    tex_wt.set_param(5, 0.6)?;
    tex_wt.set_param(9, 1.0)?;
    tex_wt.set_param(12, 0.3)?;
    tex_wt.set_param(16, 0.5)?;
    tex_wt.set_param(26, 0.6)?;
    tex_wt.set_param(27, 0.2)?;
    tex_wt.set_param(39, 0.0)?;
    tex_wt.set_param(40, 0.1)?;
    tex_wt.set_param(45, 0.03)?;
    tex_wt.set_param(41, 0.05)?;
    tex_wt.set_param(89, 0.15)?;
    tex_wt.set_param(92, 0.4)?;

    session.load_effect(tex_idx, "Spectral Time")?;
    ms(400);
    let tex_st = tex_track.device(1);
    tex_st.set_param(26, 0.3)?;
    tex_st.set_param(18, 0.4)?;
    tex_st.set_param(19, 0.6)?;
    tex_st.set_param(22, 0.7)?;

    let tex_clip = tex_track.create_clip(0, total_beats)?;
    tex_clip.set_name("texture")?;
    tex_clip.set_looping(false)?;

    let mut tex_notes = Vec::new();
    let tex_scale = [46, 49, 53, 56, 58, 61, 65, 68, 70, 73, 77, 80];

    // Particles (bars 1-8): sparse micro-bursts
    for burst in 0..12 {
        let start = burst as f32 * 2.5 + 0.5;
        let count = 4 + (burst % 5);
        for j in 0..count {
            let t = start + j as f32 * 0.08;
            if t < 32.0 {
                let p = tex_scale[(burst * 3 + j) % tex_scale.len()];
                tex_notes.push(Note::new(p, t, 0.05, 45 + (j * 8) as i32 % 40));
            }
        }
    }

    // Coalesce to Pulse: increasing density
    for section_beat in (32..96).step_by(2) {
        let beat = section_beat as f32;
        let density = if beat < 64.0 { 3 } else { 5 };
        for j in 0..density {
            let t = beat + j as f32 * 0.0625;
            let idx = (section_beat * 7 + j * 13) % tex_scale.len();
            let vel = 40 + ((section_beat * 3 + j * 11) % 50) as i32;
            tex_notes.push(Note::new(tex_scale[idx], t, 0.05, vel));
        }
    }

    // === FALSE BREATHE (bars 25-26): texture THICKENS while everything else drops ===
    // Inverted expectation — the texture goes wild when everything else goes quiet
    for section_beat in (96..104).step_by(1) {
        let beat = section_beat as f32;
        for j in 0..10 {
            let t = beat + j as f32 * 0.05;
            let idx = (section_beat * 11 + j * 7) % tex_scale.len();
            tex_notes.push(Note::new(tex_scale[idx], t, 0.04, 55 + (j * 6) as i32 % 35));
        }
    }

    // Bars 27-32: back to medium
    for section_beat in (104..128).step_by(2) {
        let beat = section_beat as f32;
        for j in 0..5 {
            let t = beat + j as f32 * 0.0625;
            let idx = (section_beat * 7 + j * 13) % tex_scale.len();
            let vel = 40 + ((section_beat * 3 + j * 11) % 50) as i32;
            tex_notes.push(Note::new(tex_scale[idx], t, 0.05, vel));
        }
    }

    // === BREATHE (bars 33-40): texture goes WILD ===
    // While kick does 3-against-4 and lead whispers, texture becomes the focus
    // Maximum micro-note density — the "geometry rotating"
    for section_beat in (128..160).step_by(1) {
        let beat = section_beat as f32;
        let density = 12; // very dense!
        for j in 0..density {
            let t = beat + j as f32 * 0.04;
            let idx = (section_beat * 7 + j * 17) % tex_scale.len();
            let vel = 35 + ((section_beat * 5 + j * 9) % 45) as i32;
            tex_notes.push(Note::new(tex_scale[idx], t, 0.03, vel));
        }
    }

    // Eruption: dense but not as crazy as breathe (the rhythm carries now)
    for section_beat in (162..188).step_by(2) {
        let beat = section_beat as f32;
        for j in 0..7 {
            let t = beat + j as f32 * 0.0625;
            let idx = (section_beat * 7 + j * 13) % tex_scale.len();
            let vel = 45 + ((section_beat * 3 + j * 11) % 50) as i32;
            tex_notes.push(Note::new(tex_scale[idx], t, 0.05, vel));
        }
    }
    // False ending: texture silent too
    // After false ending: dense
    for section_beat in (192..224).step_by(1) {
        let beat = section_beat as f32;
        let density = if beat < 210.0 { 8 } else { 5 };
        for j in 0..density {
            let t = beat + j as f32 * 0.05;
            let idx = (section_beat * 11 + j * 7) % tex_scale.len();
            let vel = 40 + ((section_beat * 3 + j * 13) % 50) as i32;
            tex_notes.push(Note::new(tex_scale[idx], t, 0.04, vel));
        }
    }

    // Dissolve: texture is one of the LAST things remaining
    for section_beat in (224..252).step_by(3) {
        let beat = section_beat as f32;
        let density = ((252 - section_beat) / 4).max(2);
        for j in 0..density {
            let t = beat + j as f32 * 0.08;
            let idx = (section_beat * 7 + j * 13) % tex_scale.len();
            let vel = 30 + ((section_beat + j) % 20) as i32;
            tex_notes.push(Note::new(tex_scale[idx], t, 0.05, vel));
        }
    }

    add_notes_batched(&tex_clip, &tex_notes)?;

    // Automate wavetable position
    tex_clip.automate_smooth(0, 4, &[
        (0.0, 0.2), (32.0, 0.4), (64.0, 0.3),
        (96.0, 0.7),     // false breathe: wild morph
        (104.0, 0.4),    // back to normal
        (128.0, 0.5),    // breathe: scanning
        (144.0, 0.8),    // peak texture morph
        (160.0, 0.35),
        (192.0, 0.6), (204.0, 0.85),
        (224.0, 0.6), (240.0, 0.3), (256.0, 0.15),
    ], 1.0)?;

    // Spectral spray builds especially during breathe
    tex_clip.automate_smooth(1, 20, &[
        (0.0, 0.0), (32.0, 0.1), (64.0, 0.15),
        (96.0, 0.35),     // false breathe
        (104.0, 0.15),
        (128.0, 0.4),     // breathe: spray opens up
        (144.0, 0.7),     // peak spray
        (160.0, 0.2),
        (192.0, 0.45), (204.0, 0.6),
        (224.0, 0.8), (240.0, 0.95), (256.0, 1.0),
    ], 1.0)?;

    tex_clip.automate_smooth(1, 26, &[
        (0.0, 0.2), (64.0, 0.3),
        (96.0, 0.5),      // false breathe: spectral presence
        (104.0, 0.3),
        (128.0, 0.55),    // breathe: more wet
        (144.0, 0.7),
        (160.0, 0.3),
        (192.0, 0.5), (224.0, 0.65), (240.0, 0.8), (256.0, 0.9),
    ], 1.0)?;

    println!("  done");

    // ================================================================
    // TRACK 6: PAD (Collision — elastic glass)
    // ================================================================
    println!("building track 6: PAD (elastic glass)...");
    let pad_track = session.create_midi_track(-1)?;
    pad_track.set_name("pad")?;
    let pad_idx = pad_track.track_idx;

    session.load_instrument(pad_idx, "Collision")?;
    ms(500);

    let pad_col = pad_track.device(0);
    pad_col.set_param(7, 1.0)?;
    pad_col.set_param(8, 0.5)?;
    pad_col.set_param(11, 0.6)?;
    pad_col.set_param(14, 0.2)?;
    pad_col.set_param(32, 1.0)?;
    pad_col.set_param(33, 2.0)?;
    pad_col.set_param(34, 2.0)?;
    pad_col.set_param(41, 0.9)?;
    pad_col.set_param(45, 0.7)?;
    pad_col.set_param(52, 0.5)?;
    pad_col.set_param(54, 0.45)?;
    pad_col.set_param(56, 0.95)?;
    pad_col.set_param(64, 0.55)?;
    pad_col.set_param(65, 1.0)?;
    pad_col.set_param(66, 1.0)?;
    pad_col.set_param(67, 2.0)?;
    pad_col.set_param(68, 5.0)?;
    pad_col.set_param(69, 0.2)?;
    pad_col.set_param(74, 0.85)?;
    pad_col.set_param(78, 0.8)?;
    pad_col.set_param(87, 0.5)?;
    pad_col.set_param(97, 0.4)?;
    pad_col.set_param(1, 1.0)?;

    // Add Reverb for lushness
    session.load_effect(pad_idx, "Reverb")?;
    ms(300);
    let pad_rev = pad_track.device(1);
    let _ = pad_rev.set_param_by_name("Decay Time", 0.75);
    let _ = pad_rev.set_param_by_name("Dry/Wet", 0.45);

    let pad_clip = pad_track.create_clip(0, total_beats)?;
    pad_clip.set_name("pad")?;
    pad_clip.set_looping(false)?;

    let mut pad_notes = Vec::new();

    // === LUSH VOICINGS with staggered "bloom" entries ===
    // Each chord blooms open — root first, upper voices follow
    // Wider voicings: 7ths, 9ths, spread across 2+ octaves

    // Helper: add a blooming chord
    let bloom = |notes: &mut Vec<Note>, pitches: &[i32], start: f32, dur: f32, base_vel: i32| {
        for (i, &p) in pitches.iter().enumerate() {
            let offset = i as f32 * 0.15;  // each voice enters 0.15 beats later
            let vel = (base_vel - i as i32 * 3).max(30);
            notes.push(Note::new(p, start + offset, dur - offset - 0.5, vel));
        }
    };

    // I. PARTICLES (bars 1-8): Bbm9 — vast, open
    bloom(&mut pad_notes, &[46, 53, 58, 68, 72], 0.0, 16.0, 55);   // Bb2 F3 Bb3 Ab4 C5

    // Transition chord (bars 5-8): Ab add9
    bloom(&mut pad_notes, &[44, 51, 56, 58, 63], 16.0, 16.0, 50);  // Ab2 Eb3 Ab3 Bb3 Eb4

    // II. COALESCE (bars 9-16): Bbm7 → Dbmaj7
    bloom(&mut pad_notes, &[46, 53, 58, 63, 68], 32.0, 16.0, 58);  // Bb2 F3 Bb3 Eb4 Ab4
    bloom(&mut pad_notes, &[49, 53, 61, 65, 68], 48.0, 16.0, 55);  // Db3 F3 Db4 F4 Ab4

    // III. PULSE (bars 17-32): richer harmonic rhythm
    bloom(&mut pad_notes, &[46, 53, 58, 63, 68], 64.0, 8.0, 70);   // Bbm9
    bloom(&mut pad_notes, &[49, 56, 61, 65, 72], 72.0, 8.0, 68);   // Db/Ab add 9
    bloom(&mut pad_notes, &[44, 51, 58, 63, 68], 80.0, 8.0, 70);   // Ab add11
    bloom(&mut pad_notes, &[46, 51, 56, 63, 65], 88.0, 8.0, 68);   // Bb sus4/F

    // False breathe (bars 25-26): pad holds. UNEXPECTED chord — Gb major!
    // This chord is "wrong" — brighter, outside the expected harmony. Ear-catching.
    bloom(&mut pad_notes, &[42, 54, 58, 61, 66], 96.0, 8.0, 62);   // Gb2 Gb3 Bb3 Db4 Gb4

    // Surge back (bars 27-28): resolves to Bbm
    bloom(&mut pad_notes, &[46, 53, 58, 68, 72], 104.0, 8.0, 75);  // Bbm9 — relief!

    // Bars 29-32: descending progression
    bloom(&mut pad_notes, &[44, 51, 56, 63, 68], 112.0, 8.0, 70);  // Ab maj7
    bloom(&mut pad_notes, &[41, 48, 53, 60, 65], 120.0, 8.0, 68);  // F7 sus

    // IV. BREATHE (bars 33-40): spacious, add9 voicings
    bloom(&mut pad_notes, &[46, 53, 58, 63, 68, 72], 128.0, 16.0, 60); // Bbm9 add C — 6 notes!
    bloom(&mut pad_notes, &[44, 51, 56, 58, 63, 68], 144.0, 16.0, 55); // Ab add9/11

    // V. ERUPTION (bars 41-56):
    // Silence at 160-161.5 — pad too!
    bloom(&mut pad_notes, &[46, 53, 58, 68], 161.5, 8.5, 75);      // Bbm — explosive return
    bloom(&mut pad_notes, &[49, 56, 61, 65], 170.0, 8.0, 73);      // Db
    bloom(&mut pad_notes, &[44, 51, 58, 63], 178.0, 8.0, 75);      // Ab sus
    // False ending bar 48 — pad rings through silence (long decay on Collision)
    bloom(&mut pad_notes, &[41, 48, 53, 65], 186.0, 2.0, 70);      // F — cuts short!

    // After false ending: MAXIMUM harmonic intensity
    bloom(&mut pad_notes, &[46, 53, 58, 63, 68, 72], 192.0, 8.0, 80); // Bbm9 add C — widest
    bloom(&mut pad_notes, &[49, 56, 61, 65, 72, 75], 200.0, 8.0, 78); // Db maj9 #11 — brightness
    bloom(&mut pad_notes, &[44, 51, 56, 63, 68], 208.0, 8.0, 73);     // Ab
    bloom(&mut pad_notes, &[46, 53, 58, 68], 216.0, 8.0, 68);         // Bbm — settling

    // VI. DISSOLVE (bars 57-64): chords thin, voices drop away
    bloom(&mut pad_notes, &[46, 58, 68, 72], 224.0, 12.0, 55);     // Bbm — losing voices
    bloom(&mut pad_notes, &[46, 58, 68], 236.0, 8.0, 45);           // just the shell
    bloom(&mut pad_notes, &[46, 58], 244.0, 8.0, 35);               // octave
    pad_notes.push(Note::new(46, 252.0, 4.0, 25));                   // single note — last breath

    add_notes_batched(&pad_clip, &pad_notes)?;

    // Automate inharmonics — increasingly alien
    pad_clip.automate_smooth(0, 54, &[
        (0.0, 0.25), (64.0, 0.3),
        (96.0, 0.5),     // false breathe: sudden alien quality
        (104.0, 0.35),   // resolves
        (128.0, 0.4),    // breathe
        (160.0, 0.5),
        (192.0, 0.65),
        (204.0, 0.75),   // peak alien
        (224.0, 0.8),
        (256.0, 0.9),    // dissolve: most alien, fading
    ], 2.0)?;

    println!("  done");

    // ================================================================
    // TRACK 7: SUB (Analog — pure sine sub-bass)
    // ================================================================
    println!("building track 7: SUB (pure sine)...");
    let sub_track = session.create_midi_track(-1)?;
    sub_track.set_name("sub")?;
    let sub_idx = sub_track.track_idx;

    session.load_instrument(sub_idx, "Analog")?;
    ms(400);

    let sub_dev = sub_track.device(0);
    sub_dev.set_param(38, 1.0)?;
    sub_dev.set_param(39, 0.0)?;
    sub_dev.set_param(40, -2.0)?;
    sub_dev.set_param(105, 0.0)?;
    sub_dev.set_param(34, 0.0)?;
    sub_dev.set_param(53, 0.0)?;
    sub_dev.set_param(89, 0.08)?;
    sub_dev.set_param(90, 0.7)?;
    sub_dev.set_param(92, 0.8)?;
    sub_dev.set_param(94, 0.5)?;

    let sub_clip = sub_track.create_clip(0, total_beats)?;
    sub_clip.set_name("sub")?;
    sub_clip.set_looping(false)?;

    let mut sub_notes = Vec::new();

    // Sub follows chord roots. Enters at coalesce.
    let sub_roots: Vec<(i32, f32, f32)> = vec![
        (46, 32.0, 16.0),
        (49, 48.0, 16.0),
        // Pulse
        (46, 64.0, 8.0), (49, 72.0, 8.0), (44, 80.0, 8.0), (46, 88.0, 8.0),
        // False breathe: sub holds Gb — reinforces the "wrong" harmony
        (42, 96.0, 8.0),
        // Surge back
        (46, 104.0, 8.0), (44, 112.0, 8.0), (41, 120.0, 8.0),
        // Breathe
        (46, 128.0, 16.0), (44, 144.0, 16.0),
        // Eruption (enters late with everything else)
        (46, 161.5, 8.5), (49, 170.0, 8.0), (44, 178.0, 8.0), (41, 186.0, 2.0),
        // After false ending
        (46, 192.0, 8.0), (49, 200.0, 8.0), (44, 208.0, 8.0), (46, 216.0, 8.0),
        // Dissolve — sub stays until the very end
        (46, 224.0, 16.0),
        (46, 240.0, 14.0),  // final sub drone
    ];

    for &(pitch, start, dur) in &sub_roots {
        sub_notes.push(Note::new(pitch, start, dur - 0.5, 90));
    }

    sub_clip.add_notes(&sub_notes)?;
    println!("  done");

    // ================================================================
    // SIDECHAIN: Compressor on bass
    // ================================================================
    println!("\nloading compressor on bass (sidechain pump)...");
    session.load_effect(bass_idx, "Compressor")?;
    ms(400);
    let bass_comp = bass_track.device(1);
    let _ = bass_comp.set_param_by_name("Threshold", 0.2);
    let _ = bass_comp.set_param_by_name("Ratio", 0.9);
    let _ = bass_comp.set_param_by_name("Attack", 0.0);
    let _ = bass_comp.set_param_by_name("Release", 0.3);
    let _ = bass_comp.set_param_by_name("Output Gain", 0.6);
    println!("  done");

    // ================================================================
    // COPY TO ARRANGEMENT
    // ================================================================
    println!("\ncopying clips to arrangement view...");
    for i in 0..7 {
        let t = session.track(i);
        if t.has_clip(0).unwrap_or(false) {
            t.duplicate_clip_to_arrangement(0, 0.0)?;
            ms(100);
        }
    }
    println!("  done — switch to arrangement view to listen anytime\n");

    // ================================================================
    // PLAY
    // ================================================================
    println!("╔══════════════════════════════════════════════╗");
    println!("║         E M E R G E N C E  v2                ║");
    println!("║                                              ║");
    println!("║  intricate rhythms. lush voicings.            ║");
    println!("║  subverted expectations. every sound from     ║");
    println!("║  raw oscillators.                             ║");
    println!("╚══════════════════════════════════════════════╝\n");

    for i in 0..7 {
        let t = session.track(i);
        if t.has_clip(0).unwrap_or(false) {
            let c = t.clip(0);
            let _ = c.fire();
        }
    }

    session.set_time(0.0)?;
    session.play()?;

    println!("  I. PARTICLES      bars  1-8   — texture + pad bloom from silence");
    println!(" II. COALESCE       bars  9-16  — kick emerges, cross-rhythms hint");
    println!("III. PULSE          bars 17-24  — syncopated groove, displaced snares");
    println!("     FALSE BREATHE  bars 25-26  — everything drops... Gb chord?!");
    println!("     SURGE          bars 27-32  — comes back harder than before");
    println!(" IV. BREATHE        bars 33-40  — 3-against-4 kick, texture takes over");
    println!("     BUILD          bars 39-40  — snare roll, ascending lead...");
    println!("     HELD BREATH    bar  41     — 1.5 beats of silence.");
    println!("  V. ERUPTION       bars 41-56  — maximum intensity, polyrhythmic fury");
    println!("     FALSE ENDING   bar  48     — everything cuts. is it over?");
    println!("     MAXIMUM        bars 49-56  — NO. peak intensity.");
    println!(" VI. DISSOLVE       bars 57-64  — elements exit one by one");
    println!();
    println!("total: ~114 seconds at 135 BPM");
    println!("listening...\n");

    thread::sleep(Duration::from_secs(116));

    session.stop()?;
    println!("╔══════════════════════════════════════════════╗");
    println!("║                  fin.                        ║");
    println!("╚══════════════════════════════════════════════╝");

    Ok(())
}
