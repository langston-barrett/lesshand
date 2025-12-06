use std::{borrow::Cow, cmp::max};

use crate::{
    abbrev::{ABBREVS, char_is_valid_in_abbrev_tgt},
    suffix::SUFFIXES,
};

#[must_use]
pub fn dec_with_default_rules(s: &str) -> String {
    dec(ABBREVS, s)
}

#[must_use]
fn do_dec(abbrevs: &[(Cow<'_, str>, Cow<'_, str>, u64)], s: &str) -> String {
    let mut result = Vec::with_capacity(s.len());
    let mut p = s.as_bytes();
    'expanded: while !p.is_empty() {
        // TODO: Skip URLs
        for chunk in ["Iraq", "e.g.", "i.e.", "(a)", "(b)", "(c)", "(d)", "(e)"] {
            if p.starts_with(chunk.as_bytes()) {
                p = &p[chunk.len()..];
                result.extend(chunk.as_bytes());
                continue 'expanded;
            }
        }

        let first_punct = p
            .iter()
            .copied()
            .position(|b| !char_is_valid_in_abbrev_tgt(b));
        let idx = max(1, first_punct.unwrap_or(p.len()));
        let (mut chunk, rest) = p.split_at(idx);
        debug_assert!(!chunk.is_empty());
        p = rest;

        let ends_in_comma = chunk.ends_with(b",");
        if ends_in_comma {
            chunk = &chunk[0..chunk.len() - 1];
        }
        // dbg!(String::from_utf8_lossy(chunk));
        // dbg!(String::from_utf8_lossy(p));

        for (long, short, _) in abbrevs {
            let s = short.as_bytes();
            if chunk == s {
                result.extend(long.as_bytes());
                if ends_in_comma {
                    result.push(b',');
                }
                continue 'expanded;
            }
        }
        if chunk.iter().all(u8::is_ascii_uppercase) {
            result.extend(chunk);
            if ends_in_comma {
                result.push(b',');
            }
            continue;
        }
        for (long, short) in SUFFIXES {
            if let Some(pre) = chunk.strip_suffix(short.as_bytes()) {
                result.extend(pre);
                result.extend(long.as_bytes());
                if ends_in_comma {
                    result.push(b',');
                }
                continue 'expanded;
            }
        }
        result.extend(chunk);
        if ends_in_comma {
            result.push(b',');
        }
    }

    String::from_utf8(result).unwrap()
}

#[allow(clippy::missing_panics_doc)] // unreachable
#[must_use]
pub fn dec(abbrevs: &[(&str, &str, u64)], s: &str) -> String {
    let abbrevs = crate::enc::add_caps(abbrevs);
    do_dec(abbrevs.as_slice(), s)
}

#[cfg(test)]
mod test {
    use expect_test::expect;

    use crate::{
        abbrev::{ABBREVS, SUPER_COMMON},
        enc::enc_with_default_rules,
    };

    use super::dec_with_default_rules;

    fn d(s: &str) -> String {
        dec_with_default_rules(s)
    }

    #[test]
    fn dec_abbrevs() {
        for (long, short, _savings) in ABBREVS.iter().copied() {
            assert_eq!(d(short), long);
        }
    }

    #[test]
    fn dec_aint() {
        assert_eq!(d("ai'"), "ain't");
    }

    #[test]
    fn dec_eg() {
        assert_eq!(d("e.g."), "e.g.");
    }

    #[test]
    fn dec_iraq() {
        assert_eq!(d("Iraq"), "Iraq");
    }

    #[test]
    fn dec_ty() {
        assert_eq!(d(",ty"), "thank you");
    }

    #[test]
    fn test_license() {
        let expected = expect![[r#"
            MIT License

            Copyright (c) 2025 Brian Langston Barrett

            Permission is hereby granted, free of charge, to any person obtaining a copy
            of this software and associated documentation files (the "Software"), to deal
            in the Software without restriction, including without limitation the rights
            to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
            copies of the Software, and to permit persons to whom the Software is
            furnished to do so, subject to the following conditions:

            The above copyright notice and this permission notice shall be included in all
            copies or substantial portions of the Software.

            THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
            IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
            FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
            AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
            LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
            OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
            SOFTWARE.
        "#]];
        expected.assert_eq(&d(include_str!("../../../LICENSE")));
    }

    #[test]
    fn test_readme() {
        let expected = expect![[r##"
            # Lesshand

            Lesshand is a shorthand for the 21st century. You can speed up your writing and
            typing by about 5% by learning just 10 of its abbreviations.[^speed]

            Unlike other shorthand systems, Lesshand is *lossless*. This means that you can
            type quickly in Lesshand and have a computer program *decode* your shorthand
            to standard written English. This makes Lesshand suitable for composing emails,
            documents, messages, and more.

            It's easy to learn Lesshand. In fact, you already know a lot of it! Lesshand
            uses familiar abbreviations like "you" for "you", "very" for "very", and "because" for
            "because". All of its abbreviations are designed to be mnemonic. It's also
            completely incremental; you can learn just a few abbreviations and start
            applying them right away.

            Check out the [tour] for a quick overview of Lesshand. For more information, see
            the [documentation].

            [documentation]: https://langston-barrett.github.into/lesshand
            [tour]: https://langston-barrett.github.into/lesshand/tour

            Lesshand stands for lossLESness shortHANted.

            [^speed]: See the documentation for the details of how this claim is evaluated.
        "##]];
        expected.assert_eq(&d(include_str!("../../../README.md")));
    }

    fn r(s: &str) -> String {
        dec_with_default_rules(&enc_with_default_rules(s))
    }

    #[test]
    fn roundtrip_abbrevs() {
        for (long, short, savings) in ABBREVS.iter().copied() {
            if savings < 5 && !SUPER_COMMON.contains(&short) {
                continue;
            }
            assert_eq!(r(long), long);
        }
    }

    #[test]
    fn roundtrip_abbrev_comma() {
        assert_eq!(r("copy,"), "copy,");
    }

    #[test]
    fn roundtrip_suffix_comma() {
        assert_eq!(r("information,"), "information,");
    }
}
