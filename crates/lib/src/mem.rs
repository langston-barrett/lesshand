// ref: memorability

#[must_use]
fn shared_chars(long: &str, short: &str, mut score: usize) -> usize {
    let mut cs = short.chars();
    let mut c0 = cs.next();
    for c in long.chars() {
        match c0 {
            None => return score,
            Some(v) => {
                if c == v || (v == 'q' && ['i', 'n', 'g'].contains(&c)) {
                    score += 1;
                    c0 = cs.next();
                }
            }
        }
    }
    if c0.is_some() {
        return 0;
    }
    score
}

#[must_use]
pub fn memorability(abbrevs: &[(&str, &str)], long: &str, mut short: &str) -> usize {
    let mut score = 0;

    if long.is_empty() || short.is_empty() {
        return 0;
    }

    if short.starts_with(',') {
        short = &short[1..];
    }

    let long = long.to_ascii_lowercase();

    if short.ends_with('\'') {
        for (long0, short0) in abbrevs.iter().copied() {
            if short0 == &short[0..short.len() - 1] && long.starts_with(long0) {
                score += 10;
            }
        }
    }

    if long.chars().next() == short.chars().next() {
        score += 3;
    }
    if long.chars().last() == short.chars().last() {
        score += 3;
    }
    if long.starts_with(short) {
        score += short.len();
    }

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let vowelless = long
        .chars()
        .filter(|c| !vowels.contains(c))
        .collect::<String>();
    if short == vowelless {
        score += short.len();
    }

    score = shared_chars(&long, short, score);

    if short.contains(',') {
        score = 10; // namespace
    }

    if long.contains('\'') && !short.contains('\'') {
        score = score.saturating_sub(1);
    }

    score
}

#[cfg(test)]
mod test {
    use std::fmt::Write as _;

    use expect_test::expect;

    fn abbrevs() -> Vec<(&'static str, &'static str)> {
        crate::abbrev::ABBREVS
            .iter()
            .copied()
            .map(|(long, short, _)| (long, short))
            .collect()
    }

    fn m(long: &str, short: &str) -> usize {
        super::memorability(&abbrevs(), long, short)
    }

    fn shared(long: &str, short: &str) -> usize {
        super::shared_chars(long, short, 0)
    }

    #[test]
    fn shared_chars() {
        assert_eq!(shared("and", "d"), 1);
        assert_eq!(shared("apartment", "apt"), 3);
        assert_eq!(shared("apartment", "appt"), 0);
    }

    #[test]
    fn empty() {
        assert_eq!(m("", ""), 0);
        assert_eq!(m("whatever", ""), 0);
    }

    #[test]
    fn contraction() {
        assert_eq!(m("aren't", "r'"), 12);
    }

    #[test]
    fn first_last() {
        assert_eq!(m("very", "vy"), 8);
    }

    #[test]
    fn also_actually() {
        assert_eq!(m("also", "al"), 7);
        assert_eq!(m("actually", "al"), 5);
    }

