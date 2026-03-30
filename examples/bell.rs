/// Bell sound on the Analog — FM-style synthesis with inharmonic partials,
/// fast attack, long decay, no sustain.

use ableton::{Note, Session};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::connect()?;
    session.set_tempo(90.0)?;

    let track = session.track(0);
    let dev = track.device(0);
    println!("setting up bell patch on {:?}", track.get_name()?);

    // -- Oscillator 1: sine wave (fundamental) --
    dev.set_param(38, 1.0)?;    // OSC1 On
    dev.set_param(39, 0.0)?;    // OSC1 Shape = Sine
    dev.set_param(40, 0.0)?;    // OSC1 Octave = 0
    dev.set_param(41, 0.0)?;    // OSC1 Semi = 0
    dev.set_param(44, 0.5)?;    // OSC1 Detune = center
    dev.set_param(46, 0.5)?;    // OSC1 PW = center
    dev.set_param(48, 1.0)?;    // OSC1 Balance = full
    dev.set_param(52, 0.75)?;   // OSC1 Level

    // -- Oscillator 2: sine, tuned to a pure harmonic (octave + fifth) --
    dev.set_param(105, 1.0)?;   // OSC2 On
    dev.set_param(106, 0.0)?;   // OSC2 Shape = Sine
    dev.set_param(107, 1.0)?;   // OSC2 Octave = +1
    dev.set_param(108, 0.0)?;   // OSC2 Semi = 0 (pure octave — no inharmonicity)
    dev.set_param(111, 0.5)?;   // OSC2 Detune = center (no beating)
    dev.set_param(115, 1.0)?;   // OSC2 Balance
    dev.set_param(119, 0.4)?;   // OSC2 Level (subtle upper partial)

    // -- Amp envelope 1: bell shape (instant attack, long decay, no sustain) --
    dev.set_param(76, 1.0)?;    // AMP1 On
    dev.set_param(78, 0.55)?;   // AMP1 Level
    dev.set_param(89, 0.0)?;    // AEG1 Attack = instant
    dev.set_param(90, 0.88)?;   // AEG1 Decay = very long ring
    dev.set_param(91, 0.5)?;    // AEG1 < Vel
    dev.set_param(92, 0.0)?;    // AEG1 Sustain = 0
    dev.set_param(93, 1.0)?;    // AEG1 S Time = max
    dev.set_param(94, 0.85)?;   // AEG1 Release = very long ring-out

    // -- Amp envelope 2: slightly shorter (upper partial decays faster) --
    dev.set_param(143, 1.0)?;   // AMP2 On
    dev.set_param(145, 0.45)?;  // AMP2 Level
    dev.set_param(155, 0.0)?;   // AEG2 Attack = instant
    dev.set_param(156, 0.75)?;  // AEG2 Decay = long (but still shorter than fundamental)
    dev.set_param(158, 0.5)?;   // AEG2 < Vel
    dev.set_param(159, 0.0)?;   // AEG2 Sustain = 0
    dev.set_param(160, 1.0)?;   // AEG2 S Time
    dev.set_param(161, 0.7)?;   // AEG2 Release

    // -- Filter: open with resonance for that ringing purity --
    dev.set_param(53, 1.0)?;    // F1 On
    dev.set_param(54, 0.0)?;    // F1 Type = LP
    dev.set_param(57, 0.9)?;    // F1 Freq = wide open
    dev.set_param(59, 0.45)?;   // F1 Resonance = ringing
    dev.set_param(62, 0.15)?;   // F1 Freq < Env (just a touch of brightness on attack)
    dev.set_param(71, 0.7)?;    // FEG1 Decay = long
    dev.set_param(73, 0.0)?;    // FEG1 Sustain = 0

    // -- Noise off --
    dev.set_param(34, 0.0)?;    // Noise Off

    // -- No vibrato, no glide --
    dev.set_param(23, 0.0)?;    // Vibrato Off
    dev.set_param(30, 0.0)?;    // Glide Off

    // -- Clean up old clip --
    if track.has_clip(0).unwrap_or(false) {
        track.delete_clip(0)?;
        thread::sleep(Duration::from_millis(100));
    }

    // -- Create a bell melody --
    let clip = track.create_clip(0, 16.0)?;
    clip.set_name("bells")?;
    clip.set_looping(true)?;

    // Gamelan-ish bell melody — pentatonic, varying velocities and spacing
    let melody = [
        // bar 1 — slow, spacious
        (72, 0.0,  1.5, 110),  // C5
        (67, 2.0,  1.5, 90),   // G4
        (64, 4.0,  2.0, 100),  // E4
        (60, 6.5,  1.5, 85),   // C4

        // bar 2 — higher, brighter
        (76, 8.0,  1.0, 115),  // E5
        (72, 9.0,  1.0, 95),   // C5
        (79, 10.0, 1.5, 105),  // G5
        (76, 11.5, 0.5, 75),   // E5 (ghost)
        (72, 12.0, 2.0, 100),  // C5
        (67, 14.0, 2.0, 90),   // G4
    ];

    let notes: Vec<Note> = melody.iter()
        .map(|&(p, s, d, v)| Note::new(p, s, d, v))
        .collect();
    clip.add_notes(&notes)?;

    // Fire it
    clip.fire()?;
    session.play()?;
    println!("bells ringing...");
    thread::sleep(Duration::from_secs(16));

    session.stop()?;
    println!("done — clip is still there, hit play to hear it again");

    Ok(())
}
