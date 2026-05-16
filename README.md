# agent-event-emit

[![crates.io](https://img.shields.io/crates/v/agent-event-emit.svg)](https://crates.io/crates/agent-event-emit)

Structured event emitter for agent runs. JSON-line-ready Events with
monotonic ids, run id, and ms timestamps. No sink chosen; you write
the line where you want.

```rust
use agent_event_emit::Emitter;
use serde_json::json;
let mut e = Emitter::new("run-123");
let ev = e.emit("tool_call", json!({"name": "read_file"}));
println!("{}", ev.to_json_line());
```

MIT or Apache-2.0.

## Repository Health

This repository includes a dependency-free health check for core documentation, metadata, and CI wiring. Run it locally before publishing changes:

```sh
python3 scripts/check_repository_health.py
```

The same check runs in GitHub Actions on pushes and pull requests.
