// TODO: src and tgt need to have alpha char
// TODO: if "ws'" is an abbreviation, "ws" must be one
// TODO: if "w's" is an abbreviation, "w" must be one
// TODO: if "wq" is an abbreviation, "w" must be one

use std::{error, fmt};

use crate::{
    abbrev::{char_is_valid_in_abbrev_src, char_is_valid_in_abbrev_tgt},
    ambiguity::ambiguity,
    effort::qwerty::qwerty_effort,
    mem::memorability,
    words::WORDS,
};

#[non_exhaustive]
#[derive(Debug)]
pub enum RequirementError {
    NotTypeable(String),
    AlreadyAWord(String),
    TooMuchEffort(String),
    InvalidSrc(String),
    InvalidTgt(String),
    NotUnique {
        long0: String,
        long1: String,
        short: String,
    },
    NotMemorable(String),
    NotUniquelyMemorable {
        long0: String,
        long1: String,
        short: String,
    },
    KnownAmbiguous(String),
    Ambiguous(String),
}

impl fmt::Display for RequirementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequirementError::NotTypeable(s) => {
                write!(f, "[R001]: Can't type '{s}' on an ANSI US QWERTY keyboard")
            }
            RequirementError::AlreadyAWord(s) => {
                write!(f, "[R002]: '{s}' can't be an abbreviation, it is a word")
            }
            RequirementError::TooMuchEffort(s) => {
                write!(f, "[R003]: '{s}' doesn't save enough effort")
            }
            RequirementError::InvalidSrc(s) => {
                write!(f, "[R004]: Invalid character in source '{s}'")
            }
            RequirementError::InvalidTgt(s) => {
                write!(f, "[R005]: Invalid character in target '{s}'")
            }
            RequirementError::NotUnique {
                long0,
                long1,
                short,
            } => {
                write!(
                    f,
                    "[R006]: '{long0}' and '{long1}' both abbreviate to '{short}'"
                )
            }
            RequirementError::NotMemorable(s) => {
                write!(f, "[R007]: '{s}' is not memorable enough")
            }
            RequirementError::NotUniquelyMemorable {
                long0,
                long1,
                short,
            } => {
                write!(
                    f,
                    "[R008]: '{short}' would be memorable for both '{long0}' and '{long1}'"
                )
            }
            RequirementError::KnownAmbiguous(k) => {
                write!(f, "[R009]: '{k}' is a known highly ambiguous abbreviation")
            }
            RequirementError::Ambiguous(s) => write!(f, "[R010]: '{s}' is highly ambiguous"),
        }
    }
}

impl error::Error for RequirementError {}

// ref: r001-typeable
fn check_typeable(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    for (_, short) in abbrevs.iter().copied() {
        match qwerty_effort(short) {
            Some(_) => (),
            None => return Err(RequirementError::NotTypeable(short.to_owned())),
        }
    }
    Ok(())
}

// ref: r002-words
fn check_words(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    const EXCEPTIONS: &[(&str, &str)] = &[("I", "i"), ("I'd", "id"), ("I'm", "im")];
    for (long, short) in abbrevs.iter().copied() {
        if EXCEPTIONS.contains(&(long, short)) {
            continue;
        }
        if WORDS.contains(&short) {
            return Err(RequirementError::AlreadyAWord(short.to_owned()));
        }
    }
    Ok(())
}

// ref: r003-effort
fn check_effort(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    const EXCEPTIONS: &[(&str, &str)] = &[
        ("PayPal", "paypal"),
        // numbers, for consistency
        ("one", "n1"),
        ("two", "n2"),
        ("four", "n4"),
        ("five", "n5"),
        ("six", "n6"),
        ("ten", "n10"),
        ("tenth", "o10"),
    ];
    const MIN_SAVINGS: u64 = 5; // TODO(strictness): increase this?
    for (long, short) in abbrevs.iter().copied() {
        if EXCEPTIONS.contains(&(long, short)) || short.contains(',') {
            continue;
        }
        let long_effort =
            qwerty_effort(long).ok_or_else(|| RequirementError::NotTypeable(long.to_owned()))?;
        let short_effort =
            qwerty_effort(short).ok_or_else(|| RequirementError::NotTypeable(short.to_owned()))?;
        let min = if WORDS[..128].contains(&long) {
            1
        } else {
            MIN_SAVINGS
        };
        if long_effort < short_effort + min {
            return Err(RequirementError::TooMuchEffort(short.to_owned()));
        }
    }
    Ok(())
}

