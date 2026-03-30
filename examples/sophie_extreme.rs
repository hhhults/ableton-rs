/// SOPHIE extreme — pushing synthesis into uncharted territory.
///
/// Five sounds that shouldn't exist:
///   1. Liquid Metal — FM feedback loop that eats itself
///   2. Elastic Glass — collision model of an impossible material
///   3. Subatomic — wavetable granular stutter at the edge of perception
///   4. Breathing Geometry — two Operators phase-locked into alien harmonics
///   5. The Drop — everything converging into one impossible moment

use ableton::{Note, Session};
use std::thread;
use std::time::Duration;

fn wait(secs: u64) { thread::sleep(Duration::from_secs(secs)); }
fn ms(millis: u64) { thread::sleep(Duration::from_millis(millis)); }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::connect()?;
    println!("connected\n");

    // Clean slate
    let n = session.num_tracks()?;
    for i in (1..n).rev() {
        session.delete_track(i)?;
        ms(100);
    }
    let t0 = session.track(0);
    while t0.num_devices()? > 0 {
        t0.delete_device(0)?;
        ms(150);
    }
    for s in 0..8 {
        if t0.has_clip(s).unwrap_or(false) { t0.delete_clip(s)?; }
    }
    ms(200);

    session.set_tempo(135.0)?;

    // ================================================================
    // 1. LIQUID METAL
    // ================================================================
    println!("╔══════════════════════════════════════════════╗");
    println!("║  1. LIQUID METAL                             ║");
    println!("║  FM operator feeding back into itself —       ║");
    println!("║  the sound has mass and viscosity             ║");
    println!("╚══════════════════════════════════════════════╝\n");

    session.load_instrument(0, "Operator")?;
    ms(500);
    let op = t0.device(0);
    let params = op.parameters()?;
    let pm: std::collections::HashMap<String, usize> = params.iter()
        .map(|p| (p.name.clone(), p.index)).collect();

    let set = |name: &str, val: f32| -> Result<(), Box<dyn std::error::Error>> {
        if let Some(&idx) = pm.get(name) { op.set_param(idx as i32, val)?; }
        Ok(())
    };

    // Algorithm with feedback — D feeds back into itself while modulating C
    // C modulates B, B modulates A. A cascade of FM.
    set("Algorithm", 0.1)?;

    // A = carrier, pure sine, this is what we hear
    set("Osc-A Level", 1.0)?;
    set("Osc-A Wave", 0.0)?;
    set("A Coarse", 0.5)?;  // 1:1

    // B = first modulator, slightly detuned ratio
    set("Osc-B On", 1.0)?;
    set("Osc-B Level", 0.6)?;
    set("Osc-B Wave", 0.0)?;
    set("B Coarse", 0.68)?;  // irrational-ish ratio — metallic

    // C = deeper modulator
    set("Osc-C On", 1.0)?;
    set("Osc-C Level", 0.45)?;
    set("Osc-C Wave", 0.0)?;
    set("C Coarse", 0.33)?;  // sub-harmonic modulation

    // D = self-modulating feedback oscillator
    set("Osc-D On", 1.0)?;
    set("Osc-D Level", 0.55)?;
    set("Osc-D Wave", 0.0)?;
    set("D Coarse", 0.82)?;  // another irrational ratio
    set("Osc-D Feedback", 0.6)?;  // THE KEY — self-modulation creates chaos

    // Filter: resonant, will sweep
    set("Filter On", 1.0)?;
    set("Filter Freq", 0.4)?;
    set("Filter Res", 0.55)?;

    // Envelope: medium attack — sound blooms
    set("Ae Attack", 0.15)?;
    set("Ae Decay", 0.6)?;
    set("Ae Sustain", 0.5)?;
    set("Ae Release", 0.45)?;

    // Pitch env — slight downward bend on attack
    set("Pe Init", 0.6)?;
    set("Pe Peak", 0.55)?;
    set("Pe Time", 0.2)?;

    let clip = t0.create_clip(0, 16.0)?;
    clip.set_name("liquid metal")?;
    clip.set_looping(true)?;

    // Pattern: sparse low notes — let the timbre speak
    let notes = vec![
        Note::new(30, 0.0, 2.0, 120),
        Note::new(30, 3.0, 0.5, 100), Note::new(30, 3.5, 0.25, 80),
        Note::new(30, 4.0, 3.0, 115),
        Note::new(42, 7.5, 0.5, 105),
        Note::new(30, 8.0, 2.0, 120),
        Note::new(35, 10.5, 0.5, 95), Note::new(30, 11.0, 1.0, 110),
        Note::new(30, 12.5, 0.25, 90), Note::new(30, 12.75, 0.25, 85),
        Note::new(30, 13.0, 3.0, 125),
    ];
    clip.add_notes(&notes)?;

    // Automate: FM depth morphs — the metal liquefies and solidifies
    if let Some(&idx) = pm.get("Osc-B Level") {
        clip.automate_smooth(0, idx as i32, &[
            (0.0, 0.3), (4.0, 0.85), (8.0, 0.2), (12.0, 0.95), (16.0, 0.3),
        ], 0.125)?;
    }
    if let Some(&idx) = pm.get("Osc-D Feedback") {
        clip.automate_smooth(0, idx as i32, &[
            (0.0, 0.3), (4.0, 0.7), (8.0, 0.4), (12.0, 0.85), (16.0, 0.3),
        ], 0.125)?;
    }
    // Filter sweep — reveal different harmonics
    if let Some(&idx) = pm.get("Filter Freq") {
        clip.automate_smooth(0, idx as i32, &[
            (0.0, 0.15), (4.0, 0.7), (6.0, 0.2), (10.0, 0.85),
            (13.0, 0.1), (15.0, 0.6), (16.0, 0.15),
        ], 0.125)?;
    }

    clip.fire()?;
    session.play()?;
    println!("   the metal flows...\n");
    wait(16);

    // ================================================================
    // 2. ELASTIC GLASS
    // ================================================================
    println!("╔══════════════════════════════════════════════╗");
    println!("║  2. ELASTIC GLASS                            ║");
    println!("║  collision model of a material that can't     ║");
    println!("║  exist — glass that stretches instead of      ║");
    println!("║  breaking, hit with a mallet made of light    ║");
    println!("╚══════════════════════════════════════════════╝\n");

    session.stop()?;
    ms(300);

    // Remove Operator, load Collision
    t0.delete_device(0)?;
    ms(200);
    t0.delete_clip(0)?;
    ms(100);

    session.load_instrument(0, "Collision")?;
    ms(500);
    let col = t0.device(0);

    // Mallet: impossibly soft AND bright at the same time
    col.set_param(7, 1.0)?;     // Mallet On
    col.set_param(8, 0.7)?;     // Mallet Volume
    col.set_param(9, 0.6)?;     // Volume < Vel
    col.set_param(11, 0.9)?;    // Stiffness = VERY hard
    col.set_param(12, 0.5)?;    // Stiffness < Vel
    col.set_param(14, 0.4)?;    // Noise Amount
    col.set_param(17, 0.9)?;    // Noise Color = bright

    // Resonator 1: "glass" — high inharmonics, very long decay
    col.set_param(32, 1.0)?;    // Res 1 On
    col.set_param(33, 2.0)?;    // Type = Marimba (tubular resonance)
    col.set_param(34, 2.0)?;    // Quality = max
    col.set_param(41, 0.92)?;   // Decay = extremely long
    col.set_param(45, 0.85)?;   // Material = very metallic
    col.set_param(48, 0.2)?;    // Radius = near the edge
    col.set_param(52, 0.7)?;    // Brightness
    col.set_param(54, 0.7)?;    // Inharmonics = HIGH — alien partials
    col.set_param(56, 1.0)?;    // Opening = fully open
    col.set_param(64, 0.65)?;   // Volume

    // Resonator 2: tuned to create interference with Res 1
    col.set_param(65, 1.0)?;    // Res 2 On
    col.set_param(66, 1.0)?;    // Type = Beam
    col.set_param(67, 2.0)?;    // Quality = max
    col.set_param(68, 5.0)?;    // Tune = a fifth up
    col.set_param(69, 0.3)?;    // Fine Tune = microtonal offset — beating
    col.set_param(74, 0.85)?;   // Decay = long
    col.set_param(78, 0.9)?;    // Material = metallic
    col.set_param(81, 0.6)?;    // Radius
    col.set_param(85, 0.6)?;    // Brightness
    col.set_param(87, 0.6)?;    // Inharmonics = high
    col.set_param(89, 0.95)?;   // Opening
    col.set_param(97, 0.5)?;    // Volume

    // Pitch envelope: each hit bends downward — like glass sagging
    col.set_param(38, 0.35)?;   // Res 1 Pitch Env
    col.set_param(40, 0.6)?;    // Pitch Env Time

    // Structure: parallel — both resonators excited simultaneously
    col.set_param(1, 1.0)?;     // Structure

    let clip = t0.create_clip(0, 16.0)?;
    clip.set_name("elastic glass")?;
    clip.set_looping(true)?;

    // Pattern: let each note ring, use the full pitch range
    let notes = vec![
        Note::new(72, 0.0, 2.0, 100),
        Note::new(84, 2.5, 1.0, 85),
        Note::new(67, 4.0, 3.0, 110),
        Note::new(79, 7.0, 0.5, 75), Note::new(76, 7.5, 0.5, 70),
        Note::new(60, 8.0, 3.0, 115),
        Note::new(72, 11.0, 0.5, 90), Note::new(74, 11.5, 0.5, 85),
        Note::new(55, 12.0, 2.0, 105),
        Note::new(67, 14.0, 2.0, 95),
    ];
    clip.add_notes(&notes)?;

    // Automate inharmonics — the glass becomes more alien over time
    clip.automate_smooth(0, 54, &[
        (0.0, 0.3), (4.0, 0.7), (8.0, 0.4), (12.0, 0.9), (16.0, 0.3),
    ], 0.25)?;

    clip.fire()?;
    session.play()?;
    println!("   glass bending...\n");
    wait(16);

    // ================================================================
    // 3. SUBATOMIC
    // ================================================================
    println!("╔══════════════════════════════════════════════╗");
    println!("║  3. SUBATOMIC                                ║");
    println!("║  wavetable at micro-timescales —              ║");
    println!("║  notes so fast they become timbre,            ║");
    println!("║  rhythm dissolves into texture                ║");
    println!("╚══════════════════════════════════════════════╝\n");

    session.stop()?;
    ms(300);
    t0.delete_device(0)?;
    ms(200);
    t0.delete_clip(0)?;
    ms(100);

    session.set_tempo(140.0)?;

    session.load_instrument(0, "Wavetable")?;
    ms(500);
    let wt = t0.device(0);

    // Sharp, clicky wavetable — short envelope, digital texture
    wt.set_param(4, 0.6)?;     // Osc 1 Pos = complex region
    wt.set_param(5, 0.7)?;     // Osc 1 Effect 1
    wt.set_param(9, 1.0)?;     // Osc 2 On
    wt.set_param(12, 0.35)?;   // Osc 2 Pos = different region
    wt.set_param(16, 0.7)?;    // Osc 2 Gain
    wt.set_param(10, 12.0)?;   // Osc 2 Transpose = octave up
    wt.set_param(26, 0.7)?;    // Filter Freq
    wt.set_param(27, 0.35)?;   // Filter Res
    wt.set_param(39, 0.0)?;    // Amp Attack = instant
    wt.set_param(40, 0.15)?;   // Amp Decay = very short
    wt.set_param(45, 0.05)?;   // Amp Sustain = almost zero
    wt.set_param(41, 0.08)?;   // Amp Release = tiny

    let clip = t0.create_clip(0, 8.0)?;
    clip.set_name("subatomic")?;
    clip.set_looping(true)?;

    // MICRO-NOTES: 64th notes — so fast they become a texture
    // The pitch pattern creates a timbre when played this fast
    let mut notes = Vec::new();
    let scale = [48, 51, 55, 58, 60, 63, 67, 70, 72, 75, 79, 82];
    for i in 0..128 {
        let start = i as f32 * 0.0625; // 64th notes
        let pitch_idx = match i % 16 {
            0..=3 => i % 4,
            4..=7 => 4 + (i % 3),
            8..=11 => 7 + (i % 4),
            _ => 11 - (i % 5),
        };
        let pitch = scale[pitch_idx % scale.len()];
        let vel = 60 + ((i * 13 + 7) % 67) as i32;
        notes.push(Note::new(pitch, start, 0.05, vel));
    }
    clip.add_notes(&notes)?;

    // Automate wavetable position — the texture morphs
    clip.automate_smooth(0, 4, &[
        (0.0, 0.2), (2.0, 0.8), (4.0, 0.1), (6.0, 0.9), (8.0, 0.2),
    ], 0.0625)?;

    // Filter sweep reveals different spectral slices of the micro-pattern
    clip.automate_smooth(0, 26, &[
        (0.0, 0.1), (2.0, 0.9), (3.0, 0.2), (5.0, 0.85), (7.0, 0.15), (8.0, 0.5),
    ], 0.0625)?;

    clip.fire()?;
    session.play()?;
    println!("   particles...\n");
    wait(14);

    // ================================================================
    // 4. BREATHING GEOMETRY
    // ================================================================
    println!("╔══════════════════════════════════════════════╗");
    println!("║  4. BREATHING GEOMETRY                       ║");
    println!("║  two tracks of Operator phase-locked into     ║");
    println!("║  complementary alien harmonics — each one     ║");
    println!("║  incomplete, together they form a shape       ║");
    println!("║  that rotates in spectral space               ║");
    println!("╚══════════════════════════════════════════════╝\n");

    session.stop()?;
    ms(300);
    session.set_tempo(100.0)?;

    // Keep Wavetable on track 0, add Operator on track 1
    let t1 = session.create_midi_track(-1)?;
    t1.set_name("geometry B")?;
    let t1_idx = t1.track_idx;

    session.load_instrument(t1_idx, "Operator")?;
    ms(500);
    let op2 = t1.device(0);
    let params2 = op2.parameters()?;
    let pm2: std::collections::HashMap<String, usize> = params2.iter()
        .map(|p| (p.name.clone(), p.index)).collect();

    let set2 = |name: &str, val: f32| -> Result<(), Box<dyn std::error::Error>> {
        if let Some(&idx) = pm2.get(name) { op2.set_param(idx as i32, val)?; }
        Ok(())
    };

    // Wavetable side: evolving pad with sub
    wt.set_param(4, 0.4)?;
    wt.set_param(9, 1.0)?;
    wt.set_param(12, 0.7)?;
    wt.set_param(16, 0.5)?;
    wt.set_param(17, 1.0)?;    // Sub On
    wt.set_param(19, 0.4)?;    // Sub Gain
    wt.set_param(26, 0.55)?;   // Filter
    wt.set_param(27, 0.2)?;    // Res
    wt.set_param(39, 0.5)?;    // Attack = slow bloom
    wt.set_param(40, 0.8)?;    // Decay = long
    wt.set_param(45, 0.6)?;    // Sustain
    wt.set_param(41, 0.7)?;    // Release = long
    wt.set_param(89, 0.35)?;   // Unison

    // Operator side: crystalline FM harmonics
    set2("Algorithm", 0.2)?;
    set2("Osc-A Level", 0.8)?;
    set2("Osc-A Wave", 0.0)?;
    set2("Osc-B On", 1.0)?;
    set2("Osc-B Level", 0.55)?;
    set2("B Coarse", 0.72)?;   // non-integer — shimmery
    set2("B Fine", 0.52)?;
    set2("Osc-C On", 1.0)?;
    set2("Osc-C Level", 0.35)?;
    set2("C Coarse", 0.58)?;   // another weird ratio
    set2("Filter On", 1.0)?;
    set2("Filter Freq", 0.5)?;
    set2("Filter Res", 0.4)?;
    set2("Ae Attack", 0.35)?;
    set2("Ae Decay", 0.75)?;
    set2("Ae Sustain", 0.55)?;
    set2("Ae Release", 0.65)?;

    // Clips: same chord progression on both, but Operator is inverted voicing
    t0.delete_clip(0)?;
    ms(100);

    let clip_a = t0.create_clip(0, 16.0)?;
    clip_a.set_name("geometry A")?;
    clip_a.set_looping(true)?;

    // Wavetable: low voicing
    let notes_a = vec![
        Note::new(36, 0.0, 4.0, 90), Note::new(43, 0.0, 4.0, 80),
        Note::new(48, 0.0, 4.0, 75),
        Note::new(34, 4.0, 4.0, 90), Note::new(41, 4.0, 4.0, 80),
        Note::new(46, 4.0, 4.0, 75),
        Note::new(31, 8.0, 4.0, 90), Note::new(38, 8.0, 4.0, 80),
        Note::new(43, 8.0, 4.0, 75),
        Note::new(33, 12.0, 4.0, 90), Note::new(40, 12.0, 4.0, 80),
        Note::new(45, 12.0, 4.0, 75),
    ];
    clip_a.add_notes(&notes_a)?;

    // Automate WT position
    clip_a.automate_smooth(0, 4, &[
        (0.0, 0.2), (4.0, 0.5), (8.0, 0.3), (12.0, 0.7), (16.0, 0.2),
    ], 0.25)?;

    let clip_b = t1.create_clip(0, 16.0)?;
    clip_b.set_name("geometry B")?;
    clip_b.set_looping(true)?;

    // Operator: high voicing — the complement
    let notes_b = vec![
        Note::new(60, 0.0, 4.0, 85), Note::new(67, 0.0, 4.0, 75),
        Note::new(72, 0.2, 3.5, 70),
        Note::new(58, 4.0, 4.0, 85), Note::new(65, 4.0, 4.0, 75),
        Note::new(70, 4.2, 3.5, 70),
        Note::new(55, 8.0, 4.0, 85), Note::new(62, 8.0, 4.0, 75),
        Note::new(67, 8.2, 3.5, 70),
        Note::new(57, 12.0, 4.0, 85), Note::new(64, 12.0, 4.0, 75),
        Note::new(69, 12.2, 3.5, 70),
    ];
    clip_b.add_notes(&notes_b)?;

    // Automate FM depth — operator harmonics shift
    if let Some(&idx) = pm2.get("Osc-B Level") {
        clip_b.automate_smooth(0, idx as i32, &[
            (0.0, 0.4), (4.0, 0.7), (8.0, 0.3), (12.0, 0.8), (16.0, 0.4),
        ], 0.25)?;
    }

    clip_a.fire()?;
    clip_b.fire()?;
    session.play()?;
    println!("   the shape rotates...\n");

    // Slowly cross-morph the wavetable and FM simultaneously
    for i in 0..32 {
        let wt_pos = 0.2 + (i as f32 / 32.0 * std::f32::consts::PI).sin() * 0.4;
        wt.set_param(4, wt_pos)?;
        ms(500);
    }

    // ================================================================
    // 5. THE DROP
    // ================================================================
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  5. THE DROP                                  ║");
    println!("║  everything converges — both tracks feeding    ║");
    println!("║  into spectral time, frozen, then released     ║");
    println!("║  into beat repeat chaos. the geometry shatters.║");
    println!("╚══════════════════════════════════════════════╝\n");

    // Add Spectral Time to track 0
    session.load_effect(0, "Spectral Time")?;
    ms(400);
    let st = t0.device(1);

    // Build tension: spectral feedback accumulates
    println!("   building...");
    st.set_param(26, 0.5)?;    // Dry Wet
    st.set_param(18, 0.7)?;    // Feedback
    st.set_param(19, 0.7)?;    // Tilt
    st.set_param(22, 0.9)?;    // Stereo spread

    for i in 0..16 {
        let spray = i as f32 / 16.0 * 0.8;
        let feedback = 0.5 + i as f32 / 16.0 * 0.4;
        st.set_param(20, spray)?;
        st.set_param(18, feedback)?;
        ms(500);
    }

    // FREEZE at peak tension
    println!("   FREEZE");
    st.set_param(2, 1.0)?;
    wait(3);

    // Add Beat Repeat on top of the frozen spectrum
    session.load_effect(0, "Beat Repeat")?;
    ms(400);
    let br = t0.device(2);
    let _ = br.set_param_by_name("Chance", 1.0);
    let _ = br.set_param_by_name("Grid", 0.6);
    let _ = br.set_param_by_name("Variation", 0.9);
    let _ = br.set_param_by_name("Pitch", -8.0);
    let _ = br.set_param_by_name("Pitch Decay", 0.7);
    let _ = br.set_param_by_name("Volume", 0.9);
    let _ = br.set_param_by_name("Decay", 0.4);

    println!("   shattering...");
    wait(4);

    // RELEASE — unfreeze into the chaos
    println!("   RELEASE");
    st.set_param(2, 0.0)?;

    // Sweep everything simultaneously
    for i in 0..20 {
        let spray = 0.8 - (i as f32 / 20.0) * 0.7;
        let freq_shift = 0.5 + (i as f32 / 20.0 * std::f32::consts::PI * 3.0).sin() * 0.3;
        st.set_param(20, spray)?;
        st.set_param(23, freq_shift)?;

        // Also morph the wavetable
        let wt_pos = (i as f32 / 20.0 * std::f32::consts::PI * 2.0).sin().abs();
        wt.set_param(4, wt_pos)?;

        ms(400);
    }

    // Final freeze
    println!("   ...");
    st.set_param(2, 1.0)?;
    st.set_param(20, 0.95)?;
    wait(4);

    // Silence
    session.stop()?;
    st.set_param(2, 0.0)?;

    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  sounds that didn't exist before just now     ║");
    println!("╚══════════════════════════════════════════════╝\n");

    Ok(())
}
