/// Showcases the difference between Ableton's time-based effects:
/// Beat Repeat, Delay, Echo, Looper, and Spectral Time.
///
/// Loads each one in sequence on the same pattern so you can
/// hear the contrast.

use ableton::{Note, Session};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::connect()?;
    session.set_tempo(110.0)?;

    let track = session.track(0);

    // Clear all devices
    while track.num_devices()? > 0 {
        track.delete_device(0)?;
        thread::sleep(Duration::from_millis(200));
    }

    // Load Analog with a clean, punchy pluck — good for hearing repeats
    println!("setting up source sound...");
    session.load_instrument(0, "Analog")?;
    thread::sleep(Duration::from_millis(300));

    let dev = track.device(0);
    // Short pluck: saw wave, fast envelope, moderate filter
    dev.set_param(39, 1.0)?;    // OSC1 Shape = Saw
    dev.set_param(105, 0.0)?;   // OSC2 Off
    dev.set_param(34, 0.0)?;    // Noise Off
    dev.set_param(57, 0.6)?;    // F1 Freq
    dev.set_param(59, 0.2)?;    // F1 Resonance
    dev.set_param(62, 0.5)?;    // F1 Freq < Env
    dev.set_param(71, 0.3)?;    // FEG1 Decay
    dev.set_param(73, 0.1)?;    // FEG1 Sustain
    dev.set_param(89, 0.0)?;    // AEG1 Attack = 0
    dev.set_param(90, 0.35)?;   // AEG1 Decay
    dev.set_param(92, 0.0)?;    // AEG1 Sustain = 0 (pluck)
    dev.set_param(94, 0.25)?;   // AEG1 Release
    dev.set_param(23, 0.0)?;    // Vibrato Off
    dev.set_param(30, 0.0)?;    // Glide Off

    // Create a rhythmic pattern with gaps — so repeats/echoes are clearly audible
    if track.has_clip(0).unwrap_or(false) {
        track.delete_clip(0)?;
        thread::sleep(Duration::from_millis(100));
    }

    let clip = track.create_clip(0, 8.0)?;
    clip.set_name("demo pattern")?;
    clip.set_looping(true)?;

    // Syncopated pattern with space between notes
    let notes = vec![
        Note::new(60, 0.0,  0.25, 110),  // C
        Note::new(63, 0.75, 0.25, 90),   // Eb
        Note::new(67, 1.5,  0.25, 100),  // G
        Note::new(60, 2.5,  0.25, 105),  // C
        Note::new(65, 3.0,  0.5,  95),   // F
        // gap
        Note::new(72, 4.0,  0.25, 115),  // C5
        Note::new(70, 4.5,  0.25, 85),   // Bb
        Note::new(67, 5.0,  0.5,  100),  // G
        // gap
        Note::new(63, 6.5,  0.25, 90),   // Eb
        Note::new(60, 7.0,  0.5,  100),  // C
    ];
    clip.add_notes(&notes)?;

    // -- DRY: play with no effects first --
    println!("\n=== DRY (no effects) ===");
    println!("listen to the raw pattern — this is the baseline\n");
    clip.fire()?;
    session.play()?;
    thread::sleep(Duration::from_secs(8));

    // ============================================================
    // 1. DELAY — classic clean digital echo
    // ============================================================
    println!("=== DELAY ===");
    println!("simple digital echo — clean, precise repeats");
    println!("each note bounces back at a fixed time interval");
    println!("two independent delay lines (L/R) for stereo width\n");

    session.load_effect(0, "Delay")?;
    thread::sleep(Duration::from_millis(400));

    // Get the delay device (device index 1, after Analog)
    let delay_dev = track.device(1);
    let delay_params = delay_dev.parameters()?;
    for p in &delay_params {
        if p.value != 0.0 {
            // only print non-zero to keep it readable
        }
    }
    // Set for a clear dotted-eighth echo
    delay_dev.set_param_by_name("Feedback", 0.45)?;
    delay_dev.set_param_by_name("Dry/Wet", 0.35)?;

    thread::sleep(Duration::from_secs(10));

    // Remove delay
    track.delete_device(1)?;
    thread::sleep(Duration::from_millis(300));

    // ============================================================
    // 2. ECHO — warmer, more character
    // ============================================================
    println!("=== ECHO ===");
    println!("ableton's modern delay — warmer, with modulation and character");
    println!("has built-in ducking (echoes get quieter when new notes play)");
    println!("reverb tail on the repeats, modulation adds chorus-like wobble\n");

    session.load_effect(0, "Echo")?;
    thread::sleep(Duration::from_millis(400));

    let echo_dev = track.device(1);
    echo_dev.set_param_by_name("Feedback", 0.55)?;
    echo_dev.set_param_by_name("Dry Wet", 0.4)?;
    // Try to enable modulation and reverb if available
    let _ = echo_dev.set_param_by_name("Mod Rate", 0.35);
    let _ = echo_dev.set_param_by_name("Mod Depth", 0.3);
    let _ = echo_dev.set_param_by_name("Reverb Level", 0.4);

    thread::sleep(Duration::from_secs(10));

    track.delete_device(1)?;
    thread::sleep(Duration::from_millis(300));

    // ============================================================
    // 3. BEAT REPEAT — rhythmic glitch/stutter
    // ============================================================
    println!("=== BEAT REPEAT ===");
    println!("grabs a slice of audio and stutters it rhythmically");
    println!("not an echo — it captures a MOMENT and repeats it");
    println!("can pitch-decay the repeats, randomize when it triggers\n");

    session.load_effect(0, "Beat Repeat")?;
    thread::sleep(Duration::from_millis(400));

    let br_dev = track.device(1);
    // Set it to trigger frequently so the demo is obvious
    let _ = br_dev.set_param_by_name("Chance", 0.7);
    let _ = br_dev.set_param_by_name("Gate", 0.5);
    let _ = br_dev.set_param_by_name("Grid", 0.4);
    let _ = br_dev.set_param_by_name("Variation", 0.3);
    let _ = br_dev.set_param_by_name("Volume", 0.8);
    let _ = br_dev.set_param_by_name("Decay", 0.3);
    let _ = br_dev.set_param_by_name("Pitch", -3.0);
    let _ = br_dev.set_param_by_name("Pitch Decay", 0.4);
    let _ = br_dev.set_param_by_name("Mix Type", 1.0);  // Insert mode

    thread::sleep(Duration::from_secs(10));

    track.delete_device(1)?;
    thread::sleep(Duration::from_millis(300));

    // ============================================================
    // 4. SPECTRAL TIME — frequency-domain weirdness
    // ============================================================
    println!("=== SPECTRAL TIME ===");
    println!("works in the FREQUENCY domain, not the time domain");
    println!("can freeze a moment and smear it across the spectrum");
    println!("or delay different frequencies by different amounts");
    println!("sounds otherworldly — nothing else does this\n");

    session.load_effect(0, "Spectral Time")?;
    thread::sleep(Duration::from_millis(400));

    let st_dev = track.device(1);
    let _ = st_dev.set_param_by_name("Dry/Wet", 0.5);
    let _ = st_dev.set_param_by_name("Freeze", 0.0);  // start unfrozen
    let _ = st_dev.set_param_by_name("Delay Time", 0.4);
    let _ = st_dev.set_param_by_name("Feedback", 0.5);
    let _ = st_dev.set_param_by_name("Tilt", 0.6);     // high freqs delayed more
    let _ = st_dev.set_param_by_name("Spray", 0.3);     // randomize delays per bin
    let _ = st_dev.set_param_by_name("Resolution", 0.5);

    println!("   (spectral delay mode — different frequencies echo at different rates)");
    thread::sleep(Duration::from_secs(8));

    // Now try freeze mode
    println!("   (freezing the spectrum...)");
    let _ = st_dev.set_param_by_name("Freeze", 1.0);
    thread::sleep(Duration::from_secs(5));
    let _ = st_dev.set_param_by_name("Freeze", 0.0);

    thread::sleep(Duration::from_secs(3));

    track.delete_device(1)?;
    thread::sleep(Duration::from_millis(300));

    // ============================================================
    // 5. LOOPER — not really an "effect", more like a loop pedal
    // ============================================================
    println!("=== LOOPER ===");
    println!("this one is different — it's a real-time loop pedal");
    println!("records audio, plays it back, lets you overdub layers");
    println!("not really a delay/echo — it's for building up loops live");
    println!("loading it so you can see it, but it needs manual interaction\n");

    session.load_effect(0, "Looper")?;
    thread::sleep(Duration::from_millis(400));
    println!("   (looper loaded — press the record button in the UI to try it)");
    thread::sleep(Duration::from_secs(5));

    track.delete_device(1)?;
    thread::sleep(Duration::from_millis(300));

    // ============================================================
    // Done — back to dry
    // ============================================================
    session.stop()?;
    println!("\n=== DONE ===");
    println!("back to dry. the clip is still there if you want to experiment.");

    Ok(())
}
