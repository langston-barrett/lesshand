#![allow(long_running_const_eval)]
use std::{collections::HashMap, sync::LazyLock};

struct QwertyEffort {
    c: char,
    distance: u8,
    modified: bool,
}

const KEY_EFFORTS: [QwertyEffort; 95] = [
    QwertyEffort {
        c: ' ',
        distance: 0,
        modified: false,
    },
    QwertyEffort {
        c: '!',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '"',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '#',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '$',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '%',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '&',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '(',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: ')',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '*',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '+',
        distance: 3,
        modified: true,
    },
    QwertyEffort {
        c: ',',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: '-',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '.',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: '/',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '0',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '1',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '2',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '3',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '4',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '5',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '6',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '7',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '8',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '9',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: ':',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: ';',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: '<',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: '=',
        distance: 3,
        modified: false,
    },
    QwertyEffort {
        c: '>',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: '?',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '@',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: 'A',
        distance: 0,
        modified: true,
    },
    QwertyEffort {
        c: 'B',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'C',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'D',
        distance: 0,
        modified: true,
    },
    QwertyEffort {
        c: 'E',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'F',
        distance: 0,
        modified: true,
    },
    QwertyEffort {
        c: 'G',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'H',
        distance: 0,
        modified: true,
    },
    QwertyEffort {
        c: 'I',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'J',
        distance: 0,
        modified: true,
    },
    QwertyEffort {
        c: 'K',
        distance: 0,
        modified: true,
    },
    QwertyEffort {
        c: 'L',
        distance: 0,
        modified: true,
    },
    QwertyEffort {
        c: 'M',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'N',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'O',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'P',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'Q',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'R',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'S',
        distance: 0,
        modified: true,
    },
    QwertyEffort {
        c: 'T',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'U',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'V',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'W',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'X',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'Y',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: 'Z',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: '[',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: '\'',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '\\',
        distance: 3,
        modified: false,
    },
    QwertyEffort {
        c: ']',
        distance: 2,
        modified: false,
    },
    QwertyEffort {
        c: '^',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '_',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '`',
        distance: 3,
        modified: false,
    },
    QwertyEffort {
        c: 'a',
        distance: 0,
        modified: false,
    },
    QwertyEffort {
        c: 'b',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'c',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'd',
        distance: 0,
        modified: false,
    },
    QwertyEffort {
        c: 'e',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'f',
        distance: 0,
        modified: false,
    },
    QwertyEffort {
        c: 'g',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'h',
        distance: 0,
        modified: false,
    },
    QwertyEffort {
        c: 'i',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'j',
        distance: 0,
        modified: false,
    },
    QwertyEffort {
        c: 'k',
        distance: 0,
        modified: false,
    },
    QwertyEffort {
        c: 'l',
        distance: 0,
        modified: false,
    },
    QwertyEffort {
        c: 'm',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'n',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'o',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'p',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'q',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'r',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 's',
        distance: 0,
        modified: false,
    },
    QwertyEffort {
        c: 't',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'u',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'v',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'w',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'x',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'y',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: 'z',
        distance: 1,
        modified: false,
    },
    QwertyEffort {
        c: '{',
        distance: 1,
        modified: true,
    },
    QwertyEffort {
        c: '|',
        distance: 3,
        modified: true,
    },
    QwertyEffort {
        c: '}',
        distance: 2,
        modified: true,
    },
    QwertyEffort {
        c: '~',
        distance: 3,
        modified: true,
    },
];

const fn one_key_effort(key: &QwertyEffort) -> u64 {
    if key.c == ' ' {
        return 1;
    }
    let mut effort: u64 = 2;
    effort += key.distance as u64;
    if key.modified {
        effort += 2;
    }
    effort
}

const PRECOMPUTED_EFFORT_ARRAY: [(char, u64); 95] = {
    let mut result: [(char, u64); 95] = [(' ', 0); 95];
    let mut idx = 0;
    while idx < 95 {
        let k = &KEY_EFFORTS[idx];
        result[idx] = (k.c, one_key_effort(k));
        idx += 1;
    }
    result
};

pub const QWERTY_CHARS: [char; 95] = {
    let mut result: [char; 95] = [' '; 95];
    let mut idx = 0;
    while idx < 95 {
        let k = &KEY_EFFORTS[idx];
        result[idx] = k.c;
        idx += 1;
    }
    result
};

pub static PRECOMPUTED_EFFORTS: LazyLock<HashMap<char, u64>> =
    LazyLock::new(|| HashMap::from(PRECOMPUTED_EFFORT_ARRAY));

