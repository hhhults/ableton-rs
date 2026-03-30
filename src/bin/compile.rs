//! metaritual-compile — Rust replacement for the Python compiler.
//!
//! Reads IR JSON from stdin, compiles it to Ableton Live.
//!
//!     cargo run --example water_braid | cargo run --bin compile
//!     cargo run --example twinkle | cargo run --bin compile

use std::io::Read;

fn main() {
    let mut json = String::new();
    std::io::stdin().read_to_string(&mut json).expect("failed to read stdin");

    if json.trim().is_empty() {
        eprintln!("error: no input (pipe IR JSON to stdin)");
        std::process::exit(1);
    }

    let patch = match ableton::IrPatch::from_json(&json) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error parsing IR JSON: {e}");
            std::process::exit(1);
        }
    };

    eprintln!("Parsed patch '{}': {} nodes, {} edges, {} BPM",
        patch.label, patch.nodes.len(), patch.edges.len(), patch.bpm);

    // Check for --dry-run
    let dry_run = std::env::args().any(|a| a == "--dry-run");

    if dry_run {
        eprintln!("Dry run — not connecting to Ableton");
        eprintln!("Would create:");
        for node in &patch.nodes {
            eprintln!("  {:?}", node.label);
        }
        return;
    }

    let session = match ableton::Session::connect() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error connecting to Ableton: {e}");
            eprintln!("Is Ableton Live running with AbletonOSC?");
            std::process::exit(1);
        }
    };

    match ableton::compiler::compile(&patch, &session) {
        Ok(result) => {
            println!("{}", result.summary());

            // Fire all clips and play
            if let Ok(tracks) = session.tracks() {
                for track in &tracks {
                    if track.has_clip(0).unwrap_or(false) {
                        let clip = track.clip(0);
                        let _ = clip.fire();
                    }
                }
            }
            let _ = session.play();
            println!("Playing!");
        }
        Err(e) => {
            eprintln!("compile error: {e}");
            std::process::exit(1);
        }
    }
}
