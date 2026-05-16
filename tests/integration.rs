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
