/// CHRYSALIS — metamorphosis.
///
/// The blooming spatial beauty of BLOOM meets the rhythmic momentum
/// of EMERGENCE. Harmony teleports through distant keys via enharmonic
/// pivot tones. Pollen clouds rise from the chord tones beneath them.
///
/// Every sound synthesized. No samples. No presets.
/// 92 BPM, ~3 minutes, 7 tracks.
///
/// Pivot-tone progression:
///   Dbmaj7 →[Ab=G#]→ Amaj7 →[E]→ Fmaj7 →[F]→ Bbm7 →[Ab=G#]→ Emaj7
///   →[D#=Eb]→ Cm7 →[C,Eb,G]→ Abmaj7 →[Ab,C]→ Dbmaj7
///
/// Structure:
///   Cocoon    (bars 1-8)    — enclosed, muffled, one held chord
///   Crack     (bars 9-16)   — first modulation, kick enters
///   Emerge    (bars 17-28)  — full pivot chain, rhythms crystallize
///   Flight    (bars 29-40)  — max intensity, fast harmonic motion
///   Dissolve  (bars 41-48)  — fragments, one last distant chord
///   Echo      (bars 49-52)  — mirror of cocoon, transformed

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

/// Bloom a chord: stagger entries by `gap` beats, velocity crescendo.
fn bloom(notes: &mut Vec<Note>, pitches: &[i32], start: f32, dur: f32, gap: f32, base_vel: i32) {
    for (i, &p) in pitches.iter().enumerate() {
        let t = start + i as f32 * gap;
        let d = dur - i as f32 * gap;
        let vel = (base_vel + i as i32 * 4).min(127);
        if d > 0.1 {
            notes.push(Note::new(p, t, d, vel));
        }
    }
}