    #[test]
    fn abbrevs_memorability() {
        let expected = expect![[r#"
            about --> abt: 9
            actually --> acty: 10
            after --> af: 7
            also --> al: 7
            and --> d: 4
            aren't --> r': 12
            are --> r: 2
            around --> ard: 9
            be --> b: 6
            because --> bc: 5
            been --> bn: 10
            before --> bf: 5
            between --> bw: 5
            but --> bt: 10
            can --> c: 5
            can't --> c': 15
            could --> cd: 8
            couldn't --> cd': 16
            down --> dn: 8
            especially --> esp: 9
            every --> evy: 9
            for --> f: 5
            forward --> fwd: 9
            from --> m: 4
            further --> furth: 13
            her --> hr: 10
            his --> hs: 10
            how --> hw: 10
            however --> hwv: 6
            I'd --> id: 7
            I --> i: 8
            I'll --> il: 7
            I'm --> im: 7
            into --> io: 8
            isn't --> s': 12
            is --> s: 5
            I've --> iv: 4
            just --> j: 5
            much --> mh: 8
            need --> nd: 10
            needn't --> nd': 16
            never --> nv: 5
            not --> x: 0
            okay --> k: 1
            only --> oy: 8
            otherwise --> othw: 7
            over --> ov: 7
            please --> plz: 0
            really --> rly: 9
            several --> sev: 9
            shouldn't --> sd': 16
            should --> sd: 8
            thanks --> thx: 0
            that --> tt: 8
            the --> l: 0
            they --> e: 1
            this --> h: 1
            though --> tho: 9
            through --> thro: 11
            together --> tog: 9
            tomorrow --> tmrw: 10
            to --> t: 6
            very --> v: 5
            wasn't --> w': 15
            was --> w: 5
            were --> ee: 5
            weren't --> ee': 13
            will --> ll: 5
            willn't --> ll': 13
            without --> wo: 5
            won't --> wo': 6
            wouldn't --> wd': 16
            would --> wd: 8
            yeah --> yh: 10
            yes --> y: 5
            you're --> u': 12
            your --> ur: 5
            you --> u: 4
            address --> addr: 11
            apartment --> aptm: 7
            apartment's --> aptm': 18
            apartments --> aptms: 11
            apartments' --> aptms': 22
            appointment --> appt: 10
            appointment's --> appt': 18
            appointments --> appts: 11
            appointments' --> appts': 22
            babies --> bbs: 12
            babies' --> bbs': 24
            baby --> bb: 5
            baby's --> bb': 16
            billion --> bil: 9
            billion's --> bil': 17
            billions --> bils: 10
            billions' --> bils': 21
            complex --> cx: 8
            context --> ctx: 6
            context's --> ctx': 17
            contexts' --> ctxs': 11
            department --> dept: 10
            department's --> dept': 18
            departments --> depts: 11
            departments' --> depts': 22
            dozen --> doz: 9
            dozen's --> doz': 17
            dozens --> dozs: 10
            dozens' --> dozs': 21
            email --> eml: 9
            email's --> eml': 17
            emails --> emls: 10
            emails' --> emls': 21
            everything --> evthg: 11
            everything's --> evthg': 19
            government --> govt: 10
            government's --> govt': 18
            governments --> govts: 11
            governments' --> govts': 22
            hundred --> hun: 9
            hundred's --> hun': 17
            hundreds --> huns: 10
            hundreds' --> huns': 21
            information --> info: 11
            interest --> ist: 9
            interest's --> ist': 17
            interesting --> istq: 7
            interests --> ists: 10
            interests' --> ists': 21
            management --> mgmt: 10
            management's --> mgmt': 18
            market --> mkt: 9
            marketing --> mktq: 7
            market's --> mkt': 17
            markets --> mkts: 10
            markets' --> mkts': 21
            maximum --> maxm: 10
            maximum's --> maxm': 18
            meeting --> mtg: 9
            meeting's --> mtg': 17
            meetings --> mtgs: 10
            meetings' --> mtgs': 21
            million --> mil: 9
            million's --> mil': 17
            millions --> mils: 10
            millions' --> mils': 21
            minimum --> minm: 10
            minimum's --> minm': 18
            move --> mv: 7
            moving --> mvq: 6
            move's --> mv': 16
            moves --> mvs: 12
            moves' --> mvs': 24
            organization --> org: 9
            organization's --> org': 17
            organizations --> orgs: 10
            organizations' --> orgs': 21
            other --> oth: 9
            othering --> othq: 7
            other's --> oth': 17
            others --> oths: 10
            others' --> oths': 21
            people --> ppl: 9
            people's --> ppl': 17
            president --> pres: 11
            president's --> pres': 18
            presidents' --> press': 12
            question --> q: 5
            questioning --> qq: 5
            question's --> q': 15
            questions --> qs: 8
            questions' --> qs': 19
            request --> rqst: 14
            request's --> rqst': 18
            requests --> rqsts: 16
            requests' --> rqsts': 28
            requirement --> reqm: 7
            requirement's --> reqm': 18
            requirements --> reqms: 11
            requirements' --> reqms': 22
            right --> rt: 8
            righting --> rtq: 6
            right's --> rt': 16
            rights --> rts: 9
            rights' --> rts': 20
            service's --> svc': 17
            services --> svcs: 10
            services' --> svcs': 21
            service --> svc: 6
            something --> smth: 7
            something's --> smth': 18
            somethings --> smths: 11
            somethings' --> smths': 22
            thing's --> tg': 16
            things --> tgs: 9
            things' --> tgs': 20
            thing --> tg: 8
            thousands's --> thous': 19
            thousandss' --> thouss': 13
            thousands --> thous: 16
            thousand --> thou: 11
            tonight's --> tn': 16
            tonight --> tn: 5
            trillion's --> tril': 18
            trillions --> trils: 11
            trillions' --> trils': 22
            trillion --> tril: 11
            have --> hv: 7
            having --> hvq: 6
            keeping --> kpq: 6
            keep --> kp: 10
            knowing --> kwq: 6
            knowingly --> kwqly: 11
            know --> kw: 8
            minimize --> minz: 7
            minimizing --> minzq: 8
            maximize --> maxz: 7
            maximizing --> maxzq: 8
            good --> gd: 10
            international --> intl: 10
            internationally --> intlly: 12
            little --> lil: 6
            littlest --> lilst: 11
            major --> mjr: 12
            minor --> mnr: 12
            as far as I know --> ,afaik: 8
            be right back --> ,brb: 6
            for real --> ,fr: 5
            for your information --> ,fyi: 6
            I don't care --> ,idc: 5
            I don't know --> ,idk: 5
            if I remember correctly --> ,iirc: 7
            if I understand correctly --> ,iiuc: 7
            I love you --> ,ily: 6
            I love you so much --> ,ilysm: 8
            in case you missed it --> ,icymi: 8
            in my honest opinion --> ,imho: 7
            in my opinion --> ,imo: 6
            just kidding --> ,jk: 5
            looks good to me --> ,lgtm: 7
            no problem --> ,np: 5
            of course --> ,ofc: 6
            on my way --> ,omw: 6
            right now --> ,rn: 5
            talk to you later --> ,ttyl: 7
            talk to you soon --> ,ttys: 7
            thank you so much --> ,tysm: 7
            thank you --> ,ty: 5
            to be honest --> ,tbh: 6
            with respect to --> ,wrt: 6
            you're welcome --> ,yw: 4
            ain't --> ai': 6
            didn't --> di': 6
            don't --> do': 6
            hadn't --> had': 7
            hasn't --> has': 7
            mustn't --> must': 8
            oughtn't --> ought': 9
            Sunday --> sund: 11
            Sundays --> sunds: 11
            Sunday's --> sund': 8
            Sundays' --> sunds': 12
            Monday --> mond: 11
            Mondays --> monds: 11
            Monday's --> mond': 8
            Mondays' --> monds': 12
            Tuesday --> tues: 11
            Tuesdays --> tuess: 11
            Tuesday's --> tues': 8
            Tuesdays' --> tuess': 12
            Wednesday --> weds: 7
            Wednesdays --> wedss: 11
            Wednesday's --> weds': 8
            Wednesdays' --> wedss': 12
            Thursday --> thur: 11
            Thursdays --> thurs: 16
            Thursday's --> thur': 8
            Thursdays' --> thurs': 12
            Friday --> frid: 11
            Fridays --> frids: 11
            Friday's --> frid': 8
            Fridays' --> frids': 12
            Saturday --> satd: 7
            Saturdays --> satds: 11
            Saturday's --> satd': 8
            Saturdays' --> satds': 12
            January --> jan: 9
            Januarys --> jans: 10
            January's --> jan': 7
            Januarys' --> jans': 11
            February --> feb: 9
            Februarys --> febs: 10
            February's --> feb': 7
            Februarys' --> febs': 11
            March --> mar: 9
            Marchs --> mars: 10
            March's --> mar': 7
            Marchs' --> mars': 11
            April --> apr: 9
            Aprils --> aprs: 10
            April's --> apr': 7
            Aprils' --> aprs': 11
            June --> jun: 9
            Junes --> juns: 10
            June's --> jun': 7
            Junes' --> juns': 11
            July --> jul: 9
            Julys --> juls: 10
            July's --> jul': 7
            Julys' --> juls': 11
            August --> aug: 9
            Augusts --> augs: 10
            August's --> aug': 7
            Augusts' --> augs': 11
            September --> sep: 9
            Septembers --> seps: 10
            September's --> sep': 7
            Septembers' --> seps': 11
            October --> oct: 9
            Octobers --> octs: 10
            October's --> oct': 7
            Octobers' --> octs': 11
            November --> nov: 9
            Novembers --> novs: 10
            November's --> nov': 7
            Novembers' --> novs': 11
            December --> dec: 9
            Decembers --> decs: 10
            December's --> dec': 7
            Decembers' --> decs': 11
            Brooklyn --> ,bk: 5
            Chicago --> ,chi: 9
            London --> ,lon: 12
            Los Angeles --> ,lax: 0
            Mexico City --> ,cdmx: 0
            New York City --> ,nyc: 6
            Portland --> ,pdx: 0
            Seattle --> ,sea: 9
            St Louis --> ,stl: 6
            Alabama --> ,al: 7
            Alaska --> ,ak: 5
            Arizona --> ,az: 5
            Arkansas --> ,ar: 7
            California --> ,ca: 10
            Colorado --> ,co: 10
            Connecticut --> ,ct: 8
            Delaware --> ,de: 10
            Florida --> ,fl: 7
            Georgia --> ,ga: 8
            Hawaii --> ,hi: 8
            Idaho --> ,id: 7
            Illinois --> ,il: 7
            Indiana --> ,in: 7
            Iowa --> ,ia: 8
            Kansas --> ,ks: 8
            Kentucky --> ,ky: 8
            Louisiana --> ,la: 8
            Maine --> ,me: 8
            Maryland --> ,md: 8
            Massachusetts --> ,ma: 7
            Michigan --> ,mi: 7
            Minnesota --> ,mn: 5
            Mississippi --> ,ms: 5
            Missouri --> ,mo: 5
            Montana --> ,mt: 5
            Nebraska --> ,ne: 7
            Nevada --> ,nv: 5
            New Hampshire --> ,nh: 5
            New Jersey --> ,nj: 5
            New Mexico --> ,nm: 5
            New York --> ,ny: 5
            North Carolina --> ,nc: 5
            North Dakota --> ,nd: 5
            Ohio --> ,oh: 7
            Oklahoma --> ,ok: 7
            Oregon --> ,or: 7
            Pennsylvania --> ,pa: 8
            Rhode Island --> ,ri: 5
            South Carolina --> ,sc: 5
            South Dakota --> ,sd: 5
            Tennessee --> ,tn: 5
            Texas --> ,tx: 5
            Utah --> ,ut: 7
            Vermont --> ,vt: 8
            Virginia --> ,va: 8
            Washington --> ,wa: 7
            West Virginia --> ,wv: 5
            Wisconsin --> ,wi: 7
            Wyoming --> ,wy: 7
        "#]];
        let abbrevs = abbrevs();
        let mut s = String::with_capacity(2 * abbrevs.len());
        for (long, short) in &abbrevs {
            if !short.starts_with(',') && short.contains(',') {
                continue;
            }
            if (short.starts_with('n') || short.starts_with('o'))
                && short.chars().skip(1).all(|c| c.is_ascii_digit())
            {
                continue;
            }
            let m = super::memorability(&abbrevs, long, short);
            writeln!(s, "{long} --> {short}: {m}").unwrap();
        }
        expected.assert_eq(&s);
    }
}
