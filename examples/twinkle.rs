//! Play Twinkle Twinkle Little Star directly in Ableton via OSC.
//!
//!     cargo run --example twinkle
//!
//! Requires Ableton Live with AbletonOSC running.

use ableton::{Note, Session};
use std::thread;
use std::time::Duration;

fn main() -> ableton::Result<()> {
    let session = Session::connect()?;
    println!("Connected to Ableton Live {:?}", session.version()?);

    session.set_tempo(100.0)?;

    // Create two tracks: melody + bass
    let melody_track = session.create_midi_track(-1)?;
    melody_track.set_name("Melody")?;
    let melody_idx = melody_track.track_idx;

    let bass_track = session.create_midi_track(-1)?;
    bass_track.set_name("Bass")?;
    let bass_idx = bass_track.track_idx;

    // Load instruments
    session.load_instrument(melody_idx, "Analog")?;
    thread::sleep(Duration::from_millis(300));
    session.load_instrument(bass_idx, "Analog")?;
    thread::sleep(Duration::from_millis(300));

    // Load effects on melody track
    session.load_effect(melody_idx, "Reverb")?;
    thread::sleep(Duration::from_millis(200));
    session.load_effect(melody_idx, "Delay")?;
    thread::sleep(Duration::from_millis(200));

    // Build melody notes
    // Quarter note = 1 beat, half note = 2 beats at 100 BPM in 4/4
    let q = 1.0_f32; // quarter
    let h = 2.0_f32; // half

    let mut melody_notes = Vec::new();
    let mut t = 0.0_f32;

    // Helper to push a note and advance time
    let note = |notes: &mut Vec<Note>, pitch: i32, dur: f32, time: &mut f32| {
        notes.push(Note::new(pitch, *time, dur, 100));
        *time += dur;
    };

    // Phrase 1: C C G G A A G-
    note(&mut melody_notes, 60, q, &mut t);
    note(&mut melody_notes, 60, q, &mut t);
    note(&mut melody_notes, 67, q, &mut t);
    note(&mut melody_notes, 67, q, &mut t);
    note(&mut melody_notes, 69, q, &mut t);
    note(&mut melody_notes, 69, q, &mut t);
    note(&mut melody_notes, 67, h, &mut t);

    // Phrase 2: F F E E D D C-
    note(&mut melody_notes, 65, q, &mut t);
    note(&mut melody_notes, 65, q, &mut t);
    note(&mut melody_notes, 64, q, &mut t);
    note(&mut melody_notes, 64, q, &mut t);
    note(&mut melody_notes, 62, q, &mut t);
    note(&mut melody_notes, 62, q, &mut t);
    note(&mut melody_notes, 60, h, &mut t);

    // Phrase 3: G G F F E E D-
    note(&mut melody_notes, 67, q, &mut t);
    note(&mut melody_notes, 67, q, &mut t);
    note(&mut melody_notes, 65, q, &mut t);
    note(&mut melody_notes, 65, q, &mut t);
    note(&mut melody_notes, 64, q, &mut t);
    note(&mut melody_notes, 64, q, &mut t);
    note(&mut melody_notes, 62, h, &mut t);

    // Phrase 4: G G F F E E D-
    note(&mut melody_notes, 67, q, &mut t);
    note(&mut melody_notes, 67, q, &mut t);
    note(&mut melody_notes, 65, q, &mut t);
    note(&mut melody_notes, 65, q, &mut t);
    note(&mut melody_notes, 64, q, &mut t);
    note(&mut melody_notes, 64, q, &mut t);
    note(&mut melody_notes, 62, h, &mut t);

    // Phrase 5 (reprise): C C G G A A G-
    note(&mut melody_notes, 60, q, &mut t);
    note(&mut melody_notes, 60, q, &mut t);
    note(&mut melody_notes, 67, q, &mut t);
    note(&mut melody_notes, 67, q, &mut t);
    note(&mut melody_notes, 69, q, &mut t);
    note(&mut melody_notes, 69, q, &mut t);
    note(&mut melody_notes, 67, h, &mut t);

    // Phrase 6 (reprise): F F E E D D C-
    note(&mut melody_notes, 65, q, &mut t);
    note(&mut melody_notes, 65, q, &mut t);
    note(&mut melody_notes, 64, q, &mut t);
    note(&mut melody_notes, 64, q, &mut t);
    note(&mut melody_notes, 62, q, &mut t);
    note(&mut melody_notes, 62, q, &mut t);
    note(&mut melody_notes, 60, h, &mut t);

    let melody_length = t;

    // Build bass notes (whole notes under each phrase)
    let mut bass_notes = Vec::new();
    t = 0.0;
    let w = 4.0_f32; // whole note = 4 beats

    // Under phrase 1: C, G
    note(&mut bass_notes, 48, w, &mut t);
    note(&mut bass_notes, 43, w, &mut t);
    // Under phrase 2: F, C
    note(&mut bass_notes, 41, w, &mut t);
    note(&mut bass_notes, 36, w, &mut t);
    // Under phrase 3: G, F
    note(&mut bass_notes, 43, w, &mut t);
    note(&mut bass_notes, 41, w, &mut t);
    // Under phrase 4: G, F
    note(&mut bass_notes, 43, w, &mut t);
    note(&mut bass_notes, 41, w, &mut t);
    // Under phrase 5: C, G
    note(&mut bass_notes, 48, w, &mut t);
    note(&mut bass_notes, 43, w, &mut t);
    // Under phrase 6: F, C
    note(&mut bass_notes, 41, w, &mut t);
    note(&mut bass_notes, 36, w, &mut t);

    // Create clips
    let melody_clip = melody_track.create_clip(0, melody_length)?;
    melody_clip.set_name("melody")?;
    melody_clip.set_looping(true)?;
    melody_clip.add_notes(&melody_notes)?;

    let bass_clip = bass_track.create_clip(0, melody_length)?;
    bass_clip.set_name("bass")?;
    bass_clip.set_looping(true)?;
    bass_clip.add_notes(&bass_notes)?;

    println!(
        "Created {} melody notes, {} bass notes ({} beats)",
        melody_notes.len(),
        bass_notes.len(),
        melody_length,
    );

    // Fire both clips and play
    melody_clip.fire()?;
    bass_clip.fire()?;
    session.play()?;

    println!("Playing!");
    Ok(())
}