/// Generate a pollen cloud from chord tones transposed to high octaves.
/// The texture rises FROM the harmony.
fn pollen_from(notes: &mut Vec<Note>, chord_tones: &[i32], start: f32, dur: f32, density: f32, vel: i32) {
    // Transpose chord tones up to MIDI 73-96 range
    let high_tones: Vec<i32> = chord_tones.iter().flat_map(|&t| {
        let pc = t % 12; // pitch class
        // Two octaves of each tone in the sparkle range
        vec![pc + 72, pc + 84]
    }).collect();

    let mut t = start;
    let mut i = 0;
    while t < start + dur {
        let p = high_tones[i % high_tones.len()];
        let v = vel + (i as i32 % 7) * 2 - 6; // slight variation
        notes.push(Note::new(p, t, 0.05, v.max(15).min(100)));
        t += density;
        i += 1;
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

    session.set_tempo(92.0)?;
    let total_beats: f32 = 208.0; // 52 bars

    // ================================================================
    // Chord definitions (MIDI notes, wide voicings)
    // Each chord voiced across 2-3 octaves for lushness.
    // Pivot tones marked with (P).
    // ================================================================

    // Dbmaj7:  Db2, F3,  Ab3(P), C4       — Ab will become G# in Amaj7
    let db_maj7: &[i32] = &[37, 53, 56, 60];
    // Amaj7:   A1,  C#3, E3,     G#3(P)   — G#=Ab pivot, E will carry to Fmaj7
    let a_maj7: &[i32] = &[33, 49, 52, 56];
    // Fmaj7:   F2,  A3,  C4(P),  E4(P)    — E from Amaj7, F carries to Bbm
    let f_maj7: &[i32] = &[41, 57, 60, 64];
    // Bbm7:    Bb1, Db3, F3(P),  Ab3(P)   — F from Fmaj7, Ab→G# for Emaj7
    let bb_m7: &[i32] = &[34, 49, 53, 56];
    // Emaj7:   E2,  G#3(P), B3,  D#4(P)   — G#=Ab from Bbm7, D#=Eb for Cm7
    let e_maj7: &[i32] = &[40, 56, 59, 63];
    // Cm7:     C2,  Eb3(P), G3,  Bb3      — Eb=D# from Emaj7
    let c_m7: &[i32] = &[36, 51, 55, 58];
    // Abmaj7:  Ab1, C3(P),  Eb3, G3(P)    — C and G from Cm7, Ab→home
    let ab_maj7: &[i32] = &[32, 48, 51, 55];

    // Pitch classes for pollen generation
    let db_maj7_pc: &[i32] = &[1, 5, 8, 0];   // Db, F, Ab, C
    let a_maj7_pc: &[i32] = &[9, 1, 4, 8];     // A, C#, E, G#
    let f_maj7_pc: &[i32] = &[5, 9, 0, 4];     // F, A, C, E
    let bb_m7_pc: &[i32] = &[10, 1, 5, 8];     // Bb, Db, F, Ab
    let e_maj7_pc: &[i32] = &[4, 8, 11, 3];    // E, G#, B, D#
    let c_m7_pc: &[i32] = &[0, 3, 7, 10];      // C, Eb, G, Bb
    let ab_maj7_pc: &[i32] = &[8, 0, 3, 7];    // Ab, C, Eb, G

    // ================================================================
    // TRACK 1: SHELL — Collision (harmonic body, pivot-tone chords)
    // ================================================================
    println!("building track 1: SHELL (elastic glass)...");
    let shell_track = session.create_midi_track(-1)?;
    shell_track.set_name("shell")?;
    let shell_idx = shell_track.track_idx;

    session.load_instrument(shell_idx, "Collision")?;
    ms(500);

    let shell_dev = shell_track.device(0);
    shell_dev.set_param(1, 1.0)?;    // parallel resonators
    shell_dev.set_param(7, 1.0)?;    // Mallet On
    shell_dev.set_param(8, 0.4)?;    // Mallet Volume
    shell_dev.set_param(11, 0.5)?;   // Stiffness
    shell_dev.set_param(14, 0.12)?;  // Noise
    shell_dev.set_param(32, 1.0)?;   // Res 1 On
    shell_dev.set_param(33, 2.0)?;   // Marimba
    shell_dev.set_param(34, 2.0)?;   // Quality max
    shell_dev.set_param(41, 0.9)?;   // Decay long
    shell_dev.set_param(45, 0.7)?;   // Material metallic
    shell_dev.set_param(52, 0.5)?;   // Brightness
    shell_dev.set_param(54, 0.3)?;   // Inharmonics
    shell_dev.set_param(56, 0.9)?;   // Opening
    shell_dev.set_param(64, 0.5)?;   // Volume
    shell_dev.set_param(65, 1.0)?;   // Res 2 On
    shell_dev.set_param(66, 1.0)?;   // Beam
    shell_dev.set_param(67, 2.0)?;   // Quality
    shell_dev.set_param(68, 5.0)?;   // Tune — 5th harmonic
    shell_dev.set_param(69, 0.2)?;   // Fine Tune — beating
    shell_dev.set_param(74, 0.85)?;  // Res 2 Decay
    shell_dev.set_param(78, 0.8)?;   // Res 2 Material
    shell_dev.set_param(87, 0.35)?;  // Res 2 Inharmonics
    shell_dev.set_param(97, 0.35)?;  // Res 2 Volume

    let shell_clip = shell_track.create_clip(0, total_beats)?;
    shell_clip.set_name("shell")?;
    shell_clip.set_looping(false)?;

    let mut shell_notes = Vec::new();

    // === COCOON (bars 1-8, beats 0-31): Dbmaj7 held, enclosed ===
    bloom(&mut shell_notes, db_maj7, 2.0, 28.0, 0.5, 45);

    // === CRACK (bars 9-16, beats 32-63): first teleportation ===
    // Dbmaj7 sustains... then Amaj7 enters underneath
    bloom(&mut shell_notes, db_maj7, 32.0, 12.0, 0.3, 55);
    // Amaj7 — the first foreign chord. Disorienting.
    bloom(&mut shell_notes, a_maj7, 46.0, 14.0, 0.3, 55);

    // === EMERGE (bars 17-28, beats 64-111): full pivot chain ===
    // Each chord ~6 bars, blooming open
    bloom(&mut shell_notes, f_maj7, 64.0, 10.0, 0.25, 60);
    bloom(&mut shell_notes, bb_m7, 76.0, 10.0, 0.25, 60);
    bloom(&mut shell_notes, e_maj7, 88.0, 10.0, 0.2, 65);
    bloom(&mut shell_notes, c_m7, 100.0, 10.0, 0.2, 65);

    // === FLIGHT (bars 29-40, beats 112-159): fast harmonic motion ===
    // Chords change every 4 bars now — accelerating
    bloom(&mut shell_notes, ab_maj7, 112.0, 8.0, 0.15, 70);
    bloom(&mut shell_notes, db_maj7, 120.0, 8.0, 0.15, 70); // HOME — brief relief
    // NOW ACCELERATE: chords every 2 bars
    bloom(&mut shell_notes, a_maj7, 128.0, 6.0, 0.12, 75);
    bloom(&mut shell_notes, f_maj7, 134.0, 6.0, 0.12, 75);
    bloom(&mut shell_notes, bb_m7, 140.0, 6.0, 0.12, 75);
    // Peak: every bar
    bloom(&mut shell_notes, e_maj7, 146.0, 3.5, 0.1, 80);
    bloom(&mut shell_notes, c_m7, 149.5, 3.5, 0.1, 80);
    bloom(&mut shell_notes, ab_maj7, 153.0, 3.5, 0.1, 80);
    bloom(&mut shell_notes, db_maj7, 156.0, 3.5, 0.1, 80);

    // === DISSOLVE (bars 41-48, beats 160-191) ===
    // One last distant chord — then fragments
    // Bmaj7: the MOST distant from Db. A tritone away.
    let b_maj7: &[i32] = &[35, 51, 54, 59]; // B1, Eb3(=D#3), F#3, B3
    bloom(&mut shell_notes, b_maj7, 160.0, 8.0, 0.4, 55);

    // Fragments: individual notes from Dbmaj7, falling
    shell_notes.push(Note::new(60, 170.0, 4.0, 45));  // C4
    shell_notes.push(Note::new(56, 173.0, 5.0, 40));  // Ab3
    shell_notes.push(Note::new(53, 177.0, 4.0, 35));  // F3
    shell_notes.push(Note::new(37, 182.0, 6.0, 30));  // Db2

    // === ECHO (bars 49-52, beats 192-207) ===
    // Dbmaj7 again, but voiced differently. Thinner. Higher.
    bloom(&mut shell_notes, &[49, 60, 65, 68], 194.0, 12.0, 0.5, 30);

    add_notes_batched(&shell_clip, &shell_notes)?;

    // Shell automation: inharmonics rise during flight
    shell_clip.automate_smooth(0, 54, &[
        (0.0, 0.2),   (32.0, 0.3),  (64.0, 0.4),
        (112.0, 0.55), (140.0, 0.7), (160.0, 0.5),
        (192.0, 0.25), (208.0, 0.15),
    ], 0.5)?;
    shell_clip.automate_smooth(0, 52, &[ // Brightness
        (0.0, 0.25),  (32.0, 0.4),  (64.0, 0.55),
        (112.0, 0.7), (140.0, 0.85), (160.0, 0.5),
        (192.0, 0.25), (208.0, 0.1),
    ], 0.5)?;

    println!("  done");

    // ================================================================
    // TRACK 2: PULSE — Analog (the heartbeat)
    // ================================================================
    println!("building track 2: PULSE (kick)...");
    let pulse_track = session.create_midi_track(-1)?;
    pulse_track.set_name("pulse")?;
    let pulse_idx = pulse_track.track_idx;

    session.load_instrument(pulse_idx, "Analog")?;
    ms(400);

    let pulse_dev = pulse_track.device(0);
    pulse_dev.set_param(38, 1.0)?;   // OSC1 On
    pulse_dev.set_param(39, 0.0)?;   // Sine
    pulse_dev.set_param(40, -1.0)?;  // Octave down
    pulse_dev.set_param(49, 0.7)?;   // PEG Amount
    pulse_dev.set_param(43, 0.1)?;   // PEG Time — fast pitch drop
    pulse_dev.set_param(105, 0.0)?;  // OSC2 Off
    pulse_dev.set_param(34, 0.0)?;   // Noise Off
    pulse_dev.set_param(53, 0.0)?;   // Filter Off
    pulse_dev.set_param(89, 0.0)?;   // Attack instant
    pulse_dev.set_param(90, 0.3)?;   // Decay
    pulse_dev.set_param(92, 0.0)?;   // Sustain zero
    pulse_dev.set_param(94, 0.15)?;  // Release

    let pulse_clip = pulse_track.create_clip(0, total_beats)?;
    pulse_clip.set_name("pulse")?;
    pulse_clip.set_looping(false)?;

    let mut pulse_notes = Vec::new();

    // === COCOON: silence. No heartbeat yet. ===

    // === CRACK (bars 9-16): first heartbeats, isolated ===
    pulse_notes.push(Note::new(36, 36.0, 0.2, 65));
    pulse_notes.push(Note::new(36, 44.0, 0.2, 70));
    pulse_notes.push(Note::new(36, 48.0, 0.2, 75));
    pulse_notes.push(Note::new(36, 52.0, 0.2, 70));
    pulse_notes.push(Note::new(36, 56.0, 0.2, 80));
    pulse_notes.push(Note::new(36, 58.0, 0.15, 55)); // ghost
    pulse_notes.push(Note::new(36, 60.0, 0.2, 85));

    // === EMERGE (bars 17-28): heartbeat steadies ===
    // Syncopated groove — not 4-on-the-floor, displaced
    for bar in 0..12 {
        let b = 64.0 + bar as f32 * 4.0;
        pulse_notes.push(Note::new(36, b, 0.2, 100));          // 1
        pulse_notes.push(Note::new(36, b + 1.25, 0.15, 55));   // ghost
        pulse_notes.push(Note::new(36, b + 1.75, 0.2, 85));    // displaced 2

        // Variation per 4-bar group
        match bar % 4 {
            0 => {
                pulse_notes.push(Note::new(36, b + 3.0, 0.2, 90)); // 4
            }
            1 => {
                pulse_notes.push(Note::new(36, b + 2.5, 0.15, 50));
                pulse_notes.push(Note::new(36, b + 3.25, 0.2, 85));
            }
            2 => {
                pulse_notes.push(Note::new(36, b + 3.0, 0.2, 95));
                pulse_notes.push(Note::new(36, b + 3.5, 0.15, 55)); // ghost fill
            }
            _ => {
                // Accelerating into next phrase
                pulse_notes.push(Note::new(36, b + 2.75, 0.15, 60));
                pulse_notes.push(Note::new(36, b + 3.0, 0.15, 65));
                pulse_notes.push(Note::new(36, b + 3.25, 0.15, 70));
                pulse_notes.push(Note::new(36, b + 3.5, 0.15, 75));
            }
        }
    }

    // === FLIGHT (bars 29-40): heavy, driving ===
    for bar in 0..12 {
        let b = 112.0 + bar as f32 * 4.0;
        let intensity = (bar as f32 / 12.0 * 20.0) as i32;

        // Core pattern: 1, and-of-2, 3
        pulse_notes.push(Note::new(36, b, 0.2, 110 + intensity));
        pulse_notes.push(Note::new(36, b + 0.75, 0.12, 45 + intensity));  // ghost
        pulse_notes.push(Note::new(36, b + 1.5, 0.2, 90 + intensity));
        pulse_notes.push(Note::new(36, b + 2.0, 0.2, 105 + intensity));

        // Second half varies — more fills as we go
        if bar < 4 {
            pulse_notes.push(Note::new(36, b + 3.0, 0.2, 95));
        } else if bar < 8 {
            pulse_notes.push(Note::new(36, b + 2.75, 0.15, 55));
            pulse_notes.push(Note::new(36, b + 3.0, 0.2, 100));
            pulse_notes.push(Note::new(36, b + 3.5, 0.12, 60));
        } else {
            // Dense — 8th note drive for last 4 bars
            for i in 4..8 {
                let t = b + i as f32 * 0.5;
                let v = if i % 2 == 0 { 100 } else { 55 };
                pulse_notes.push(Note::new(36, t, 0.15, v));
            }
        }
    }

    // === DISSOLVE: kick thins out ===
    pulse_notes.push(Note::new(36, 160.0, 0.2, 90));
    pulse_notes.push(Note::new(36, 163.0, 0.2, 75));
    pulse_notes.push(Note::new(36, 168.0, 0.2, 65));
    pulse_notes.push(Note::new(36, 174.0, 0.2, 55));
    pulse_notes.push(Note::new(36, 182.0, 0.2, 40)); // last heartbeat

    add_notes_batched(&pulse_clip, &pulse_notes)?;
    println!("  done");

    // ================================================================
    // TRACK 3: NERVE — Operator (FM bass following pivot roots)
    // ================================================================
    println!("building track 3: NERVE (FM bass)...");
    let nerve_track = session.create_midi_track(-1)?;
    nerve_track.set_name("nerve")?;
    let nerve_idx = nerve_track.track_idx;

    session.load_instrument(nerve_idx, "Operator")?;
    ms(500);

    let nerve_op = Op::new(nerve_track.device(0))?;
    nerve_op.set("Algorithm", 0.1)?;
    nerve_op.set("Osc-A Level", 1.0)?;
    nerve_op.set("Osc-A Wave", 0.0)?;
    nerve_op.set("Osc-B On", 1.0)?;
    nerve_op.set("Osc-B Level", 0.6)?;
    nerve_op.set("Osc-B Wave", 0.0)?;
    nerve_op.set("B Coarse", 0.55)?;
    nerve_op.set("Osc-C On", 1.0)?;
    nerve_op.set("Osc-C Level", 0.45)?;
    nerve_op.set("Osc-C Wave", 0.0)?;
    nerve_op.set("C Coarse", 0.4)?;
    nerve_op.set("Osc-D On", 1.0)?;
    nerve_op.set("Osc-D Level", 0.55)?;
    nerve_op.set("Osc-D Wave", 0.0)?;
    nerve_op.set("D Coarse", 0.5)?;
    nerve_op.set("Osc-D Feedback", 0.65)?;
    nerve_op.set("Filter On", 1.0)?;
    nerve_op.set("Filter Freq", 0.35)?;
    nerve_op.set("Filter Res", 0.5)?;
    nerve_op.set("Ae Attack", 0.05)?;
    nerve_op.set("Ae Decay", 0.45)?;
    nerve_op.set("Ae Sustain", 0.35)?;
    nerve_op.set("Ae Release", 0.3)?;

    let nerve_clip = nerve_track.create_clip(0, total_beats)?;
    nerve_clip.set_name("nerve")?;
    nerve_clip.set_looping(false)?;

    let mut nerve_notes = Vec::new();

    // Bass follows the ROOT of each chord through the pivot chain.
    // The root motion itself tells the story of the teleportation.
    // Db(25) → A(21) → F(29) → Bb(22) → E(28) → C(24) → Ab(20) → Db(25)

    // === CRACK (bars 13-16): first bass hints ===
    nerve_notes.push(Note::new(25, 50.0, 3.0, 65));   // Db1
    nerve_notes.push(Note::new(37, 53.0, 1.0, 50));   // Db2 bubble

    // === EMERGE: bass follows each chord root ===
    // F section
    nerve_notes.push(Note::new(29, 64.0, 4.0, 80));   // F1
    nerve_notes.push(Note::new(41, 66.5, 1.0, 55));   // F2
    nerve_notes.push(Note::new(29, 70.0, 3.0, 75));
    nerve_notes.push(Note::new(29, 74.0, 1.5, 70));

    // Bb section
    nerve_notes.push(Note::new(22, 76.0, 4.0, 80));   // Bb0
    nerve_notes.push(Note::new(34, 78.0, 1.0, 55));   // Bb1
    nerve_notes.push(Note::new(22, 81.0, 3.0, 75));
    nerve_notes.push(Note::new(34, 84.0, 1.5, 60));
    nerve_notes.push(Note::new(22, 86.0, 1.5, 70));

    // E section — alien root
    nerve_notes.push(Note::new(28, 88.0, 3.5, 85));   // E1
    nerve_notes.push(Note::new(40, 90.0, 1.0, 60));   // E2
    nerve_notes.push(Note::new(28, 93.0, 3.0, 80));
    nerve_notes.push(Note::new(28, 97.0, 1.5, 70));

    // C section
    nerve_notes.push(Note::new(24, 100.0, 4.0, 80));  // C1
    nerve_notes.push(Note::new(36, 102.0, 1.0, 55));  // C2
    nerve_notes.push(Note::new(24, 105.0, 3.0, 75));
    nerve_notes.push(Note::new(36, 108.0, 1.0, 60));
    nerve_notes.push(Note::new(24, 110.0, 1.5, 70));

    // === FLIGHT: bass gets syncopated, driving ===
    // Ab
    nerve_notes.push(Note::new(20, 111.75, 3.0, 95)); // anticipation!
    nerve_notes.push(Note::new(32, 114.0, 0.5, 60));
    nerve_notes.push(Note::new(20, 115.0, 2.0, 90));
    nerve_notes.push(Note::new(32, 117.5, 0.5, 55));
    nerve_notes.push(Note::new(20, 118.0, 1.5, 85));

    // Db — home!
    nerve_notes.push(Note::new(25, 119.75, 2.5, 95));
    nerve_notes.push(Note::new(37, 122.0, 0.5, 60));
    nerve_notes.push(Note::new(25, 123.0, 2.0, 90));
    nerve_notes.push(Note::new(37, 125.5, 0.5, 55));
    nerve_notes.push(Note::new(25, 126.0, 1.5, 85));

    // Fast section: A → F → Bb (2 bars each)
    // A
    nerve_notes.push(Note::new(21, 127.75, 1.5, 95));
    nerve_notes.push(Note::new(33, 129.5, 0.5, 60));
    nerve_notes.push(Note::new(21, 130.0, 1.0, 90));
    nerve_notes.push(Note::new(21, 131.5, 0.5, 55));
    nerve_notes.push(Note::new(33, 132.0, 1.0, 85));
    // F
    nerve_notes.push(Note::new(29, 133.75, 1.5, 95));
    nerve_notes.push(Note::new(41, 135.0, 0.5, 60));
    nerve_notes.push(Note::new(29, 136.0, 1.5, 90));
    nerve_notes.push(Note::new(29, 138.0, 1.0, 80));
    // Bb
    nerve_notes.push(Note::new(22, 139.75, 1.5, 95));
    nerve_notes.push(Note::new(34, 141.0, 0.5, 60));
    nerve_notes.push(Note::new(22, 142.0, 1.5, 90));
    nerve_notes.push(Note::new(34, 144.0, 1.0, 75));

    // Peak: E → C → Ab → Db (1 bar each)
    nerve_notes.push(Note::new(28, 146.0, 1.5, 100));
    nerve_notes.push(Note::new(40, 147.5, 0.5, 65));
    nerve_notes.push(Note::new(24, 149.5, 1.5, 100));
    nerve_notes.push(Note::new(36, 151.0, 0.5, 65));
    nerve_notes.push(Note::new(20, 153.0, 1.5, 95));
    nerve_notes.push(Note::new(32, 154.5, 0.5, 60));
    nerve_notes.push(Note::new(25, 156.0, 3.0, 90));  // Db — landing

    // === DISSOLVE: bass fades ===
    nerve_notes.push(Note::new(23, 160.0, 4.0, 65));  // B0 — tritone root!
    nerve_notes.push(Note::new(25, 168.0, 5.0, 50));  // Db1 — returning
    nerve_notes.push(Note::new(25, 178.0, 6.0, 35));  // long fade

    add_notes_batched(&nerve_clip, &nerve_notes)?;

    // Nerve automation
    if let Some(fb_idx) = nerve_op.idx("Osc-D Feedback") {
        nerve_clip.automate_smooth(0, fb_idx, &[
            (0.0, 0.3),   (50.0, 0.5),  (64.0, 0.6),
            (88.0, 0.7),  // E chord — more alien
            (112.0, 0.75), (140.0, 0.85), (160.0, 0.5),
            (192.0, 0.2),  (208.0, 0.1),
        ], 0.5)?;
    }
    if let Some(f_idx) = nerve_op.idx("Filter Freq") {
        nerve_clip.automate_smooth(0, f_idx, &[
            (0.0, 0.2),   (50.0, 0.3),  (64.0, 0.4),
            (112.0, 0.5), (140.0, 0.65), (160.0, 0.35),
            (192.0, 0.15), (208.0, 0.1),
        ], 0.5)?;
    }

    println!("  done");

    // ================================================================
    // TRACK 4: SPARK — Operator (melody from PIVOT TONES)
    // ================================================================
    println!("building track 4: SPARK (pivot melody)...");
    let spark_track = session.create_midi_track(-1)?;
    spark_track.set_name("spark")?;
    let spark_idx = spark_track.track_idx;

    session.load_instrument(spark_idx, "Operator")?;
    ms(500);

    let spark_op = Op::new(spark_track.device(0))?;
    spark_op.set("Algorithm", 0.15)?;
    spark_op.set("Osc-A Level", 0.85)?;
    spark_op.set("Osc-A Wave", 0.0)?;
    spark_op.set("Osc-B On", 1.0)?;
    spark_op.set("Osc-B Level", 0.35)?;
    spark_op.set("Osc-B Wave", 0.0)?;
    spark_op.set("B Coarse", 0.75)?;
    spark_op.set("B Fine", 0.52)?;
    spark_op.set("Osc-C On", 1.0)?;
    spark_op.set("Osc-C Level", 0.2)?;
    spark_op.set("Osc-C Wave", 0.0)?;
    spark_op.set("C Coarse", 0.6)?;
    spark_op.set("Filter On", 1.0)?;
    spark_op.set("Filter Freq", 0.65)?;
    spark_op.set("Filter Res", 0.2)?;
    spark_op.set("Ae Attack", 0.0)?;
    spark_op.set("Ae Decay", 0.5)?;
    spark_op.set("Ae Sustain", 0.2)?;
    spark_op.set("Ae Release", 0.55)?;

    let spark_clip = spark_track.create_clip(0, total_beats)?;
    spark_clip.set_name("spark")?;
    spark_clip.set_looping(false)?;

    let mut spark_notes = Vec::new();

    // The spark melody plays the PIVOT TONES — the notes that
    // exist in both the outgoing and incoming chord. These are
    // the hinges of the harmonic teleportation. The listener
    // hears them as a thread of continuity through alien territory.

    // Cocoon: single Db note, the seed
    spark_notes.push(Note::new(73, 8.0, 0.3, 50));    // Db5

    // Crack: Ab/G# — the first pivot tone
    // Ab appears in Dbmaj7 and (as G#) in Amaj7
    spark_notes.push(Note::new(80, 44.0, 0.25, 60));  // Ab5
    spark_notes.push(Note::new(80, 45.5, 0.2, 55));   // repeat — it's important
    spark_notes.push(Note::new(68, 47.0, 0.3, 50));   // Ab4 — lower octave echo

    // Emerge pivot: E (Amaj7 → Fmaj7)
    spark_notes.push(Note::new(76, 63.0, 0.25, 60));  // E5
    spark_notes.push(Note::new(64, 63.5, 0.3, 55));   // E4 — two octaves

    // F pivot (Fmaj7 → Bbm7)
    spark_notes.push(Note::new(77, 75.0, 0.25, 60));  // F5
    spark_notes.push(Note::new(65, 75.5, 0.2, 55));

    // Ab/G# pivot (Bbm7 → Emaj7)
    spark_notes.push(Note::new(80, 87.0, 0.25, 65));  // Ab5 again!
    spark_notes.push(Note::new(68, 87.5, 0.2, 55));

    // D#/Eb pivot (Emaj7 → Cm7)
    spark_notes.push(Note::new(75, 99.0, 0.25, 65));  // Eb5
    spark_notes.push(Note::new(63, 99.5, 0.2, 55));

    // C and G pivot (Cm7 → Abmaj7)
    spark_notes.push(Note::new(72, 109.0, 0.2, 60));  // C5
    spark_notes.push(Note::new(79, 109.5, 0.2, 60));  // G5

    // Ab/C pivot (Abmaj7 → Dbmaj7)
    spark_notes.push(Note::new(80, 119.0, 0.25, 65)); // Ab5
    spark_notes.push(Note::new(72, 119.5, 0.2, 60));  // C5

    // FLIGHT: pivots come faster, more melodic
    // A→F: C#/Db pivot
    spark_notes.push(Note::new(73, 127.0, 0.2, 65));  // Db5=C#5
    spark_notes.push(Note::new(61, 127.5, 0.2, 55));

    // F→Bb: F stays
    spark_notes.push(Note::new(77, 133.0, 0.2, 65));
    // Bb→E: Ab/G# again
    spark_notes.push(Note::new(80, 139.0, 0.2, 70));  // the recurring pivot!

    // Peak: pivots cascade like a waterfall
    spark_notes.push(Note::new(75, 146.0, 0.15, 70)); // Eb (E→C)
    spark_notes.push(Note::new(72, 147.0, 0.15, 65)); // C (C→Ab)
    spark_notes.push(Note::new(79, 148.0, 0.15, 65)); // G
    spark_notes.push(Note::new(80, 149.5, 0.15, 70)); // Ab (Ab→Db)
    spark_notes.push(Note::new(72, 150.5, 0.15, 65)); // C
    spark_notes.push(Note::new(80, 153.0, 0.2, 65));  // Ab
    spark_notes.push(Note::new(73, 155.0, 0.3, 60));  // Db — home

    // Dissolve: last sparks
    spark_notes.push(Note::new(75, 162.0, 0.3, 50));  // Eb (D# — pivot to B)
    spark_notes.push(Note::new(80, 170.0, 0.4, 40));  // Ab
    spark_notes.push(Note::new(73, 180.0, 0.5, 30));  // Db — final

    // Echo: one last Ab — the most important pivot of all
    spark_notes.push(Note::new(80, 196.0, 0.3, 35));

    add_notes_batched(&spark_clip, &spark_notes)?;
    println!("  done");

    // ================================================================
    // TRACK 5: CLOUD — Wavetable (pollen rising from the harmony)
    // ================================================================
    println!("building track 5: CLOUD (pollen from harmony)...");
    let cloud_track = session.create_midi_track(-1)?;
    cloud_track.set_name("cloud")?;
    let cloud_idx = cloud_track.track_idx;

    session.load_instrument(cloud_idx, "Wavetable")?;
    ms(500);

    let cloud_dev = cloud_track.device(0);
    cloud_dev.set_param(4, 0.45)?;   // Osc 1 Pos
    cloud_dev.set_param(5, 0.55)?;   // Effect 1
    cloud_dev.set_param(9, 1.0)?;    // Osc 2 On
    cloud_dev.set_param(12, 0.35)?;  // Osc 2 Pos
    cloud_dev.set_param(16, 0.45)?;  // Osc 2 Gain
    cloud_dev.set_param(26, 0.55)?;  // Filter Freq
    cloud_dev.set_param(27, 0.2)?;   // Filter Res
    cloud_dev.set_param(39, 0.0)?;   // Attack instant
    cloud_dev.set_param(40, 0.08)?;  // Decay very short
    cloud_dev.set_param(45, 0.02)?;  // Sustain near zero
    cloud_dev.set_param(41, 0.04)?;  // Release tiny
    cloud_dev.set_param(89, 0.15)?;  // Unison

    let cloud_clip = cloud_track.create_clip(0, total_beats)?;
    cloud_clip.set_name("cloud")?;
    cloud_clip.set_looping(false)?;

    let mut cloud_notes = Vec::new();

    // Pollen clouds generated FROM chord tones.
    // Each cloud uses the current chord's pitch classes, transposed high.
    // When chords change, the pollen changes — the texture grows
    // directly from the harmony, like spores from a flower.

    // === COCOON: first wisps of pollen from Dbmaj7 ===
    pollen_from(&mut cloud_notes, db_maj7_pc, 12.0, 2.0, 0.1, 30);
    pollen_from(&mut cloud_notes, db_maj7_pc, 22.0, 2.5, 0.09, 35);

    // === CRACK: pollen shifts with modulation ===
    pollen_from(&mut cloud_notes, db_maj7_pc, 34.0, 4.0, 0.08, 40);
    // The moment of teleportation — BOTH chord tones overlap!
    pollen_from(&mut cloud_notes, db_maj7_pc, 44.0, 3.0, 0.08, 35);
    pollen_from(&mut cloud_notes, a_maj7_pc, 45.5, 4.0, 0.08, 40);

    // === EMERGE: pollen tracks each chord ===
    pollen_from(&mut cloud_notes, f_maj7_pc, 65.0, 5.0, 0.07, 45);
    pollen_from(&mut cloud_notes, bb_m7_pc, 77.0, 5.0, 0.07, 45);
    pollen_from(&mut cloud_notes, e_maj7_pc, 89.0, 5.0, 0.07, 45);
    pollen_from(&mut cloud_notes, c_m7_pc, 101.0, 5.0, 0.07, 45);

    // Transition clouds — tones from BOTH chords during pivot
    pollen_from(&mut cloud_notes, f_maj7_pc, 74.0, 3.0, 0.08, 35);
    pollen_from(&mut cloud_notes, bb_m7_pc, 75.0, 3.0, 0.08, 35);

    pollen_from(&mut cloud_notes, bb_m7_pc, 86.0, 3.0, 0.08, 35);
    pollen_from(&mut cloud_notes, e_maj7_pc, 87.0, 3.0, 0.08, 35);

    pollen_from(&mut cloud_notes, e_maj7_pc, 98.0, 3.0, 0.08, 35);
    pollen_from(&mut cloud_notes, c_m7_pc, 99.0, 3.0, 0.08, 35);

    // === FLIGHT: dense pollen, fast changes ===
    pollen_from(&mut cloud_notes, ab_maj7_pc, 112.0, 4.0, 0.05, 50);
    pollen_from(&mut cloud_notes, db_maj7_pc, 120.0, 4.0, 0.05, 50);
    pollen_from(&mut cloud_notes, a_maj7_pc, 128.0, 3.0, 0.05, 55);
    pollen_from(&mut cloud_notes, f_maj7_pc, 134.0, 3.0, 0.05, 55);
    pollen_from(&mut cloud_notes, bb_m7_pc, 140.0, 3.0, 0.05, 55);

    // Peak: pollen from ALL chord tones simultaneously
    // The garden at maximum bloom
    pollen_from(&mut cloud_notes, e_maj7_pc, 146.0, 2.0, 0.04, 50);
    pollen_from(&mut cloud_notes, c_m7_pc, 149.0, 2.0, 0.04, 50);
    pollen_from(&mut cloud_notes, ab_maj7_pc, 152.0, 2.0, 0.04, 50);
    pollen_from(&mut cloud_notes, db_maj7_pc, 155.0, 3.0, 0.04, 50);

    // === DISSOLVE: pollen thins, slows ===
    // B major tones for the tritone chord
    let b_maj7_pc: &[i32] = &[11, 3, 6, 10]; // B, D#, F#, A#
    pollen_from(&mut cloud_notes, b_maj7_pc, 161.0, 3.0, 0.08, 40);
    pollen_from(&mut cloud_notes, db_maj7_pc, 170.0, 2.0, 0.1, 35);
    pollen_from(&mut cloud_notes, db_maj7_pc, 180.0, 1.5, 0.12, 30);

    // === ECHO: last pollen, very sparse ===
    pollen_from(&mut cloud_notes, db_maj7_pc, 196.0, 2.0, 0.15, 25);

    add_notes_batched(&cloud_clip, &cloud_notes)?;

    // Add Spectral Time to cloud track
    session.load_effect(cloud_idx, "Spectral Time")?;
    ms(400);
    let spectral = cloud_track.device(1);
    spectral.set_param(14, 0.3)?;    // Delay Time
    spectral.set_param(18, 0.5)?;    // Feedback
    spectral.set_param(19, 0.55)?;   // Tilt — freq-dependent delay
    spectral.set_param(20, 0.25)?;   // Spray — per-bin randomize
    spectral.set_param(22, 0.65)?;   // Stereo Spread
    spectral.set_param(23, 0.1)?;    // Freq Shift — shimmer
    spectral.set_param(24, 0.45)?;   // Delay Mix
    spectral.set_param(26, 0.5)?;    // Dry/Wet

    println!("  done");

    // ================================================================
    // TRACK 6: WIRE — Analog (mechanical rhythm, evolving patterns)
    // ================================================================
    println!("building track 6: WIRE (mechanical pulse)...");
    let wire_track = session.create_midi_track(-1)?;
    wire_track.set_name("wire")?;
    let wire_idx = wire_track.track_idx;

    session.load_instrument(wire_idx, "Analog")?;
    ms(400);

    let wire_dev = wire_track.device(0);
    wire_dev.set_param(38, 1.0)?;    // OSC1 On
    wire_dev.set_param(39, 2.0)?;    // Square
    wire_dev.set_param(40, 2.0)?;    // Two octaves up
    wire_dev.set_param(52, 0.2)?;    // OSC1 Level low
    wire_dev.set_param(34, 1.0)?;    // Noise On
    wire_dev.set_param(35, 0.8)?;    // Bright noise
    wire_dev.set_param(37, 0.35)?;   // Noise Level
    wire_dev.set_param(105, 0.0)?;   // OSC2 Off
    wire_dev.set_param(53, 1.0)?;    // Filter On
    wire_dev.set_param(54, 1.0)?;    // HP
    wire_dev.set_param(57, 0.6)?;    // Freq
    wire_dev.set_param(59, 0.6)?;    // Resonance
    wire_dev.set_param(62, 0.5)?;    // Filter Env
    wire_dev.set_param(71, 0.08)?;   // FEG Decay
    wire_dev.set_param(73, 0.0)?;    // FEG Sustain
    wire_dev.set_param(89, 0.0)?;    // Attack instant
    wire_dev.set_param(90, 0.1)?;    // Decay
    wire_dev.set_param(92, 0.0)?;    // Sustain zero
    wire_dev.set_param(94, 0.05)?;   // Release

    let wire_clip = wire_track.create_clip(0, total_beats)?;
    wire_clip.set_name("wire")?;
    wire_clip.set_looping(false)?;

    let mut wire_notes = Vec::new();

    // Euclidean patterns that CHANGE with the harmony.
    // Each chord gets its own rhythmic pattern — the rhythm
    // metamorphoses alongside the harmony.

    let e58: [bool; 8] = [true, false, true, false, true, true, false, true];
    let e38: [bool; 8] = [true, false, false, true, false, false, true, false];
    let e57: [bool; 7] = [true, false, true, true, false, true, false];

    // === CRACK (bars 13-16): first ticks ===
    wire_notes.push(Note::new(72, 50.0, 0.04, 45));
    wire_notes.push(Note::new(84, 54.0, 0.03, 40));
    wire_notes.push(Note::new(72, 58.0, 0.04, 50));
    wire_notes.push(Note::new(60, 61.0, 0.05, 45));

    // === EMERGE: euclidean patterns ===
    // F section: E(5,8)
    for bar in 0..3 {
        let b = 64.0 + bar as f32 * 4.0;
        for (i, &hit) in e58.iter().enumerate() {
            if hit {
                wire_notes.push(Note::new(72, b + i as f32 * 0.5, 0.04, 65));
            }
        }
    }

    // Bb section: E(3,8) — sparser, different feel
    for bar in 0..3 {
        let b = 76.0 + bar as f32 * 4.0;
        for (i, &hit) in e38.iter().enumerate() {
            if hit {
                wire_notes.push(Note::new(60, b + i as f32 * 0.5, 0.05, 60));
            }
        }
        wire_notes.push(Note::new(84, b + 2.0, 0.03, 50));
    }

    // E section: E(5,7) — 7 subdivisions! Against 4/4!
    for bar in 0..3 {
        let b = 88.0 + bar as f32 * 4.0;
        for (i, &hit) in e57.iter().enumerate() {
            if hit {
                let t = b + i as f32 * (4.0 / 7.0);
                wire_notes.push(Note::new(72, t, 0.04, 65));
            }
        }
    }

    // C section: back to E(5,8) but with accent variation
    for bar in 0..3 {
        let b = 100.0 + bar as f32 * 4.0;
        for (i, &hit) in e58.iter().enumerate() {
            if hit {
                let pitch = if i % 3 == 0 { 60 } else { 72 };
                let vel = if i == 0 { 75 } else { 55 };
                wire_notes.push(Note::new(pitch, b + i as f32 * 0.5, 0.04, vel));
            }
        }
    }

    // === FLIGHT: stacked euclidean layers ===
    for bar in 0..12 {
        let b = 112.0 + bar as f32 * 4.0;

        // Layer 1: E(5,8) on 72
        for (i, &hit) in e58.iter().enumerate() {
            if hit {
                wire_notes.push(Note::new(72, b + i as f32 * 0.5, 0.04, 70));
            }
        }

        // Layer 2: E(3,8) on 60, offset by a 16th
        for (i, &hit) in e38.iter().enumerate() {
            if hit {
                wire_notes.push(Note::new(60, b + i as f32 * 0.5 + 0.125, 0.04, 50));
            }
        }

        // Bar-dependent accent: 5-over-4 ping
        if bar >= 4 {
            let mut t = b;
            for _ in 0..5 {
                if t < b + 4.0 {
                    wire_notes.push(Note::new(84, t, 0.03, 55));
                }
                t += 0.8; // 5 events in 4 beats
            }
        }
    }

    // === DISSOLVE: wire fragments ===
    wire_notes.push(Note::new(72, 160.0, 0.04, 50));
    wire_notes.push(Note::new(84, 164.0, 0.03, 45));
    wire_notes.push(Note::new(60, 170.0, 0.05, 40));
    wire_notes.push(Note::new(72, 176.0, 0.04, 35));
    wire_notes.push(Note::new(84, 184.0, 0.03, 25));

    add_notes_batched(&wire_clip, &wire_notes)?;

    wire_clip.automate_smooth(0, 57, &[ // Filter Freq
        (0.0, 0.4),  (50.0, 0.5),  (64.0, 0.55),
        (88.0, 0.65), (112.0, 0.7), (140.0, 0.85),
        (160.0, 0.5), (192.0, 0.3),
    ], 0.5)?;

    println!("  done");

    // ================================================================
    // TRACK 7: GROUND — Analog (sub foundation)
    // ================================================================
    println!("building track 7: GROUND (sub)...");
    let ground_track = session.create_midi_track(-1)?;
    ground_track.set_name("ground")?;
    let ground_idx = ground_track.track_idx;

    session.load_instrument(ground_idx, "Analog")?;
    ms(400);

    let ground_dev = ground_track.device(0);
    ground_dev.set_param(38, 1.0)?;   // OSC1 On
    ground_dev.set_param(39, 0.0)?;   // Sine
    ground_dev.set_param(40, -2.0)?;  // Two octaves down
    ground_dev.set_param(105, 0.0)?;  // OSC2 Off
    ground_dev.set_param(34, 0.0)?;   // Noise Off
    ground_dev.set_param(53, 0.0)?;   // Filter Off
    ground_dev.set_param(89, 0.12)?;  // Soft attack
    ground_dev.set_param(90, 0.7)?;   // Decay
    ground_dev.set_param(92, 0.85)?;  // Sustain
    ground_dev.set_param(94, 0.5)?;   // Release

    let ground_clip = ground_track.create_clip(0, total_beats)?;
    ground_clip.set_name("ground")?;
    ground_clip.set_looping(false)?;

    let mut ground_notes = Vec::new();

    // Sub follows chord roots. Long held tones.
    ground_notes.push(Note::new(37, 4.0, 26.0, 55));    // Db2 — cocoon
    ground_notes.push(Note::new(37, 32.0, 13.0, 60));   // Db2 — crack
    ground_notes.push(Note::new(33, 46.0, 16.0, 60));   // A1
    ground_notes.push(Note::new(41, 64.0, 11.0, 65));   // F2
    ground_notes.push(Note::new(34, 76.0, 11.0, 65));   // Bb1
    ground_notes.push(Note::new(40, 88.0, 11.0, 70));   // E2
    ground_notes.push(Note::new(36, 100.0, 11.0, 65));  // C2
    ground_notes.push(Note::new(32, 112.0, 7.0, 70));   // Ab1
    ground_notes.push(Note::new(37, 120.0, 7.0, 70));   // Db2
    ground_notes.push(Note::new(33, 128.0, 5.0, 75));   // A1
    ground_notes.push(Note::new(41, 134.0, 5.0, 75));   // F2
    ground_notes.push(Note::new(34, 140.0, 5.0, 75));   // Bb1
    // Peak: fast root motion
    ground_notes.push(Note::new(40, 146.0, 3.0, 75));   // E2
    ground_notes.push(Note::new(36, 149.5, 3.0, 70));   // C2
    ground_notes.push(Note::new(32, 153.0, 3.0, 70));   // Ab1
    ground_notes.push(Note::new(37, 156.0, 3.5, 65));   // Db2

    // Dissolve
    ground_notes.push(Note::new(35, 160.0, 8.0, 55));   // B1 — tritone
    ground_notes.push(Note::new(37, 170.0, 10.0, 45));  // Db2 — returning
    ground_notes.push(Note::new(37, 184.0, 10.0, 35));  // fade

    // Echo
    ground_notes.push(Note::new(37, 196.0, 10.0, 25));  // barely there

    add_notes_batched(&ground_clip, &ground_notes)?;
    println!("  done");

    // ================================================================
    // PLAYBACK
    // ================================================================
    println!("\n--- CHRYSALIS: metamorphosis ---");
    println!("92 BPM, {} beats ({} bars)\n", total_beats as i32, total_beats as i32 / 4);
    println!("tracks:");
    println!("  1. SHELL  — Collision (pivot-tone chords, blooming voicings)");
    println!("  2. PULSE  — Analog kick (heartbeat, from isolated to driving)");
    println!("  3. NERVE  — Operator FM (bass following chord roots through pivots)");
    println!("  4. SPARK  — Operator crystalline (melody of pivot tones)");
    println!("  5. CLOUD  — Wavetable + Spectral Time (pollen rising FROM harmony)");
    println!("  6. WIRE   — Analog HP (euclidean patterns that change with chords)");
    println!("  7. GROUND — Analog sub (felt foundation)");
    println!("\npivot chain: Db →[Ab=G#]→ A →[E]→ F →[F]→ Bb →[Ab=G#]→ E →[D#=Eb]→ C →[C,G]→ Ab →[Ab,C]→ Db");
    println!("\nplaying...");

    session.set_time(0.0)?;
    ms(200);
    session.play()?;

    Ok(())
}
