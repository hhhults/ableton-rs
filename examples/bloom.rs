/// BLOOM — blooming cyborg flowers.
///
/// Elastic glass petals unfurling over FM bubble stems.
/// Crystalline dew. Subatomic pollen clouds. Wire thorns.
///
/// Every sound synthesized from raw oscillators. No samples.
/// 78 BPM, ~2:40, 6 tracks.
///
/// Structure:
///   Germination (bars 1-8)   — near-silence, first resonance
///   First Bloom (bars 9-16)  — petals open, dew appears
///   Growth (bars 17-28)      — all elements, harmonic motion
///   Full Bloom (bars 29-40)  — peak density, cross-rhythms
///   Spore (bars 41-48)       — fragmentation, dissolution
///   Seed (bars 49-52)        — one tone, silence

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

    // Clear existing tracks
    let n = session.num_tracks()?;
    for i in (0..n).rev() {
        session.delete_track(i)?;
        ms(100);
    }
    ms(300);

    session.set_tempo(78.0)?;
    let total_beats: f32 = 208.0; // 52 bars

    // ================================================================
    // TRACK 1: PETAL — Collision (elastic glass, blooming pad chords)
    // ================================================================
    println!("building track 1: PETAL (elastic glass)...");
    let petal_track = session.create_midi_track(-1)?;
    petal_track.set_name("petal")?;
    let petal_idx = petal_track.track_idx;

    session.load_instrument(petal_idx, "Collision")?;
    ms(500);

    let petal_dev = petal_track.device(0);
    // Elastic glass — two resonators with beating, high inharmonics
    petal_dev.set_param(1, 1.0)?;    // Structure = parallel (two resonators)
    petal_dev.set_param(7, 1.0)?;    // Mallet On
    petal_dev.set_param(8, 0.45)?;   // Mallet Volume
    petal_dev.set_param(11, 0.55)?;  // Stiffness — medium-hard mallet
    petal_dev.set_param(14, 0.15)?;  // Noise Amount — slight breath
    petal_dev.set_param(32, 1.0)?;   // Res 1 On
    petal_dev.set_param(33, 2.0)?;   // Marimba type
    petal_dev.set_param(34, 2.0)?;   // Quality max
    petal_dev.set_param(41, 0.92)?;  // Decay very long — petals sustain
    petal_dev.set_param(45, 0.75)?;  // Material — metallic
    petal_dev.set_param(52, 0.55)?;  // Brightness
    petal_dev.set_param(54, 0.35)?;  // Inharmonics — starts mild, will automate up
    petal_dev.set_param(56, 0.95)?;  // Opening — wide resonance
    petal_dev.set_param(64, 0.5)?;   // Volume
    petal_dev.set_param(65, 1.0)?;   // Res 2 On
    petal_dev.set_param(66, 1.0)?;   // Beam type (different from Res 1)
    petal_dev.set_param(67, 2.0)?;   // Quality max
    petal_dev.set_param(68, 5.0)?;   // Tune — 5th above for harmonic tension
    petal_dev.set_param(69, 0.25)?;  // Fine Tune — beating interval
    petal_dev.set_param(74, 0.88)?;  // Res 2 Decay — slightly shorter than Res 1
    petal_dev.set_param(78, 0.85)?;  // Res 2 Material — more metallic
    petal_dev.set_param(87, 0.4)?;   // Res 2 Inharmonics
    petal_dev.set_param(97, 0.35)?;  // Res 2 Volume

    let petal_clip = petal_track.create_clip(0, total_beats)?;
    petal_clip.set_name("petal")?;
    petal_clip.set_looping(false)?;

    let mut petal_notes = Vec::new();

    // Harmony: Dbmaj9 → Bbm11 → Gbmaj7 → Ebm9 → Abmaj7 → Fm7
    // Using MIDI notes, wide voicings, bloom stagger

    // Helper: bloom a chord (stagger entries by `gap` beats)
    fn bloom_chord(notes: &mut Vec<Note>, pitches: &[u8], start: f32, dur: f32, gap: f32, base_vel: i32) {
        for (i, &p) in pitches.iter().enumerate() {
            let t = start + i as f32 * gap;
            let d = dur - i as f32 * gap;
            // Crescendo as voices enter — the flower opens
            let vel = (base_vel + i as i32 * 5).min(127);
            if d > 0.0 {
                notes.push(Note::new(p as i32, t, d, vel));
            }
        }
    }

    // === GERMINATION (bars 1-8, beats 0-31) ===
    // A single tone rings out. Like a seed cracking.
    // Just the root — Db4 (61) — alone.
    petal_notes.push(Note::new(61, 4.0, 6.0, 55));  // bar 2: first sound

    // Second tone — a 5th above, much later. The seed is stirring.
    petal_notes.push(Note::new(68, 14.0, 5.0, 45));  // Ab4

    // Third: a cluster. Close intervals. Tight. About to open.
    petal_notes.push(Note::new(61, 24.0, 6.0, 50));  // Db4
    petal_notes.push(Note::new(63, 25.0, 5.0, 40));  // Eb4
    petal_notes.push(Note::new(68, 25.5, 4.5, 45));  // Ab4

    // === FIRST BLOOM (bars 9-16, beats 32-63) ===
    // Chords bloom open for the first time. Wide voicings across 2+ octaves.

    // Dbmaj9 bloom: Db2, Ab3, Eb4, F4, C5 = [37, 56, 63, 65, 72]
    bloom_chord(&mut petal_notes, &[37, 56, 63, 65, 72], 32.0, 14.0, 0.35, 50);

    // Bbm11 bloom: Bb1, F3, Ab3, Db4, Eb4 = [34, 53, 56, 61, 63]
    bloom_chord(&mut petal_notes, &[34, 53, 56, 61, 63], 48.0, 14.0, 0.3, 55);

    // === GROWTH (bars 17-28, beats 64-111) ===
    // Progressions with voice leading. Chords bloom and close.

    // Gbmaj7: Gb2, Db3, F3, Bb3, Db4 = [42, 49, 53, 58, 61]
    bloom_chord(&mut petal_notes, &[42, 49, 53, 58, 61], 64.0, 10.0, 0.25, 60);

    // Ebm9: Eb2, Bb3, Db4, Gb4, F4 = [39, 58, 61, 66, 65]
    // Note: F and Gb adjacent — dissonant cluster, cyborg tension
    bloom_chord(&mut petal_notes, &[39, 58, 61, 65, 66], 76.0, 10.0, 0.25, 60);

    // Abmaj7: Ab2, Eb3, G3, C4, Eb4 = [44, 51, 55, 60, 63]
    bloom_chord(&mut petal_notes, &[44, 51, 55, 60, 63], 88.0, 10.0, 0.2, 65);

    // Fm7: F2, C3, Eb3, Ab3, C4 = [41, 48, 51, 56, 60]
    bloom_chord(&mut petal_notes, &[41, 48, 51, 56, 60], 100.0, 10.0, 0.2, 65);

    // === FULL BLOOM (bars 29-40, beats 112-159) ===
    // Maximum lushness. Denser chords. Faster blooms. Harmonic surprises.

    // Dbmaj9 — now 6 voices, faster bloom
    bloom_chord(&mut petal_notes, &[37, 49, 56, 63, 65, 72], 112.0, 10.0, 0.15, 70);

    // Bbm11 — wide, 6 voices
    bloom_chord(&mut petal_notes, &[34, 46, 53, 58, 63, 68], 124.0, 10.0, 0.15, 70);

    // THE SURPRISE: E major 7 — completely outside the key
    // E maj7: E2, B2, D#3, G#3, B3 = [40, 47, 51, 56, 59]
    // This is the alien moment. The cyborg reveals itself.
    bloom_chord(&mut petal_notes, &[40, 47, 51, 56, 59], 136.0, 10.0, 0.12, 75);

    // Resolution: Dbmaj9 but inverted, voices enter from TOP down
    // (reverse bloom — flower closing then reopening)
    {
        let pitches: &[i32] = &[72, 68, 63, 56, 49, 37];
        for (i, &p) in pitches.iter().enumerate() {
            let t = 148.0 + i as f32 * 0.2;
            let d = 10.0 - i as f32 * 0.2;
            let vel = (80 - i as i32 * 5).max(45);
            petal_notes.push(Note::new(p, t, d, vel));
        }
    }

    // === SPORE (bars 41-48, beats 160-191) ===
    // Chords fragment. Individual notes ring out and die.
    // Like petals falling.

    petal_notes.push(Note::new(72, 160.0, 4.0, 60));   // C5 — high, alone
    petal_notes.push(Note::new(63, 163.0, 5.0, 50));   // Eb4
    petal_notes.push(Note::new(56, 166.0, 4.0, 45));   // Ab3
    petal_notes.push(Note::new(68, 168.0, 3.0, 40));   // Ab4 — echo
    petal_notes.push(Note::new(49, 172.0, 5.0, 40));   // Db3
    petal_notes.push(Note::new(61, 175.0, 4.0, 35));   // Db4 — octave ghost
    petal_notes.push(Note::new(37, 180.0, 6.0, 30));   // Db2 — deep root
    petal_notes.push(Note::new(63, 184.0, 5.0, 25));   // Eb4 — last shimmer

    // === SEED (bars 49-52, beats 192-207) ===
    // One final tone. The seed of the next bloom.
    petal_notes.push(Note::new(61, 194.0, 10.0, 35));  // Db4 — rings into silence

    add_notes_batched(&petal_clip, &petal_notes)?;

    // Petal automation: Inharmonics rises across the piece
    // The flower becomes more alien as it grows
    petal_clip.automate_smooth(0, 54, &[ // Res 1 Inharmonics
        (0.0, 0.2),
        (32.0, 0.3),      // first bloom
        (64.0, 0.4),      // growth
        (112.0, 0.5),     // full bloom
        (136.0, 0.65),    // the surprise — peak alien
        (148.0, 0.5),     // resolution
        (160.0, 0.6),     // spore — more metallic
        (192.0, 0.35),    // seed — return to organic
        (208.0, 0.2),
    ], 0.5)?;

    petal_clip.automate_smooth(0, 87, &[ // Res 2 Inharmonics
        (0.0, 0.25),
        (32.0, 0.35),
        (64.0, 0.45),
        (112.0, 0.55),
        (136.0, 0.7),     // peak
        (160.0, 0.55),
        (192.0, 0.3),
        (208.0, 0.2),
    ], 0.5)?;

    // Brightness rises and falls with structure
    petal_clip.automate_smooth(0, 52, &[ // Brightness
        (0.0, 0.3),
        (32.0, 0.45),
        (64.0, 0.55),
        (112.0, 0.7),
        (136.0, 0.8),     // surprise — bright
        (148.0, 0.65),
        (160.0, 0.5),     // spore
        (192.0, 0.3),
        (208.0, 0.15),    // seed — dark
    ], 0.5)?;

    println!("  done");

    // ================================================================
    // TRACK 2: STEM — Operator (FM bubble bass)
    // ================================================================
    println!("building track 2: STEM (FM bubble)...");
    let stem_track = session.create_midi_track(-1)?;
    stem_track.set_name("stem")?;
    let stem_idx = stem_track.track_idx;

    session.load_instrument(stem_idx, "Operator")?;
    ms(500);

    let stem_op = Op::new(stem_track.device(0))?;
    // FM bubble — high feedback, liquid, SOPHIE-style
    stem_op.set("Algorithm", 0.1)?;
    stem_op.set("Osc-A Level", 1.0)?;
    stem_op.set("Osc-A Wave", 0.0)?;        // Sine carrier
    stem_op.set("Osc-B On", 1.0)?;
    stem_op.set("Osc-B Level", 0.65)?;
    stem_op.set("Osc-B Wave", 0.0)?;
    stem_op.set("B Coarse", 0.6)?;          // Non-integer — liquid quality
    stem_op.set("Osc-C On", 1.0)?;
    stem_op.set("Osc-C Level", 0.5)?;
    stem_op.set("Osc-C Wave", 0.0)?;
    stem_op.set("C Coarse", 0.45)?;         // Another non-integer
    stem_op.set("Osc-D On", 1.0)?;
    stem_op.set("Osc-D Level", 0.6)?;
    stem_op.set("Osc-D Wave", 0.0)?;
    stem_op.set("D Coarse", 0.5)?;
    stem_op.set("Osc-D Feedback", 0.7)?;   // High feedback — bubbling
    stem_op.set("Filter On", 1.0)?;
    stem_op.set("Filter Freq", 0.35)?;
    stem_op.set("Filter Res", 0.55)?;       // Resonant — pronounced bubble
    stem_op.set("Ae Attack", 0.08)?;        // Slightly soft attack
    stem_op.set("Ae Decay", 0.5)?;
    stem_op.set("Ae Sustain", 0.35)?;
    stem_op.set("Ae Release", 0.3)?;

    let stem_clip = stem_track.create_clip(0, total_beats)?;
    stem_clip.set_name("stem")?;
    stem_clip.set_looping(false)?;

    let mut stem_notes = Vec::new();

    // Bass enters in Growth section. Slow, deliberate.
    // Root movement: Db → Bb → Gb → Eb → Ab → F

    // === GROWTH (bars 17-28): stem emerges ===
    // First: a single bass tone, held. Like a root breaking ground.
    stem_notes.push(Note::new(25, 64.0, 6.0, 70));    // Db1 — deep root
    stem_notes.push(Note::new(37, 67.0, 2.0, 55));    // Db2 — octave answer, delayed

    // Bb bass
    stem_notes.push(Note::new(22, 76.0, 5.0, 75));    // Bb0
    stem_notes.push(Note::new(34, 78.5, 2.0, 50));    // Bb1 bubble

    // Starts moving more. Stem growing.
    // Ab bass with rhythmic pulse
    stem_notes.push(Note::new(32, 88.0, 3.0, 80));    // Ab1
    stem_notes.push(Note::new(44, 90.0, 1.0, 55));    // Ab2
    stem_notes.push(Note::new(32, 92.0, 2.0, 75));    // Ab1 again
    stem_notes.push(Note::new(44, 94.5, 1.0, 50));    // bubble

    // F bass — chromatic interest
    stem_notes.push(Note::new(29, 100.0, 3.0, 80));   // F1
    stem_notes.push(Note::new(41, 101.5, 1.5, 55));   // F2
    stem_notes.push(Note::new(29, 104.0, 3.0, 70));   // F1
    stem_notes.push(Note::new(41, 106.0, 1.0, 50));   // F2 bubble
    stem_notes.push(Note::new(29, 108.0, 2.0, 65));   // F1 tapering

    // === FULL BLOOM (bars 29-40): bass in full rhythm ===
    // Db bass — now with syncopation
    stem_notes.push(Note::new(25, 111.75, 2.5, 95));  // anticipation!
    stem_notes.push(Note::new(37, 114.0, 0.5, 60));   // bubble
    stem_notes.push(Note::new(25, 115.0, 1.5, 90));
    stem_notes.push(Note::new(25, 117.0, 0.5, 55));   // ghost
    stem_notes.push(Note::new(37, 117.75, 0.25, 50));
    stem_notes.push(Note::new(25, 118.0, 2.0, 95));
    stem_notes.push(Note::new(25, 120.5, 0.5, 55));
    stem_notes.push(Note::new(37, 121.0, 0.5, 65));
    stem_notes.push(Note::new(25, 122.0, 1.5, 85));

    // Bb bass — driving
    stem_notes.push(Note::new(22, 123.75, 2.5, 95));  // anticipation
    stem_notes.push(Note::new(34, 126.0, 0.5, 60));
    stem_notes.push(Note::new(22, 127.0, 1.0, 90));
    stem_notes.push(Note::new(22, 128.5, 0.5, 55));
    stem_notes.push(Note::new(34, 129.0, 0.5, 65));
    stem_notes.push(Note::new(22, 130.0, 2.0, 85));
    stem_notes.push(Note::new(34, 132.5, 0.5, 60));
    stem_notes.push(Note::new(22, 133.0, 1.5, 90));

    // THE SURPRISE CHORD: E bass — jarring root
    stem_notes.push(Note::new(28, 136.0, 3.0, 100));  // E1 — foreign root
    stem_notes.push(Note::new(40, 137.5, 1.0, 65));   // E2 bubble
    stem_notes.push(Note::new(28, 140.0, 2.0, 90));   // E1
    stem_notes.push(Note::new(40, 141.0, 0.5, 55));   // E2
    stem_notes.push(Note::new(28, 142.0, 2.0, 80));
    stem_notes.push(Note::new(28, 145.0, 1.0, 70));   // fading E

    // Resolution back to Db
    stem_notes.push(Note::new(25, 147.75, 3.0, 90));  // Db1 — relief
    stem_notes.push(Note::new(37, 149.0, 1.5, 60));
    stem_notes.push(Note::new(25, 152.0, 2.0, 80));
    stem_notes.push(Note::new(37, 154.0, 1.0, 55));
    stem_notes.push(Note::new(25, 156.0, 3.0, 70));   // sustain into spore

    // === SPORE (bars 41-48): bass fragments ===
    stem_notes.push(Note::new(25, 162.0, 3.0, 55));   // Db1 — weakening
    stem_notes.push(Note::new(37, 167.0, 2.0, 45));   // Db2 — ghost
    stem_notes.push(Note::new(25, 172.0, 4.0, 40));   // long, quiet
    stem_notes.push(Note::new(25, 180.0, 6.0, 30));   // very long, barely there

    // === SEED: bass gone. The root is in the earth. ===

    add_notes_batched(&stem_clip, &stem_notes)?;

    // Stem automation: feedback increases during full bloom, alien at surprise
    if let Some(fb_idx) = stem_op.idx("Osc-D Feedback") {
        stem_clip.automate_smooth(0, fb_idx, &[
            (0.0, 0.3),
            (64.0, 0.5),      // growth entry
            (88.0, 0.6),
            (112.0, 0.7),     // full bloom
            (136.0, 0.9),     // surprise — maximum bubble
            (148.0, 0.65),    // resolution
            (160.0, 0.5),     // spore
            (192.0, 0.3),
            (208.0, 0.1),
        ], 0.5)?;
    }
    if let Some(f_idx) = stem_op.idx("Filter Freq") {
        stem_clip.automate_smooth(0, f_idx, &[
            (0.0, 0.2),
            (64.0, 0.3),
            (88.0, 0.4),
            (112.0, 0.5),
            (136.0, 0.6),     // opens for surprise
            (148.0, 0.45),
            (160.0, 0.3),
            (192.0, 0.15),
            (208.0, 0.1),
        ], 0.5)?;
    }
    if let Some(r_idx) = stem_op.idx("Filter Res") {
        stem_clip.automate_smooth(0, r_idx, &[
            (0.0, 0.3),
            (64.0, 0.45),
            (112.0, 0.55),
            (136.0, 0.7),     // pronounced resonance at surprise
            (148.0, 0.5),
            (192.0, 0.3),
            (208.0, 0.2),
        ], 0.5)?;
    }

    println!("  done");

    // ================================================================
    // TRACK 3: DEW — Operator (crystalline FM, high melodic droplets)
    // ================================================================
    println!("building track 3: DEW (crystalline drops)...");
    let dew_track = session.create_midi_track(-1)?;
    dew_track.set_name("dew")?;
    let dew_idx = dew_track.track_idx;

    session.load_instrument(dew_idx, "Operator")?;
    ms(500);

    let dew_op = Op::new(dew_track.device(0))?;
    // Crystalline — non-integer FM for bell-like drops
    dew_op.set("Algorithm", 0.15)?;
    dew_op.set("Osc-A Level", 0.8)?;
    dew_op.set("Osc-A Wave", 0.0)?;
    dew_op.set("Osc-B On", 1.0)?;
    dew_op.set("Osc-B Level", 0.35)?;
    dew_op.set("Osc-B Wave", 0.0)?;
    dew_op.set("B Coarse", 0.75)?;          // non-integer = sparkle
    dew_op.set("B Fine", 0.52)?;            // slight detuning
    dew_op.set("Osc-C On", 1.0)?;
    dew_op.set("Osc-C Level", 0.2)?;
    dew_op.set("Osc-C Wave", 0.0)?;
    dew_op.set("C Coarse", 0.6)?;
    dew_op.set("Filter On", 1.0)?;
    dew_op.set("Filter Freq", 0.7)?;        // bright — dew glistens
    dew_op.set("Filter Res", 0.2)?;
    dew_op.set("Ae Attack", 0.0)?;          // instant attack — percussive drop
    dew_op.set("Ae Decay", 0.45)?;          // medium ring
    dew_op.set("Ae Sustain", 0.15)?;        // mostly decay
    dew_op.set("Ae Release", 0.6)?;         // long tail — dew evaporating

    let dew_clip = dew_track.create_clip(0, total_beats)?;
    dew_clip.set_name("dew")?;
    dew_clip.set_looping(false)?;

    let mut dew_notes = Vec::new();

    // Dew drops are sparse, high-register, irregular.
    // They land on chord tones but at unexpected times.
    // Like water beading on glass petals.

    // Scale: Db major pentatonic = Db, Eb, F, Ab, Bb
    // High register: MIDI 73-96 (Db5 to C7)

    // === GERMINATION (bars 5-8): first drops ===
    dew_notes.push(Note::new(80, 18.0, 0.15, 50));    // Ab5 — first drop
    dew_notes.push(Note::new(85, 22.5, 0.12, 45));    // Db6
    dew_notes.push(Note::new(77, 27.0, 0.18, 40));    // F5
    dew_notes.push(Note::new(82, 30.0, 0.1, 50));     // Bb5

    // === FIRST BLOOM (bars 9-16): drops gathering ===
    // More frequent, still irregular. Some drops in pairs.
    dew_notes.push(Note::new(85, 33.0, 0.15, 55));    // Db6
    dew_notes.push(Note::new(80, 35.5, 0.12, 50));    // Ab5
    dew_notes.push(Note::new(87, 36.0, 0.1, 45));     // Eb6
    // Pair — two drops close together
    dew_notes.push(Note::new(73, 39.0, 0.2, 60));     // Db5
    dew_notes.push(Note::new(80, 39.3, 0.15, 50));    // Ab5

    dew_notes.push(Note::new(82, 42.0, 0.12, 55));    // Bb5
    dew_notes.push(Note::new(77, 44.5, 0.15, 50));    // F5
    dew_notes.push(Note::new(85, 46.0, 0.1, 60));     // Db6
    // Triple drop
    dew_notes.push(Note::new(80, 48.5, 0.12, 55));    // Ab5
    dew_notes.push(Note::new(87, 48.8, 0.1, 50));     // Eb6
    dew_notes.push(Note::new(92, 49.0, 0.08, 45));    // Ab6 — highest yet

    dew_notes.push(Note::new(73, 52.0, 0.2, 60));     // Db5
    dew_notes.push(Note::new(85, 54.0, 0.12, 55));    // Db6
    dew_notes.push(Note::new(77, 57.5, 0.15, 50));    // F5
    dew_notes.push(Note::new(80, 60.0, 0.12, 55));    // Ab5
    dew_notes.push(Note::new(82, 62.0, 0.1, 60));     // Bb5

    // === GROWTH (bars 17-28): dew becomes melody-like ===
    // Short phrases emerge from the drops. Not quite a melody,
    // but motifs that recur. The cyborg's attempt at singing.

    // Motif 1: descending 3 notes
    dew_notes.push(Note::new(92, 65.0, 0.2, 65));     // Ab6
    dew_notes.push(Note::new(87, 65.75, 0.2, 60));    // Eb6
    dew_notes.push(Note::new(85, 66.5, 0.3, 55));     // Db6

    // Space... then a single drop
    dew_notes.push(Note::new(80, 70.0, 0.15, 50));    // Ab5

    // Motif 1 again, transposed
    dew_notes.push(Note::new(89, 73.0, 0.2, 65));     // F6
    dew_notes.push(Note::new(85, 73.75, 0.2, 60));    // Db6
    dew_notes.push(Note::new(82, 74.5, 0.3, 55));     // Bb5

    // Answer: ascending
    dew_notes.push(Note::new(77, 78.0, 0.15, 60));    // F5
    dew_notes.push(Note::new(80, 78.5, 0.15, 65));    // Ab5
    dew_notes.push(Note::new(85, 79.0, 0.2, 70));     // Db6

    // Denser drops
    dew_notes.push(Note::new(82, 82.0, 0.12, 55));
    dew_notes.push(Note::new(87, 83.0, 0.15, 60));
    dew_notes.push(Note::new(73, 84.5, 0.2, 55));
    dew_notes.push(Note::new(80, 86.0, 0.12, 50));
    dew_notes.push(Note::new(92, 87.0, 0.1, 65));     // high ping

    // Cross-rhythm: drops every 1.5 beats (3-against-4)
    // Creates tension against the 4/4 pulse
    {
        let drop_pitches: &[i32] = &[85, 80, 87, 82, 77, 92, 73, 85, 80, 87];
        let mut t = 90.0;
        for (i, &p) in drop_pitches.iter().enumerate() {
            let vel = 55 + (i as i32 % 3) * 8;
            dew_notes.push(Note::new(p, t, 0.15, vel));
            t += 1.5;
        }
    }

    // === FULL BLOOM (bars 29-40): dew in full flight ===
    // Fast cascading drops. Like rain on the cyborg garden.

    // Cascading descent — 7 notes falling
    for (i, &p) in [96i32, 92, 89, 87, 85, 82, 80].iter().enumerate() {
        dew_notes.push(Note::new(p, 112.0 + i as f32 * 0.2, 0.25, 70 - i as i32 * 3));
    }

    // Paired drops — call and response between registers
    dew_notes.push(Note::new(92, 116.0, 0.15, 65));
    dew_notes.push(Note::new(73, 116.5, 0.2, 60));    // low answer
    dew_notes.push(Note::new(89, 118.0, 0.15, 65));
    dew_notes.push(Note::new(77, 118.5, 0.2, 60));

    // More cascades
    for (i, &p) in [85i32, 80, 77, 73, 68].iter().enumerate() {
        dew_notes.push(Note::new(p, 120.0 + i as f32 * 0.25, 0.3, 65 - i as i32 * 3));
    }

    // Dense cluster at the harmonic surprise
    // Use notes from E major scale: E, G#, B, D#, F# = 76, 80, 83, 87, 90
    // These clash against the Db major dew — the cyborg's confusion
    dew_notes.push(Note::new(83, 136.5, 0.2, 70));    // B5
    dew_notes.push(Note::new(90, 137.0, 0.15, 65));   // F#6
    dew_notes.push(Note::new(76, 138.0, 0.2, 70));    // E5
    dew_notes.push(Note::new(83, 139.0, 0.12, 60));   // B5
    dew_notes.push(Note::new(87, 140.0, 0.15, 65));   // Eb6/D#6 — pivot tone!
    // D# is enharmonic to Eb — the bridge between worlds
    dew_notes.push(Note::new(80, 141.5, 0.2, 60));    // Ab5/G#5 — another pivot
    dew_notes.push(Note::new(92, 143.0, 0.15, 55));   // Ab6

    // Resolution: back to Db major drops, relieved
    dew_notes.push(Note::new(85, 148.0, 0.2, 65));    // Db6
    dew_notes.push(Note::new(80, 149.0, 0.15, 60));   // Ab5
    dew_notes.push(Note::new(87, 150.5, 0.12, 55));   // Eb6
    dew_notes.push(Note::new(73, 152.0, 0.2, 60));    // Db5
    dew_notes.push(Note::new(82, 154.0, 0.15, 55));   // Bb5
    dew_notes.push(Note::new(85, 156.0, 0.12, 50));   // Db6
    dew_notes.push(Note::new(80, 158.0, 0.15, 45));   // Ab5

    // === SPORE (bars 41-48): drops slow down ===
    dew_notes.push(Note::new(92, 162.0, 0.15, 45));   // Ab6
    dew_notes.push(Note::new(85, 166.0, 0.2, 40));    // Db6
    dew_notes.push(Note::new(80, 172.0, 0.25, 35));   // Ab5
    dew_notes.push(Note::new(87, 178.0, 0.15, 30));   // Eb6
    dew_notes.push(Note::new(85, 186.0, 0.2, 25));    // Db6 — last drop

    // === SEED: silence. The dew has evaporated. ===

    add_notes_batched(&dew_clip, &dew_notes)?;

    // Dew automation: FM depth increases during bloom
    if let Some(b_idx) = dew_op.idx("Osc-B Level") {
        dew_clip.automate_smooth(0, b_idx, &[
            (0.0, 0.2),
            (32.0, 0.3),
            (64.0, 0.35),
            (112.0, 0.45),     // full bloom
            (136.0, 0.55),     // surprise — more metallic
            (160.0, 0.3),
            (192.0, 0.15),
            (208.0, 0.1),
        ], 0.5)?;
    }

    println!("  done");

    // ================================================================
    // TRACK 4: POLLEN — Wavetable (subatomic micro-grain clouds)
    // ================================================================
    println!("building track 4: POLLEN (subatomic texture)...");
    let pollen_track = session.create_midi_track(-1)?;
    pollen_track.set_name("pollen")?;
    let pollen_idx = pollen_track.track_idx;

    session.load_instrument(pollen_idx, "Wavetable")?;
    ms(500);

    let pollen_dev = pollen_track.device(0);
    // Subatomic — ultra-short envelope, micro-notes become timbre
    pollen_dev.set_param(4, 0.5)?;    // Osc 1 Pos
    pollen_dev.set_param(5, 0.6)?;    // Effect 1
    pollen_dev.set_param(9, 1.0)?;    // Osc 2 On
    pollen_dev.set_param(12, 0.3)?;   // Osc 2 Pos
    pollen_dev.set_param(16, 0.5)?;   // Osc 2 Gain
    pollen_dev.set_param(26, 0.55)?;  // Filter Freq
    pollen_dev.set_param(27, 0.25)?;  // Filter Res
    pollen_dev.set_param(39, 0.0)?;   // Attack instant
    pollen_dev.set_param(40, 0.08)?;  // Decay very short
    pollen_dev.set_param(45, 0.02)?;  // Sustain near zero
    pollen_dev.set_param(41, 0.04)?;  // Release tiny
    pollen_dev.set_param(89, 0.2)?;   // Unison for thickness

    let pollen_clip = pollen_track.create_clip(0, total_beats)?;
    pollen_clip.set_name("pollen")?;
    pollen_clip.set_looping(false)?;

    let mut pollen_notes = Vec::new();

    // Pollen: dense clouds of micro-notes (0.04-0.08 beat duration, ~0.06 apart)
    // Notes so fast they blur into timbre. Like granular synthesis.
    // Different pitch clusters = different "colors" of pollen.

    // Helper: generate a pollen cloud at a given pitch cluster
    fn pollen_cloud(notes: &mut Vec<Note>, pitches: &[i32], start: f32, cloud_dur: f32, density: f32, vel: i32) {
        let step = density;
        let mut t = start;
        let mut i = 0;
        while t < start + cloud_dur {
            let p = pitches[i % pitches.len()];
            notes.push(Note::new(p, t, 0.05, vel + (i as i32 % 5) * 3));
            t += step;
            i += 1;
        }
    }

    // === GROWTH (bars 21-28): first pollen clouds ===
    // Sparse, short clouds. Testing the air.
    pollen_cloud(&mut pollen_notes, &[73, 80, 85], 82.0, 1.5, 0.06, 40);
    pollen_cloud(&mut pollen_notes, &[68, 75, 80], 90.0, 2.0, 0.07, 45);
    pollen_cloud(&mut pollen_notes, &[65, 72, 77], 98.0, 1.5, 0.06, 40);
    pollen_cloud(&mut pollen_notes, &[73, 80, 85], 106.0, 2.0, 0.07, 45);

    // === FULL BLOOM (bars 29-40): pollen EVERYWHERE ===
    // Dense, overlapping clouds. Multiple pitch centers.

    // Cloud bursts between chords
    pollen_cloud(&mut pollen_notes, &[73, 80, 85, 87], 113.0, 3.0, 0.05, 50);
    pollen_cloud(&mut pollen_notes, &[68, 75, 80, 82], 118.0, 2.5, 0.06, 50);

    // Big cloud during Bbm section
    pollen_cloud(&mut pollen_notes, &[70, 75, 82, 87], 125.0, 4.0, 0.05, 55);

    // Huge cloud during bridge chord
    pollen_cloud(&mut pollen_notes, &[70, 77, 82, 85], 131.0, 3.0, 0.06, 50);

    // Surprise chord: E major pollen — dissonant against everything
    // E major tones: E, G#, B = 76, 80, 83 — but shifted high
    pollen_cloud(&mut pollen_notes, &[76, 83, 88, 92], 137.0, 4.0, 0.04, 55);

    // Resolution cloud — Db tones, calmer
    pollen_cloud(&mut pollen_notes, &[73, 80, 85, 92], 144.0, 3.0, 0.06, 50);

    // Final bloom clouds — dense and shimmering
    pollen_cloud(&mut pollen_notes, &[73, 77, 80, 85], 149.0, 4.0, 0.05, 50);
    pollen_cloud(&mut pollen_notes, &[68, 73, 80, 87], 155.0, 3.0, 0.06, 45);

    // === SPORE (bars 41-48): pollen drifts, thins ===
    pollen_cloud(&mut pollen_notes, &[80, 85, 92], 162.0, 2.0, 0.08, 40);
    pollen_cloud(&mut pollen_notes, &[73, 80, 87], 170.0, 1.5, 0.09, 35);
    pollen_cloud(&mut pollen_notes, &[85, 92], 178.0, 1.0, 0.1, 30);
    // Last wisps
    pollen_cloud(&mut pollen_notes, &[80, 87], 186.0, 0.8, 0.12, 25);

    add_notes_batched(&pollen_clip, &pollen_notes)?;

    // Pollen automation: wavetable position sweeps during clouds
    pollen_clip.automate_smooth(0, 4, &[ // Osc 1 Position
        (0.0, 0.3),
        (82.0, 0.4),
        (112.0, 0.5),
        (125.0, 0.6),
        (137.0, 0.8),     // surprise — extreme position
        (148.0, 0.5),
        (160.0, 0.4),
        (192.0, 0.2),
        (208.0, 0.1),
    ], 0.5)?;

    pollen_clip.automate_smooth(0, 26, &[ // Filter Freq
        (0.0, 0.4),
        (82.0, 0.5),
        (112.0, 0.6),
        (137.0, 0.8),
        (160.0, 0.5),
        (192.0, 0.3),
        (208.0, 0.2),
    ], 0.5)?;

    println!("  done");

    // ================================================================
    // TRACK 5: WIRE — Analog (mechanical rhythm, the cyborg's pulse)
    // ================================================================
    println!("building track 5: WIRE (cyborg pulse)...");
    let wire_track = session.create_midi_track(-1)?;
    wire_track.set_name("wire")?;
    let wire_idx = wire_track.track_idx;

    session.load_instrument(wire_idx, "Analog")?;
    ms(400);

    let wire_dev = wire_track.device(0);
    // Thin, metallic, clicky — not a normal synth perc.
    // Very high-frequency, resonant, short. Like electricity.
    wire_dev.set_param(38, 1.0)?;    // OSC1 On
    wire_dev.set_param(39, 2.0)?;    // Square wave — harsh
    wire_dev.set_param(40, 2.0)?;    // Two octaves UP — high and thin
    wire_dev.set_param(52, 0.2)?;    // OSC1 Level low
    wire_dev.set_param(34, 1.0)?;    // Noise On
    wire_dev.set_param(35, 0.8)?;    // Noise Color — bright noise
    wire_dev.set_param(37, 0.4)?;    // Noise Level
    wire_dev.set_param(105, 0.0)?;   // OSC2 Off
    wire_dev.set_param(53, 1.0)?;    // Filter On
    wire_dev.set_param(54, 1.0)?;    // HP filter — only highs pass
    wire_dev.set_param(57, 0.6)?;    // Filter Freq — cuts lows
    wire_dev.set_param(59, 0.65)?;   // Resonance — pronounced ring
    wire_dev.set_param(62, 0.5)?;    // Filter Env amount
    wire_dev.set_param(71, 0.08)?;   // FEG Decay — very short
    wire_dev.set_param(73, 0.0)?;    // FEG Sustain zero
    wire_dev.set_param(89, 0.0)?;    // Attack instant
    wire_dev.set_param(90, 0.1)?;    // Decay very short
    wire_dev.set_param(92, 0.0)?;    // Sustain zero
    wire_dev.set_param(94, 0.05)?;   // Release minimal

    let wire_clip = wire_track.create_clip(0, total_beats)?;
    wire_clip.set_name("wire")?;
    wire_clip.set_looping(false)?;

    let mut wire_notes = Vec::new();

    // Wire is the mechanical heartbeat. Euclidean-derived patterns
    // that shift and phase against the organic elements.
    // Uses different pitches for different timbral colors:
    // 60 = mid click, 72 = high tick, 84 = ultra-high ping, 48 = low thud

    // === FIRST BLOOM (bars 13-16): wire wakes up ===
    // Isolated ticks. Like a machine powering on.
    wire_notes.push(Note::new(72, 50.0, 0.04, 45));
    wire_notes.push(Note::new(72, 53.0, 0.04, 50));
    wire_notes.push(Note::new(84, 55.5, 0.03, 40));
    wire_notes.push(Note::new(72, 58.0, 0.04, 55));
    wire_notes.push(Note::new(60, 60.0, 0.05, 50));
    wire_notes.push(Note::new(72, 62.5, 0.04, 50));

    // === GROWTH (bars 17-28): euclidean patterns emerge ===
    // Euclidean(5,8) at different phases per 4-bar group

    // Group A (bars 17-20): E(5,8) — [x.x.xx.x]
    let e58: [bool; 8] = [true, false, true, false, true, true, false, true];
    for bar in 0..4 {
        let b = 64.0 + bar as f32 * 4.0;
        for (i, &hit) in e58.iter().enumerate() {
            if hit {
                let t = b + i as f32 * 0.5;
                let vel = if i == 0 { 80 } else { 60 };
                wire_notes.push(Note::new(72, t, 0.04, vel));
            }
        }
    }

    // Group B (bars 21-24): E(3,8) — [x..x..x.] — sparser, contrasting
    let e38: [bool; 8] = [true, false, false, true, false, false, true, false];
    for bar in 0..4 {
        let b = 80.0 + bar as f32 * 4.0;
        for (i, &hit) in e38.iter().enumerate() {
            if hit {
                let t = b + i as f32 * 0.5;
                wire_notes.push(Note::new(60, t, 0.05, 65));
            }
        }
        // Add high ping on beat 3 — accent
        wire_notes.push(Note::new(84, b + 2.0, 0.03, 55));
    }

    // Group C (bars 25-28): E(7,12) over 3 beats — 7 in 12 sixteenths
    // Creates a loping 3/4 feel against 4/4
    let e712: [bool; 12] = [true, false, true, true, false, true, false, true, true, false, true, false];
    for bar in 0..4 {
        let b = 96.0 + bar as f32 * 4.0;
        for (i, &hit) in e712.iter().enumerate() {
            if hit {
                let t = b + i as f32 * 0.25; // 16th note grid
                let pitch = if i % 4 == 0 { 60 } else { 72 };
                wire_notes.push(Note::new(pitch, t, 0.04, 60 + (i as i32 % 3) * 5));
            }
        }
    }

    // === FULL BLOOM (bars 29-40): wire at maximum complexity ===

    // Phase 1 (bars 29-34): two euclidean layers stacked
    // E(5,8) on 72 + E(3,8) on 60 = polymetric wire
    for bar in 0..6 {
        let b = 112.0 + bar as f32 * 4.0;
        // Layer 1: E(5,8)
        for (i, &hit) in e58.iter().enumerate() {
            if hit {
                let t = b + i as f32 * 0.5;
                wire_notes.push(Note::new(72, t, 0.04, 70));
            }
        }
        // Layer 2: E(3,8) offset by 1 16th — phasing
        for (i, &hit) in e38.iter().enumerate() {
            if hit {
                let t = b + i as f32 * 0.5 + 0.125; // offset!
                wire_notes.push(Note::new(60, t, 0.04, 55));
            }
        }
    }

    // Surprise chord section: wire goes haywire
    // Rapid irregular bursts — the machine glitching
    {
        let glitch_times: &[f32] = &[
            136.0, 136.12, 136.25, 136.5,
            137.0, 137.08, 137.25,
            138.0, 138.06, 138.12, 138.25, 138.5, 138.62,
            139.5, 139.56, 139.62, 139.75, 139.88,
            140.0, 140.25,
            141.0, 141.12, 141.25, 141.5, 141.75,
            142.0,
            143.0, 143.5,
            144.0, 144.12, 144.25,
            145.0,
        ];
        for (i, &t) in glitch_times.iter().enumerate() {
            let pitch = [72, 84, 60, 72, 84, 60, 72][i % 7];
            let vel = 50 + (i as i32 % 4) * 10;
            wire_notes.push(Note::new(pitch, t, 0.03, vel));
        }
    }

    // Phase 2 (bars 38-40): wire accelerates then cuts
    // Accelerating clicks — fill to spore
    for i in 0..16 {
        let t = 148.0 + (0..i).map(|j| 0.25 - j as f32 * 0.01).sum::<f32>();
        if t < 158.0 {
            wire_notes.push(Note::new(72, t, 0.03, 55 + i * 3));
        }
    }

    // Hard cut before spore — 2 beats of silence, then...
    // Single isolated ticks in spore
    wire_notes.push(Note::new(84, 162.0, 0.03, 60));

    // === SPORE (bars 41-48): wire sputters and dies ===
    wire_notes.push(Note::new(72, 166.0, 0.04, 45));
    wire_notes.push(Note::new(84, 170.0, 0.03, 40));
    wire_notes.push(Note::new(60, 175.0, 0.05, 35));
    wire_notes.push(Note::new(72, 180.0, 0.04, 30));
    wire_notes.push(Note::new(84, 188.0, 0.03, 25));  // last tick

    add_notes_batched(&wire_clip, &wire_notes)?;

    // Wire automation: filter freq rises during bloom, creating higher pitch
    wire_clip.automate_smooth(0, 57, &[ // Filter Freq
        (0.0, 0.4),
        (50.0, 0.5),
        (64.0, 0.55),
        (96.0, 0.6),
        (112.0, 0.7),
        (136.0, 0.85),     // glitch — filter wide open
        (148.0, 0.65),
        (160.0, 0.5),
        (192.0, 0.3),
        (208.0, 0.2),
    ], 0.5)?;

    // Resonance peaks at surprise
    wire_clip.automate_smooth(0, 59, &[ // Resonance
        (0.0, 0.5),
        (112.0, 0.6),
        (136.0, 0.8),      // PEAK — self-oscillating almost
        (148.0, 0.55),
        (192.0, 0.4),
        (208.0, 0.3),
    ], 0.5)?;

    println!("  done");

    // ================================================================
    // TRACK 6: ROOT — Analog (sub bass foundation)
    // ================================================================
    println!("building track 6: ROOT (sub)...");
    let root_track = session.create_midi_track(-1)?;
    root_track.set_name("root")?;
    let root_idx = root_track.track_idx;

    session.load_instrument(root_idx, "Analog")?;
    ms(400);

    let root_dev = root_track.device(0);
    // Pure sine sub — two octaves down, no filter
    root_dev.set_param(38, 1.0)?;    // OSC1 On
    root_dev.set_param(39, 0.0)?;    // Sine
    root_dev.set_param(40, -2.0)?;   // Two octaves down
    root_dev.set_param(105, 0.0)?;   // OSC2 Off
    root_dev.set_param(34, 0.0)?;    // Noise Off
    root_dev.set_param(53, 0.0)?;    // Filter Off
    root_dev.set_param(89, 0.15)?;   // Slow attack — sub breathes in
    root_dev.set_param(90, 0.7)?;    // Decay
    root_dev.set_param(92, 0.85)?;   // Sustain high
    root_dev.set_param(94, 0.6)?;    // Release long

    let root_clip = root_track.create_clip(0, total_beats)?;
    root_clip.set_name("root")?;
    root_clip.set_looping(false)?;

    let mut root_notes = Vec::new();

    // Sub is almost inaudible but FELT. It grounds everything.
    // Very long held tones following the harmonic roots.

    // === GERMINATION (bars 3-8): sub drone begins ===
    root_notes.push(Note::new(37, 8.0, 22.0, 60));    // Db2 — long drone

    // === FIRST BLOOM (bars 9-16): follows chord roots ===
    root_notes.push(Note::new(37, 32.0, 15.0, 65));   // Db2
    root_notes.push(Note::new(34, 48.0, 15.0, 65));   // Bb1

    // === GROWTH: follows progression ===
    root_notes.push(Note::new(30, 64.0, 11.0, 70));   // Gb1
    root_notes.push(Note::new(27, 76.0, 11.0, 70));   // Eb1
    root_notes.push(Note::new(32, 88.0, 11.0, 70));   // Ab1
    root_notes.push(Note::new(29, 100.0, 11.0, 70));   // F1

    // === FULL BLOOM: stronger sub ===
    root_notes.push(Note::new(37, 112.0, 11.0, 80));  // Db2
    root_notes.push(Note::new(34, 124.0, 11.0, 80));  // Bb1
    root_notes.push(Note::new(28, 136.0, 11.0, 85));  // E1 — surprise root
    root_notes.push(Note::new(37, 148.0, 11.0, 75));  // Db2 — resolution

    // === SPORE: sub fades ===
    root_notes.push(Note::new(37, 160.0, 12.0, 55));  // Db2
    root_notes.push(Note::new(37, 174.0, 14.0, 40));  // very quiet

    // === SEED: sub is the very last thing ===
    root_notes.push(Note::new(37, 192.0, 14.0, 30));  // barely perceptible Db2

    add_notes_batched(&root_clip, &root_notes)?;
    println!("  done");

    // ================================================================
    // Load Spectral Time on POLLEN track as audio effect
    // ================================================================
    println!("adding Spectral Time to pollen track...");
    session.load_effect(pollen_idx, "Spectral Time")?;
    ms(500);
    // Spectral Time is now device 1 (after Wavetable at device 0)
    let spectral_dev = pollen_track.device(1);
    spectral_dev.set_param(14, 0.35)?;   // Delay Time
    spectral_dev.set_param(18, 0.45)?;   // Delay Feedback
    spectral_dev.set_param(19, 0.6)?;    // Delay Tilt — different freqs delayed differently
    spectral_dev.set_param(20, 0.3)?;    // Delay Spray — randomize per frequency bin
    spectral_dev.set_param(22, 0.7)?;    // Stereo Spread
    spectral_dev.set_param(23, 0.15)?;   // Frequency Shift — slight shimmer
    spectral_dev.set_param(24, 0.5)?;    // Delay Mix
    spectral_dev.set_param(26, 0.55)?;   // Dry/Wet

    println!("  done");

    // ================================================================
    // PLAYBACK
    // ================================================================
    println!("\n--- BLOOM: blooming cyborg flowers ---");
    println!("78 BPM, {} beats ({} bars)\n", total_beats as i32, total_beats as i32 / 4);
    println!("tracks:");
    println!("  1. PETAL  — Collision elastic glass (blooming pad chords)");
    println!("  2. STEM   — Operator FM bubble (liquid bass)");
    println!("  3. DEW    — Operator crystalline (bell-like droplets)");
    println!("  4. POLLEN — Wavetable + Spectral Time (micro-grain clouds)");
    println!("  5. WIRE   — Analog HP filtered (mechanical pulse)");
    println!("  6. ROOT   — Analog sine sub (felt, not heard)");
    println!("\nplaying...");

    session.set_time(0.0)?;
    ms(200);
    session.play()?;

    Ok(())
}