// ref: r004-valid-src
// ref: r005-valid-tgt
fn check_valid_src_tgt(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    #[allow(clippy::unnecessary_to_owned)]
    for (long, short) in abbrevs.iter().copied() {
        if long
            .as_bytes()
            .iter()
            .copied()
            .any(|c| !char_is_valid_in_abbrev_src(c))
        {
            return Err(RequirementError::InvalidSrc(long.to_owned()));
        }
        if short
            .as_bytes()
            .iter()
            .copied()
            .any(|c| !char_is_valid_in_abbrev_tgt(c))
        {
            return Err(RequirementError::InvalidTgt(short.to_owned()));
        }
    }
    Ok(())
}

// ref: r006-unique
fn check_unique(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    for (i0, (long0, short0)) in abbrevs.iter().copied().enumerate() {
        for (i1, (long1, short1)) in abbrevs.iter().copied().enumerate() {
            if i0 == i1 {
                continue;
            }
            if short0 == short1 {
                return Err(RequirementError::NotUnique {
                    long0: long0.to_owned(),
                    long1: long1.to_owned(),
                    short: short0.to_owned(),
                });
            }
        }
    }
    Ok(())
}

fn is_num(short: &str) -> bool {
    [
        "n0", "n1", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9", "n10", "n11", "n12", "n13",
        "n14", "n15", "n16", "n17", "n18", "n19", "n20", "o0", "o1", "o2", "o3", "o4", "o5", "o6",
        "o7", "o8", "o9", "o10", "o11", "o12", "o13", "o14", "o15", "o16", "o17", "o18", "o19",
        "o20",
    ]
    .contains(&short)
}

// ref: r007-memorable
fn check_memorable(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    const EXCEPTIONS: &[(&str, &str)] = &[
        ("having", "hvq"),
        ("I've", "iv"),
        ("okay", "k"),
        ("please", "plz"),
        ("thanks", "thx"),
    ];
    const MIN_MEMORABILITY: usize = 5;
    for (long, short) in abbrevs.iter().copied() {
        if WORDS[..128].contains(&long) && short.len() == 1 {
            continue;
        }
        if EXCEPTIONS.contains(&(long, short)) || short.contains(',') || is_num(short) {
            continue;
        }
        if memorability(abbrevs, long, short) < MIN_MEMORABILITY {
            return Err(RequirementError::NotMemorable(short.to_owned()));
        }
    }
    Ok(())
}

fn to_lemma(w0: &str) -> &str {
    for suf in ["'s", "s'", "es", "s", "er", "est", "ily", "ly"] {
        if let Some(w) = w0.strip_suffix(suf) {
            return w;
        }
    }
    w0
}

fn is_inflection_of(w0: &str, w: &str) -> bool {
    w0 == to_lemma(w) || to_lemma(w0) == to_lemma(w)
}

