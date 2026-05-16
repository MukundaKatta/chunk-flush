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
