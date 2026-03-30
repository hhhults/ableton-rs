//! Tests for IR JSON parsing — runs the metaritual Rust DSL and parses its output.

use std::process::Command;

fn run_example(name: &str) -> String {
    let output = Command::new("cargo")
        .args(["run", "--example", name])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_string() + "/../metaritual")
        .output()
        .expect("failed to run example");
    assert!(output.status.success(), "example {name} failed: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).expect("non-utf8 output")
}

#[test]
fn parse_twinkle_ir() {
    let json = run_example("twinkle");
    let patch = ableton::IrPatch::from_json(&json).expect("parse failed");

    assert_eq!(patch.label, "twinkle");
    assert_eq!(patch.bpm, 100.0);
    assert!(!patch.nodes.is_empty());
    assert!(!patch.edges.is_empty());
}

#[test]
fn twinkle_has_correct_node_types() {
    let json = run_example("twinkle");
    let patch = ableton::IrPatch::from_json(&json).unwrap();

    let sources: Vec<_> = patch.nodes.iter()
        .filter(|n| matches!(n.kind, ableton::compiler::IrNodeKind::Source(_)))
        .collect();
    let patterns: Vec<_> = patch.nodes.iter()
        .filter(|n| matches!(n.kind, ableton::compiler::IrNodeKind::Pattern(_)))
        .collect();
    let effects: Vec<_> = patch.nodes.iter()
        .filter(|n| matches!(n.kind, ableton::compiler::IrNodeKind::Effect(_)))
        .collect();

    assert_eq!(sources.len(), 2, "should have 2 sources (piano, pad)");
    assert_eq!(patterns.len(), 2, "should have 2 patterns (melody, bass)");
    assert_eq!(effects.len(), 2, "should have 2 effects (reverb, delay)");
}

#[test]
fn twinkle_melody_has_notes() {
    let json = run_example("twinkle");
    let patch = ableton::IrPatch::from_json(&json).unwrap();

    let pattern = patch.nodes.iter()
        .find(|n| n.label == "melody")
        .expect("no melody node");

    if let ableton::compiler::IrNodeKind::Pattern(ref clip) = pattern.kind {
        assert!(clip.events.len() > 30, "melody should have 42 notes, got {}", clip.events.len());
        // First note should be C4 (60)
        assert_eq!(clip.events[0].value, 60.0);
    } else {
        panic!("melody node is not a pattern");
    }
}

#[test]
fn twinkle_bass_has_notes() {
    let json = run_example("twinkle");
    let patch = ableton::IrPatch::from_json(&json).unwrap();

    let pattern = patch.nodes.iter()
        .find(|n| n.label == "bass")
        .expect("no bass node");

    if let ableton::compiler::IrNodeKind::Pattern(ref clip) = pattern.kind {
        assert_eq!(clip.events.len(), 12, "bass should have 12 notes");
        // First bass note should be C3 (48)
        assert_eq!(clip.events[0].value, 48.0);
    } else {
        panic!("bass node is not a pattern");
    }
}

#[test]
fn twinkle_has_edges() {
    let json = run_example("twinkle");
    let patch = ableton::IrPatch::from_json(&json).unwrap();
    assert_eq!(patch.edges.len(), 4, "should have 4 edges");
}

#[test]
fn twinkle_has_space() {
    let json = run_example("twinkle");
    let patch = ableton::IrPatch::from_json(&json).unwrap();
    assert!(!patch.space.pan.breakpoints.is_empty(), "pan should have breakpoints");
}

#[test]
fn twinkle_has_exposed_params() {
    let json = run_example("twinkle");
    let patch = ableton::IrPatch::from_json(&json).unwrap();
    assert_eq!(patch.exposed_params.len(), 2);
    let names: Vec<_> = patch.exposed_params.iter().map(|p| &p.name).collect();
    assert!(names.contains(&&"reverb_mix".to_string()));
    assert!(names.contains(&&"brightness".to_string()));
}

#[test]
fn parse_water_braid_ir() {
    let json = run_example("water_braid");
    let patch = ableton::IrPatch::from_json(&json).expect("parse failed");
    assert_eq!(patch.label, "water_braid");
    assert!(!patch.nodes.is_empty());
}
