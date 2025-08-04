// Defunct attempt to devise a holistic "score" that takes into account:
//
// - Effort saved
// - Memorability
// - Ambiguity
// - Frequency
#[cfg(test)]
mod test {
    use std::{cmp::max, fmt::Write as _};

    use expect_test::expect;

    use crate::{
        ambiguity::ambiguity, effort::qwerty::qwerty_effort, mem::memorability, words::WORDS,
    };

    fn abbrevs() -> Vec<(&'static str, &'static str)> {
        crate::abbrev::ABBREVS
            .iter()
            .copied()
            .map(|(long, short, _)| (long, short))
            .collect()
    }

    fn e(s: &str) -> u64 {
        qwerty_effort(s).unwrap()
    }

    fn score(abbrevs: &[(&str, &str)], long: &str, short: &str) -> isize {
        let a = isize::try_from(ambiguity(long, short)).unwrap();
        let e = isize::try_from(e(long).saturating_sub(e(short))).unwrap();
        let m = isize::try_from(memorability(abbrevs, long, short)).unwrap();
        let words = WORDS.len(); // 5016
        let rank = WORDS.iter().position(|w| *w == long).unwrap_or(words);
        let bin = 31 * rank / words; // [0..15]
        let bonuses: [isize; 32] = [
            16, 8, 4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        let bonus = bonuses[bin];
        eprintln!("BONUS {long} {bonus}");
        -(a / (50 * max(bonus, 1))) + (bonus * e) + (bonus * m)
    }

    #[test]
    fn abbrevs_score() {
        let expected = expect![[r#"
            something --> smth: 352
            people --> ppl: 304
            through --> thro: 304
            between --> bw: 304
            because --> bc: 288
            around --> ard: 287
            should --> sd: 286
            things --> tgs: 272
            before --> bf: 272
            would --> wd: 256
            after --> af: 256
            thing --> tg: 255
            really --> rly: 255
            could --> cd: 254
            right --> rt: 252
            information --> info: 248
            good --> gd: 240
            other --> oth: 240
            yeah --> yh: 240
            every --> evy: 240
            about --> abt: 240
            been --> bn: 239
            need --> nd: 237
            little --> lil: 224
            know --> kw: 224
            government --> govt: 224
            never --> nv: 224
            much --> mh: 224
            very --> v: 223
            into --> io: 223
            down --> dn: 222
            everything --> evthg: 208
            over --> ov: 208
            just --> j: 208
            how --> hw: 208
            only --> oy: 207
            his --> hs: 207
            her --> hr: 207
            but --> bt: 207
            that --> tt: 206
            president --> pres: 198
            have --> hv: 192
            also --> al: 188
            from --> m: 187
            question --> q: 185
            were --> ee: 185
            together --> tog: 184
            your --> ur: 175
            will --> ll: 175
            for --> f: 172
            million --> mil: 159
            you --> u: 158
            yes --> y: 156
            can --> c: 153
            actually --> acty: 152
            without --> wo: 150
            however --> hwv: 144
            was --> w: 141
            be --> b: 141
            this --> h: 140
            no --> n: 136
            though --> tho: 135
            and --> d: 135
            to --> t: 133
            they --> e: 129
            others --> oths: 128
            keep --> kp: 120
            having --> hvq: 112
            is --> s: 112
            of --> o: 107
            are --> r: 102
            not --> x: 96
            the --> l: 89
            several --> sev: 75
            service --> svc: 72
            international --> intl: 69
            okay --> k: 69
            major --> mjr: 68
            please --> pls: 66
            thanks --> tks: 64
            department --> dept: 53
            especially --> esp: 53
            move --> mv: 52
            services --> svcs: 44
            interest --> ist: 40
            market --> mkt: 34
            rights --> rts: 24
            baby --> bb: 22
            littlest --> lilst: 0
            internationally --> intlly: 0
            maximizing --> maxzq: 0
            maximize --> maxz: 0
            minimizing --> minzq: 0
            minimize --> minz: 0
            knowingly --> kwqly: 0
            keeping --> kpq: 0
            trillions --> trils: 0
            somethings --> smths: 0
            requirements --> reqms: 0
            requirement --> reqm: 0
            questions --> qs: 0
            organizations --> orgs: 0
            minimum --> minm: 0
            meetings --> mtgs: 0
            maximum --> maxm: 0
            markets --> mkts: 0
            marketing --> mktq: 0
            management --> mgmt: 0
            hundreds --> huns: 0
            governments --> govts: 0
            dozens --> dozs: 0
            dozen --> doz: 0
            departments --> depts: 0
            context --> ctx: 0
            complex --> cx: 0
            billions --> bils: 0
            babies --> bbs: 0
            appointments --> appts: 0
            apartments --> aptms: 0
            apartment --> aptm: 0
            address --> addr: 0
            tomorrow --> tmrw: 0
            otherwise --> othw: 0
            further --> furth: 0
            forward --> fwd: 0
            knowing --> kwq: -1
            thousands --> thous: -1
            requests --> rqsts: -1
            othering --> othq: -1
            moves --> mvs: -1
            moving --> mvq: -1
            millions --> mils: -1
            meeting --> mtg: -1
            emails --> emls: -1
            appointment --> appt: -2
            minor --> mnr: -3
            request --> rqst: -3
            trillion --> tril: -4
            thousand --> thou: -4
            interests --> ists: -4
            hundred --> hun: -4
            email --> eml: -4
            billion --> bil: -6
            organization --> org: -7
            interesting --> istq: -8
            righting --> rtq: -23
            tonight --> tn: -65
            questioning --> qq: -113
        "#]];
        let abbrevs = abbrevs();
        let mut results = Vec::with_capacity(abbrevs.len());
        for (long, short) in abbrevs.iter().copied().take(500) {
            if short.contains(',')
                || long.contains('\'')
                || short.chars().any(|c| c.is_ascii_digit())
                || long.chars().next().unwrap().is_ascii_uppercase()
            {
                continue;
            }
            let s = score(&abbrevs, long, short);
            results.push((long, short, s));
        }
        results.sort_by_key(|(_, _, s)| *s);
        results.reverse();
        let mut s = String::with_capacity(2 * abbrevs.len());
        for (long, short, a) in results {
            writeln!(s, "{long} --> {short}: {a}").unwrap();
        }
        expected.assert_eq(&s);
    }
}