// ref: r008-uniquely-memorable
fn check_uniquely_memorable(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    const EXCEPTIONS: &[(&str, &str)] = &[
        ("ain't", "ai'"),
        ("and", "d"),
        ("are", "r"),      // ok: existing
        ("because", "bc"), // ok: existing
        ("can", "c"),
        ("don't", "do'"),
        ("for", "f"),
        ("from", "m"),
        ("hadn't", "had'"),
        ("hasn't", "has'"),
        ("having", "hvq"),
        ("is", "s"), // TODO: maybe remove?
        ("just", "j"),
        ("March", "mar"),
        ("much", "mh"), // "March"
        ("no", "n"),    // "nineteen"
        ("not", "x"),
        ("of", "o"),
        ("okay", "k"), // ok: existing
        ("please", "plz"),
        ("should", "sd"),
        ("thanks", "thx"),
        ("that", "tt"),
        ("the", "l"),
        ("they", "e"), // TODO: maybe remove?
        ("this", "h"), // TODO: maybe remove?
        ("though", "tho"),
        ("thousand", "thou"),
        ("tonight", "tn"), // ok: existing abbrev. "thing", "between"
        ("to", "t"),
        ("was", "w"),
        ("were", "ee"), // TODO: maybe remove?
        ("without", "wo"),
        ("won't", "wo'"),
        ("yes", "y"),
        ("your", "ur"),
        ("you", "u"),
    ];
    const SPECIFIC_EXCEPTIONS: &[(&str, &str, &str)] = &[
        ("ain't", "ai'", "appointment's"),
        ("ain't", "ai'", "appointments'"),
        ("also", "al", "April"),
        ("apartment", "aptm", "appointment"),
        ("apartments", "aptms", "appointments"),
        ("appointment", "aptm", "apartment"),
        ("appointments", "aptms", "apartments"),
        ("baby", "bb", "babies"),
        ("down", "dn", "dozen"),
        ("his", "hs", "has"),
        ("I'll", "il", "international"),
        ("I'm", "im", "IBM"),
        ("othering", "othq", "otherwise"), // TODO: fix algorithm
        ("meeting", "mtg", "marketing"),
        ("minimize", "minz", "minimizing"),
        ("maximize", "maxz", "maximizing"),
        ("Marchs", "mars", "markets"),
        ("March's", "mar'", "market's"),
        ("March's", "mar'", "markets'"),
        ("Marchs'", "mars'", "markets'"),
        ("Aprils", "aprs", "apartments"),
        ("April's", "apr'", "apartment's"),
        ("April's", "apr'", "apartments'"),
        ("Aprils'", "aprs'", "apartments'"),
        ("never", "nv", "November"),
        ("only", "oy", "okay"),
        ("other", "oth", "otherwise"),
        ("right", "rt", "request"),
        ("right", "rt", "requirement"),
        ("rights", "rts", "requirements"),
        ("rights", "rts", "requests"),
        ("several", "sev", "seven"),
        ("service", "svc", "services"),
        ("that", "tt", "tonight"),
        ("will", "ll", "little"),
        ("questioning", "qq", "question"),
        ("questioning", "qq", "questions"),
    ];
    const WORD_EXCEPTIONS: &[(&str, &str)] = &[
        // OK, IMO:
        ("between", "bw"),
        ("could", "cd"),
        ("I'd", "id"),
        ("I'll", "il"),
        ("I'm", "im"),
        ("I've", "iv"),
        ("Marchs", "mars"),   // "Mars", "marshall"
        ("no", "n"),          // ok: existing abbrev. "nation"
        ("okay", "k"),        // ok: existing abbrev. "know"
        ("Saturday", "satd"), // "started", "stated"
        ("should", "sd"),
        ("tonight", "tn"), // ok: existing abbrev. "thing", "between"
        ("will", "ll"),    // "all", "local", "level"
        ("without", "wo"), // "who", "would"
        ("would", "wd"),
        ("yes", "y"),   // ok: existing abbrev. "you", "your"
        ("your", "ur"), //  ok: existing abbrev."under"
        // TODO reconsider:
        ("were", "ee"),
    ];
    const SPECIFIC_WORD_EXCEPTIONS: &[(&str, &str, &str)] = &[
        ("also", "al", "all"),
        ("billion", "bil", "bill"),
        ("must", "mt", "meet"), // also has "mustn't"
        ("must", "mt", "met"),  // also has "mustn't"
        ("president", "pres", "press"),
        ("rights", "rts", "rates"),
        ("several", "sev", "seven"),
        ("something", "smth", "smith"),
        ("will", "ll", "little"),
    ];
    for (i0, (long0, short)) in abbrevs.iter().copied().enumerate() {
        let m0 = memorability(abbrevs, long0, short);
        if short.contains(',') || is_num(short) {
            continue;
        }
        for (i, (long, short1)) in abbrevs.iter().copied().enumerate() {
            if i0 == i
                || short1.contains(',')
                || EXCEPTIONS.contains(&(long0, short))
                || SPECIFIC_EXCEPTIONS.contains(&(long0, short, long))
                || is_inflection_of(long0, long)
            {
                continue;
            }
            let is_more_common = WORDS.contains(&long0) && !WORDS.contains(&long);
            let m = memorability(abbrevs, long, short);
            if m >= m0 && !is_more_common {
                return Err(RequirementError::NotUniquelyMemorable {
                    long0: long0.to_owned(),
                    long1: long.to_owned(),
                    short: short.to_owned(),
                });
            }
        }
        let rank0 = WORDS
            .iter()
            .copied()
            .position(|v| v == long0)
            .unwrap_or(usize::MAX);
        if rank0 < 32 {
            // don't fight me on these really common ones
            continue;
        }
        for (rank, long) in WORDS.iter().copied().enumerate() {
            if long == long0
                || WORD_EXCEPTIONS.contains(&(long0, short))
                || SPECIFIC_WORD_EXCEPTIONS.contains(&(long0, short, long))
                || is_inflection_of(long0, long)
            {
                continue;
            }
            // If the one is way more common, then it's fine.
            //
            // "also" conflicts with "april" at 1200, so 1100 seems ok.
            if rank.saturating_sub(rank0) > 1100 {
                continue;
            }
            let m = memorability(abbrevs, long, short);
            // Ideally, this would be >=. But that causes some kind of
            // stupid failures. Revisit memorability algorithm, perhaps?
            if m > m0 {
                return Err(RequirementError::NotUniquelyMemorable {
                    long0: long0.to_owned(),
                    long1: long.to_owned(),
                    short: short.to_owned(),
                });
            }
        }
    }
    Ok(())
}

