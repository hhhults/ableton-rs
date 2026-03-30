/// SOPHIE-style and Galen Tipton-style sound design techniques.
///
/// Part 1: SOPHIE — FM bubble bass, synthesized percussion, extreme sidechain
/// Part 2: Galen — glitch chaos, spectral destruction, rapid automation

use ableton::{Note, Session};
use std::thread;
use std::time::Duration;

fn wait(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::connect()?;
    println!("connected to Ableton {:?}", session.version()?);

    // We'll use multiple tracks
    // Clean up existing tracks first (keep track 0, delete extras)
    let n = session.num_tracks()?;
    for i in (1..n).rev() {
        session.delete_track(i)?;
        thread::sleep(Duration::from_millis(100));
    }

    // Clean track 0
    let t0 = session.track(0);
    while t0.num_devices()? > 0 {
        t0.delete_device(0)?;
        thread::sleep(Duration::from_millis(150));
    }
    for slot in 0..4 {
        if t0.has_clip(slot).unwrap_or(false) {
            t0.delete_clip(slot)?;
        }
    }
    thread::sleep(Duration::from_millis(200));

    // ================================================================
    //  PART 1: SOPHIE
    // ================================================================
    println!("\n╔══════════════════════════════════════╗");
    println!("║     SOPHIE — SYNTHESIS FROM SCRATCH   ║");
    println!("╚══════════════════════════════════════╝\n");

    session.set_tempo(130.0)?;

    // --------------------------------------------------------
    // Scene 1: FM Bubble Bass (Operator)
    // --------------------------------------------------------
    println!("--- 1. FM BUBBLE BASS ---");
    println!("the signature SOPHIE sound: liquid, elastic, bubbly");
    println!("FM synthesis with specific modulator ratios creates");
    println!("timbres that feel like they have surface tension\n");

    session.load_instrument(0, "Operator")?;
    thread::sleep(Duration::from_millis(500));

    let op = t0.device(0);
    let params = op.parameters()?;
    println!("Operator — {} params. key ones:", params.len());
    for p in &params {
        // Print a curated selection
        let dominated_names = ["Osc-A", "Osc-B", "Osc-C", "Osc-D",
            "Filter", "Algorithm", "Time", "Tone",
            "A Coarse", "B Coarse", "C Coarse", "D Coarse",
            "A Fine", "B Fine", "A Level", "B Level", "C Level", "D Level"];
        if dominated_names.iter().any(|n| p.name.starts_with(n)) && p.value != 0.0 {
            println!("  [{:3}] {}: {}", p.index, p.name, p.value);
        }
    }

    // We'll set params by index since we have the full list
    // First let's find key Operator params
    let param_map: std::collections::HashMap<String, usize> = params.iter()
        .map(|p| (p.name.clone(), p.index))
        .collect();

    // Helper to set by name, silently skip if not found
    let set = |name: &str, val: f32| -> Result<(), Box<dyn std::error::Error>> {
        if let Some(&idx) = param_map.get(name) {
            op.set_param(idx as i32, val)?;
        }
        Ok(())
    };

    // FM Algorithm: B modulates A (simple 2-op FM)
    set("Algorithm", 0.0)?;  // Algorithm 1: B->A

    // Oscillator A (carrier): sine
    set("Osc-A Level", 1.0)?;
    set("Osc-A Wave", 0.0)?;     // Sine

    // Oscillator B (modulator): sine, non-integer ratio for inharmonic partials
    set("Osc-B On", 1.0)?;
    set("Osc-B Level", 0.7)?;
    set("Osc-B Wave", 0.0)?;     // Sine
    set("B Coarse", 0.46)?;       // Non-integer ratio — this creates the bubble
    set("B Fine", 0.55)?;         // Slightly detuned — shimmery

    // Oscillator C: subtle sub-harmonic body
    set("Osc-C On", 1.0)?;
    set("Osc-C Level", 0.35)?;
    set("Osc-C Wave", 0.0)?;
    set("C Coarse", 0.2)?;        // Sub ratio

    // Filter — gentle LP to tame highs
    set("Filter On", 1.0)?;
    set("Filter Freq", 0.6)?;
    set("Filter Res", 0.3)?;

    // Amp envelope: snappy but not too short
    set("Ae Attack", 0.0)?;
    set("Ae Decay", 0.5)?;
    set("Ae Sustain", 0.4)?;
    set("Ae Release", 0.35)?;

    // Create the bubble bass pattern
    let clip = t0.create_clip(0, 8.0)?;
    clip.set_name("bubble bass")?;
    clip.set_looping(true)?;

    // Classic SOPHIE-style: simple rhythmic pattern, the SOUND is the star
    let bass_notes = vec![
        Note::new(36, 0.0, 0.3, 120), Note::new(36, 0.5, 0.15, 90),
        Note::new(36, 1.0, 0.3, 115), Note::new(48, 1.5, 0.15, 85),
        Note::new(36, 2.0, 0.3, 120), Note::new(36, 2.75, 0.15, 80),
        Note::new(36, 3.0, 0.5, 110), Note::new(39, 3.75, 0.15, 95),
        Note::new(36, 4.0, 0.3, 120), Note::new(36, 4.5, 0.15, 90),
        Note::new(36, 5.0, 0.3, 115), Note::new(43, 5.5, 0.2, 100),
        Note::new(36, 6.0, 0.3, 120), Note::new(36, 6.5, 0.15, 85),
        Note::new(36, 7.0, 0.2, 110), Note::new(36, 7.25, 0.2, 105),
        Note::new(36, 7.5, 0.2, 100), Note::new(36, 7.75, 0.15, 90),
    ];
    clip.add_notes(&bass_notes)?;

    // Automate the FM amount (Osc-B Level) for that morphing bubble
    if let Some(&b_level_idx) = param_map.get("Osc-B Level") {
        clip.automate_smooth(0, b_level_idx as i32, &[
            (0.0, 0.5), (2.0, 0.9), (4.0, 0.3), (6.0, 0.85), (8.0, 0.5),
        ], 0.125)?;
    }

    clip.fire()?;
    session.play()?;
    println!("   bubbling...");
    wait(12);

    // --------------------------------------------------------
    // Scene 2: Synthesized Kick + Snare
    // --------------------------------------------------------
    println!("\n--- 2. SYNTHESIZED PERCUSSION ---");
    println!("SOPHIE never used sample packs — every drum was built");
    println!("from oscillators. pitch envelope = kick thud,");
    println!("noise burst + resonant filter = snare crack\n");

    // Create a second track for drums
    let drum_track = session.create_midi_track(-1)?;
    drum_track.set_name("synth drums")?;
    let drum_idx = drum_track.track_idx;

    session.load_instrument(drum_idx, "Analog")?;
    thread::sleep(Duration::from_millis(400));

    let drum_dev = drum_track.device(0);

    // Synthesized kick: sine with fast pitch envelope
    drum_dev.set_param(38, 1.0)?;   // OSC1 On
    drum_dev.set_param(39, 0.0)?;   // OSC1 = Sine
    drum_dev.set_param(40, -2.0)?;  // OSC1 Octave = low
    drum_dev.set_param(49, 0.7)?;   // PEG1 Amount = strong pitch drop
    drum_dev.set_param(43, 0.15)?;  // PEG1 Time = fast
    drum_dev.set_param(34, 1.0)?;   // Noise On (for snare hits)
    drum_dev.set_param(35, 0.6)?;   // Noise Color
    drum_dev.set_param(37, 0.0)?;   // Noise Level = off initially (we'll automate)
    drum_dev.set_param(105, 0.0)?;  // OSC2 Off
    drum_dev.set_param(89, 0.0)?;   // AEG1 Attack = 0
    drum_dev.set_param(90, 0.2)?;   // AEG1 Decay = short
    drum_dev.set_param(92, 0.0)?;   // AEG1 Sustain = 0
    drum_dev.set_param(94, 0.1)?;   // AEG1 Release = short
    drum_dev.set_param(57, 0.8)?;   // Filter open
    drum_dev.set_param(62, 0.6)?;   // Filter envelope
    drum_dev.set_param(71, 0.15)?;  // FEG Decay = short

    let drum_clip = drum_track.create_clip(0, 8.0)?;
    drum_clip.set_name("synth drums")?;
    drum_clip.set_looping(true)?;

    // Kick pattern with velocity accents — low notes for kick, high for snare
    let drum_notes = vec![
        // Kicks (low velocity = just sine kick)
        Note::new(36, 0.0, 0.2, 127), Note::new(36, 1.0, 0.2, 110),
        Note::new(36, 2.0, 0.2, 127), Note::new(36, 3.0, 0.2, 100),
        Note::new(36, 4.0, 0.2, 127), Note::new(36, 5.0, 0.2, 110),
        Note::new(36, 6.0, 0.2, 127), Note::new(36, 7.0, 0.2, 105),
        // Snare hits (higher pitch triggers noise burst)
        Note::new(60, 1.0, 0.15, 115), Note::new(60, 3.0, 0.15, 120),
        Note::new(60, 5.0, 0.15, 115), Note::new(60, 7.0, 0.15, 120),
        // Ghost snares
        Note::new(60, 3.75, 0.1, 60), Note::new(60, 7.5, 0.1, 55),
    ];
    drum_clip.add_notes(&drum_notes)?;

    // Automate noise level — burst it on snare hits
    drum_clip.automate(0, 37, &[
        (0.0, 0.25, 0.0), (0.5, 0.5, 0.0),
        (1.0, 0.1, 0.7), (1.1, 0.4, 0.0),   // snare hit
        (2.0, 0.5, 0.0), (2.5, 0.5, 0.0),
        (3.0, 0.1, 0.7), (3.1, 0.4, 0.0),   // snare hit
        (3.75, 0.1, 0.3), (3.85, 0.15, 0.0), // ghost
        (4.0, 0.5, 0.0), (4.5, 0.5, 0.0),
        (5.0, 0.1, 0.7), (5.1, 0.4, 0.0),
        (6.0, 0.5, 0.0), (6.5, 0.5, 0.0),
        (7.0, 0.1, 0.7), (7.1, 0.25, 0.0),
        (7.5, 0.1, 0.3), (7.6, 0.15, 0.0),
    ])?;

    drum_clip.fire()?;
    println!("   kick + snare from raw oscillators...");
    wait(12);

    // --------------------------------------------------------
    // Scene 3: Extreme Sidechain Pump
    // --------------------------------------------------------
    println!("\n--- 3. THE PUMP ---");
    println!("extreme sidechain compression — the mix breathes");
    println!("with every kick hit. the silence between hits");
    println!("becomes part of the rhythm\n");

    // Load a compressor on the bass track, sidechain from drums
    session.load_effect(0, "Compressor")?;
    thread::sleep(Duration::from_millis(400));

    let comp = t0.device(1);
    let comp_params = comp.parameters()?;
    for p in &comp_params {
        if ["Threshold", "Ratio", "Attack", "Release", "Output Gain",
            "Dry/Wet", "Model", "Knee"].iter().any(|n| p.name.contains(n)) {
            println!("  [{:3}] {}: {}", p.index, p.name, p.value);
        }
    }

    // Set aggressive compression
    let _ = comp.set_param_by_name("Threshold", 0.15);
    let _ = comp.set_param_by_name("Ratio", 1.0);  // max ratio
    let _ = comp.set_param_by_name("Attack", 0.0);
    let _ = comp.set_param_by_name("Release", 0.35);
    let _ = comp.set_param_by_name("Output Gain", 0.7);

    println!("   compressor on bass — listen to the pumping...");
    wait(12);

    // Stop SOPHIE section
    session.stop()?;
    thread::sleep(Duration::from_millis(500));

    // ================================================================
    //  PART 2: GALEN TIPTON
    // ================================================================
    println!("\n╔══════════════════════════════════════╗");
    println!("║  GALEN TIPTON — GLITCH DESTRUCTION   ║");
    println!("╚══════════════════════════════════════╝\n");

    session.set_tempo(140.0)?;

    // Remove compressor from bass track
    if t0.num_devices()? > 1 {
        t0.delete_device(1)?;
        thread::sleep(Duration::from_millis(200));
    }

    // --------------------------------------------------------
    // Scene 4: Rapid Parameter Automation (Glitch)
    // --------------------------------------------------------
    println!("--- 4. GLITCH: RAPID PARAMETER CHAOS ---");
    println!("parameters changing dozens of times per beat");
    println!("filter, pitch, FM amount — nothing sits still");
    println!("the sound constantly mutates and breaks\n");

    // Reuse the Operator on track 0, but make the clip chaotic
    // Delete old clip, make a new one with rapid notes
    t0.delete_clip(0)?;
    thread::sleep(Duration::from_millis(100));

    let glitch_clip = t0.create_clip(0, 8.0)?;
    glitch_clip.set_name("glitch")?;
    glitch_clip.set_looping(true)?;

    // Rapid-fire notes at different pitches — micro-edits
    let mut glitch_notes = Vec::new();
    let glitch_pitches = [
        60, 60, 72, 60, 55, 67, 60, 48,
        60, 63, 60, 72, 55, 60, 67, 60,
        48, 60, 60, 72, 63, 60, 55, 60,
        72, 48, 60, 67, 60, 60, 55, 63,
    ];
    for (i, &pitch) in glitch_pitches.iter().enumerate() {
        let start = i as f32 * 0.25;
        let dur = if i % 5 == 0 { 0.125 } else { 0.2 };
        let vel = 70 + ((i * 17) % 60) as i32; // pseudo-random velocity
        glitch_notes.push(Note::new(pitch, start, dur, vel));
    }
    glitch_clip.add_notes(&glitch_notes)?;

    // CHAOS AUTOMATION: filter freq jumping around
    let mut filter_chaos: Vec<(f32, f32, f32)> = Vec::new();
    for i in 0..64 {
        let t = i as f32 * 0.125;
        // Pseudo-random filter values
        let val = match i % 7 {
            0 => 0.1, 1 => 0.9, 2 => 0.3, 3 => 0.7,
            4 => 0.15, 5 => 0.85, _ => 0.5,
        };
        filter_chaos.push((t, 0.125, val));
    }
    if let Some(&idx) = param_map.get("Filter Freq") {
        glitch_clip.automate(0, idx as i32, &filter_chaos)?;
    }

    // FM amount chaos
    let mut fm_chaos: Vec<(f32, f32, f32)> = Vec::new();
    for i in 0..32 {
        let t = i as f32 * 0.25;
        let val = match i % 5 {
            0 => 0.2, 1 => 0.95, 2 => 0.1, 3 => 0.8, _ => 0.5,
        };
        fm_chaos.push((t, 0.25, val));
    }
    if let Some(&idx) = param_map.get("Osc-B Level") {
        glitch_clip.automate(0, idx as i32, &fm_chaos)?;
    }

    glitch_clip.fire()?;
    drum_clip.fire()?;
    session.play()?;
    println!("   chaos...");
    wait(12);

    // --------------------------------------------------------
    // Scene 5: Beat Repeat Stutter
    // --------------------------------------------------------
    println!("\n--- 5. STUTTER DESTRUCTION ---");
    println!("beat repeat grabbing fragments and stuttering them");
    println!("the rhythm fractures into smaller and smaller pieces\n");

    session.load_effect(0, "Beat Repeat")?;
    thread::sleep(Duration::from_millis(400));

    let br = t0.device(1);
    let br_params = br.parameters()?;
    println!("Beat Repeat params:");
    for p in &br_params {
        println!("  [{:3}] {}: {}", p.index, p.name, p.value);
    }

    // Set for aggressive stuttering
    let _ = br.set_param_by_name("Chance", 0.8);     // trigger often
    let _ = br.set_param_by_name("Interval", 0.3);
    let _ = br.set_param_by_name("Gate", 0.4);
    let _ = br.set_param_by_name("Grid", 0.5);       // medium grid
    let _ = br.set_param_by_name("Variation", 0.5);
    let _ = br.set_param_by_name("Pitch", -4.0);     // pitch down on repeats
    let _ = br.set_param_by_name("Pitch Decay", 0.5);
    let _ = br.set_param_by_name("Volume", 0.9);
    let _ = br.set_param_by_name("Decay", 0.2);

    println!("   stuttering...");
    wait(10);

    // Speed up the grid
    println!("   making it faster...");
    let _ = br.set_param_by_name("Grid", 0.7);
    let _ = br.set_param_by_name("Chance", 0.9);
    let _ = br.set_param_by_name("Variation", 0.7);
    wait(8);

    // Remove beat repeat
    t0.delete_device(1)?;
    thread::sleep(Duration::from_millis(200));

    // --------------------------------------------------------
    // Scene 6: Spectral Destruction
    // --------------------------------------------------------
    println!("\n--- 6. SPECTRAL DESTRUCTION ---");
    println!("spectral time pushed to extremes — freeze, spray,");
    println!("frequency shift all at once. the sound dissolves");
    println!("into an alien texture that has nothing to do");
    println!("with the original\n");

    session.load_effect(0, "Spectral Time")?;
    thread::sleep(Duration::from_millis(400));

    let st = t0.device(1);

    // Start with heavy spectral delay
    st.set_param(26, 0.65)?;   // Dry Wet
    st.set_param(18, 0.75)?;   // Delay Feedback = high
    st.set_param(19, 0.8)?;    // Delay Tilt = extreme
    st.set_param(20, 0.6)?;    // Delay Spray
    st.set_param(22, 0.8)?;    // Stereo Spread
    st.set_param(23, 0.6)?;    // Frequency Shift = up

    println!("   spectral delay + spray + frequency shift...");
    wait(8);

    // Now freeze and let it self-destruct
    println!("   freezing the chaos...");
    st.set_param(2, 1.0)?;     // Frozen
    st.set_param(20, 0.95)?;   // Max spray
    st.set_param(18, 0.9)?;    // High feedback
    wait(6);

    // Sweep the frequency shift while frozen
    println!("   shifting the frozen spectrum...");
    for i in 0..20 {
        let shift = 0.5 + (i as f32 / 20.0) * 0.45;
        st.set_param(23, shift)?;
        thread::sleep(Duration::from_millis(300));
    }

    // Unfreeze into full chaos
    println!("   releasing...");
    st.set_param(2, 0.0)?;     // Unfreeze
    st.set_param(23, 0.5)?;    // Reset shift
    wait(6);

    // --------------------------------------------------------
    // Scene 7: Everything at once
    // --------------------------------------------------------
    println!("\n--- 7. TOTAL DESTRUCTION ---");
    println!("beat repeat + spectral time + rapid automation");
    println!("all at once. this is the Galen Tipton zone.\n");

    // Add beat repeat back on top of spectral time
    session.load_effect(0, "Beat Repeat")?;
    thread::sleep(Duration::from_millis(400));

    let br2 = t0.device(2);  // now it's device 2 (after Operator + Spectral Time)
    let _ = br2.set_param_by_name("Chance", 0.85);
    let _ = br2.set_param_by_name("Grid", 0.6);
    let _ = br2.set_param_by_name("Variation", 0.8);
    let _ = br2.set_param_by_name("Pitch", -6.0);
    let _ = br2.set_param_by_name("Pitch Decay", 0.6);
    let _ = br2.set_param_by_name("Volume", 0.85);
    let _ = br2.set_param_by_name("Decay", 0.3);

    // Cycle spectral params while beat repeat stutters
    for i in 0..30 {
        let spray = (i as f32 / 30.0 * 3.14159 * 2.0).sin().abs();
        let tilt = 0.3 + (i as f32 / 30.0) * 0.5;
        st.set_param(20, spray)?;
        st.set_param(19, tilt)?;

        if i == 15 {
            println!("   freezing mid-stutter...");
            st.set_param(2, 1.0)?;
        }
        if i == 22 {
            st.set_param(2, 0.0)?;
        }
        thread::sleep(Duration::from_millis(400));
    }

    // Done
    session.stop()?;

    println!("\n╔══════════════════════════════════════╗");
    println!("║              DONE                     ║");
    println!("╚══════════════════════════════════════╝");
    println!("\nall devices still loaded — mess around with them!");

    Ok(())
}
