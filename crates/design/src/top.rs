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
            53 (1.91%): for --> f (cumulative: 53 (1.91%))
            30 (1.08%): the --> l (cumulative: 83 (2.99%))
            29 (1.04%): you --> u (cumulative: 112 (4.03%))
            25 (0.90%): and --> d (cumulative: 137 (4.94%))
            22 (0.79%): this --> h (cumulative: 159 (5.73%))
            20 (0.72%): information --> info (cumulative: 179 (6.45%))
            16 (0.58%): just --> j (cumulative: 195 (7.02%))
            15 (0.54%): can --> c (cumulative: 210 (7.56%))
            13 (0.47%): because --> bc (cumulative: 223 (8.03%))
            12 (0.43%): your --> ur (cumulative: 235 (8.47%))
            10 (0.36%): of --> o (cumulative: 245 (8.83%))
            9 (0.32%): very --> v (cumulative: 254 (9.15%))
            9 (0.32%): to --> t (cumulative: 263 (9.47%))
            9 (0.32%): is --> s (cumulative: 272 (9.80%))
            8 (0.29%): right --> rt (cumulative: 280 (10.09%))
            6 (0.22%): know --> kw (cumulative: 286 (10.30%))
            6 (0.22%): other --> oth (cumulative: 292 (10.52%))
            6 (0.22%): about --> abt (cumulative: 298 (10.73%))
            5 (0.18%): have --> hv (cumulative: 303 (10.91%))
            5 (0.18%): emails --> emls (cumulative: 308 (11.10%))
            5 (0.18%): that --> tt (cumulative: 313 (11.28%))
            5 (0.18%): are --> r (cumulative: 318 (11.46%))
            5 (0.18%): also --> al (cumulative: 323 (11.64%))
            3 (0.11%): how --> hw (cumulative: 326 (11.74%))
            3 (0.11%): be --> b (cumulative: 329 (11.85%))
        "#]];
        expected.assert_eq(&t(include_str!("../../../README.md")));
    }
}
