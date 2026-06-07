use agent_event_emit::Emitter;
use serde_json::json;

#[test]
fn ids_are_monotonic() {
    let mut e = Emitter::new("r1");
    assert_eq!(e.emit("a", json!({})).id, 1);
    assert_eq!(e.emit("b", json!({})).id, 2);
    assert_eq!(e.emit("c", json!({})).id, 3);
}

#[test]
fn run_id_is_carried() {
    let mut e = Emitter::new("run-xyz");
    let ev = e.emit("x", json!(1));
    assert_eq!(ev.run_id, "run-xyz");
}

#[test]
fn json_line_roundtrips() {
    let mut e = Emitter::new("r");
    let ev = e.emit("k", json!({"foo": "bar"}));
    let line = ev.to_json_line();
    let back: agent_event_emit::Event = serde_json::from_str(&line).unwrap();
    assert_eq!(back.kind, "k");
    assert_eq!(back.payload["foo"], json!("bar"));
}

#[test]
fn count_tracks_emits() {
    let mut e = Emitter::new("r");
    assert_eq!(e.count(), 0);
    e.emit("a", json!({}));
    e.emit("b", json!({}));
    assert_eq!(e.count(), 2);
}

#[test]
fn timestamps_are_present() {
    let mut e = Emitter::new("r");
    let ev = e.emit("k", json!({}));
    assert!(ev.ts_unix_ms > 1_700_000_000_000);
}

#[test]
fn run_id_accessor_returns_supplied_id() {
    let e = Emitter::new("run-accessor");
    assert_eq!(e.run_id(), "run-accessor");
}

#[test]
fn cloned_emitter_has_independent_counter() {
    let mut e = Emitter::new("r");
    e.emit("a", json!({}));
    let mut cloned = e.clone();
    // Both continue from the same point but advance independently.
    assert_eq!(e.emit("b", json!({})).id, 2);
    assert_eq!(cloned.emit("b", json!({})).id, 2);
    assert_eq!(e.count(), 2);
    assert_eq!(cloned.count(), 2);
}

#[test]
fn kind_is_carried_into_event() {
    let mut e = Emitter::new("r");
    let ev = e.emit("llm_call", json!({}));
    assert_eq!(ev.kind, "llm_call");
}