#[must_use]
pub(crate) const fn const_two_key_effort(last: Option<char>, key: char) -> Option<u64> {
    // TODO(lb, low): The second of two modified keys should be discounted the
    // cost of the modifier.
    match last {
        Some(l) if l == key => Some(2),
        _ => {
            let mut i = 0;
            #[allow(long_running_const_eval)]
            let l = PRECOMPUTED_EFFORT_ARRAY.len();
            while i < l {
                let (k, e) = PRECOMPUTED_EFFORT_ARRAY[i];

                if key == k {
                    return Some(e);
                }
                i += 1;
            }
            None
        }
    }
}

#[must_use]
pub fn two_key_effort(last: Option<char>, key: char) -> Option<u64> {
    // TODO(lb, low): The second of two modified keys should be discounted the
    // cost of the modifier.
    match last {
        Some(l) if l == key => Some(2),
        _ => PRECOMPUTED_EFFORTS.get(&key).copied(),
    }
}

// ref: qwerty-effort
#[must_use]
pub fn qwerty_effort_fill(s: &str, fill: u64) -> u64 {
    let mut effort = 0;
    let mut last = None;
    for c in s.chars() {
        if let Some(two) = two_key_effort(last, c) {
            effort += two;
            last = Some(c);
        } else {
            effort += fill;
        }
    }
    effort
}

#[must_use]
pub(crate) const fn const_qwerty_effort(s: &str) -> Option<u64> {
    let mut effort = 0;
    let mut last = None;
    let mut i = 0;
    while i < s.len() {
        let c = s.as_bytes()[i];
        let c = char::from_u32(c as u32).unwrap();
        if let Some(two) = const_two_key_effort(last, c) {
            effort += two;
            last = Some(c);
        } else {
            return None;
        }
        i += 1;
    }
    Some(effort)
}

#[must_use]
pub(crate) const fn const_qwerty_benefit(long: &str, short: &str) -> u64 {
    const_qwerty_effort(long)
        .unwrap()
        .saturating_sub(const_qwerty_effort(short).unwrap())
}

macro_rules! abbrev {
    ($s1:expr, $s2:expr) => {
        ($s1, $s2, const_qwerty_benefit($s1, $s2))
    };
}
#[allow(unused_imports)]
pub(crate) use abbrev;

// ref: qwerty-effort
#[must_use]
pub fn qwerty_effort(s: &str) -> Option<u64> {
    let mut effort = 0;
    let mut last = None;
    for c in s.chars() {
        if let Some(two) = two_key_effort(last, c) {
            effort += two;
            last = Some(c);
        } else {
            return None;
        }
    }
    Some(effort)
}

#[cfg(test)]
mod test_qwerty_effort {
    use std::fmt::Write as _;

    use expect_test::expect;

    use crate::{abbrev::ABBREVS, effort::qwerty::const_qwerty_effort};

    use super::qwerty_effort;

    fn e(s: &str) -> u64 {
        qwerty_effort(s).unwrap()
    }

    #[test]
    fn test_qwerty_effort_single_char() {
        // ref: qwerty-effort-examples
        assert_eq!(e(" "), 1);
        assert_eq!(e("f"), 2);
        assert_eq!(e("["), 3);
        assert_eq!(e("F"), 4);
        assert_eq!(e(":"), 5);
        assert_eq!(e("@"), 6);
        assert_eq!(e("~"), 7);
    }

    #[test]
    fn test_qwerty_effort_i() {
        assert_eq!(e("i"), 3);
        assert_eq!(e("I"), 5);
    }

    proptest::proptest! {
        #[test]
        fn qwerty_effort_const(s in "\\PC*") {
            assert_eq!(qwerty_effort(&s), const_qwerty_effort(&s));
        }
    }

