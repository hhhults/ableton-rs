/// Deep dive into Spectral Time and Wavetable — two of Ableton's
/// most interesting devices.

use ableton::{Note, Session};
use std::thread;
use std::time::Duration;

fn wait(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::connect()?;
    session.set_tempo(105.0)?;

    let track = session.track(0);

    // Clear everything
    while track.num_devices()? > 0 {
        track.delete_device(0)?;
        thread::sleep(Duration::from_millis(200));
    }

    // ================================================================
    // PART 1: SPECTRAL TIME
    // ================================================================
    println!("╔══════════════════════════════════════╗");
    println!("║       SPECTRAL TIME DEEP DIVE        ║");
    println!("╚══════════════════════════════════════╝\n");

    // Load a clean, sustained sound source — Analog pad
    session.load_instrument(0, "Analog")?;
    thread::sleep(Duration::from_millis(300));
    let analog = track.device(0);

    // Set up a bright, sustained tone — good for spectral processing
    analog.set_param(39, 1.0)?;    // OSC1 = Saw
    analog.set_param(105, 1.0)?;   // OSC2 On
    analog.set_param(106, 2.0)?;   // OSC2 = Square
    analog.set_param(111, 0.52)?;  // OSC2 slight detune
    analog.set_param(57, 0.7)?;    // Filter open
    analog.set_param(59, 0.1)?;    // Low resonance
    analog.set_param(89, 0.1)?;    // Slow attack
    analog.set_param(90, 0.7)?;    // Long decay
    analog.set_param(92, 0.6)?;    // Medium sustain
    analog.set_param(94, 0.5)?;    // Medium release
    analog.set_param(62, 0.3)?;    // Some filter env

    // Create a pattern with variety — chords + melody + space
    if track.has_clip(0).unwrap_or(false) {
        track.delete_clip(0)?;
        thread::sleep(Duration::from_millis(100));
    }
    let clip = track.create_clip(0, 8.0)?;
    clip.set_name("spectral source")?;
    clip.set_looping(true)?;

    let notes = vec![
        // Chord hit
        Note::new(48, 0.0, 1.5, 100), Note::new(55, 0.0, 1.5, 90),
        Note::new(60, 0.0, 1.5, 85),
        // Melody
        Note::new(67, 2.0, 0.5, 105), Note::new(65, 2.5, 0.5, 95),
        Note::new(63, 3.0, 1.0, 100),
        // Chord hit
        Note::new(53, 4.0, 1.5, 100), Note::new(57, 4.0, 1.5, 90),
        Note::new(60, 4.0, 1.5, 85),
        // Melody
        Note::new(65, 6.0, 0.5, 100), Note::new(67, 6.5, 0.75, 110),
        Note::new(63, 7.5, 0.5, 85),
    ];
    clip.add_notes(&notes)?;

    // Load Spectral Time
    session.load_effect(0, "Spectral Time")?;
    thread::sleep(Duration::from_millis(400));
    let st = track.device(1);

    // Print all params so we can see everything
    let params = st.parameters()?;
    println!("Spectral Time — {} parameters:", params.len());
    for p in &params {
        println!("  [{:3}] {}: {}", p.index, p.name, p.value);
    }
    println!();

    clip.fire()?;
    session.play()?;

    // -- Scene 1: Spectral Delay with Tilt --
    println!("--- 1. SPECTRAL DELAY + TILT ---");
    println!("each frequency gets its own delay time");
    println!("tilt = high freqs delayed MORE than low freqs");
    println!("the chord smears upward like a prism splitting light\n");

    // Correct param names from the printout:
    // [2] Frozen, [14] Delay Time Seconds, [18] Delay Feedback,
    // [19] Delay Tilt, [20] Delay Spray, [22] Delay Stereo Spread,
    // [23] Delay Frequency Shift, [24] Delay Mix, [26] Dry Wet
    st.set_param(26, 0.5)?;    // Dry Wet
    st.set_param(18, 0.5)?;    // Delay Feedback
    st.set_param(20, 0.0)?;    // Delay Spray = off
    st.set_param(19, 0.8)?;    // Delay Tilt = highs delayed more
    st.set_param(14, 0.45)?;   // Delay Time
    st.set_param(2, 0.0)?;     // Frozen = off
    st.set_param(22, 0.6)?;    // Stereo Spread
    wait(10);

    // -- Scene 2: Reverse Tilt --
    println!("--- 2. REVERSE TILT ---");
    println!("now low freqs are delayed more — bass echoes trail behind");
    println!("highs come through immediately, lows smear out\n");

    st.set_param(19, 0.15)?;   // Delay Tilt = lows delayed more
    wait(10);

    // -- Scene 3: Spray (frequency randomization) --
    println!("--- 3. SPRAY ---");
    println!("randomizes the delay time PER frequency bin");
    println!("the sound scatters and diffuses — like audio fog\n");

    st.set_param(19, 0.5)?;    // Tilt = center
    st.set_param(20, 0.7)?;    // Delay Spray = high
    st.set_param(18, 0.6)?;    // Feedback
    wait(10);

    // -- Scene 4: High feedback + spray = wash --
    println!("--- 4. HIGH FEEDBACK + SPRAY = SPECTRAL WASH ---");
    println!("each frequency echoes and scatters independently");
    println!("builds into a dense, shimmering cloud of harmonics\n");

    st.set_param(18, 0.8)?;    // Delay Feedback = high
    st.set_param(20, 0.9)?;    // Delay Spray = very high
    st.set_param(26, 0.6)?;    // Dry Wet = more wet
    wait(10);

    // -- Scene 5: Freeze --
    println!("--- 5. FREEZE ---");
    println!("captures the spectrum of THIS EXACT MOMENT");
    println!("holds it indefinitely — time stops, the frequencies sustain");
    println!("like a photograph of sound\n");

    st.set_param(20, 0.0)?;    // Spray off
    st.set_param(18, 0.5)?;    // Feedback = medium
    st.set_param(26, 0.7)?;    // More wet
    thread::sleep(Duration::from_secs(3));
    println!("   ...freezing NOW");
    st.set_param(2, 1.0)?;     // Frozen = ON
    wait(6);

    println!("   ...unfreezing");
    st.set_param(2, 0.0)?;     // Frozen = OFF
    wait(4);

    // -- Scene 6: Freeze + Spray = frozen shimmer --
    println!("--- 6. FREEZE + SPRAY ---");
    println!("freeze a moment, then spray it — the frozen spectrum");
    println!("starts drifting and scattering. ethereal.\n");

    println!("   ...freezing");
    st.set_param(2, 1.0)?;     // Frozen = ON
    thread::sleep(Duration::from_secs(2));
    println!("   ...adding spray");
    st.set_param(20, 0.5)?;    // Spray = medium
    wait(4);
    st.set_param(20, 0.9)?;    // Spray = high
    wait(4);

    // -- Scene 7: Frequency Shift --
    println!("--- 7. FREQUENCY SHIFT ---");
    println!("shifts all frequency bins up or down on each echo");
    println!("the spectrum slides — like a barber pole of sound\n");

    st.set_param(2, 0.0)?;     // Unfreeze
    st.set_param(20, 0.3)?;    // Moderate spray
    st.set_param(18, 0.65)?;   // Good feedback
    st.set_param(23, 0.65)?;   // Frequency Shift = up
    st.set_param(26, 0.55)?;   // Dry Wet
    wait(10);

    // Reset freq shift
    st.set_param(23, 0.5)?;    // Frequency Shift = center

    // Clean up
    st.set_param(2, 0.0)?;     // Frozen off
    st.set_param(20, 0.0)?;    // Spray off
    session.stop()?;
    thread::sleep(Duration::from_millis(500));

    // Remove Spectral Time
    track.delete_device(1)?;
    thread::sleep(Duration::from_millis(300));

    // ================================================================
    // PART 2: WAVETABLE
    // ================================================================
    println!("\n╔══════════════════════════════════════╗");
    println!("║        WAVETABLE DEEP DIVE           ║");
    println!("╚══════════════════════════════════════╝\n");

    // Remove Analog, load Wavetable
    track.delete_device(0)?;
    thread::sleep(Duration::from_millis(200));

    session.load_instrument(0, "Wavetable")?;
    thread::sleep(Duration::from_millis(500));
    let wt = track.device(0);
    println!("loaded: {:?}\n", wt.name()?);

    // Print parameters
    let params = wt.parameters()?;
    println!("Wavetable — {} parameters:", params.len());
    for p in &params {
        println!("  [{:3}] {}: {}", p.index, p.name, p.value);
    }
    println!();

    // Create a versatile pattern for showcasing different timbres
    if track.has_clip(0).unwrap_or(false) {
        track.delete_clip(0)?;
        thread::sleep(Duration::from_millis(100));
    }
    let clip = track.create_clip(0, 16.0)?;
    clip.set_name("wavetable showcase")?;
    clip.set_looping(true)?;

    // Pattern: bass notes, chords, melody — tests the full range
    let notes = vec![
        // Bass
        Note::new(36, 0.0, 0.75, 110), Note::new(36, 1.0, 0.75, 100),
        Note::new(39, 2.0, 0.75, 105), Note::new(36, 3.0, 0.75, 100),
        // Chord
        Note::new(60, 4.0, 2.0, 90), Note::new(63, 4.0, 2.0, 85),
        Note::new(67, 4.0, 2.0, 80), Note::new(72, 4.0, 2.0, 75),
        // Melody
        Note::new(72, 6.0, 0.5, 105), Note::new(75, 6.5, 0.5, 100),
        Note::new(79, 7.0, 1.0, 110),
        // Bass
        Note::new(41, 8.0, 0.75, 110), Note::new(41, 9.0, 0.75, 100),
        Note::new(43, 10.0, 0.75, 105), Note::new(41, 11.0, 0.75, 100),
        // Chord
        Note::new(65, 12.0, 2.0, 90), Note::new(68, 12.0, 2.0, 85),
        Note::new(72, 12.0, 2.0, 80),
        // Melody
        Note::new(77, 14.0, 0.5, 100), Note::new(75, 14.5, 0.5, 95),
        Note::new(72, 15.0, 1.0, 105),
    ];
    clip.add_notes(&notes)?;

    clip.fire()?;
    session.play()?;

    // Now cycle through different Wavetable configurations
    // We'll discover the param names from the printout above
    // Common Wavetable params:
    // - Osc 1 Pos (wavetable position - THE key parameter)
    // - Osc 1 Effect (wavetable effect type)
    // - Filter Type, Filter Freq, Filter Res
    // - Amp Env Attack, Decay, Sustain, Release
    // - Sub On/Off, Sub Level
    // - Unison Voices, Unison Amount

    // Correct param names from the printout:
    // [4] Osc 1 Pos, [5] Osc 1 Effect 1, [8] Osc 1 Gain,
    // [9] Osc 2 On, [12] Osc 2 Pos, [17] Sub On, [19] Sub Gain,
    // [21] Filter 1 On, [26] Filter 1 Freq, [27] Filter 1 Res,
    // [39] Amp Attack, [40] Amp Decay, [41] Amp Release, [45] Amp Sustain,
    // [89] Unison Amount, [91] Glide, [92] Volume

    println!("--- 1. CLEAN DIGITAL ---");
    println!("wavetable position at 0 — simple, pure waveforms");
    println!("this is wavetable at its most basic\n");

    wt.set_param(4, 0.0)?;     // Osc 1 Pos = start
    wt.set_param(26, 1.0)?;    // Filter 1 Freq = open
    wt.set_param(27, 0.0)?;    // Filter 1 Res = none
    wt.set_param(89, 0.0)?;    // Unison = off
    wt.set_param(39, 0.08)?;   // Amp Attack = fast
    wt.set_param(40, 0.5)?;    // Amp Decay
    wt.set_param(45, 0.7)?;    // Amp Sustain
    wt.set_param(41, 0.5)?;    // Amp Release
    wait(10);

    println!("--- 2. SCANNING THE WAVETABLE ---");
    println!("sweeping the position — the timbre morphs continuously");
    println!("this is what makes wavetable special\n");

    for i in 0..30 {
        let pos = i as f32 / 30.0;
        wt.set_param(4, pos)?;  // Osc 1 Pos
        thread::sleep(Duration::from_millis(400));
    }
    println!();

    println!("--- 3. THICK UNISON ---");
    println!("stack detuned voices — instant supersaw territory");
    println!("the sound gets WIDE and full\n");

    wt.set_param(4, 0.3)?;     // Osc 1 Pos
    wt.set_param(89, 0.55)?;   // Unison Amount = thick
    wait(10);

    println!("--- 4. DUAL OSCILLATOR ---");
    println!("two oscillators at different wavetable positions");
    println!("each morphing independently — complex interactions\n");

    wt.set_param(89, 0.2)?;    // Moderate unison
    wt.set_param(9, 1.0)?;     // Osc 2 On
    wt.set_param(12, 0.6)?;    // Osc 2 Pos = different position
    wt.set_param(16, 0.8)?;    // Osc 2 Gain
    wt.set_param(10, 0.0)?;    // Osc 2 Transp = same octave
    wait(10);

    println!("--- 5. FILTERED BASS ---");
    println!("low position, heavy filter, short decay");
    println!("wavetable doing subtractive — but richer starting harmonics\n");

    wt.set_param(9, 0.0)?;     // Osc 2 Off
    wt.set_param(89, 0.0)?;    // Unison off
    wt.set_param(4, 0.15)?;    // Osc 1 Pos = low
    wt.set_param(26, 0.25)?;   // Filter 1 Freq = low
    wt.set_param(27, 0.5)?;    // Filter 1 Res = resonant
    wt.set_param(40, 0.35)?;   // Amp Decay = short
    wt.set_param(45, 0.15)?;   // Amp Sustain = low
    wait(10);

    println!("--- 6. DIGITAL TEXTURE ---");
    println!("high wavetable position — complex, alien waveforms");
    println!("harmonics that don't exist in nature\n");

    wt.set_param(26, 0.85)?;   // Filter open
    wt.set_param(27, 0.1)?;    // Low res
    wt.set_param(4, 0.85)?;    // Osc 1 Pos = high
    wt.set_param(40, 0.7)?;    // Amp Decay = long
    wt.set_param(45, 0.6)?;    // Amp Sustain
    wt.set_param(41, 0.5)?;    // Amp Release
    wait(10);

    println!("--- 7. EVOLVING PAD ---");
    println!("slow position sweep + unison + sub oscillator");
    println!("the timbre breathes and shifts — alive\n");

    wt.set_param(17, 1.0)?;    // Sub On
    wt.set_param(19, 0.4)?;    // Sub Gain
    wt.set_param(89, 0.4)?;    // Unison Amount
    wt.set_param(39, 0.45)?;   // Amp Attack = slow
    wt.set_param(40, 0.8)?;    // Amp Decay = long
    wt.set_param(45, 0.7)?;    // Amp Sustain
    wt.set_param(41, 0.65)?;   // Amp Release = long
    wt.set_param(26, 0.65)?;   // Filter = medium

    // Slow sweep with both oscillators
    wt.set_param(9, 1.0)?;     // Osc 2 On
    wt.set_param(16, 0.6)?;    // Osc 2 Gain
    for i in 0..40 {
        let pos1 = 0.1 + (i as f32 / 40.0) * 0.7;
        let pos2 = 0.8 - (i as f32 / 40.0) * 0.6; // opposite direction
        wt.set_param(4, pos1)?;   // Osc 1 sweeps forward
        wt.set_param(12, pos2)?;  // Osc 2 sweeps backward
        thread::sleep(Duration::from_millis(400));
    }

    // Done
    session.stop()?;
    println!("\n╔══════════════════════════════════════╗");
    println!("║              ALL DONE!               ║");
    println!("╚══════════════════════════════════════╝");
    println!("\nwavetable is still loaded — try moving Osc 1 Pos yourself!");

    Ok(())
}
