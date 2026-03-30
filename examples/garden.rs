/// GARDEN — the full bloom. Maximum lush.
///
/// Everything learned from BLOOM and CHRYSALIS combined.
/// Pivot-tone harmony through wide 6-voice chords.
/// Pollen clouds rising from harmony. Warm Collision.
/// A second pad layer for depth. Melodic spark phrases.
///
/// 76 BPM, ~3 minutes, 8 tracks.
///
/// Pivot chain (same as CHRYSALIS but voiced wider):
///   Dbmaj9 →[Ab=G#]→ Amaj9 →[E]→ Fmaj9 →[F]→ Bbm9
///   →[Ab=G#]→ Emaj7 →[D#=Eb]→ Cm9 →[C,G]→ Abmaj9 →[Ab,C]→ Dbmaj9

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

/// Bloom: voices enter one by one, crescendo, wide stagger.
fn bloom(notes: &mut Vec<Note>, pitches: &[i32], start: f32, dur: f32, gap: f32, base_vel: i32) {
    for (i, &p) in pitches.iter().enumerate() {
        let t = start + i as f32 * gap;
        let d = dur - i as f32 * gap;
        let vel = (base_vel + i as i32 * 3).min(127);
        if d > 0.1 {
            notes.push(Note::new(p, t, d, vel));
        }
    }
}

/// Reverse bloom: voices enter from TOP down, decrescendo.
fn bloom_rev(notes: &mut Vec<Note>, pitches: &[i32], start: f32, dur: f32, gap: f32, base_vel: i32) {
    for (i, &p) in pitches.iter().rev().enumerate() {
        let t = start + i as f32 * gap;
        let d = dur - i as f32 * gap;
        let vel = (base_vel - i as i32 * 3).max(25);
        if d > 0.1 {
            notes.push(Note::new(p, t, d, vel));
        }
    }
}

