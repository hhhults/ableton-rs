# ableton-rs

A Rust client for controlling Ableton Live via OSC.

Connects to a running Ableton Live instance through the [AbletonOSC](https://github.com/ideoforms/AbletonOSC) remote script. Create tracks, write MIDI clips, load instruments and effects, automate parameters, and compile routing graphs — all from Rust.

## Quick start

```rust
use ableton::Session;

let session = Session::connect()?;

// Set tempo and create a track
session.set_tempo(120.0)?;
let track = session.create_midi_track(-1)?;

// Load an instrument
track.load_instrument("Analog")?;

// Create a clip with notes
let clip = track.create_clip(0, 4.0)?;  // slot 0, 4 beats long
clip.add_note(60, 0.0, 1.0, 100)?;      // C4, beat 0, 1 beat, velocity 100
clip.add_note(64, 1.0, 1.0, 80)?;       // E4
clip.add_note(67, 2.0, 1.0, 80)?;       // G4
clip.add_note(72, 3.0, 0.5, 100)?;      // C5

// Fire the clip
clip.fire()?;
```

## Features

### Session control
```rust
session.set_tempo(128.0)?;
session.play()?;
session.stop()?;
session.undo()?;
session.redo()?;
let tempo = session.get_tempo()?;
```

### Tracks
```rust
let track = session.create_midi_track(-1)?;  // -1 = append
track.set_name("bass")?;
track.set_volume(0.8)?;
track.set_panning(-0.3)?;
track.set_mute(false)?;
track.set_solo(true)?;

// Load instruments and effects
track.load_instrument("Operator")?;
track.load_sample("/path/to/sample.wav")?;
track.load_effect("Auto Filter")?;

// Device parameters
track.set_device_parameter(0, 5, 0.7)?;  // device 0, param 5, value 0.7
let params = track.get_device_parameters(0)?;
```

### Clips
```rust
let clip = track.create_clip(0, 8.0)?;
clip.add_note(60, 0.0, 0.5, 100)?;
clip.remove_notes(0.0, 4.0, 48, 84)?;

// Batch note operations
let notes = vec![
    Note { pitch: 60, start: 0.0, duration: 0.5, velocity: 100, mute: false },
    Note { pitch: 64, start: 0.5, duration: 0.5, velocity: 80, mute: false },
];
clip.set_notes(&notes)?;
```

### Return tracks & scenes
```rust
let returns = session.get_return_tracks()?;
let scenes = session.get_scenes()?;
session.fire_scene(0)?;
```

### Incremental updates

`LiveSession` wraps `Session` with change detection for live-coding workflows:

```rust
use ableton::LiveSession;

let mut live = LiveSession::connect()?;

// First compile — creates everything
let result = live.compile(&patch_json)?;  // ChangeKind::Initial

// Modify only patterns — fast path, no track recreation
let result = live.compile(&updated_json)?;  // ChangeKind::ClipsOnly

// Change tempo only — instant
let result = live.compile(&tempo_json)?;  // ChangeKind::TempoOnly
```

Change detection categories:
- **Initial** — first compile, full setup
- **ClipsOnly** — only patterns/notes changed, skip track creation
- **TempoOnly** — instant tempo adjustment
- **Full** — structure changed, teardown and rebuild
- **NoOp** — nothing changed

### IR compilation

Compile [metaritual](https://github.com/hhhults/metaritual) routing graphs into Ableton sessions:

```rust
use ableton::compiler::{IrPatch, compile};

let patch: IrPatch = IrPatch::from_json(&json_string)?;
let result = compile(&session, &patch)?;
```

The compiler handles:
- Creating tracks for each source node
- Loading instruments (Analog, Operator, Simpler, Collision, Wavetable)
- Building effect chains
- Writing MIDI clips from pattern data
- Setting device parameters

## Architecture

```
Your Rust code
    ↓ ableton-rs (this crate)
    ↓ OSC over UDP
AbletonOSC (Remote Script, port 11000/11001)
    ↓
Ableton Live
```

The OSC transport uses a background receiver thread with synchronization for request/response patterns. Default timeout is 2 seconds.

## Prerequisites

- Ableton Live with [AbletonOSC](https://github.com/ideoforms/AbletonOSC) installed
- OSC ports: send on 11000, receive on 11001

## Building

```bash
cargo build
cargo test
```

## Examples

```bash
cargo run --example twinkle
cargo run --example bell
cargo run --example bloom
```

## License

MIT
