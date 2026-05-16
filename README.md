# chunk-flush

[![crates.io](https://img.shields.io/crates/v/chunk-flush.svg)](https://crates.io/crates/chunk-flush)

Flush-on-newline buffer for streaming LLM output. Holds chars until a
newline or the size cap, then emits a chunk.

```rust
use chunk_flush::Flusher;
let mut f = Flusher::new(500);
let out = f.push("hello world\n");
```

Zero deps. MIT or Apache-2.0.

## Repository Health

This repository includes a dependency-free health check for core documentation, metadata, and CI wiring. Run it locally before publishing changes:

```sh
python3 scripts/check_repository_health.py
```

The same check runs in GitHub Actions on pushes and pull requests.
