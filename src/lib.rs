//! # agent-event-emit
//!
//! Structured event emitter for agent runs. One event per significant
//! step (LLM call, tool call, error, etc.). Each event carries:
//!
//! - `run_id` — caller-supplied; same across the whole run.
//! - `id` — monotonic per-emitter; assigned on emit.
//! - `ts_unix_ms` — wall-clock millis when emitted.
//! - `kind` — short string label.
//! - `payload` — arbitrary `serde_json::Value`.
//!
//! Serializes cleanly to JSON Lines for log shipping.
//!
//! ## Example
//!
//! ```
//! use agent_event_emit::Emitter;
//! use serde_json::json;
//! let mut e = Emitter::new("run-123");
//! let ev = e.emit("tool_call", json!({"name": "read_file", "path": "a.txt"}));
//! assert_eq!(ev.id, 1);
//! assert_eq!(ev.run_id, "run-123");
//! ```

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

/// One emitted event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Run identifier supplied to the emitter.
    pub run_id: String,
    /// Monotonic event id, 1-based.
    pub id: u64,
    /// Wall-clock millis since the Unix epoch.
    pub ts_unix_ms: u64,
    /// Short event kind label.
    pub kind: String,
    /// Caller-provided payload.
    pub payload: Value,
}

impl Event {
    /// Serialize as one JSON line (no trailing newline).
    ///
    /// # Panics
    ///
    /// Panics only if serialization fails, which cannot happen for a
    /// well-formed [`Event`]: every field is JSON-serializable and the
    /// payload is already a [`serde_json::Value`].
    #[must_use]
    pub fn to_json_line(&self) -> String {
        serde_json::to_string(self).expect("serialize Event")
    }
}

/// Stateful event emitter for a single run.
#[derive(Debug, Clone)]
pub struct Emitter {
    run_id: String,
    next_id: u64,
}

impl Emitter {
    /// Build an emitter for `run_id`. First emit will have id=1.
    #[must_use]
    pub fn new(run_id: impl Into<String>) -> Self {
        Self {
            run_id: run_id.into(),
            next_id: 1,
        }
    }

    /// Emit an event. Returns the constructed event (also write it
    /// wherever you want — this crate doesn't pick a sink).
    pub fn emit(&mut self, kind: impl Into<String>, payload: Value) -> Event {
        let id = self.next_id;
        self.next_id += 1;
        Event {
            run_id: self.run_id.clone(),
            id,
            ts_unix_ms: now_ms(),
            kind: kind.into(),
            payload,
        }
    }

    /// Current run id.
    #[must_use]
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// How many events have been emitted so far.
    #[must_use]
    pub fn count(&self) -> u64 {
        self.next_id - 1
    }
}

fn now_ms() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        // `as_millis` returns u128; millis since the epoch fit in u64 for
        // hundreds of millions of years, so the cast cannot truncate.
        Ok(d) => u64::try_from(d.as_millis()).unwrap_or(u64::MAX),
        Err(_) => 0,
    }
}
