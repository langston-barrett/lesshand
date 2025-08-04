use std::io;

use lesshand::{abbrev::ABBREVS, effort::qwerty::qwerty_effort_fill, enc::enc};

#[allow(clippy::cast_precision_loss)]
pub(crate) fn top(mut w: impl io::Write, s0: &str, limit: usize) -> io::Result<()> {
    let e0 = qwerty_effort_fill(s0, 0);
    let mut results = Vec::with_capacity(ABBREVS.len());
    for rule @ (long, _short, _) in ABBREVS.iter().copied() {
        let s = enc(&[rule], /* suffixes */ false, s0);
        let e = qwerty_effort_fill(&s, 0);
        let savings = e0.saturating_sub(e);
        debug_assert!(
            s0.contains(long) || savings == 0,
            "Savings of {e0} - {e} = {savings} for {long}"
        );
        if savings != 0 {
            results.push((savings, rule));
        }
    }
    results.sort_by_key(|(savings, _)| *savings);
    results.reverse();
    let mut cumulative = 0;
    for (savings, (long, short, _)) in results.into_iter().take(limit) {
        cumulative += savings;
        // TODO: lpad fails expect tests...?
        // let savings = lpad(&savings.to_string(), 0);
        let percent = 100.0 - ((e0.saturating_sub(savings) as f64 / e0 as f64) * 100.0);
        let cum_percent = 100.0 - ((e0.saturating_sub(cumulative) as f64 / e0 as f64) * 100.0);
        writeln!(
            w,
            "{savings} ({percent:.2}%): {long} --> {short} (cumulative: {cumulative} ({cum_percent:.2}%))"
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use expect_test::expect;

    fn t(s: &str) -> String {
        let mut result = Vec::with_capacity(256); // guess
        super::top(&mut result, s, 32).unwrap();
        String::from_utf8(result).unwrap()
    }

    #[test]
    fn test_goals() {
        let expected = expect![[r#"
            90 (1.40%): should --> sd (cumulative: 90 (1.40%))
            83 (1.29%): requirements --> reqms (cumulative: 173 (2.70%))
            57 (0.89%): you --> u (cumulative: 230 (3.58%))
            41 (0.64%): the --> l (cumulative: 271 (4.22%))
            35 (0.55%): and --> d (cumulative: 306 (4.77%))
            30 (0.47%): can --> c (cumulative: 336 (5.23%))
            29 (0.45%): that --> tt (cumulative: 365 (5.69%))
            29 (0.45%): for --> f (cumulative: 394 (6.14%))
            22 (0.34%): this --> h (cumulative: 416 (6.48%))
            18 (0.28%): your --> ur (cumulative: 434 (6.76%))
            18 (0.28%): of --> o (cumulative: 452 (7.04%))
            15 (0.23%): to --> t (cumulative: 467 (7.28%))
            15 (0.23%): be --> b (cumulative: 482 (7.51%))
            9 (0.14%): never --> nv (cumulative: 491 (7.65%))
            9 (0.14%): is --> s (cumulative: 500 (7.79%))
            8 (0.12%): right --> rt (cumulative: 508 (7.91%))
            8 (0.12%): would --> wd (cumulative: 516 (8.04%))
            8 (0.12%): can't --> c' (cumulative: 524 (8.16%))
            6 (0.09%): others --> oths (cumulative: 530 (8.26%))
            6 (0.09%): other --> oth (cumulative: 536 (8.35%))
            6 (0.09%): won't --> wo' (cumulative: 542 (8.44%))
            5 (0.08%): have --> hv (cumulative: 547 (8.52%))
            5 (0.08%): are --> r (cumulative: 552 (8.60%))
            5 (0.08%): also --> al (cumulative: 557 (8.68%))
            3 (0.05%): no --> n (cumulative: 560 (8.72%))
            3 (0.05%): how --> hw (cumulative: 563 (8.77%))
        "#]];
        expected.assert_eq(&t(include_str!("../../../doc/goals.md")));
    }

    #[test]
    fn test_license() {
        let expected = expect![[r#"
            47 (1.38%): the --> l (cumulative: 47 (1.38%))
            28 (0.82%): without --> wo (cumulative: 75 (2.20%))
            21 (0.62%): to --> t (cumulative: 96 (2.81%))
            20 (0.59%): and --> d (cumulative: 116 (3.40%))
            16 (0.47%): this --> h (cumulative: 132 (3.87%))
            8 (0.23%): rights --> rts (cumulative: 140 (4.10%))
            8 (0.23%): of --> o (cumulative: 148 (4.34%))
            6 (0.18%): is --> s (cumulative: 154 (4.51%))
            3 (0.09%): be --> b (cumulative: 157 (4.60%))
        "#]];
        expected.assert_eq(&t(include_str!("../../../LICENSE")));
    }

    #[test]
    fn test_readme() {
        let expected = expect![[r#"
            53 (1.88%): for --> f (cumulative: 53 (1.88%))
            29 (1.03%): you --> u (cumulative: 82 (2.90%))
            25 (0.88%): and --> d (cumulative: 107 (3.79%))
            24 (0.85%): the --> l (cumulative: 131 (4.64%))
            22 (0.78%): this --> h (cumulative: 153 (5.42%))
            20 (0.71%): information --> info (cumulative: 173 (6.12%))
            16 (0.57%): just --> j (cumulative: 189 (6.69%))
            15 (0.53%): can --> c (cumulative: 204 (7.22%))
            13 (0.46%): because --> bc (cumulative: 217 (7.68%))
            12 (0.42%): your --> ur (cumulative: 229 (8.11%))
            10 (0.35%): of --> o (cumulative: 239 (8.46%))
            9 (0.32%): very --> v (cumulative: 248 (8.78%))
            9 (0.32%): to --> t (cumulative: 257 (9.10%))
            9 (0.32%): is --> s (cumulative: 266 (9.42%))
            8 (0.28%): right --> rt (cumulative: 274 (9.70%))
            6 (0.21%): know --> kw (cumulative: 280 (9.91%))
            6 (0.21%): other --> oth (cumulative: 286 (10.12%))
            6 (0.21%): about --> abt (cumulative: 292 (10.34%))
            5 (0.18%): have --> hv (cumulative: 297 (10.51%))
            5 (0.18%): emails --> emls (cumulative: 302 (10.69%))
            5 (0.18%): that --> tt (cumulative: 307 (10.87%))
            5 (0.18%): are --> r (cumulative: 312 (11.04%))
            5 (0.18%): also --> al (cumulative: 317 (11.22%))
            3 (0.11%): how --> hw (cumulative: 320 (11.33%))
            3 (0.11%): be --> b (cumulative: 323 (11.43%))
        "#]];
        expected.assert_eq(&t(include_str!("../../../README.md")));
    }
}
