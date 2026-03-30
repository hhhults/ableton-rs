/// Analog synth demo — creates clips with different sounds and automations
/// to showcase what the Analog can do.
///
/// Make sure Ableton is running with an Analog on track 1.

use ableton::{Note, Session};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::connect()?;
    let v = session.version()?;
    println!("connected to Ableton {}.{}", v.0, v.1);

    session.set_tempo(120.0)?;

    let track = session.track(0);
    let dev = track.device(0);
    println!("track: {:?}, device: {:?}", track.get_name()?, dev.name()?);

    // Clean up any existing clips in slots 0-4
    for slot in 0..5 {
        if track.has_clip(slot).unwrap_or(false) {
            track.delete_clip(slot)?;
        }
    }
    thread::sleep(Duration::from_millis(200));

    // ========================================================
    // CLIP 0 — "Acid Bass": saw wave, resonant filter sweep
    // ========================================================
    println!("\n--- creating clip 0: acid bass ---");
    let clip0 = track.create_clip(0, 8.0)?;
    clip0.set_name("acid bass")?;
    clip0.set_looping(true)?;

    // Repeating bass pattern — 16th notes in C minor
    let mut notes = Vec::new();
    let bass_pitches = [36, 36, 48, 36, 39, 36, 43, 36, // C2 octave jumps + Eb + G
                        36, 36, 48, 36, 41, 36, 43, 48]; // with F
    for (i, &pitch) in bass_pitches.iter().enumerate() {
        let start = i as f32 * 0.5;
        let vel = if i % 4 == 0 { 110 } else { 80 + (i % 3) as i32 * 10 };
        notes.push(Note::new(pitch, start, 0.4, vel));
    }
    clip0.add_notes(&notes)?;

    // Automate filter frequency: sweep up and back down
    // Param 57 = F1 Freq (0.0 = closed, 1.0 = open)
    clip0.automate_smooth(0, 57, &[
        (0.0, 0.05), (2.0, 0.7), (4.0, 0.15), (6.0, 0.9), (8.0, 0.05),
    ], 0.125)?;

    // Automate resonance: pulse it
    // Param 59 = F1 Resonance
    clip0.automate_smooth(0, 59, &[
        (0.0, 0.6), (2.0, 0.9), (4.0, 0.4), (6.0, 1.0), (8.0, 0.6),
    ], 0.125)?;

    // ========================================================
    // CLIP 1 — "Shimmer Pad": detuned saws, slow attack, long release
    // ========================================================
    println!("--- creating clip 1: shimmer pad ---");
    let clip1 = track.create_clip(1, 16.0)?;
    clip1.set_name("shimmer pad")?;
    clip1.set_looping(true)?;

    // Long sustained chords — Cm9 → Abmaj7 → Fm7 → G7
    let chords: &[(&[i32], f32)] = &[
        (&[60, 63, 67, 70, 74], 0.0),  // Cm9
        (&[56, 60, 63, 67], 4.0),       // Abmaj7
        (&[53, 57, 60, 63], 8.0),       // Fm7
        (&[55, 59, 62, 65], 12.0),      // G7
    ];
    let mut pad_notes = Vec::new();
    for &(pitches, start) in chords {
        for &p in pitches {
            pad_notes.push(Note::new(p, start, 3.8, 70));
        }
    }
    clip1.add_notes(&pad_notes)?;

    // Automate filter: very slow sweep
    clip1.automate_smooth(0, 57, &[
        (0.0, 0.3), (8.0, 0.8), (16.0, 0.3),
    ], 0.25)?;

    // ========================================================
    // CLIP 2 — "Squelch Lead": square wave, PWM, vibrato
    // ========================================================
    println!("--- creating clip 2: squelch lead ---");
    let clip2 = track.create_clip(2, 8.0)?;
    clip2.set_name("squelch lead")?;
    clip2.set_looping(true)?;

    // Melodic line in C minor
    let melody = [
        (72, 0.0, 0.75), (75, 1.0, 0.5), (79, 1.5, 0.75), (77, 2.5, 0.5),
        (75, 3.0, 1.0), (72, 4.0, 0.75), (70, 5.0, 0.5), (67, 5.5, 1.5),
        (72, 7.0, 0.75),
    ];
    let lead_notes: Vec<Note> = melody.iter()
        .map(|&(p, s, d)| Note::new(p, s, d, 100))
        .collect();
    clip2.add_notes(&lead_notes)?;

    // Automate pulse width: wobble
    // Param 46 = OSC1 PW
    clip2.automate_smooth(0, 46, &[
        (0.0, 0.2), (2.0, 0.8), (4.0, 0.2), (6.0, 0.8), (8.0, 0.2),
    ], 0.125)?;

    // ========================================================
    // CLIP 3 — "Perc Hit": noise + short envelope
    // ========================================================
    println!("--- creating clip 3: perc hits ---");
    let clip3 = track.create_clip(3, 4.0)?;
    clip3.set_name("perc hits")?;
    clip3.set_looping(true)?;

    // Irregular percussive hits
    let perc = [
        (48, 0.0, 0.1, 127), (48, 0.5, 0.1, 90), (48, 1.25, 0.1, 110),
        (48, 2.0, 0.1, 127), (48, 2.75, 0.1, 80), (48, 3.0, 0.1, 100),
        (48, 3.5, 0.1, 70),
    ];
    let perc_notes: Vec<Note> = perc.iter()
        .map(|&(p, s, d, v)| Note::new(p, s, d, v))
        .collect();
    clip3.add_notes(&perc_notes)?;

    // ========================================================
    // CLIP 4 — "Arp Sweep": arpeggiated chords with full filter sweep
    // ========================================================
    println!("--- creating clip 4: arp sweep ---");
    let clip4 = track.create_clip(4, 16.0)?;
    clip4.set_name("arp sweep")?;
    clip4.set_looping(true)?;

    // Fast arpeggiated notes through a chord progression
    let arp_chords: &[&[i32]] = &[
        &[60, 63, 67, 72], // Cm
        &[58, 63, 67, 70], // Bb/D
        &[56, 60, 63, 68], // Ab
        &[55, 58, 62, 67], // G
    ];
    let mut arp_notes = Vec::new();
    for (ci, chord) in arp_chords.iter().enumerate() {
        let base = ci as f32 * 4.0;
        for beat in 0..8 {
            let note_idx = beat % chord.len();
            let octave = if beat >= 4 { 12 } else { 0 };
            let start = base + beat as f32 * 0.5;
            arp_notes.push(Note::new(
                chord[note_idx] + octave,
                start,
                0.4,
                85 + (beat % 3) as i32 * 10,
            ));
        }
    }
    clip4.add_notes(&arp_notes)?;

    // Epic filter sweep across the whole 16 bars
    clip4.automate_smooth(0, 57, &[
        (0.0, 0.02), (4.0, 0.2), (8.0, 0.5), (12.0, 0.9), (16.0, 0.02),
    ], 0.125)?;

    // Resonance builds
    clip4.automate_smooth(0, 59, &[
        (0.0, 0.1), (8.0, 0.7), (12.0, 0.9), (16.0, 0.1),
    ], 0.125)?;

    // ========================================================
    // Now let's play through them! Set up Analog for each clip.
    // ========================================================

    println!("\n=== DEMO START ===\n");

    // --- Clip 0: Acid Bass ---
    println!(">> ACID BASS: saw wave, resonant low-pass, filter envelope");
    dev.set_param(39, 1.0)?;   // OSC1 Shape = Saw
    dev.set_param(52, 0.85)?;  // OSC1 Level
    dev.set_param(105, 0.0)?;  // OSC2 Off
    dev.set_param(34, 0.0)?;   // Noise Off
    dev.set_param(54, 1.0)?;   // F1 Type = LP
    dev.set_param(62, 0.8)?;   // F1 Freq < Env (strong envelope)
    dev.set_param(71, 0.35)?;  // FEG1 Decay = short-medium
    dev.set_param(73, 0.1)?;   // FEG1 Sustain = low
    dev.set_param(89, 0.0)?;   // AEG1 Attack = instant
    dev.set_param(90, 0.4)?;   // AEG1 Decay
    dev.set_param(92, 0.6)?;   // AEG1 Sustain
    dev.set_param(94, 0.3)?;   // AEG1 Release = short
    dev.set_param(23, 0.0)?;   // Vibrato Off
    dev.set_param(30, 0.0)?;   // Glide Off

    clip0.fire()?;
    session.play()?;
    println!("   playing for 8 seconds...");
    thread::sleep(Duration::from_secs(8));

    // --- Clip 1: Shimmer Pad ---
    println!("\n>> SHIMMER PAD: two detuned saws, slow attack, wide stereo");
    dev.set_param(39, 1.0)?;    // OSC1 Shape = Saw
    dev.set_param(105, 1.0)?;   // OSC2 On
    dev.set_param(106, 1.0)?;   // OSC2 Shape = Saw
    dev.set_param(111, 0.55)?;  // OSC2 Detune (slight)
    dev.set_param(108, 0.0)?;   // OSC2 Semi = 0
    dev.set_param(52, 0.7)?;    // OSC1 Level
    dev.set_param(119, 0.7)?;   // OSC2 Level
    dev.set_param(57, 0.6)?;    // F1 Freq = medium-open
    dev.set_param(59, 0.15)?;   // F1 Resonance = gentle
    dev.set_param(62, 0.3)?;    // F1 Freq < Env = gentle
    dev.set_param(89, 0.55)?;   // AEG1 Attack = slow
    dev.set_param(90, 0.7)?;    // AEG1 Decay = long
    dev.set_param(92, 0.65)?;   // AEG1 Sustain = medium
    dev.set_param(94, 0.7)?;    // AEG1 Release = long

    clip1.fire()?;
    println!("   playing for 12 seconds...");
    thread::sleep(Duration::from_secs(12));

    // --- Clip 2: Squelch Lead ---
    println!("\n>> SQUELCH LEAD: square wave, pulse width mod, vibrato");
    dev.set_param(39, 2.0)?;    // OSC1 Shape = Square
    dev.set_param(105, 0.0)?;   // OSC2 Off
    dev.set_param(57, 0.45)?;   // F1 Freq = medium
    dev.set_param(59, 0.5)?;    // F1 Resonance = noticeable
    dev.set_param(62, 0.6)?;    // F1 Freq < Env
    dev.set_param(71, 0.5)?;    // FEG1 Decay
    dev.set_param(73, 0.3)?;    // FEG1 Sustain
    dev.set_param(89, 0.02)?;   // AEG1 Attack = fast
    dev.set_param(90, 0.5)?;    // AEG1 Decay
    dev.set_param(92, 0.7)?;    // AEG1 Sustain
    dev.set_param(94, 0.45)?;   // AEG1 Release
    dev.set_param(23, 1.0)?;    // Vibrato On
    dev.set_param(24, 0.45)?;   // Vib Speed
    dev.set_param(26, 0.2)?;    // Vib Amount = subtle
    dev.set_param(25, 0.3)?;    // Vib Fade-In

    clip2.fire()?;
    println!("   playing for 8 seconds...");
    thread::sleep(Duration::from_secs(8));

    // --- Clip 3: Perc Hits ---
    println!("\n>> PERC HITS: noise + oscillator, very short envelope");
    dev.set_param(39, 0.0)?;    // OSC1 Shape = Sine
    dev.set_param(34, 1.0)?;    // Noise On
    dev.set_param(35, 0.7)?;    // Noise Color = bright
    dev.set_param(37, 0.6)?;    // Noise Level
    dev.set_param(52, 0.5)?;    // OSC1 Level = mix with noise
    dev.set_param(57, 0.7)?;    // F1 Freq = fairly open
    dev.set_param(59, 0.3)?;    // F1 Resonance
    dev.set_param(62, 0.9)?;    // F1 Freq < Env = strong
    dev.set_param(71, 0.15)?;   // FEG1 Decay = very short
    dev.set_param(73, 0.0)?;    // FEG1 Sustain = zero
    dev.set_param(89, 0.0)?;    // AEG1 Attack = instant
    dev.set_param(90, 0.15)?;   // AEG1 Decay = very short
    dev.set_param(92, 0.0)?;    // AEG1 Sustain = zero
    dev.set_param(94, 0.08)?;   // AEG1 Release = very short
    dev.set_param(23, 0.0)?;    // Vibrato Off

    clip3.fire()?;
    println!("   playing for 6 seconds...");
    thread::sleep(Duration::from_secs(6));

    // --- Clip 4: Arp Sweep ---
    println!("\n>> ARP SWEEP: saw arpeggio with epic filter sweep + glide");
    dev.set_param(39, 1.0)?;    // OSC1 Shape = Saw
    dev.set_param(34, 0.0)?;    // Noise Off
    dev.set_param(52, 0.85)?;   // OSC1 Level
    dev.set_param(105, 1.0)?;   // OSC2 On
    dev.set_param(106, 1.0)?;   // OSC2 Shape = Saw
    dev.set_param(107, 1.0)?;   // OSC2 Octave = +1
    dev.set_param(119, 0.5)?;   // OSC2 Level = blended
    dev.set_param(59, 0.4)?;    // F1 Resonance
    dev.set_param(62, 0.5)?;    // F1 Freq < Env
    dev.set_param(71, 0.45)?;   // FEG1 Decay
    dev.set_param(73, 0.2)?;    // FEG1 Sustain
    dev.set_param(89, 0.0)?;    // AEG1 Attack = instant
    dev.set_param(90, 0.4)?;    // AEG1 Decay
    dev.set_param(92, 0.5)?;    // AEG1 Sustain
    dev.set_param(94, 0.35)?;   // AEG1 Release
    dev.set_param(30, 1.0)?;    // Glide On
    dev.set_param(31, 0.3)?;    // Glide Time = moderate

    clip4.fire()?;
    println!("   playing for 16 seconds (full sweep)...");
    thread::sleep(Duration::from_secs(16));

    // Stop
    session.stop()?;
    println!("\n=== DEMO DONE ===");
    println!("clips are still in your session — trigger them yourself to keep exploring!");

    Ok(())
}
