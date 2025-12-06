use std::{borrow::Cow, cmp::max};

use crate::{
    abbrev::{ABBREVS, SUPER_COMMON, char_is_valid_in_abbrev_src},
    suffix::SUFFIXES,
};

fn enc_abbrevs() -> Vec<(&'static str, &'static str, u64)> {
    let mut abbrevs = Vec::from(ABBREVS);
    abbrevs.sort_unstable_by_key(|(_, _, savings)| *savings);
    abbrevs.reverse();
    // Not worth bothering with ineffective abbreviations
    // TODO: Needs more evaluation of the right threshold
    abbrevs.retain(|(_long, short, savings)| *savings >= 5 || SUPER_COMMON.contains(short));
    abbrevs
}

fn enc_chunk(
    abbrevs: &[(Cow<'_, str>, Cow<'_, str>, u64)],
    suffixes: bool,
    result: &mut Vec<u8>,
    mut chunk: &[u8],
) {
    debug_assert!(chunk.len() == 1 || chunk.iter().all(|b| char_is_valid_in_abbrev_src(*b)));
    'outer: while !chunk.is_empty() {
        // dbg!(String::from_utf8_lossy(chunk));
        for (long, short, _) in abbrevs {
            let l = long.as_bytes();
            let l_len = l.len();
            if chunk.len() < l_len {
                continue;
            }
            if chunk.len() > l_len {
                let next = chunk.get(l_len).unwrap();
                if next.is_ascii_alphabetic() {
                    continue;
                }
            }
            if chunk.starts_with(l) {
                result.extend(short.as_bytes());
                chunk = &chunk[l_len..];
                continue 'outer;
            }
        }

        let first_space = chunk.iter().copied().position(|b| b == b' ');
        let idx = first_space.unwrap_or(chunk.len());
        let (word, rest) = chunk.split_at(idx);
        if word.is_empty() {
            result.push(b' ');
            chunk = &chunk[1..];
            continue 'outer;
        }
        chunk = rest;
        if !suffixes || word.iter().all(u8::is_ascii_uppercase) {
            result.extend(word);
            continue 'outer;
        }
        for (long, short) in SUFFIXES {
            if let Some(pre) = word.strip_suffix(long.as_bytes()) {
                result.extend(pre);
                result.extend(short.as_bytes());
                continue 'outer;
            }
        }
        result.extend(word);
    }
}

#[must_use]
pub fn enc_with_default_rules(s: &str) -> String {
    enc(enc_abbrevs().as_slice(), true, s)
}

fn split_chunk(p: &[u8]) -> (&[u8], &[u8]) {
    let first_punct = p
        .iter()
        .copied()
        .position(|b| !char_is_valid_in_abbrev_src(b));
    let idx = max(1, first_punct.unwrap_or(p.len()));
    p.split_at(idx)
}

pub(crate) fn add_caps<'a>(
    abbrevs: &[(&'a str, &'a str, u64)],
) -> Vec<(Cow<'a, str>, Cow<'a, str>, u64)> {
    fn cap(s: &str) -> String {
        let (f, r) = s.split_at(1);
        format!("{}{r}", f.to_ascii_uppercase())
    }
    let caps = abbrevs.iter().map(|(long, short, e)| {
        (
            Cow::Owned::<str>(cap(long)),
            Cow::Owned::<str>(format!(";{short}")),
            *e,
        )
    });
    let mut abbrevs = abbrevs
        .iter()
        .copied()
        .map(|(long, short, e)| (Cow::Borrowed(long), Cow::Borrowed(short), e))
        .collect::<Vec<(Cow<'_, str>, Cow<'_, str>, u64)>>();
    abbrevs.extend(caps);
    abbrevs
}

#[allow(clippy::missing_panics_doc)] // unreachable
#[must_use]
pub fn enc(abbrevs: &[(&str, &str, u64)], suffixes: bool, s: &str) -> String {
    let abbrevs = add_caps(abbrevs);
    let mut result = Vec::with_capacity(s.len());
    let mut p = s.as_bytes();
    while !p.is_empty() {
        if p.starts_with("’".as_bytes()) {
            // TODO: skip this chunk, its a contraction using "’"
        }
        let (chunk, rest) = split_chunk(p);
        debug_assert!(!chunk.is_empty());
        p = rest;
        enc_chunk(abbrevs.as_slice(), suffixes, &mut result, chunk);
    }
    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod test {
    use expect_test::expect;

    use crate::abbrev::{ABBREVS, SUPER_COMMON};

    use super::enc_with_default_rules;

    fn e(s: &str) -> String {
        enc_with_default_rules(s)
    }

    #[test]
    fn enc_abbrevs() {
        for (long, short, savings) in ABBREVS.iter().copied() {
            if savings < 5 && !SUPER_COMMON.contains(&short) {
                continue;
            }
            assert_eq!(e(long), short);
        }
    }

    #[test]
    fn enc_ty() {
        assert_eq!(e("thank you"), ",ty");
    }

    #[test]
    fn enc_license() {
        let expected = expect![[r#"
            MIT License

            Copyright (c) 2025 Brian Langston Barrett

            Permission s hereby granD, free o charge, to any person obtainq a copy
            o h software d associaD documentA files (l "Software"), to deal
            in l Software wo restricT, includq wo limitA l rts
            to use, copy, modify, merge, publish, distribute, sublicense, d/or sell
            copies o l Software, d to permit persons to whom l Software s
            furnished to do so, subject to l followq conditions:

            ;l above copyright notice d h permission notice shall be included in all
            copies or substantial portions o l Software.

            THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
            IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
            FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
            AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
            LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
            OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
            SOFTWARE.
        "#]];
        expected.assert_eq(&e(include_str!("../../../LICENSE")));
    }

    #[test]
    fn enc_readme() {
        let expected = expect![[r##"
            # Lesshand

            Lesshand s a shorthand f l 21st century. ;u c speed up ur writq d
            typq by abt 5% by learnq j 10 o its abbreviations.[^speed]

            Unlike oth shorthand systems, Lesshand s *lossless*. ;h means tt u c
            type quickly in Lesshand d hv a computer program *decode* ur shorthand
            to standard written English. ;h makes Lesshand suitB f composq emls,
            documents, messages, d more.

            It's easy to learn Lesshand. In fact, u already kw a lot o it! Lesshand
            uses familiar abbreviations like "u" f "u", "v" f "v", d "bc" f
            "bc". All o its abbreviations r designed to be mnemonic. It's al
            completely incremental; u c learn j a few abbreviations d start
            applyq them rt away.

            Check out l [tour] f a quick overview o Lesshand. ;f more info, see
            l [documentA].

            [documentA]: https://langston-barrett.github.io/lesshand
            [tour]: https://langston-barrett.github.io/lesshand/tour

            Lesshand stands f lossLESS shortHAND.

            [^speed]: See l documentA f l details o how h claim s evaluaD.
        "##]];
        expected.assert_eq(&e(include_str!("../../../README.md")));
    }
}
