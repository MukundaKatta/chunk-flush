use chunk_flush::Flusher;

#[test]
fn flushes_on_newline() {
    let mut f = Flusher::new(500);
    assert_eq!(f.push("hello"), None);
    assert_eq!(f.push(" world\n").as_deref(), Some("hello world\n"));
}

#[test]
fn flushes_at_cap_without_newline() {
    let mut f = Flusher::new(3);
    assert_eq!(f.push("abc"), None);
    assert_eq!(f.push("d").as_deref(), Some("abcd"));
}

#[test]
fn multi_line_yields_all_complete() {
    let mut f = Flusher::new(1000);
    let r = f.push("one\ntwo\nthree");
    assert_eq!(r.as_deref(), Some("one\ntwo\n"));
    // "three" still buffered
    assert!(!f.is_empty());
}

#[test]
fn force_flush_returns_pending() {
    let mut f = Flusher::new(100);
    f.push("tail");
    assert_eq!(f.flush(), "tail");
    assert!(f.is_empty());
}

#[test]
fn empty_push_returns_none() {
    let mut f = Flusher::new(100);
    assert_eq!(f.push(""), None);
}

#[test]
fn tail_over_cap_after_newline_is_not_retained() {
    // After flushing up to the last newline, a leftover tail that
    // exceeds the cap must also be emitted so nothing lingers above
    // the cap. The whole over-cap prefix comes back in one chunk.
    let mut f = Flusher::new(2);
    assert_eq!(f.push("x\nABCDE").as_deref(), Some("x\nABCDE"));
    assert!(f.is_empty());
}

#[test]
fn tail_under_cap_after_newline_is_retained() {
    // A leftover tail at or below the cap stays buffered for line
    // alignment (the documented "yield complete lines" behavior).
    let mut f = Flusher::new(10);
    assert_eq!(f.push("one\ntwo\nthree").as_deref(), Some("one\ntwo\n"));
    assert!(!f.is_empty());
    assert_eq!(f.flush(), "three");
}

#[test]
fn cap_counts_unicode_scalar_values_not_bytes() {
    // The cap is measured in chars (Unicode scalar values), not bytes.
    // Three 3-byte chars (9 bytes) must not trip a cap of 3.
    let mut f = Flusher::new(3);
    assert_eq!(f.push("日本語"), None);
    assert!(!f.is_empty());
    // A fourth char puts the buffer beyond the cap, so it flushes.
    assert_eq!(f.push("学").as_deref(), Some("日本語学"));
}

#[test]
fn flush_on_empty_buffer_returns_empty_string() {
    let mut f = Flusher::new(100);
    assert_eq!(f.flush(), "");
    assert!(f.is_empty());
}
