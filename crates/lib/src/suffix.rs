// ref: suffix
pub(crate) const SUFFIXES: &[(&str, &str)] = &[
    ("able", "B"),
    ("ation", "A"),
    ("ent", "N"),
    ("hood", "O"),
    ("ing", "q"),
    ("ity", "Y"),
    ("ment", "M"),
    ("ness", "S"),
    ("ship", "H"),
    ("ted", "D"),
    ("tion", "T"),
];

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::SUFFIXES;

    #[test]
    fn test_abbrevs() {
        assert_eq!(
            SUFFIXES.len(),
            SUFFIXES.iter().copied().collect::<HashMap<_, _>>().len()
        );
        assert_eq!(
            SUFFIXES.len(),
            SUFFIXES
                .iter()
                .copied()
                .map(|(l, r)| (r, l))
                .collect::<HashMap<_, _>>()
                .len()
        );
    }
}
