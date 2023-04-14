pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

pub fn is_not_blank(s: &str) -> bool {
    s.trim().len() > 0
}