// ref: r009-known-ambiguous
fn check_known_ambiguous(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    const KNOWN_AMBIGUOUS: &[&str] = &[
        "ar", // "another", "around"
        "cm", // "come", "came"
        "ev", // "even", "ever", "every"
        "lg", // "large", "lag", "long"
        "lk", // "like", "look"
        "mt", // "might", "most", "must"
        "nw", // "new", "now"
        "se", // "sea", "see", "set", "she"
        "sm", // "same", "some"
        "te", // "the", "their", "there"
        "th", // "the", "that", "their", "this"
        "tr", // "their", "they're"
        "wn", // "wane", "won", "when"
        "wh", // "with", "what", "when", "where", "why"
        "wr", // "were", "where"
        "wt", // "wait", "what"
        "yr", // "year", "your"
    ];
    #[allow(clippy::unnecessary_to_owned)] // TODO: report false positive
    for (_, short) in abbrevs.iter().copied() {
        if KNOWN_AMBIGUOUS.contains(&short) {
            return Err(RequirementError::KnownAmbiguous(short.to_owned()));
        }
    }
    Ok(())
}

// ref: r010-unambiguous
fn check_unambiguous(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    const EXCEPTIONS: &[(&str, &str)] = &[
        ("I'd", "id"),
        ("I'll", "il"),
        ("need", "nd"),        // TODO: investigate?
        ("okay", "k"),         // ok: existing abbrev
        ("questioning", "qq"), // ok: existing plural
        ("question", "q"),     // ok: existing abbrev
        ("questions", "qs"),   // ok: existing plural
        ("righting", "rtq"),   // ok: inflection
        ("thing", "tg"),       // TODO: investigate?
        ("tonight", "tn"),     // ok: existing abbrev
        ("yes", "y"),          // ok: existing abbrev
    ];
    const MAX_AMBIGUITY: usize = 1100;
    for (long, short) in abbrevs.iter().copied() {
        if short.contains(',') {
            continue;
        }
        if WORDS[..128].contains(&long) {
            continue;
        }
        if EXCEPTIONS.contains(&(long, short)) {
            continue;
        }
        if ambiguity(long, short) > MAX_AMBIGUITY {
            return Err(RequirementError::Ambiguous(short.to_owned()));
        }
    }
    Ok(())
}

#[allow(clippy::missing_errors_doc)]
pub fn check(abbrevs: &[(&str, &str)]) -> Result<(), RequirementError> {
    check_typeable(abbrevs)?;
    check_words(abbrevs)?;
    check_effort(abbrevs)?;
    check_valid_src_tgt(abbrevs)?;
    check_unique(abbrevs)?;
    check_memorable(abbrevs)?;
    check_uniquely_memorable(abbrevs)?;
    check_known_ambiguous(abbrevs)?;
    check_unambiguous(abbrevs)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::abbrev::ABBREVS;

    fn abbrevs() -> Vec<(&'static str, &'static str)> {
        ABBREVS
            .iter()
            .copied()
            .map(|(long, short, _)| (long, short))
            .collect()
    }

    #[test]
    fn to_lemma() {
        assert_eq!(super::to_lemma("apartment"), "apartment");
        assert_eq!(super::to_lemma("apartments"), "apartment");
        assert_eq!(super::to_lemma("apartment's"), "apartment");
    }

    #[test]
    fn is_inflection_of() {
        assert!(super::is_inflection_of("apartment", "apartment's"));
        assert!(super::is_inflection_of("apartment", "apartments"));
        assert!(super::is_inflection_of("apartment", "apartments'"));
        assert!(super::is_inflection_of("apartment's", "apartments'"));
        assert!(super::is_inflection_of("other", "others"));
    }

    #[test]
    fn check_typeable() {
        super::check_typeable(abbrevs().as_slice()).unwrap();
    }

    #[test]
    fn check_words() {
        super::check_words(abbrevs().as_slice()).unwrap();
    }

    #[test]
    fn check_effort() {
        super::check_effort(abbrevs().as_slice()).unwrap();
    }

    #[test]
    fn check_effort_sad() {
        assert!(super::check_effort(&[("sad", "sd")]).is_err());
    }

    #[test]
    fn check_valid_src_tgt() {
        super::check_valid_src_tgt(abbrevs().as_slice()).unwrap();
    }

    #[test]
    fn check_unique() {
        super::check_unique(abbrevs().as_slice()).unwrap();
    }

    #[test]
    fn check_memorable() {
        super::check_memorable(abbrevs().as_slice()).unwrap();
    }

    #[test]
    fn check_uniquely_memorable() {
        super::check_uniquely_memorable(abbrevs().as_slice()).unwrap();
    }

    #[test]
    fn check_uniquely_memorable_cp() {
        assert!(super::check_uniquely_memorable(&[("copy", "cp")]).is_err());
        assert!(super::check_uniquely_memorable(&[("cup", "cp")]).is_ok());
        assert!(super::check_uniquely_memorable(&[("cop", "cp")]).is_ok());
    }

    #[test]
    fn check_known_ambiguous() {
        super::check_known_ambiguous(abbrevs().as_slice()).unwrap();
    }

    #[test]
    fn check_unambiguous() {
        super::check_unambiguous(abbrevs().as_slice()).unwrap();
    }
}
