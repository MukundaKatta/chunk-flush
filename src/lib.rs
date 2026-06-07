//! # chunk-flush
//!
//! Flush-on-newline buffer for streaming LLM output. Holds chars until
//! either a newline arrives or `max_pending_chars` is exceeded, then
//! emits the buffered chunk.
//!
//! The point: keep the UI/log from churning on every single token
//! delta, without making it laggy on long lines.
//!
//! ## Example
//!
//! ```
//! use chunk_flush::Flusher;
//! let mut f = Flusher::new(500);
//! assert_eq!(f.push("hello"), None);     // no \n yet, under cap
//! assert_eq!(f.push(" world\n").as_deref(), Some("hello world\n"));
//! ```

#![deny(missing_docs)]

/// Streaming buffer with flush-on-newline + size cap.
#[derive(Debug, Clone)]
pub struct Flusher {
    buf: String,
    max_pending_chars: usize,
}

impl Flusher {
    /// Build a flusher with a max-pending cap, measured in chars
    /// (Unicode scalar values, not bytes). Once the buffer holds more
    /// than this many chars it auto-flushes even without a newline, so
    /// nothing ever lingers above the cap.
    pub fn new(max_pending_chars: usize) -> Self {
        Self {
            buf: String::new(),
            max_pending_chars,
        }
    }

    /// Push next chunk. Returns the new emitted chunk when one is
    /// ready, else `None`.
    pub fn push(&mut self, chunk: &str) -> Option<String> {
        self.buf.push_str(chunk);
        // If the buffer contains a newline, flush everything up to
        // (and including) the last newline.
        if let Some(last_nl) = self.buf.rfind('\n') {
            let split = last_nl + 1;
            let mut out: String = self.buf.drain(..split).collect();
            // If the leftover tail still exceeds the cap, flush it too
            // so nothing lingers above the cap. We can only return a
            // single chunk per push, so fold the residue into `out`.
            if self.buf.chars().count() > self.max_pending_chars {
                out.push_str(&self.flush());
            }
            return Some(out);
        }
        // No newline yet — check the size cap.
        if self.buf.chars().count() > self.max_pending_chars {
            return Some(self.flush());
        }
        None
    }

    /// Force-flush whatever is buffered.
    pub fn flush(&mut self) -> String {
        std::mem::take(&mut self.buf)
    }

    /// True when nothing is buffered.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}
