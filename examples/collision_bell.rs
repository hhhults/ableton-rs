/// Bell sound using Collision — physical modeling synthesis.
/// Collision models a mallet striking a resonating body, which is
/// exactly how a real bell works.

use ableton::{Note, Session};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = Session::connect()?;
    session.set_tempo(90.0)?;

    let track = session.track(0);

    // Remove existing devices
    while track.num_devices()? > 0 {
        track.delete_device(0)?;
        thread::sleep(Duration::from_millis(200));
    }

    // Load Collision
    println!("loading Collision...");
    session.load_instrument(0, "Collision")?;
    thread::sleep(Duration::from_millis(500));

    let dev = track.device(0);
    println!("loaded: {:?}", dev.name()?);

    // -- Mallet: hard, bright strike --
    dev.set_param(7, 1.0)?;     // Mallet On
    dev.set_param(8, 0.6)?;     // Mallet Volume
    dev.set_param(9, 0.4)?;     // Mallet Volume < Vel (velocity sensitive)
    dev.set_param(11, 0.75)?;   // Mallet Stiffness = hard (metallic attack)
    dev.set_param(12, 0.3)?;    // Mallet Stiffness < Vel (harder when hit harder)
    dev.set_param(14, 0.25)?;   // Mallet Noise Amount = some transient
    dev.set_param(17, 0.65)?;   // Mallet Noise Color = bright

    // -- Resonator 1: the bell body --
    dev.set_param(32, 1.0)?;    // Res 1 On
    dev.set_param(33, 1.0)?;    // Res 1 Type = Beam (good for metallic tones)
    dev.set_param(34, 2.0)?;    // Res 1 Quality = high
    dev.set_param(41, 0.82)?;   // Res 1 Decay = long ring
    dev.set_param(42, -0.3)?;   // Res 1 Decay < Vel (softer hits ring longer)
    dev.set_param(45, 0.65)?;   // Res 1 Material = metallic
    dev.set_param(48, 0.35)?;   // Res 1 Radius = strike position
    dev.set_param(52, 0.55)?;   // Res 1 Brightness = shimmery
    dev.set_param(54, 0.4)?;    // Res 1 Inharmonics = bell-like partials!
    dev.set_param(56, 0.85)?;   // Res 1 Opening = open (undamped)
    dev.set_param(64, 0.6)?;    // Res 1 Volume

    // -- Resonator 2: sympathetic resonance (like a bell's body modes) --
    dev.set_param(65, 1.0)?;    // Res 2 On
    dev.set_param(66, 2.0)?;    // Res 2 Type = Marimba
    dev.set_param(67, 2.0)?;    // Res 2 Quality = high
    dev.set_param(68, 7.0)?;    // Res 2 Tune = up (higher partial)
    dev.set_param(69, 0.15)?;   // Res 2 Fine Tune = slightly off (shimmer)
    dev.set_param(74, 0.7)?;    // Res 2 Decay = long
    dev.set_param(78, 0.5)?;    // Res 2 Material
    dev.set_param(81, 0.4)?;    // Res 2 Radius
    dev.set_param(85, 0.45)?;   // Res 2 Brightness
    dev.set_param(87, 0.35)?;   // Res 2 Inharmonics
    dev.set_param(89, 0.9)?;    // Res 2 Opening = open
    dev.set_param(97, 0.35)?;   // Res 2 Volume (quieter, it's the shimmer)

    // -- Structure: how the resonators interact --
    dev.set_param(1, 0.0)?;     // Structure = 1>2 (mallet hits Res 1, feeds into Res 2)

    // Clean up old clip
    if track.has_clip(0).unwrap_or(false) {
        track.delete_clip(0)?;
        thread::sleep(Duration::from_millis(100));
    }

    // Create the bell melody — same gamelan-ish pattern
    let clip = track.create_clip(0, 16.0)?;
    clip.set_name("collision bells")?;
    clip.set_looping(true)?;

    let melody = [
        (72, 0.0,  1.5, 110),
        (67, 2.0,  1.5, 90),
        (64, 4.0,  2.0, 100),
        (60, 6.5,  1.5, 85),
        (76, 8.0,  1.0, 115),
        (72, 9.0,  1.0, 95),
        (79, 10.0, 1.5, 105),
        (76, 11.5, 0.5, 75),
        (72, 12.0, 2.0, 100),
        (67, 14.0, 2.0, 90),
    ];

    let notes: Vec<Note> = melody.iter()
        .map(|&(p, s, d, v)| Note::new(p, s, d, v))
        .collect();
    clip.add_notes(&notes)?;

    clip.fire()?;
    session.play()?;
    println!("\nbells ringing — listen to the difference from Analog!");
    thread::sleep(Duration::from_secs(16));

    session.stop()?;
    println!("done");

    Ok(())
}