/// Pollen cloud from chord tones — texture rises from harmony.
fn pollen_from(notes: &mut Vec<Note>, chord_pcs: &[i32], start: f32, dur: f32, density: f32, vel: i32) {
    let high: Vec<i32> = chord_pcs.iter().flat_map(|&pc| {
        vec![pc + 72, pc + 84, pc + 78] // three octaves of shimmer
    }).collect();
    let mut t = start;
    let mut i = 0;
    while t < start + dur {
        let p = high[i % high.len()];
        let v = vel + (i as i32 % 7) * 2 - 5;
        notes.push(Note::new(p, t, 0.05, v.max(12).min(100)));
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

    session.set_tempo(76.0)?;
    let total_beats: f32 = 224.0; // 56 bars, ~2:57

    // ================================================================
    // CHORDS — 6-voice wide voicings across 3 octaves
    // 9th chords for maximum lushness
    // ================================================================

    // Dbmaj9:  Db2, Ab3, C4, F4, Eb5, Ab5
    let db_maj9: &[i32] = &[37, 56, 60, 65, 75, 80];
    // Amaj9:   A1, E3, G#3, C#4, B4, E5
    let a_maj9: &[i32] = &[33, 52, 56, 61, 71, 76];
    // Fmaj9:   F2, C4, E4, A4, G5, C6
    let f_maj9: &[i32] = &[41, 60, 64, 69, 79, 84];
    // Bbm9:    Bb1, F3, Ab3, Db4, C5, F5
    let bb_m9: &[i32] = &[34, 53, 56, 61, 72, 77];
    // Emaj7:   E2, B3, D#4, G#4, B4, E5
    let e_maj7: &[i32] = &[40, 59, 63, 68, 71, 76];
    // Cm9:     C2, G3, Bb3, Eb4, D5, G5
    let c_m9: &[i32] = &[36, 55, 58, 63, 74, 79];
    // Abmaj9:  Ab1, Eb3, G3, C4, Bb4, Eb5
    let ab_maj9: &[i32] = &[32, 51, 55, 60, 70, 75];

    // Pitch classes for pollen
    let db_pc: &[i32] = &[1, 5, 8, 0, 3];     // Db F Ab C Eb
    let a_pc: &[i32] = &[9, 1, 4, 8, 11];      // A C# E G# B
    let f_pc: &[i32] = &[5, 9, 0, 4, 7];       // F A C E G
    let bb_pc: &[i32] = &[10, 1, 5, 8, 0];     // Bb Db F Ab C
    let e_pc: &[i32] = &[4, 8, 11, 3, 6];      // E G# B D# F#
    let c_pc: &[i32] = &[0, 3, 7, 10, 2];      // C Eb G Bb D
    let ab_pc: &[i32] = &[8, 0, 3, 7, 10];     // Ab C Eb G Bb

    // ================================================================
    // TRACK 1: CANOPY — Collision (main lush pad)
    // ================================================================
    println!("building track 1: CANOPY (elastic glass, warm)...");
    let canopy_track = session.create_midi_track(-1)?;
    canopy_track.set_name("canopy")?;
    let canopy_idx = canopy_track.track_idx;

    session.load_instrument(canopy_idx, "Collision")?;
    ms(500);

    let canopy_dev = canopy_track.device(0);
    // WARM elastic glass — lower inharmonics, higher brightness, max decay
    canopy_dev.set_param(1, 1.0)?;    // parallel
    canopy_dev.set_param(7, 1.0)?;    // Mallet On
    canopy_dev.set_param(8, 0.35)?;   // Mallet Volume — softer
    canopy_dev.set_param(11, 0.4)?;   // Stiffness — softer mallet = warmer
    canopy_dev.set_param(14, 0.1)?;   // Noise — less
    canopy_dev.set_param(32, 1.0)?;   // Res 1 On
    canopy_dev.set_param(33, 2.0)?;   // Marimba
    canopy_dev.set_param(34, 2.0)?;   // Quality max
    canopy_dev.set_param(41, 0.95)?;  // Decay — maximum sustain
    canopy_dev.set_param(45, 0.55)?;  // Material — slightly less metallic = warmer
    canopy_dev.set_param(52, 0.6)?;   // Brightness — higher
    canopy_dev.set_param(54, 0.25)?;  // Inharmonics — LOWER than before = warmer
    canopy_dev.set_param(56, 0.95)?;  // Opening
    canopy_dev.set_param(64, 0.55)?;  // Volume
    canopy_dev.set_param(65, 1.0)?;   // Res 2 On
    canopy_dev.set_param(66, 1.0)?;   // Beam
    canopy_dev.set_param(67, 2.0)?;   // Quality
    canopy_dev.set_param(68, 5.0)?;   // Tune
    canopy_dev.set_param(69, 0.15)?;  // Fine Tune — gentler beating
    canopy_dev.set_param(74, 0.92)?;  // Res 2 Decay — very long
    canopy_dev.set_param(78, 0.6)?;   // Res 2 Material — warmer
    canopy_dev.set_param(87, 0.3)?;   // Res 2 Inharmonics — lower
    canopy_dev.set_param(97, 0.4)?;   // Res 2 Volume

    let canopy_clip = canopy_track.create_clip(0, total_beats)?;
    canopy_clip.set_name("canopy")?;
    canopy_clip.set_looping(false)?;

    let mut canopy_notes = Vec::new();

    // Slow blooms. 6 voices. Wide. Luxurious stagger (0.4-0.5 beat gaps).

    // === SOIL (bars 1-8): Dbmaj9 held, enormous ===
    bloom(&mut canopy_notes, db_maj9, 2.0, 28.0, 0.5, 40);

    // === ROOTS (bars 9-16): first modulation ===
    bloom(&mut canopy_notes, db_maj9, 32.0, 12.0, 0.4, 50);
    bloom(&mut canopy_notes, a_maj9, 46.0, 14.0, 0.45, 50);

    // === STEMS (bars 17-28): the pivot chain unfolds ===
    bloom(&mut canopy_notes, f_maj9, 64.0, 11.0, 0.35, 55);
    bloom(&mut canopy_notes, bb_m9, 76.0, 11.0, 0.35, 55);
    bloom(&mut canopy_notes, e_maj7, 88.0, 11.0, 0.3, 60);
    bloom(&mut canopy_notes, c_m9, 100.0, 11.0, 0.3, 60);

    // === CANOPY (bars 29-40): full bloom, accelerating pivots ===
    bloom(&mut canopy_notes, ab_maj9, 112.0, 9.0, 0.25, 65);
    bloom(&mut canopy_notes, db_maj9, 122.0, 9.0, 0.25, 65);
    bloom(&mut canopy_notes, a_maj9, 132.0, 6.0, 0.2, 70);
    bloom(&mut canopy_notes, f_maj9, 138.0, 6.0, 0.2, 70);
    bloom(&mut canopy_notes, bb_m9, 144.0, 5.0, 0.15, 72);
    // Peak: fast chords
    bloom(&mut canopy_notes, e_maj7, 150.0, 3.5, 0.12, 75);
    bloom(&mut canopy_notes, c_m9, 153.5, 3.5, 0.12, 75);
    bloom(&mut canopy_notes, ab_maj9, 157.0, 3.5, 0.12, 75);

    // === FRUIT (bars 41-48): settling, reverse blooms ===
    bloom_rev(&mut canopy_notes, db_maj9, 162.0, 10.0, 0.45, 65);
    bloom(&mut canopy_notes, ab_maj9, 174.0, 10.0, 0.5, 50);

    // === SEED (bars 49-56): returning to origin ===
    bloom(&mut canopy_notes, db_maj9, 190.0, 16.0, 0.6, 35);

    // Final: just the root and the 9th, ringing
    canopy_notes.push(Note::new(37, 208.0, 14.0, 25));  // Db2
    canopy_notes.push(Note::new(75, 210.0, 12.0, 20));  // Eb5 — the 9th

    add_notes_batched(&canopy_clip, &canopy_notes)?;

    canopy_clip.automate_smooth(0, 54, &[ // Inharmonics — gentle rise
        (0.0, 0.2),  (32.0, 0.25), (64.0, 0.3),
        (112.0, 0.4), (150.0, 0.5), (162.0, 0.35),
        (190.0, 0.2), (224.0, 0.15),
    ], 0.5)?;
    canopy_clip.automate_smooth(0, 52, &[ // Brightness
        (0.0, 0.4),  (32.0, 0.5),  (64.0, 0.6),
        (112.0, 0.7), (150.0, 0.8), (162.0, 0.6),
        (190.0, 0.4), (224.0, 0.2),
    ], 0.5)?;

    println!("  done");

    // ================================================================
    // TRACK 2: MOSS — Operator (warm FM pad, second layer)
    // ================================================================
    println!("building track 2: MOSS (warm FM pad)...");
    let moss_track = session.create_midi_track(-1)?;
    moss_track.set_name("moss")?;
    let moss_idx = moss_track.track_idx;

    session.load_instrument(moss_idx, "Operator")?;
    ms(500);

    let moss_op = Op::new(moss_track.device(0))?;
    // Warm, soft FM pad — low modulation, slow attack, long release
    moss_op.set("Algorithm", 0.05)?;  // parallel carriers = warm
    moss_op.set("Osc-A Level", 0.7)?;
    moss_op.set("Osc-A Wave", 0.0)?;
    moss_op.set("Osc-B On", 1.0)?;
    moss_op.set("Osc-B Level", 0.3)?;   // gentle modulation
    moss_op.set("Osc-B Wave", 0.0)?;
    moss_op.set("B Coarse", 0.5)?;      // integer ratio = harmonic
    moss_op.set("Osc-C On", 1.0)?;
    moss_op.set("Osc-C Level", 0.25)?;
    moss_op.set("Osc-C Wave", 0.0)?;
    moss_op.set("C Coarse", 0.5)?;
    moss_op.set("Osc-D On", 0.0)?;      // only 3 ops — simpler = warmer
    moss_op.set("Filter On", 1.0)?;
    moss_op.set("Filter Freq", 0.4)?;
    moss_op.set("Filter Res", 0.15)?;   // low res = smooth
    moss_op.set("Ae Attack", 0.25)?;    // SLOW attack — blooms in
    moss_op.set("Ae Decay", 0.6)?;
    moss_op.set("Ae Sustain", 0.55)?;
    moss_op.set("Ae Release", 0.65)?;   // long tail

    let moss_clip = moss_track.create_clip(0, total_beats)?;
    moss_clip.set_name("moss")?;
    moss_clip.set_looping(false)?;

    let mut moss_notes = Vec::new();

    // Moss doubles the canopy harmony one octave higher with just 3-4 voices.
    // Soft, slow attack — it BREATHES in underneath the Collision chords.
    // Creates depth through layering.

    // Dbmaj7 high: F3, Ab3, C4, Eb4
    // Amaj7 high: C#3, E3, G#3, B3
    // etc — just the upper voices

    // === ROOTS: moss begins ===
    for &p in &[65, 68, 72, 75] {  // F4 Ab4 C5 Eb5
        moss_notes.push(Note::new(p, 36.0, 22.0, 35));
    }

    // A maj upper voices
    for &p in &[61, 64, 68, 71] {  // C#4 E4 G#4 B4
        moss_notes.push(Note::new(p, 48.0, 14.0, 35));
    }

    // === STEMS ===
    // F maj
    for &p in &[64, 69, 72, 79] { moss_notes.push(Note::new(p, 64.0, 11.0, 40)); }
    // Bbm
    for &p in &[61, 65, 68, 72] { moss_notes.push(Note::new(p, 76.0, 11.0, 40)); }
    // E maj
    for &p in &[63, 68, 71, 76] { moss_notes.push(Note::new(p, 88.0, 11.0, 45)); }
    // Cm
    for &p in &[63, 67, 70, 74] { moss_notes.push(Note::new(p, 100.0, 11.0, 45)); }

    // === CANOPY: moss is lushest here ===
    for &p in &[60, 63, 67, 70, 75] { moss_notes.push(Note::new(p, 112.0, 9.0, 50)); }
    for &p in &[60, 65, 68, 72, 75] { moss_notes.push(Note::new(p, 122.0, 9.0, 50)); }
    for &p in &[61, 64, 68, 71, 76] { moss_notes.push(Note::new(p, 132.0, 6.0, 50)); }
    for &p in &[64, 69, 72, 76, 79] { moss_notes.push(Note::new(p, 138.0, 6.0, 50)); }
    for &p in &[61, 65, 68, 72, 77] { moss_notes.push(Note::new(p, 144.0, 5.0, 50)); }
    // Peak
    for &p in &[63, 68, 71, 76] { moss_notes.push(Note::new(p, 150.0, 3.5, 55)); }
    for &p in &[63, 67, 70, 74] { moss_notes.push(Note::new(p, 153.5, 3.5, 55)); }
    for &p in &[60, 63, 67, 70] { moss_notes.push(Note::new(p, 157.0, 3.5, 55)); }

    // === FRUIT + SEED: moss fades ===
    for &p in &[60, 65, 72, 75] { moss_notes.push(Note::new(p, 162.0, 12.0, 40)); }
    for &p in &[60, 68, 75] { moss_notes.push(Note::new(p, 190.0, 16.0, 25)); }

    add_notes_batched(&moss_clip, &moss_notes)?;

    // Moss automation: filter opens with structure
    if let Some(f_idx) = moss_op.idx("Filter Freq") {
        moss_clip.automate_smooth(0, f_idx, &[
            (0.0, 0.25), (36.0, 0.35), (64.0, 0.45),
            (112.0, 0.55), (150.0, 0.65), (162.0, 0.45),
            (190.0, 0.3), (224.0, 0.15),
        ], 0.5)?;
    }

    println!("  done");

    // ================================================================
    // TRACK 3: HEARTWOOD — Analog kick
    // ================================================================
    println!("building track 3: HEARTWOOD (kick)...");
    let hw_track = session.create_midi_track(-1)?;
    hw_track.set_name("heartwood")?;
    let hw_idx = hw_track.track_idx;

    session.load_instrument(hw_idx, "Analog")?;
    ms(400);

    let hw_dev = hw_track.device(0);
    hw_dev.set_param(38, 1.0)?;
    hw_dev.set_param(39, 0.0)?;
    hw_dev.set_param(40, -1.0)?;
    hw_dev.set_param(49, 0.65)?;   // PEG — slightly softer thud
    hw_dev.set_param(43, 0.12)?;
    hw_dev.set_param(105, 0.0)?;
    hw_dev.set_param(34, 0.0)?;
    hw_dev.set_param(53, 0.0)?;
    hw_dev.set_param(89, 0.0)?;
    hw_dev.set_param(90, 0.32)?;   // longer decay — rounder
    hw_dev.set_param(92, 0.0)?;
    hw_dev.set_param(94, 0.18)?;

    let hw_clip = hw_track.create_clip(0, total_beats)?;
    hw_clip.set_name("heartwood")?;
    hw_clip.set_looping(false)?;

    let mut hw_notes = Vec::new();

    // Gentler than CHRYSALIS — more groove, less assault.
    // Enters later, with space.

    // === STEMS (bar 17): first isolated beats ===
    hw_notes.push(Note::new(36, 66.0, 0.2, 65));
    hw_notes.push(Note::new(36, 72.0, 0.2, 70));
    hw_notes.push(Note::new(36, 76.0, 0.2, 75));
    hw_notes.push(Note::new(36, 80.0, 0.2, 70));
    hw_notes.push(Note::new(36, 84.0, 0.2, 80));
    hw_notes.push(Note::new(36, 88.0, 0.2, 80));

    // === Steady groove from bar 25 ===
    for bar in 0..4 {
        let b = 96.0 + bar as f32 * 4.0;
        hw_notes.push(Note::new(36, b, 0.2, 90));
        hw_notes.push(Note::new(36, b + 1.75, 0.2, 75));
        hw_notes.push(Note::new(36, b + 3.0, 0.15, 50)); // ghost
    }

    // === CANOPY (bars 29-40): full groove ===
    for bar in 0..12 {
        let b = 112.0 + bar as f32 * 4.0;
        hw_notes.push(Note::new(36, b, 0.2, 100));
        hw_notes.push(Note::new(36, b + 0.75, 0.12, 45));   // ghost
        hw_notes.push(Note::new(36, b + 1.75, 0.2, 85));

        match bar % 4 {
            0 => { hw_notes.push(Note::new(36, b + 3.0, 0.2, 90)); }
            1 => {
                hw_notes.push(Note::new(36, b + 2.5, 0.15, 50));
                hw_notes.push(Note::new(36, b + 3.25, 0.2, 80));
            }
            2 => {
                hw_notes.push(Note::new(36, b + 3.0, 0.2, 95));
                hw_notes.push(Note::new(36, b + 3.5, 0.12, 50));
            }
            _ => {
                // Fill
                hw_notes.push(Note::new(36, b + 2.75, 0.12, 55));
                hw_notes.push(Note::new(36, b + 3.0, 0.12, 60));
                hw_notes.push(Note::new(36, b + 3.25, 0.12, 65));
                hw_notes.push(Note::new(36, b + 3.5, 0.12, 70));
            }
        }
    }

    // === FRUIT: decelerating ===
    hw_notes.push(Note::new(36, 162.0, 0.2, 85));
    hw_notes.push(Note::new(36, 165.0, 0.2, 75));
    hw_notes.push(Note::new(36, 170.0, 0.2, 65));
    hw_notes.push(Note::new(36, 176.0, 0.2, 55));
    hw_notes.push(Note::new(36, 184.0, 0.2, 40));

    add_notes_batched(&hw_clip, &hw_notes)?;
    println!("  done");

    // ================================================================
    // TRACK 4: SAP — Operator (FM bass, liquid)
    // ================================================================
    println!("building track 4: SAP (FM bass)...");
    let sap_track = session.create_midi_track(-1)?;
    sap_track.set_name("sap")?;
    let sap_idx = sap_track.track_idx;

    session.load_instrument(sap_idx, "Operator")?;
    ms(500);

    let sap_op = Op::new(sap_track.device(0))?;
    sap_op.set("Algorithm", 0.1)?;
    sap_op.set("Osc-A Level", 1.0)?;
    sap_op.set("Osc-A Wave", 0.0)?;
    sap_op.set("Osc-B On", 1.0)?;
    sap_op.set("Osc-B Level", 0.55)?;
    sap_op.set("Osc-B Wave", 0.0)?;
    sap_op.set("B Coarse", 0.55)?;
    sap_op.set("Osc-C On", 1.0)?;
    sap_op.set("Osc-C Level", 0.4)?;
    sap_op.set("Osc-C Wave", 0.0)?;
    sap_op.set("C Coarse", 0.42)?;
    sap_op.set("Osc-D On", 1.0)?;
    sap_op.set("Osc-D Level", 0.5)?;
    sap_op.set("Osc-D Wave", 0.0)?;
    sap_op.set("D Coarse", 0.5)?;
    sap_op.set("Osc-D Feedback", 0.25)?;   // much less feedback = less acid
    sap_op.set("Filter On", 1.0)?;
    sap_op.set("Filter Freq", 0.4)?;      // slightly more open
    sap_op.set("Filter Res", 0.2)?;       // lower res = smoother, less squelch
    sap_op.set("Ae Attack", 0.06)?;
    sap_op.set("Ae Decay", 0.5)?;
    sap_op.set("Ae Sustain", 0.4)?;
    sap_op.set("Ae Release", 0.35)?;

    let sap_clip = sap_track.create_clip(0, total_beats)?;
    sap_clip.set_name("sap")?;
    sap_clip.set_looping(false)?;

    let mut sap_notes = Vec::new();

    // Bass follows chord roots through the pivot chain.
    // Slower, more melodic than CHRYSALIS.

    // === ROOTS (bar 13): bass emerges ===
    sap_notes.push(Note::new(37, 50.0, 5.0, 65));    // Db2
    sap_notes.push(Note::new(49, 53.0, 2.0, 50));    // Db3

    // A bass
    sap_notes.push(Note::new(33, 58.0, 5.0, 65));    // A1
    sap_notes.push(Note::new(45, 61.0, 2.0, 50));    // A2

    // === STEMS ===
    sap_notes.push(Note::new(41, 64.0, 5.0, 75));    // F2
    sap_notes.push(Note::new(53, 67.0, 1.5, 55));
    sap_notes.push(Note::new(41, 70.0, 4.0, 70));
    sap_notes.push(Note::new(53, 74.0, 1.5, 50));

    sap_notes.push(Note::new(34, 76.0, 5.0, 75));    // Bb1
    sap_notes.push(Note::new(46, 79.0, 1.5, 55));
    sap_notes.push(Note::new(34, 82.0, 4.0, 70));
    sap_notes.push(Note::new(46, 86.0, 1.5, 50));

    sap_notes.push(Note::new(40, 88.0, 5.0, 80));    // E2
    sap_notes.push(Note::new(52, 91.0, 1.5, 55));
    sap_notes.push(Note::new(40, 94.0, 4.0, 75));
    sap_notes.push(Note::new(52, 98.0, 1.5, 50));

    sap_notes.push(Note::new(36, 100.0, 5.0, 75));   // C2
    sap_notes.push(Note::new(48, 103.0, 1.5, 55));
    sap_notes.push(Note::new(36, 106.0, 4.0, 70));
    sap_notes.push(Note::new(48, 110.0, 1.5, 50));

    // === CANOPY: syncopated bass ===
    // Ab
    sap_notes.push(Note::new(32, 111.75, 3.0, 90));
    sap_notes.push(Note::new(44, 114.5, 0.5, 55));
    sap_notes.push(Note::new(32, 116.0, 2.5, 85));
    sap_notes.push(Note::new(32, 119.0, 2.0, 80));
    // Db
    sap_notes.push(Note::new(37, 121.75, 3.0, 90));
    sap_notes.push(Note::new(49, 124.5, 0.5, 55));
    sap_notes.push(Note::new(37, 126.0, 2.5, 85));
    sap_notes.push(Note::new(49, 129.0, 0.5, 50));
    sap_notes.push(Note::new(37, 130.0, 1.5, 80));
    // A
    sap_notes.push(Note::new(33, 131.75, 2.0, 90));
    sap_notes.push(Note::new(45, 133.5, 0.5, 55));
    sap_notes.push(Note::new(33, 134.0, 2.0, 85));
    sap_notes.push(Note::new(45, 136.5, 0.5, 50));
    // F
    sap_notes.push(Note::new(41, 137.75, 2.0, 90));
    sap_notes.push(Note::new(53, 139.5, 0.5, 55));
    sap_notes.push(Note::new(41, 140.0, 2.0, 85));
    sap_notes.push(Note::new(53, 142.5, 0.5, 50));
    // Bb
    sap_notes.push(Note::new(34, 143.75, 2.0, 90));
    sap_notes.push(Note::new(46, 145.5, 0.5, 55));
    // Peak: fast roots
    sap_notes.push(Note::new(40, 150.0, 1.5, 95));   // E2
    sap_notes.push(Note::new(36, 153.5, 1.5, 90));   // C2
    sap_notes.push(Note::new(32, 157.0, 2.0, 85));   // Ab1
    sap_notes.push(Note::new(37, 159.5, 2.0, 80));   // Db2

    // Fruit
    sap_notes.push(Note::new(37, 162.0, 5.0, 60));
    sap_notes.push(Note::new(32, 170.0, 6.0, 50));
    sap_notes.push(Note::new(37, 180.0, 8.0, 35));

    add_notes_batched(&sap_clip, &sap_notes)?;

    if let Some(fb_idx) = sap_op.idx("Osc-D Feedback") {
        sap_clip.automate_smooth(0, fb_idx, &[
            (0.0, 0.1), (50.0, 0.15), (64.0, 0.2),
            (88.0, 0.25), (112.0, 0.3), (150.0, 0.35),
            (162.0, 0.2), (190.0, 0.1), (224.0, 0.05),
        ], 0.5)?;
    }

    println!("  done");

    // ================================================================
    // TRACK 5: DEW — Operator crystalline (melodic pivot tones)
    // ================================================================
    println!("building track 5: DEW (crystalline melody)...");
    let dew_track = session.create_midi_track(-1)?;
    dew_track.set_name("dew")?;
    let dew_idx = dew_track.track_idx;

    session.load_instrument(dew_idx, "Operator")?;
    ms(500);

    let dew_op = Op::new(dew_track.device(0))?;
    dew_op.set("Algorithm", 0.15)?;
    dew_op.set("Osc-A Level", 0.85)?;
    dew_op.set("Osc-A Wave", 0.0)?;
    dew_op.set("Osc-B On", 1.0)?;
    dew_op.set("Osc-B Level", 0.3)?;
    dew_op.set("Osc-B Wave", 0.0)?;
    dew_op.set("B Coarse", 0.75)?;
    dew_op.set("B Fine", 0.52)?;
    dew_op.set("Osc-C On", 1.0)?;
    dew_op.set("Osc-C Level", 0.2)?;
    dew_op.set("Osc-C Wave", 0.0)?;
    dew_op.set("C Coarse", 0.6)?;
    dew_op.set("Filter On", 1.0)?;
    dew_op.set("Filter Freq", 0.7)?;
    dew_op.set("Filter Res", 0.15)?;
    dew_op.set("Ae Attack", 0.0)?;
    dew_op.set("Ae Decay", 0.5)?;
    dew_op.set("Ae Sustain", 0.2)?;
    dew_op.set("Ae Release", 0.6)?;

    let dew_clip = dew_track.create_clip(0, total_beats)?;
    dew_clip.set_name("dew")?;
    dew_clip.set_looping(false)?;

    let mut dew_notes = Vec::new();

    // Dew plays longer melodic phrases than BLOOM or CHRYSALIS.
    // Built from chord tones and pivot tones. More singing.

    // === SOIL: first drops ===
    dew_notes.push(Note::new(80, 10.0, 0.3, 45));     // Ab5
    dew_notes.push(Note::new(73, 16.0, 0.25, 40));    // Db5
    dew_notes.push(Note::new(85, 24.0, 0.2, 50));     // Db6

    // === ROOTS: melody emerges ===
    // Phrase over Dbmaj7: Db Eb F Ab C
    dew_notes.push(Note::new(85, 34.0, 0.3, 55));     // Db6
    dew_notes.push(Note::new(80, 35.5, 0.25, 50));    // Ab5
    dew_notes.push(Note::new(77, 37.0, 0.3, 50));     // F5
    dew_notes.push(Note::new(80, 39.0, 0.4, 55));     // Ab5 — lingers (PIVOT)
    dew_notes.push(Note::new(75, 41.0, 0.25, 45));    // Eb5
    dew_notes.push(Note::new(73, 43.0, 0.5, 50));     // Db5

    // Pivot to Amaj7: Ab stays as G# — same note, new world
    dew_notes.push(Note::new(80, 46.0, 0.5, 55));     // G#5/Ab5 — THE PIVOT
    dew_notes.push(Note::new(76, 48.0, 0.3, 50));     // E5 (new chord tone)
    dew_notes.push(Note::new(73, 49.5, 0.3, 50));     // C#5/Db5 — enharmonic!
    dew_notes.push(Note::new(80, 51.0, 0.25, 55));    // G#5
    dew_notes.push(Note::new(83, 52.5, 0.3, 50));     // B5
    dew_notes.push(Note::new(76, 54.0, 0.4, 55));     // E5 — next pivot

    // === STEMS: phrases for each chord ===
    // F section: melody rises
    dew_notes.push(Note::new(77, 65.0, 0.3, 55));     // F5
    dew_notes.push(Note::new(81, 66.5, 0.25, 50));    // A5
    dew_notes.push(Note::new(84, 68.0, 0.3, 55));     // C6
    dew_notes.push(Note::new(88, 70.0, 0.4, 60));     // E6 — peak!
    dew_notes.push(Note::new(84, 72.0, 0.3, 50));     // C6
    dew_notes.push(Note::new(79, 73.5, 0.35, 55));    // G5 — the 9th

    // Bb section: melody descends (contrasting)
    dew_notes.push(Note::new(82, 77.0, 0.3, 55));     // Bb5
    dew_notes.push(Note::new(80, 78.5, 0.3, 55));     // Ab5 (PIVOT)
    dew_notes.push(Note::new(77, 80.0, 0.25, 50));    // F5
    dew_notes.push(Note::new(73, 81.5, 0.3, 50));     // Db5
    dew_notes.push(Note::new(72, 83.0, 0.4, 55));     // C5
    dew_notes.push(Note::new(80, 85.0, 0.5, 60));     // Ab5 — lingers for pivot

    // E section: angular, foreign
    dew_notes.push(Note::new(80, 88.5, 0.3, 60));     // G#5 (pivot IN)
    dew_notes.push(Note::new(83, 90.0, 0.25, 55));    // B5
    dew_notes.push(Note::new(76, 91.5, 0.3, 55));     // E5
    dew_notes.push(Note::new(87, 93.0, 0.2, 60));     // Eb6/D#6 — high!
    dew_notes.push(Note::new(83, 94.0, 0.3, 55));     // B5
    dew_notes.push(Note::new(80, 95.5, 0.25, 50));    // G#5
    dew_notes.push(Note::new(75, 97.0, 0.4, 55));     // Eb5/D#5 (pivot OUT)

    // C section: warm resolution
    dew_notes.push(Note::new(75, 100.5, 0.3, 55));    // Eb5 (pivot IN)
    dew_notes.push(Note::new(79, 102.0, 0.3, 55));    // G5
    dew_notes.push(Note::new(84, 103.5, 0.25, 50));   // C6
    dew_notes.push(Note::new(82, 105.0, 0.3, 55));    // Bb5
    dew_notes.push(Note::new(79, 106.5, 0.35, 55));   // G5 (pivot OUT)
    dew_notes.push(Note::new(84, 108.0, 0.4, 50));    // C6

    // === CANOPY: faster phrases, cascading ===
    // Ab phrases
    dew_notes.push(Note::new(80, 112.5, 0.2, 60));
    dew_notes.push(Note::new(84, 113.0, 0.2, 60));
    dew_notes.push(Note::new(87, 113.5, 0.3, 65));
    dew_notes.push(Note::new(84, 114.5, 0.2, 55));
    dew_notes.push(Note::new(79, 115.5, 0.3, 55));

    // Db phrases
    dew_notes.push(Note::new(85, 122.5, 0.2, 60));
    dew_notes.push(Note::new(80, 123.0, 0.2, 55));
    dew_notes.push(Note::new(77, 123.5, 0.3, 55));
    dew_notes.push(Note::new(75, 124.5, 0.25, 50));
    dew_notes.push(Note::new(80, 126.0, 0.4, 55));

    // Fast pivot cascade during peak
    for (i, &p) in [80i32, 76, 77, 80, 75, 72, 79, 80, 73].iter().enumerate() {
        dew_notes.push(Note::new(p, 148.0 + i as f32 * 0.75, 0.3, 60 - (i as i32 % 3) * 5));
    }

    // Dissolve
    dew_notes.push(Note::new(80, 164.0, 0.5, 45));    // Ab — the eternal pivot
    dew_notes.push(Note::new(73, 172.0, 0.4, 40));    // Db
    dew_notes.push(Note::new(80, 180.0, 0.5, 35));    // Ab — last drop
    dew_notes.push(Note::new(75, 196.0, 0.6, 25));    // Eb — 9th, ringing into silence

    add_notes_batched(&dew_clip, &dew_notes)?;
    println!("  done");

    // ================================================================
    // TRACK 6: SPORE — Wavetable (pollen from harmony)
    // ================================================================
    println!("building track 6: SPORE (pollen from harmony)...");
    let spore_track = session.create_midi_track(-1)?;
    spore_track.set_name("spore")?;
    let spore_idx = spore_track.track_idx;

    session.load_instrument(spore_idx, "Wavetable")?;
    ms(500);

    let spore_dev = spore_track.device(0);
    spore_dev.set_param(4, 0.45)?;
    spore_dev.set_param(5, 0.55)?;
    spore_dev.set_param(9, 1.0)?;
    spore_dev.set_param(12, 0.35)?;
    spore_dev.set_param(16, 0.45)?;
    spore_dev.set_param(26, 0.6)?;
    spore_dev.set_param(27, 0.2)?;
    spore_dev.set_param(39, 0.0)?;
    spore_dev.set_param(40, 0.08)?;
    spore_dev.set_param(45, 0.02)?;
    spore_dev.set_param(41, 0.04)?;
    spore_dev.set_param(89, 0.2)?;

    let spore_clip = spore_track.create_clip(0, total_beats)?;
    spore_clip.set_name("spore")?;
    spore_clip.set_looping(false)?;

    let mut spore_notes = Vec::new();

    // Pollen rises from harmony. Each cloud uses current chord tones.
    // LUSHER: denser, more overlapping, transition clouds.

    // Soil
    pollen_from(&mut spore_notes, db_pc, 8.0, 3.0, 0.1, 25);
    pollen_from(&mut spore_notes, db_pc, 20.0, 4.0, 0.09, 30);

    // Roots
    pollen_from(&mut spore_notes, db_pc, 34.0, 5.0, 0.07, 35);
    // Transition: BOTH chord tones overlap
    pollen_from(&mut spore_notes, db_pc, 44.0, 4.0, 0.08, 30);
    pollen_from(&mut spore_notes, a_pc, 45.0, 5.0, 0.07, 35);

    // Stems
    pollen_from(&mut spore_notes, f_pc, 64.0, 6.0, 0.06, 40);
    pollen_from(&mut spore_notes, f_pc, 74.0, 3.0, 0.07, 35);
    pollen_from(&mut spore_notes, bb_pc, 75.0, 3.0, 0.07, 35);
    pollen_from(&mut spore_notes, bb_pc, 76.0, 6.0, 0.06, 40);
    pollen_from(&mut spore_notes, bb_pc, 86.0, 3.0, 0.07, 35);
    pollen_from(&mut spore_notes, e_pc, 87.0, 3.0, 0.07, 35);
    pollen_from(&mut spore_notes, e_pc, 88.0, 6.0, 0.06, 40);
    pollen_from(&mut spore_notes, e_pc, 98.0, 3.0, 0.07, 35);
    pollen_from(&mut spore_notes, c_pc, 99.0, 3.0, 0.07, 35);
    pollen_from(&mut spore_notes, c_pc, 100.0, 6.0, 0.06, 40);

    // Canopy: DENSE pollen
    pollen_from(&mut spore_notes, ab_pc, 112.0, 5.0, 0.05, 45);
    pollen_from(&mut spore_notes, db_pc, 122.0, 5.0, 0.05, 45);
    pollen_from(&mut spore_notes, a_pc, 132.0, 3.0, 0.04, 48);
    pollen_from(&mut spore_notes, f_pc, 138.0, 3.0, 0.04, 48);
    pollen_from(&mut spore_notes, bb_pc, 144.0, 3.0, 0.04, 48);
    // Peak: ALL pitch classes overlap
    pollen_from(&mut spore_notes, e_pc, 150.0, 2.5, 0.035, 50);
    pollen_from(&mut spore_notes, c_pc, 152.5, 2.5, 0.035, 50);
    pollen_from(&mut spore_notes, ab_pc, 155.5, 2.5, 0.035, 50);
    pollen_from(&mut spore_notes, db_pc, 158.0, 3.0, 0.035, 48);

    // Fruit
    pollen_from(&mut spore_notes, db_pc, 164.0, 3.0, 0.07, 38);
    pollen_from(&mut spore_notes, ab_pc, 172.0, 2.5, 0.08, 35);
    pollen_from(&mut spore_notes, db_pc, 180.0, 2.0, 0.1, 30);

    // Seed
    pollen_from(&mut spore_notes, db_pc, 196.0, 3.0, 0.12, 22);

    add_notes_batched(&spore_clip, &spore_notes)?;

    // Spectral Time
    session.load_effect(spore_idx, "Spectral Time")?;
    ms(400);
    let spectral = spore_track.device(1);
    spectral.set_param(14, 0.35)?;
    spectral.set_param(18, 0.5)?;
    spectral.set_param(19, 0.6)?;
    spectral.set_param(20, 0.3)?;
    spectral.set_param(22, 0.7)?;
    spectral.set_param(23, 0.12)?;
    spectral.set_param(24, 0.5)?;
    spectral.set_param(26, 0.55)?;

    println!("  done");

    // ================================================================
    // TRACK 7: VINE — Analog (wire/rhythm, gentler)
    // ================================================================
    println!("building track 7: VINE (rhythm)...");
    let vine_track = session.create_midi_track(-1)?;
    vine_track.set_name("vine")?;
    let vine_idx = vine_track.track_idx;

    session.load_instrument(vine_idx, "Analog")?;
    ms(400);

    let vine_dev = vine_track.device(0);
    vine_dev.set_param(38, 1.0)?;
    vine_dev.set_param(39, 2.0)?;
    vine_dev.set_param(40, 2.0)?;
    vine_dev.set_param(52, 0.15)?;
    vine_dev.set_param(34, 1.0)?;
    vine_dev.set_param(35, 0.75)?;
    vine_dev.set_param(37, 0.3)?;
    vine_dev.set_param(105, 0.0)?;
    vine_dev.set_param(53, 1.0)?;
    vine_dev.set_param(54, 1.0)?;    // HP
    vine_dev.set_param(57, 0.55)?;
    vine_dev.set_param(59, 0.55)?;
    vine_dev.set_param(62, 0.45)?;
    vine_dev.set_param(71, 0.08)?;
    vine_dev.set_param(73, 0.0)?;
    vine_dev.set_param(89, 0.0)?;
    vine_dev.set_param(90, 0.12)?;
    vine_dev.set_param(92, 0.0)?;
    vine_dev.set_param(94, 0.05)?;

    let vine_clip = vine_track.create_clip(0, total_beats)?;
    vine_clip.set_name("vine")?;
    vine_clip.set_looping(false)?;

    let mut vine_notes = Vec::new();

    let e58: [bool; 8] = [true, false, true, false, true, true, false, true];
    let e38: [bool; 8] = [true, false, false, true, false, false, true, false];

    // Vines enter late, gentle
    for bar in 0..3 {
        let b = 96.0 + bar as f32 * 4.0;
        for (i, &hit) in e38.iter().enumerate() {
            if hit {
                vine_notes.push(Note::new(72, b + i as f32 * 0.5, 0.04, 50));
            }
        }
    }

    // Canopy: fuller euclidean
    for bar in 0..12 {
        let b = 112.0 + bar as f32 * 4.0;
        for (i, &hit) in e58.iter().enumerate() {
            if hit {
                vine_notes.push(Note::new(72, b + i as f32 * 0.5, 0.04, 60));
            }
        }
        if bar >= 4 {
            for (i, &hit) in e38.iter().enumerate() {
                if hit {
                    vine_notes.push(Note::new(60, b + i as f32 * 0.5 + 0.125, 0.04, 45));
                }
            }
        }
        if bar >= 8 {
            // 5-over-4
            for j in 0..5 {
                vine_notes.push(Note::new(84, b + j as f32 * 0.8, 0.03, 50));
            }
        }
    }

    // Fruit: fragments
    vine_notes.push(Note::new(72, 162.0, 0.04, 45));
    vine_notes.push(Note::new(84, 168.0, 0.03, 40));
    vine_notes.push(Note::new(60, 176.0, 0.05, 35));
    vine_notes.push(Note::new(72, 186.0, 0.04, 25));

    add_notes_batched(&vine_clip, &vine_notes)?;
    println!("  done");

    // ================================================================
    // TRACK 8: EARTH — Analog sub
    // ================================================================
    println!("building track 8: EARTH (sub)...");
    let earth_track = session.create_midi_track(-1)?;
    earth_track.set_name("earth")?;
    let earth_idx = earth_track.track_idx;

    session.load_instrument(earth_idx, "Analog")?;
    ms(400);

    let earth_dev = earth_track.device(0);
    earth_dev.set_param(38, 1.0)?;
    earth_dev.set_param(39, 0.0)?;
    earth_dev.set_param(40, -2.0)?;
    earth_dev.set_param(105, 0.0)?;
    earth_dev.set_param(34, 0.0)?;
    earth_dev.set_param(53, 0.0)?;
    earth_dev.set_param(89, 0.15)?;
    earth_dev.set_param(90, 0.7)?;
    earth_dev.set_param(92, 0.85)?;
    earth_dev.set_param(94, 0.5)?;

    let earth_clip = earth_track.create_clip(0, total_beats)?;
    earth_clip.set_name("earth")?;
    earth_clip.set_looping(false)?;

    let mut earth_notes = Vec::new();

    // Long sub drones following roots — one octave below SAP
    earth_notes.push(Note::new(25, 4.0, 26.0, 55));   // Db1
    earth_notes.push(Note::new(25, 32.0, 13.0, 58));  // Db1
    earth_notes.push(Note::new(21, 46.0, 16.0, 58));  // A0
    earth_notes.push(Note::new(29, 64.0, 11.0, 62));  // F1
    earth_notes.push(Note::new(22, 76.0, 11.0, 62));  // Bb0
    earth_notes.push(Note::new(28, 88.0, 11.0, 65));  // E1
    earth_notes.push(Note::new(24, 100.0, 11.0, 62)); // C1
    earth_notes.push(Note::new(20, 112.0, 9.0, 68));  // Ab0
    earth_notes.push(Note::new(25, 122.0, 9.0, 68));  // Db1
    earth_notes.push(Note::new(21, 132.0, 5.0, 70));  // A0
    earth_notes.push(Note::new(29, 138.0, 5.0, 70));  // F1
    earth_notes.push(Note::new(22, 144.0, 5.0, 70));  // Bb0
    earth_notes.push(Note::new(28, 150.0, 3.0, 72));  // E1
    earth_notes.push(Note::new(24, 153.5, 3.0, 68));  // C1
    earth_notes.push(Note::new(20, 157.0, 3.0, 65));  // Ab0
    earth_notes.push(Note::new(25, 162.0, 12.0, 55)); // Db1
    earth_notes.push(Note::new(20, 176.0, 12.0, 45)); // Ab0
    earth_notes.push(Note::new(25, 192.0, 16.0, 30)); // Db1 — last

    add_notes_batched(&earth_clip, &earth_notes)?;
    println!("  done");

    // ================================================================
    // PLAYBACK
    // ================================================================
    println!("\n--- GARDEN: the full bloom ---");
    println!("76 BPM, {} beats ({} bars)\n", total_beats as i32, total_beats as i32 / 4);
    println!("tracks:");
    println!("  1. CANOPY    — Collision (warm elastic glass, 6-voice blooming chords)");
    println!("  2. MOSS      — Operator FM (warm pad layer, slow attack, depth)");
    println!("  3. HEARTWOOD — Analog kick (gentle groove)");
    println!("  4. SAP       — Operator FM (liquid bass through pivot roots)");
    println!("  5. DEW       — Operator crystalline (melodic pivot-tone phrases)");
    println!("  6. SPORE     — Wavetable + Spectral Time (pollen from harmony)");
    println!("  7. VINE      — Analog HP (euclidean rhythm)");
    println!("  8. EARTH     — Analog sub (foundation)");
    println!("\nplaying...");

    session.set_time(0.0)?;
    ms(200);
    session.play()?;

    Ok(())
}
