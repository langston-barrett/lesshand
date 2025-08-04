use crate::{mem::memorability, words::WORDS};

// ref: ambiguity
#[must_use]
pub fn ambiguity(long: &str, short: &str) -> usize {
    let mut a = 0;
    for (rank, w) in WORDS.iter().copied().enumerate() {
        if w == long {
            continue;
        }
        let weight = (WORDS.len() - rank) / 1000;
        a += weight * memorability(&[], w, short);
    }
    a
}

#[must_use]
pub fn generic_ambiguity(short: &str) -> usize {
    ambiguity("", short)
}

#[cfg(test)]
mod test {
    use std::fmt::Write as _;

    use expect_test::expect;

    use crate::abbrev::ABBREVS;

    // These are things we would *like* to prevent
    #[test]
    fn known_ambiguity() {
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
        let expected = expect![[r#"
            ar: 3797
            cm: 1077
            ev: 918
            lg: 1129
            lk: 476
            mt: 1891
            nw: 341
            se: 6562
            sm: 914
            te: 5233
            th: 2296
            tr: 3287
            wn: 1131
            wh: 792
            wr: 1236
            wt: 840
            yr: 313
        "#]];
        let mut s = String::with_capacity(2 * ABBREVS.len());
        for short in KNOWN_AMBIGUOUS {
            let a = super::generic_ambiguity(short);
            writeln!(s, "{short}: {a}").unwrap();
        }
        expected.assert_eq(&s);
    }

    #[test]
    fn abbrevs_ambiguity() {
        let expected = expect![[r#"
            were --> ee: 5753
            questions --> qs: 5694
            questioning --> qq: 5671
            also --> al: 3411
            right --> rt: 3368
            tonight --> tn: 3264
            I'd --> id: 2703
            need --> nd: 2400
            I'll --> il: 2391
            should --> sd: 2231
            that --> tt: 1912
            could --> cd: 1709
            down --> dn: 1648
            into --> io: 1504
            his --> hs: 1493
            her --> hr: 1457
            will --> ll: 1436
            your --> ur: 1368
            thing --> tg: 1265
            only --> oy: 1261
            righting --> rtq: 1167
            rights --> rts: 1089
            president --> pres: 1011
            but --> bt: 948
            been --> bn: 948
            I'm --> im: 920
            without --> wo: 912
            around --> ard: 870
            interest --> ist: 834
            really --> rly: 813
            March --> mar: 750
            would --> wd: 721
            December --> dec: 612
            I've --> iv: 602
            good --> gd: 598
            million --> mil: 558
            please --> pls: 528
            October --> oct: 504
        "#]];
        let mut results = Vec::with_capacity(ABBREVS.len());
        for (long, short, _score) in ABBREVS.iter().copied() {
            if short.len() == 1 {
                continue;
            }
            if short.contains(',') {
                continue;
            }
            if (short.starts_with('n') || short.starts_with('o'))
                && short.chars().skip(1).all(|c| c.is_ascii_digit())
            {
                continue;
            }
            let a = super::ambiguity(long, short);
            results.push((long, short, a));
        }
        results.sort_by_key(|(_, _, a)| *a);
        results.reverse();
        let mut s = String::with_capacity(2 * ABBREVS.len());
        for (long, short, a) in results {
            if a > 500 {
                writeln!(s, "{long} --> {short}: {a}").unwrap();
            }
        }
        expected.assert_eq(&s);
    }
}
