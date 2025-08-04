#[allow(dead_code)] // TODO
pub(crate) fn lpad(s: &str, n: usize) -> String {
    assert!(s.len() <= n);
    let padding = n - s.len();
    let mut t = String::with_capacity(s.len() + padding);
    t.extend((0..padding).map(|_| ' '));
    t.push_str(s);
    t
}

pub(crate) fn rpad(s: &str, n: usize) -> String {
    assert!(s.len() <= n);
    let padding = n - s.len();
    let mut t = String::with_capacity(s.len() + padding);
    t.push_str(s);
    t.extend((0..padding).map(|_| ' '));
    t
}