    #[test]
    fn abbrevs_effort() {
        let expected = expect![[r#"
            about --> abt: 6
            actually --> acty: 9
            after --> af: 9
            also --> al: 5
            and --> d: 5
            aren't --> r': 11
            are --> r: 5
            around --> ard: 9
            be --> b: 3
            because --> bc: 13
            been --> bn: 5
            before --> bf: 12
            between --> bw: 14
            but --> bt: 3
            can --> c: 5
            can't --> c': 8
            could --> cd: 8
            couldn't --> cd': 14
            down --> dn: 6
            especially --> esp: 18
            every --> evy: 6
            for --> f: 6
            forward --> fwd: 11
            from --> m: 8
            further --> furth: 6
            her --> hr: 3
            his --> hs: 3
            how --> hw: 3
            however --> hwv: 12
            I'd --> id: 6
            I --> i: 2
            I'll --> il: 8
            I'm --> im: 6
            into --> io: 6
            isn't --> s': 9
            is --> s: 3
            I've --> iv: 9
            just --> j: 8
            much --> mh: 6
            need --> nd: 5
            needn't --> nd': 11
            never --> nv: 9
            no --> n: 3
            not --> x: 6
            of --> o: 2
            okay --> k: 8
            only --> oy: 5
            otherwise --> othw: 14
            over --> ov: 6
            please --> pls: 8
            really --> rly: 7
            several --> sev: 10
            shouldn't --> sd': 16
            should --> sd: 10
            thanks --> tks: 7
            that --> tt: 5
            the --> l: 6
            they --> e: 8
            this --> h: 8
            though --> tho: 8
            through --> thro: 8
            together --> tog: 14
            tomorrow --> tmrw: 11
            to --> t: 3
            very --> v: 9
            wasn't --> w': 10
            was --> w: 4
            were --> ee: 7
            weren't --> ee': 13
            will --> ll: 6
            willn't --> ll': 12
            without --> wo: 14
            won't --> wo': 6
            wouldn't --> wd': 14
            would --> wd: 8
            yeah --> yh: 5
            yes --> y: 5
            you're --> u': 12
            your --> ur: 6
            you --> u: 6
            address --> addr: 7
            apartment --> aptm: 14
            apartment's --> aptm': 16
            apartments --> aptms: 14
            apartments' --> aptms': 14
            appointment --> appt: 21
            appointment's --> appt': 23
            appointments --> appts: 21
            appointments' --> appts': 21
            babies --> bbs: 9
            babies' --> bbs': 9
            baby --> bb: 6
            baby's --> bb': 8
            billion --> bil: 11
            billion's --> bil': 13
            billions --> bils: 11
            billions' --> bils': 11
            complex --> cx: 14
            context --> ctx: 12
            context's --> ctx': 14
            contexts' --> ctxs': 12
            department --> dept: 17
            department's --> dept': 19
            departments --> depts: 17
            departments' --> depts': 17
            dozen --> doz: 6
            dozen's --> doz': 8
            dozens --> dozs: 6
            dozens' --> dozs': 6
            email --> eml: 5
            email's --> eml': 7
            emails --> emls: 5
            emails' --> emls': 5
            everything --> evthg: 15
            everything's --> evthg': 17
            government --> govt: 18
            government's --> govt': 20
            governments --> govts: 18
            governments' --> govts': 18
            hundred --> hun: 10
            hundred's --> hun': 12
            hundreds --> huns: 10
            hundreds' --> huns': 10
            information --> info: 20
            interest --> ist: 15
            interest's --> ist': 17
            interesting --> istq: 21
            interests --> ists: 15
            interests' --> ists': 15
            management --> mgmt: 16
            management's --> mgmt': 18
            market --> mkt: 8
            marketing --> mktq: 14
            market's --> mkt': 10
            markets --> mkts: 8
            markets' --> mkts': 8
            maximum --> maxm: 9
            maximum's --> maxm': 11
            meeting --> mtg: 11
            meeting's --> mtg': 13
            meetings --> mtgs: 11
            meetings' --> mtgs': 11
            million --> mil: 11
            million's --> mil': 13
            millions --> mils: 11
            millions' --> mils': 11
            minimum --> minm: 9
            minimum's --> minm': 11
            move --> mv: 6
            moving --> mvq: 9
            move's --> mv': 8
            moves --> mvs: 6
            moves' --> mvs': 6
            organization --> org: 25
            organization's --> org': 27
            organizations --> orgs: 25
            organizations' --> orgs': 25
            other --> oth: 6
            othering --> othq: 12
            other's --> oth': 8
            others --> oths: 6
            others' --> oths': 6
            people --> ppl: 10
            people's --> ppl': 12
            president --> pres: 14
            president's --> pres': 16
            presidents' --> press': 14
            question --> q: 20
            questioning --> qq: 27
            question's --> q': 22
            questions --> qs: 20
            questions' --> qs': 20
            request --> rqst: 9
            request's --> rqst': 11
            requests --> rqsts: 9
            requests' --> rqsts': 9
            requirement --> reqm: 21
            requirement's --> reqm': 23
            requirements --> reqms: 21
            requirements' --> reqms': 21
            right --> rt: 8
            righting --> rtq: 14
            right's --> rt': 10
            rights --> rts: 8
            rights' --> rts': 8
            service's --> svc': 14
            services --> svcs: 12
            services' --> svcs': 12
            service --> svc: 12
            something --> smth: 15
            something's --> smth': 17
            somethings --> smths: 15
            somethings' --> smths': 15
            thing's --> tg': 10
            things --> tgs: 8
            things' --> tgs': 8
            thing --> tg: 8
            thousands's --> thous': 11
            thousandss' --> thouss': 9
            thousands --> thous: 9
            thousand --> thou: 9
            tonight's --> tn': 16
            tonight --> tn: 14
            trillion's --> tril': 13
            trillions --> trils: 11
            trillions' --> trils': 11
            trillion --> tril: 11
            have --> hv: 5
            having --> hvq: 8
            keeping --> kpq: 11
            keep --> kp: 5
            knowing --> kwq: 12
            knowingly --> kwqly: 12
            know --> kw: 6
            minimize --> minz: 12
            minimizing --> minzq: 15
            maximize --> maxz: 12
            maximizing --> maxzq: 15
            good --> gd: 5
            international --> intl: 25
            internationally --> intlly: 25
            little --> lil: 8
            littlest --> lilst: 8
            major --> mjr: 5
            minor --> mnr: 6
            as far as I know --> ,afaik: 21
            be right back --> ,brb: 20
            for real --> ,fr: 11
            for your information --> ,fyi: 42
            I don't care --> ,idc: 22
            I don't know --> ,idk: 23
            if I remember correctly --> ,iirc: 48
            if I understand correctly --> ,iiuc: 50
            I love you --> ,ily: 16
            I love you so much --> ,ilysm: 29
            in case you missed it --> ,icymi: 32
            in my honest opinion --> ,imho: 38
            in my opinion --> ,imo: 23
            just kidding --> ,jk: 22
            looks good to me --> ,lgtm: 22
            no problem --> ,np: 18
            of course --> ,ofc: 12
            on my way --> ,omw: 10
            right now --> ,rn: 15
            talk to you later --> ,ttyl: 27
            talk to you soon --> ,ttys: 24
            thank you so much --> ,tysm: 26
            thank you --> ,ty: 13
            to be honest --> ,tbh: 19
            with respect to --> ,wrt: 27
            you're welcome --> ,yw: 31
            ain't --> ai': 6
            didn't --> di': 8
            don't --> do': 6
            hadn't --> had': 6
            hasn't --> has': 6
            mustn't --> must': 6
            oughtn't --> ought': 6
            Sunday --> sund: 7
            Sundays --> sunds: 7
            Sunday's --> sund': 9
            Sundays' --> sunds': 7
            Monday --> mond: 7
            Mondays --> monds: 7
            Monday's --> mond': 9
            Mondays' --> monds': 7
            Tuesday --> tues: 9
            Tuesdays --> tuess: 9
            Tuesday's --> tues': 11
            Tuesdays' --> tuess': 9
            Wednesday --> weds: 15
            Wednesdays --> wedss: 15
            Wednesday's --> weds': 17
            Wednesdays' --> wedss': 15
            Thursday --> thur: 11
            Thursdays --> thurs: 11
            Thursday's --> thur': 13
            Thursdays' --> thurs': 11
            Friday --> frid: 7
            Fridays --> frids: 7
            Friday's --> frid': 9
            Fridays' --> frids': 7
            Saturday --> satd: 13
            Saturdays --> satds: 13
            Saturday's --> satd': 15
            Saturdays' --> satds': 13
            January --> jan: 13
            Januarys --> jans: 13
            January's --> jan': 15
            Januarys' --> jans': 13
            February --> feb: 16
            Februarys --> febs: 16
            February's --> feb': 18
            Februarys' --> febs': 16
            March --> mar: 7
            Marchs --> mars: 7
            March's --> mar': 9
            Marchs' --> mars': 7
            April --> apr: 7
            Aprils --> aprs: 7
            April's --> apr': 9
            Aprils' --> aprs': 7
            June --> jun: 5
            Junes --> juns: 5
            June's --> jun': 7
            Junes' --> juns': 5
            July --> jul: 5
            Julys --> juls: 5
            July's --> jul': 7
            Julys' --> juls': 5
            August --> aug: 10
            Augusts --> augs: 10
            August's --> aug': 12
            Augusts' --> augs': 10
            September --> sep: 20
            Septembers --> seps: 20
            September's --> sep': 22
            Septembers' --> seps': 20
            October --> oct: 14
            Octobers --> octs: 14
            October's --> oct': 16
            Octobers' --> octs': 14
            November --> nov: 17
            Novembers --> novs: 17
            November's --> nov': 19
            Novembers' --> novs': 17
            December --> dec: 17
            Decembers --> decs: 17
            December's --> dec': 19
            Decembers' --> decs': 17
            zero --> n0: 5
            one --> n1: 2
            two --> n2: 2
            three --> n3: 6
            four --> n4: 4
            five --> n5: 4
            six --> n6: 1
            seven --> n7: 7
            eight --> n8: 7
            nine --> n9: 5
            ten --> n10: 0
            eleven --> n11: 8
            twelve --> n12: 6
            thirteen --> n13: 11
            fourteen --> n14: 11
            fifteen --> n15: 7
            sixteen --> n16: 8
            seventeen --> n17: 14
            eighteen --> n18: 11
            nineteen --> n19: 12
            twenty --> n20: 7
            zeroth --> o0: 10
            first --> o1: 6
            second --> o2: 9
            third --> o3: 6
            fourth --> o4: 9
            fifth --> o5: 5
            sixth --> o6: 6
            seventh --> o7: 12
            eighth --> o8: 9
            ninth --> o9: 7
            tenth --> o10: 3
            eleventh --> o11: 13
            twelfth --> o12: 7
            thirteenth --> o13: 16
            fourteenth --> o14: 16
            fifteenth --> o15: 12
            sixteenth --> o16: 13
            seventeenth --> o17: 19
            eighteenth --> o18: 16
            nineteenth --> o19: 17
            twentieth --> o20: 15
            Brooklyn --> ,bk: 15
            Chicago --> ,chi: 10
            London --> ,lon: 7
            Los Angeles --> ,lax: 20
            Mexico City --> ,cdmx: 21
            New York City --> ,nyc: 28
            Portland --> ,pdx: 12
            Seattle --> ,sea: 9
            St Louis --> ,stl: 13
            Alabama --> ,al: 11
            Alaska --> ,ak: 7
            Arizona --> ,az: 13
            Arkansas --> ,ar: 12
            California --> ,ca: 20
            Colorado --> ,co: 14
            Connecticut --> ,ct: 25
            Delaware --> ,de: 14
            Florida --> ,fl: 12
            Georgia --> ,ga: 14
            Hawaii --> ,hi: 8
            Idaho --> ,id: 6
            Illinois --> ,il: 15
            Indiana --> ,in: 11
            Iowa --> ,ia: 5
            Kansas --> ,ks: 8
            Kentucky --> ,ky: 16
            Louisiana --> ,la: 18
            Maine --> ,me: 7
            Maryland --> ,md: 14
            Massachusetts --> ,ma: 25
            Michigan --> ,mi: 15
            Minnesota --> ,mn: 17
            Mississippi --> ,ms: 22
            Missouri --> ,mo: 15
            Montana --> ,mt: 12
            Nebraska --> ,ne: 13
            Nevada --> ,nv: 8
            New Hampshire --> ,nh: 29
            New Jersey --> ,nj: 22
            New Mexico --> ,nm: 23
            New York --> ,ny: 16
            North Carolina --> ,nc: 31
            North Dakota --> ,nd: 25
            Ohio --> ,oh: 5
            Oklahoma --> ,ok: 13
            Oregon --> ,or: 11
            Pennsylvania --> ,pa: 25
            Rhode Island --> ,ri: 23
            South Carolina --> ,sc: 31
            South Dakota --> ,sd: 25
            Tennessee --> ,tn: 16
            Texas --> ,tx: 6
            Utah --> ,ut: 3
            Vermont --> ,vt: 14
            Virginia --> ,va: 17
            Washington --> ,wa: 21
            West Virginia --> ,wv: 30
            Wisconsin --> ,wi: 18
            Wyoming --> ,wy: 14
            Afghanistan --> c,af: 19
            Albania --> c,al: 9
            Algeria --> c,dz: 9
            American Samoa --> c,as: 29
            Andorra --> c,ad: 9
            Angola --> c,ao: 6
            Anguilla --> c,ai: 11
            Antarctica --> c,aq: 18
            Antigua and Barbuda --> c,ag: 39
            Argentina --> c,ar: 16
            Armenia --> c,am: 10
            Aruba --> c,aw: 4
            Australia --> c,au: 13
            Austria --> c,at: 9
            Azerbaijan --> c,az: 17
            Bahamas --> c,bs: 7
            Bahrain --> c,bh: 9
            Bangladesh --> c,bd: 15
            Barbados --> c,bb: 11
            Belarus --> c,by: 8
            Belgium --> c,be: 10
            Belize --> c,bz: 7
            Benin --> c,bj: 6
            Bermuda --> c,bm: 9
            Bhutan --> c,bt: 6
            Bolivia --> c,bo: 9
            Bosnia and Herzegovina --> c,ba: 49
            Botswana --> c,bw: 11
            Bouvet Island --> c,bv: 25
            Brazil --> c,br: 6
            British Indian Ocean Territory --> c,io: 74
            Brunei --> c,bn: 8
            Bulgaria --> c,bg: 11
            Burkina Faso --> c,bf: 22
            Burundi --> c,bi: 10
            Cambodia --> c,kh: 13
            Cameroon --> c,cm: 12
            Canada --> c,ca: 5
            Cape Verde --> c,cv: 18
            Cayman Islands --> c,ky: 26
            Central African Republic --> c,cf: 57
            Chad --> c,td: 0
            Chile --> c,cl: 4
            China --> c,cn: 3
            Christmas Island --> c,cx: 30
            Coc --> c,cc: 0
            Colombia --> c,co: 12
            Comoros --> c,km: 11
            Congo --> c,cg: 5
            Democratic Republic of the Congo --> c,cd: 78
            Cook Islands --> c,ck: 20
            Costa Rica --> c,cr: 17
            Ivory Coast --> c,ci: 21
            Croatia --> c,hr: 10
            Cuba --> c,cu: 1
            Cyprus --> c,cy: 7
            Czech Republic --> c,cz: 30
            Denmark --> c,dk: 10
            Djibouti --> c,dj: 14
            Dominica --> c,dm: 13
            Dominican Republic --> c,do: 42
            Ecuador --> c,ec: 9
            Egypt --> c,eg: 5
            El Salvador --> c,sv: 18
            Equatorial Guinea --> c,gq: 37
            Eritrea --> c,er: 10
            Estonia --> c,ee: 10
            Ethiopia --> c,et: 12
            Falkland Islands --> c,fk: 28
            Faroe Islands --> c,fo: 23
            Fiji --> c,fj: 2
            Finland --> c,fi: 8
            France --> c,fr: 7
            French Guiana --> c,gf: 26
            French Polynesia --> c,pf: 34
            French Southern Territories --> c,tf: 66
            Gabon --> c,ga: 5
            Gambia --> c,gm: 6
            Germany --> c,de: 11
            Ghana --> c,gh: 3
            Gibraltar --> c,gi: 14
            Greece --> c,gr: 7
            Greenland --> c,gl: 14
            Grenada --> c,gd: 9
            Guadeloupe --> c,gp: 17
            Guam --> c,gu: 1
            Guatemala --> c,gt: 13
            Guernsey --> c,gg: 14
            Guinea --> c,gn: 7
            Guyana --> c,gy: 6
            Haiti --> c,ht: 4
            Heard Island and McDonald Islands --> c,hm: 72
            Vatican City --> c,va: 25
            Honduras --> c,hn: 11
            Hong Kong --> c,hk: 17
            Hungary --> c,hu: 10
            Iceland --> c,is: 9
            India --> c,in: 3
            Indonesia --> c,id: 15
            Iran --> c,ir: 1
            Iraq --> c,iq: 1
            Ireland --> c,ie: 8
            Isle of Man --> c,im: 17
            Israel --> c,il: 6
            Italy --> c,it: 3
            Jamaica --> c,jm: 8
            Japan --> c,jp: 3
            Jersey --> c,je: 7
            Jordan --> c,jo: 6
            Kazakhstan --> c,kz: 14
            Kenya --> c,ke: 4
            Kiribati --> c,ki: 13
            Democratic People's Republic of Korea --> c,kp: 93
            South Korea --> c,kr: 20
            Kuwait --> c,kw: 7
            Kyrgyzstan --> c,kg: 18
            Laos --> c,la: 1
            Latvia --> c,lv: 6
            Lebanon --> c,lb: 10
            Lesotho --> c,ls: 10
            Liberia --> c,lr: 10
            Libya --> c,ly: 4
            Liechtenstein --> c,li: 27
            Lithuania --> c,lt: 14
            Luxembourg --> c,lu: 20
            Macao --> c,mo: 3
            Macedonia --> c,mk: 15
            Madagascar --> c,mg: 14
            Malawi --> c,mw: 5
            Malaysia --> c,my: 9
            Maldives --> c,mv: 10
            Mali --> c,ml: 1
            Malta --> c,mt: 2
            Marshall Islands --> c,mh: 28
            Martinique --> c,mq: 19
            Mauritania --> c,mr: 17
            Mauritius --> c,mu: 15
            Mayotte --> c,yt: 9
            Mexico --> c,mx: 8
            Micronesia --> c,fm: 19
            Moldova --> c,md: 9
            Monaco --> c,mc: 7
            Mongolia --> c,mn: 12
            Montenegro --> c,me: 20
            Montserrat --> c,ms: 18
            Morocco --> c,ma: 11
            Mozambique --> c,mz: 19
            Myanmar --> c,mm: 10
            Namibia --> c,na: 10
            Nauru --> c,nr: 4
            Nepal --> c,np: 3
            Netherlands --> c,nl: 19
            Netherlands Antilles --> c,an: 42
            New Caledonia --> c,nc: 25
            New Zealand --> c,nz: 19
            Nicaragua --> c,ni: 14
            Niger --> c,ne: 5
            Nigeria --> c,ng: 10
            Niue --> c,nu: 2
            Norfolk Island --> c,nf: 26
            Northern Mariana Islands --> c,mp: 53
            Norway --> c,no: 7
            Oman --> c,om: 1
            Pakistan --> c,pk: 11
            Palau --> c,pw: 2
            Palestinian Territory --> c,ps: 49
            Panama --> c,pa: 6
            Papua New Guinea --> c,pg: 35
            Paraguay --> c,py: 11
            Peru --> c,pe: 2
            Philippines --> c,ph: 20
            Pitcairn --> c,pn: 13
            Poland --> c,pl: 6
            Portugal --> c,pt: 12
            Puerto Rico --> c,pr: 23
            Qatar --> c,qa: 4
            Romania --> c,ro: 9
            Russia --> c,ru: 5
            Rwanda --> c,rw: 5
            Saint Kitts and Nevis --> c,kn: 44
            Saint Lucia --> c,lc: 20
            Saint Pierre and Miquelon --> c,pm: 57
            St Vincent and the Grenadines --> c,vc: 66
            Samoa --> c,ws: 3
            San Marino --> c,sm: 18
            Sao Tome and Principe --> c,st: 48
            Saudi Arabia --> c,sa: 22
            Senegal --> c,sn: 9
            Serbia --> c,rs: 7
            Seychelles --> c,sc: 16
            Sierra Leone --> c,sl: 24
            Singapore --> c,sg: 16
            Slovakia --> c,sk: 11
            Slovenia --> c,si: 12
            Solomon Islands --> c,sb: 29
            Somalia --> c,so: 8
            South Africa --> c,za: 22
            South Georgia and the South Sandwich Islands --> c,gs: 102
            South Sudan --> c,ss: 20
            Spain --> c,es: 4
            Sri Lanka --> c,lk: 14
            Sudan --> c,sd: 4
            Suriname --> c,sr: 13
            Svalbard and Jan Mayen --> c,sj: 46
            Swaziland --> c,sz: 13
            Sweden --> c,se: 7
            Switzerland --> c,ch: 20
            Syrian Arab Republic --> c,sy: 46
            Taiwan --> c,tw: 6
            Tajikistan --> c,tj: 16
            Tanzania --> c,tz: 11
            Thailand --> c,th: 10
            Togo --> c,tg: 2
            Tokelau --> c,tk: 9
            Tonga --> c,to: 4
            Trinidad and Tobago --> c,tt: 40
            Tunisia --> c,tn: 9
            Turkey --> c,tr: 7
            Turkmenistan --> c,tm: 23
            Turks and Caicos Islands --> c,tc: 49
            Tuvalu --> c,tv: 6
            Uganda --> c,ug: 5
            Ukraine --> c,ua: 10
            United Arab Emirates --> c,ae: 46
            United Kingdom --> c,gb: 29
            United States --> c,us: 26
            United States Minor Outlying Islands --> c,um: 88
            Uruguay --> c,uy: 10
            Uzbekistan --> c,uz: 17
            Vanuatu --> c,vu: 9
            Venezuela --> c,ve: 15
            Vietnam --> c,vn: 10
            British Virgin Islands --> c,vg: 49
            US Virgin Islands --> c,vi: 37
            Wallis and Futuna --> c,wf: 32
            Western Sahara --> c,eh: 27
            Yemen --> c,ye: 5
            Zambia --> c,zm: 6
            Zimbabwe --> c,zw: 13
            actinium --> e,ac: 12
            aluminum --> e,al: 12
            americium --> e,am: 15
            antimony --> e,sb: 12
            argon --> e,ar: 3
            arsenic --> e,as: 9
            astatine --> e,at: 10
            barium --> e,ba: 6
            berkelium --> e,bk: 14
            beryllium --> e,be: 13
            bismuth --> e,bi: 7
            bohrium --> e,bh: 9
            boron --> e,b: 6
            bromine --> e,br: 9
            cadmium --> e,cd: 8
            calcium --> e,ca: 8
            californium --> e,cf: 19
            carbon --> e,c: 8
            cerium --> e,ce: 6
            cesium --> e,cs: 6
            chlorine --> e,cl: 11
            chromium --> e,cr: 11
            cobalt --> e,co: 4
            copernicium --> e,cn: 21
            copper --> e,cu: 5
            curium --> e,cm: 6
            darmstadtium --> e,ds: 21
            dubnium --> e,db: 9
            dysprosium --> e,dy: 16
            einsteinium --> e,es: 21
            erbium --> e,er: 6
            europium --> e,eu: 12
            fermium --> e,fm: 9
            flerovium --> e,fl: 15
            fluorine --> e,f: 14
            francium --> e,fr: 11
            gadolinium --> e,gd: 16
            gallium --> e,ga: 7
            germanium --> e,ge: 14
            gold --> e,au: 0
            hafnium --> e,hf: 8
            hassium --> e,hs: 7
            helium --> e,he: 5
            holmium --> e,ho: 8
            hydrogen --> e,h: 14
            indium --> e,in: 5
            iodine --> e,i: 8
            iridium --> e,ir: 8
            iron --> e,fe: 1
            krypton --> e,kr: 9
            lanthanum --> e,la: 13
            lawrencium --> e,lr: 17
            lead --> e,pb: 0
            lithium --> e,li: 8
            livermorium --> e,lv: 21
            lutetium --> e,lu: 12
            magnesium --> e,mg: 13
            manganese --> e,mn: 12
            meitnerium --> e,mt: 18
            mendelevium --> e,md: 20
            mercury --> e,hg: 10
            molybdenum --> e,mo: 16
            moscovium --> e,mc: 14
            neodymium --> e,nd: 15
            neon --> e,ne: 0
            neptunium --> e,np: 15
            nickel --> e,ni: 4
            nihonium --> e,nh: 12
            niobium --> e,nb: 9
            nitrogen --> e,n: 15
            nobelium --> e,no: 11
            oganesson --> e,og: 12
            osmium --> e,os: 6
            oxygen --> e,o: 9
            palladium --> e,pd: 11
            phosphorus --> e,p: 17
            platinum --> e,pt: 10
            plutonium --> e,pu: 14
            polonium --> e,po: 11
            potassium --> e,k: 16
            praseodymium --> e,pr: 21
            promethium --> e,pm: 17
            protactinium --> e,pa: 24
            radium --> e,ra: 5
            radon --> e,rn: 1
            rhenium --> e,re: 8
            rhodium --> e,rh: 8
            roentgenium --> e,rg: 21
            rubidium --> e,rb: 11
            ruthenium --> e,ru: 14
            rutherfordium --> e,rf: 25
            samarium --> e,sm: 10
            scandium --> e,sc: 10
            seaborgium --> e,sg: 17
            selenium --> e,se: 11
            silicon --> e,si: 8
            silver --> e,ag: 5
            sodium --> e,na: 5
            strontium --> e,sr: 15
            sulfur --> e,s: 7
            tantalum --> e,ta: 10
            technetium --> e,tc: 17
            tellurium --> e,te: 13
            tennessine --> e,ts: 16
            terbium --> e,tb: 9
            thallium --> e,tl: 9
            thorium --> e,th: 9
            thulium --> e,tm: 7
            tin --> e,sn: 0
            titanium --> e,ti: 11
            uranium --> e,u: 11
            vanadium --> e,v: 12
            wolfram --> e,w: 9
            xenon --> e,xe: 3
            ytterbium --> e,yb: 14
            yttrium --> e,y: 11
            zinc --> e,zn: 0
            zirconium --> e,zr: 15
            National Basketball Association --> b,nba: 68
            Atlanta Hawks --> b,atl: 20
            Boston Celtics --> b,bos: 27
            Brooklyn Nets --> b,bkn: 23
            Charlotte Hornets --> b,cha: 34
            Chicago Bulls --> b,chi: 22
            Cleveland Cavaliers --> b,cle: 37
            Dallas Mavericks --> b,dal: 29
            Denver Nuggets --> b,den: 27
            Detroit Pistons --> b,det: 30
            Golden State Warriors --> b,gsw: 44
            Houston Rockets --> b,hou: 29
            Indiana Pacers --> b,ind: 25
            Los Angeles Clippers --> b,lac: 41
            Los Angeles Lakers --> b,lal: 35
            Memphis Grizzlies --> b,mem: 33
            Miami Heat --> b,mia: 15
            Milwaukee Bucks --> b,mil: 27
            Minnesota Timberwolves --> b,min: 48
            New Orleans Pelicans --> b,nop: 41
            New York Knicks --> b,nyk: 29
            Oklahoma City Thunder --> b,okc: 44
            Orlando Magic --> b,orl: 23
            Phoenix Suns --> b,phx: 21
            Portland Trail Blazers --> b,por: 45
            Sacramento Kings --> b,sac: 32
            San Antonio Spurs --> b,sas: 36
            Toronto Raptors --> b,tor: 30
            Utah Jazz --> b,uta: 10
            Washington Wizards --> b,was: 37
        "#]];
        let mut s = String::with_capacity(2 * ABBREVS.len());
        for (long, short, e) in ABBREVS {
            writeln!(s, "{long} --> {short}: {e}").unwrap();
        }
        expected.assert_eq(&s);
    }
}
