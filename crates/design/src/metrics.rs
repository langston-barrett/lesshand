#![allow(clippy::cast_precision_loss)]

use distance::levenshtein;
use lesshand::{
    dec::dec_with_default_rules,
    effort::{print::print_effort_fill, qwerty::qwerty_effort_fill},
    enc::enc_with_default_rules,
};
use std::{fmt::Write as _, iter};

use crate::string::rpad;

fn dist(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 0.00;
    }
    let mut distance = 0.0;
    for (p1, p2) in iter::zip(s1.split('\n'), s2.split('\n')) {
        // Encoding / decoding always change length, and don't introduce or
        // remove newlines.
        if p1.len() == p2.len() {
            continue;
        }
        distance += levenshtein(p1, p2) as f64;
    }
    distance
}

fn ratio_reduced(orig: usize, new: usize) -> f64 {
    if new > orig {
        return 0.0;
    }
    (orig - new) as f64 / orig as f64
}

fn ratio_reduced64(orig: u64, new: u64) -> f64 {
    if new > orig {
        return 0.0;
    }
    (orig - new) as f64 / orig as f64
}

/// See evaluation.md
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Metrics {
    /// Percentage (ratio) reduction of text when encoding English
    encode: f64,
    /// Percentage (ratio) difference of text when decoding English
    decode: f64,
    /// Percentage (ratio) difference when double encoding
    double: f64,
    /// Percentage (ratio) reduction of QWERTY effort when encoding English
    qwerty: f64,
    /// Percentage (ratio) reduction of printed effort when encoding English
    print: f64,
    /// Percentage (ratio) difference of text when encoding then decoding. This
    /// should always be equal to 'decode' if the rules are indeed lossless.
    roundtrip: f64,
    /// Original size of text
    size: usize,
}

impl Metrics {
    fn check(&self) {
        assert!(self.encode >= 0.0 && self.encode <= 1.0);
        assert!(self.decode >= 0.0 && self.decode <= 1.0);
        assert!(self.double >= 0.0 && self.double <= 1.0);
        assert!(self.qwerty >= 0.0 && self.qwerty <= 1.0);
        assert!(self.print >= 0.0 && self.print <= 1.0);
        assert!(self.roundtrip >= 0.0 && self.roundtrip <= 1.0);
    }

    fn headers() -> String {
        "QWERT   PRINT   ENC     DEC     DOUBL   ROUND".to_string()
    }

    fn one_line(&self) -> String {
        format!(
            "{:05.2}%  {:05.2}%  {:05.2}%  {:05.2}%  {:05.2}%  {:05.2}%  {}",
            100.0 * self.qwerty,
            100.0 * self.print,
            100.0 * self.encode,
            100.0 * self.decode,
            100.0 * self.double,
            100.0 * self.roundtrip,
            self.size
        )
    }

    #[allow(dead_code)]
    pub(crate) fn print_as_table(items: Vec<(String, Metrics)>) {
        if items.is_empty() {
            return;
        }
        let len = items.iter().map(|(s, _)| s.len()).max().unwrap();
        let first = String::from(" ").repeat(len + 1);
        println!("{} {}", first, Metrics::headers());
        for (k, m) in items {
            println!("{}: {}", rpad(&k, len), m.one_line());
        }
    }

    pub(crate) fn printed(&self) -> String {
        self.check();
        let mut result = String::new();
        writeln!(
            result,
            "QWERTY:    {:.2}% ({})",
            100.0 * self.qwerty,
            // ref: metric-effort
            if self.qwerty > 0.05 { "ok" } else { "FAIL" }
        )
        .unwrap();
        writeln!(
            result,
            "Print:     {:.2}% ({})",
            100.0 * self.print,
            // ref: metric-effort
            if self.print > 0.05 { "ok" } else { "FAIL" }
        )
        .unwrap();
        writeln!(
            result,
            "Encode:    {:.2}% ({})",
            100.0 * self.encode,
            // ref: metric-compression
            if self.encode > 0.03 { "ok" } else { "FAIL" }
        )
        .unwrap();
        writeln!(
            result,
            "Decode:    {:.2}% ({})",
            100.0 * self.decode,
            // ref: metric-decode
            if self.decode < 0.03 { "ok" } else { "FAIL" }
        )
        .unwrap();
        writeln!(
            result,
            "Double:    {:.2}% ({})",
            100.0 * self.double,
            // ref: metric-idempotent
            if self.double < 0.01 { "ok" } else { "FAIL" }
        )
        .unwrap();
        writeln!(
            result,
            "Roundtrip: {:.2}% ({})",
            100.0 * self.roundtrip,
            // ref: metric-roundtrip
            if self.roundtrip < 0.01 { "ok" } else { "FAIL" }
        )
        .unwrap();
        result
    }

    pub(crate) fn print(&self) {
        println!("{}", self.printed());
    }

    /// Average, weighted by size
    #[allow(dead_code)]
    pub(crate) fn average(ms: Vec<Metrics>) -> Metrics {
        let mut avg = Metrics {
            encode: 0.0,
            decode: 0.0,
            double: 0.0,
            qwerty: 0.0,
            print: 0.0,
            roundtrip: 0.0,
            size: 0,
        };
        for m in ms {
            m.check();
            avg.encode += m.encode * m.size as f64;
            avg.decode += m.decode * m.size as f64;
            avg.double += m.double * m.size as f64;
            avg.qwerty += m.qwerty * m.size as f64;
            avg.print += m.print * m.size as f64;
            avg.roundtrip += m.roundtrip * m.size as f64;
            avg.size += m.size;
        }
        avg.encode /= avg.size as f64;
        avg.decode /= avg.size as f64;
        avg.double /= avg.size as f64;
        avg.qwerty /= avg.size as f64;
        avg.print /= avg.size as f64;
        avg.roundtrip /= avg.size as f64;
        avg
    }
}

fn do_calc_double(orig_len: usize, encoded: &str) -> f64 {
    let denom = (2 * orig_len) as f64;
    dist(encoded, &enc_with_default_rules(encoded)) / denom
}

fn do_calc_roundtrip(s: &str, encoded: &str) -> f64 {
    let denom = (2 * s.len()) as f64;
    dist(s, &dec_with_default_rules(encoded)) / denom
}

fn calc_decode(s: &str) -> f64 {
    let denom = (2 * s.len()) as f64;
    dist(s, &dec_with_default_rules(s)) / denom
}

pub(crate) fn metrics(s: &str) -> Metrics {
    let orig_qwerty = qwerty_effort_fill(s, 0);
    let orig_print = print_effort_fill(s, 0);
    let encoded = enc_with_default_rules(s);
    let enc_qwerty = qwerty_effort_fill(&encoded, 0);
    let enc_print = print_effort_fill(&encoded, 0);
    let m = Metrics {
        decode: calc_decode(s),
        double: do_calc_double(s.len(), &encoded),
        encode: ratio_reduced(s.len(), encoded.len()),
        print: ratio_reduced64(orig_print, enc_print),
        qwerty: ratio_reduced64(orig_qwerty, enc_qwerty),
        roundtrip: do_calc_roundtrip(s, &encoded),
        size: s.len(),
    };
    m.check();
    m
}

#[cfg(test)]
mod test {
    use expect_test::expect;

    use super::metrics;

    fn m(s: &str) -> String {
        metrics(s).printed()
    }

    #[test]
    fn test_goals() {
        let expected = expect![[r#"
            QWERTY:    10.81% (ok)
            Print:     10.57% (ok)
            Encode:    10.45% (ok)
            Decode:    0.00% (ok)
            Double:    0.00% (ok)
            Roundtrip: 0.04% (ok)
        "#]];
        expected.assert_eq(&m(include_str!("../../../doc/goals.md")));
    }

    #[test]
    fn test_license() {
        let expected = expect![[r#"
            QWERTY:    5.45% (ok)
            Print:     6.33% (ok)
            Encode:    6.40% (ok)
            Decode:    0.00% (ok)
            Double:    0.00% (ok)
            Roundtrip: 0.00% (ok)
        "#]];
        expected.assert_eq(&m(include_str!("../../../LICENSE")));
    }

    #[test]
    fn test_readme() {
        let expected = expect![[r#"
            QWERTY:    13.80% (ok)
            Print:     12.78% (ok)
            Encode:    12.51% (ok)
            Decode:    0.95% (ok)
            Double:    0.00% (ok)
            Roundtrip: 0.95% (ok)
        "#]];
        expected.assert_eq(&m(include_str!("../../../README.md")));
    }

    const THE_IMPORTANCE_OF_BEING_EARNEST_ACT_I: &str =
        include_str!("../../../corpora/earnest.txt");

    #[test]
    fn test_earnest() {
        let expected = expect![[r#"
            QWERTY:    12.12% (ok)
            Print:     11.72% (ok)
            Encode:    10.69% (ok)
            Decode:    0.21% (ok)
            Double:    0.00% (ok)
            Roundtrip: 0.23% (ok)
        "#]];
        expected.assert_eq(&m(THE_IMPORTANCE_OF_BEING_EARNEST_ACT_I));
    }

    const WIKI_BASKETBALL: &str = r#"
Basketball

Basketball is a team sport in which two teams, most commonly of five players each, opposing one another on a rectangular court, compete with the primary objective of shooting a basketball (approximately in diameter) through the defender's hoop (a basket in diameter mounted high to a backboard at each end of the court) while preventing the opposing team from shooting through their own hoop. A field goal is worth two points, unless made from behind the three-point line, when it is worth three. After a foul, timed play stops and the player fouled or designated to shoot a technical foul is given one, two or three one-point free throws. The team with the most points at the end of the game wins, but if regulation play expires with the score tied, an additional period of play (overtime) is mandated.
Players advance the ball by bouncing it while walking or running (dribbling) or by passing it to a teammate, both of which require considerable skill. On offense, players may use a variety of shotsthe layup, the jump shot, or a dunk; on defense, they may steal the ball from a dribbler, intercept passes, or block shots; either offense or defense may collect a rebound, that is, a missed shot that bounces from rim or backboard. It is a violation to lift or drag one's pivot foot without dribbling the ball, to carry it, or to hold the ball with both hands then resume dribbling.
The five players on each side fall into five playing positions. The tallest player is usually the center, the second-tallest and strongest is the power forward, a slightly shorter but more agile player is the small forward, and the shortest players or the best ball handlers are the shooting guard and the point guard, who implements the coach's game plan by managing the execution of offensive and defensive plays (player positioning). Informally, players may play three-on-three, two-on-two, and one-on-one.
Invented in 1891 by Canadian-American gym teacher James Naismith in Springfield, Massachusetts, United States, basketball has evolved to become one of the world's most popular and widely viewed sports. The National Basketball Association (NBA) is the most significant professional basketball league in the world in terms of popularity, salaries, talent, and level of competition. Outside North America, the top clubs from national leagues qualify to continental championships such as the EuroLeague and the Basketball Champions League Americas. The FIBA Basketball World Cup and Men's Olympic Basketball Tournament are the major international events of the sport and attract top national teams from around the world. Each continent hosts regional competitions for national teams, like EuroBasket and FIBA AmeriCup.
The FIBA Women's Basketball World Cup and Women's Olympic Basketball Tournament feature top national teams from continental championships. The main North American league is the WNBA (NCAA Women's Division I Basketball Championship is also popular), whereas the strongest European clubs participate in the EuroLeague Women.
In December 1891, James Naismith, a Canadian professor of physical education and instructor at the International Young Men's Christian Association Training School (now Springfield College) in Springfield, Massachusetts, was trying to keep his gym class active on a rainy day. He sought a vigorous indoor game to keep his students occupied and at proper levels of fitness during the long New England winters. After rejecting other ideas as either too rough or poorly suited to walled-in gymnasiums, he invented a new game in which players would pass a ball to teammates and try to score points by tossing the ball into a basket mounted on a wall. Naismith wrote the basic rules and nailed a peach basket onto an elevated track. Naismith initially set up the peach basket with its bottom intact, which meant that the ball had to be retrieved manually after each "basket" or point scored. This quickly proved tedious, so Naismith removed the bottom of the basket to allow the balls to be poked out with a long dowel after each scored basket.
Basketball was originally played with a soccer ball. These round balls from "association football" were made, at the time, with a set of laces to close off the hole needed for inserting the inflatable bladder after the other sewn-together segments of the ball's cover had been flipped outside-in. These laces could cause bounce passes and dribbling to be unpredictable. Eventually a lace-free ball construction method was invented, and this change to the game was endorsed by Naismith. (Whereas in American football, the lace construction proved to be advantageous for gripping and remains to this day.) The first balls made specifically for basketball were brown, and it was only in the late 1950s that Tony Hinkle, searching for a ball that would be more visible to players and spectators alike, introduced the orange ball that is now in common use. Dribbling was not part of the original game except for the "bounce pass" to teammates. Passing the ball was the primary means of ball movement. Dribbling was eventually introduced but limited by the asymmetric shape of early balls. Dribbling was common by 1896, with a rule against the double dribble by 1898.
The peach baskets were used until 1906 when they were finally replaced by metal hoops with backboards. A further change was soon made, so the ball merely passed through. Whenever a person got the ball in the basket, his team would gain a point. Whichever team got the most points won the game. The baskets were originally nailed to the mezzanine balcony of the playing court, but this proved impractical when spectators in the balcony began to interfere with shots. The backboard was introduced to prevent this interference; it had the additional effect of allowing rebound shots. Naismith's handwritten diaries, discovered by his granddaughter in early 2006, indicate that he was nervous about the new game he had invented, which incorporated rules from a children's game called duck on a rock, as many had failed before it.
Frank Mahan, one of the players from the original first game, approached Naismith after the Christmas break, in early 1892, asking him what he intended to call his new game. Naismith replied that he hadn't thought of it because he had been focused on just getting the game started. Mahan suggested that it be called "Naismith ball", at which he laughed, saying that a name like that would kill any game. Mahan then said, "Why not call it basketball?" Naismith replied, "We have a basket and a ball, and it seems to me that would be a good name for it." The first official game was played in the YMCA gymnasium in Albany, New York, on January 20, 1892, with nine players. The game ended at 1–0; the shot was made from , on a court just half the size of a present-day Streetball or National Basketball Association (NBA) court.
At the time, soccer was being played with 10 to a team (which was increased to 11). When winter weather got too icy to play soccer, teams were taken indoors, and it was convenient to have them split in half and play basketball with five on each side. By 1897–1898 teams of five became standard.
⁣
Basketball's early adherents were dispatched to YMCAs throughout the United States, and it quickly spread through the United States and Canada. By 1895, it was well established at several women's high schools. While YMCA was responsible for initially developing and spreading the game, within a decade it discouraged the new sport, as rough play and rowdy crowds began to detract from YMCA's primary mission. However, other amateur sports clubs, colleges, and professional clubs quickly filled the void. In the years before World War I, the Amateur Athletic Union and the Intercollegiate Athletic Association of the United States (forerunner of the NCAA) vied for control over the rules for the game. The first pro league, the National Basketball League, was formed in 1898 to protect players from exploitation and to promote a less rough game. This league only lasted five years.
James Naismith was instrumental in establishing college basketball. His colleague C.O. Beamis fielded the first college basketball team just a year after the Springfield YMCA game at the suburban Pittsburgh Geneva College. Naismith himself later coached at the University of Kansas for six years, before handing the reins to renowned coach Forrest "Phog" Allen. Naismith's disciple Amos Alonzo Stagg brought basketball to the University of Chicago, while Adolph Rupp, a student of Naismith's at Kansas, enjoyed great success as coach at the University of Kentucky. On February 9, 1895, the first intercollegiate 5-on-5 game was played at Hamline University between Hamline and the School of Agriculture, which was affiliated with the University of Minnesota. The School of Agriculture won in a 9–3 game.
In 1901, colleges, including the University of Chicago, Columbia University, Cornell University, Dartmouth College, the University of Minnesota, the U.S. Naval Academy, the University of Colorado and Yale University began sponsoring men's games. In 1905, frequent injuries on the football field prompted President Theodore Roosevelt to suggest that colleges form a governing body, resulting in the creation of the Intercollegiate Athletic Association of the United States (IAAUS). In 1910, that body changed its name to the National Collegiate Athletic Association (NCAA). The first Canadian interuniversity basketball game was played at YMCA in Kingston, Ontario on February 6, 1904, when McGill UniversityNaismith's alma matervisited Queen's University. McGill won 9–7 in overtime; the score was 7–7 at the end of regulation play, and a ten-minute overtime period settled the outcome. A good turnout of spectators watched the game.
The first men's national championship tournament, the National Association of Intercollegiate Basketball tournament, which still exists as the National Association of Intercollegiate Athletics (NAIA) tournament, was organized in 1937. The first national championship for NCAA teams, the National Invitation Tournament (NIT) in New York, was organized in 1938; the NCAA national tournament began one year later. College basketball was rocked by gambling scandals from 1948 to 1951, when dozens of players from top teams were implicated in match fixing and point shaving. Partially spurred by an association with cheating, the NIT lost support to the NCAA tournament.
Before widespread school district consolidation, most American high schools were far smaller than their present-day counterparts. During the first decades of the 20th century, basketball quickly became the ideal interscholastic sport due to its modest equipment and personnel requirements. In the days before widespread television coverage of professional and college sports, the popularity of high school basketball was unrivaled in many parts of America. Perhaps the most legendary of high school teams was Indiana's Franklin Wonder Five, which took the nation by storm during the 1920s, dominating Indiana basketball and earning national recognition.
Today virtually every high school in the United States fields a basketball team in varsity competition. Basketball's popularity remains high, both in rural areas where they carry the identification of the entire community, as well as at some larger schools known for their basketball teams where many players go on to participate at higher levels of competition after graduation. In the 2016–17 season, 980,673 boys and girls represented their schools in interscholastic basketball competition, according to the National Federation of State High School Associations. The states of Illinois, Indiana and Kentucky are particularly well known for their residents' devotion to high school basketball, commonly called Hoosier Hysteria in Indiana; the critically acclaimed film "Hoosiers" shows high school basketball's depth of meaning to these communities.
There is currently no tournament to determine a national high school champion. The most serious effort was the National Interscholastic Basketball Tournament at the University of Chicago from 1917 to 1930. The event was organized by Amos Alonzo Stagg and sent invitations to state champion teams. The tournament started out as a mostly Midwest affair but grew. In 1929 it had 29 state champions. Faced with opposition from the National Federation of State High School Associations and North Central Association of Colleges and Schools that bore a threat of the schools losing their accreditation the last tournament was in 1930. The organizations said they were concerned that the tournament was being used to recruit professional players from the prep ranks. The tournament did not invite minority schools or private/parochial schools.
The National Catholic Interscholastic Basketball Tournament ran from 1924 to 1941 at Loyola University. The National Catholic Invitational Basketball Tournament from 1954 to 1978 played at a series of venues, including Catholic University, Georgetown and George Mason. The National Interscholastic Basketball Tournament for Black High Schools was held from 1929 to 1942 at Hampton Institute. The National Invitational Interscholastic Basketball Tournament was held from 1941 to 1967 starting out at Tuskegee Institute. Following a pause during World War II it resumed at Tennessee State College in Nashville. The basis for the champion dwindled after 1954 when "Brown v. Board of Education" began an integration of schools. The last tournaments were held at Alabama State College from 1964 to 1967.
Teams abounded throughout the 1920s. There were hundreds of men's professional basketball teams in towns and cities all over the United States, and little organization of the professional game. Players jumped from team to team and teams played in armories and smoky dance halls. Leagues came and went. Barnstorming squads such as the Original Celtics and two all-African American teams, the New York Renaissance Five ("Rens") and the (still existing) Harlem Globetrotters played up to two hundred games a year on their national tours.
In 1946, the Basketball Association of America (BAA) was formed. The first game was played in Toronto, Ontario, Canada between the Toronto Huskies and New York Knickerbockers on November 1, 1946. Three seasons later, in 1949, the BAA merged with the National Basketball League (NBL) to form the National Basketball Association (NBA). By the 1950s, basketball had become a major college sport, thus paving the way for a growth of interest in professional basketball. In 1959, a basketball hall of fame was founded in Springfield, Massachusetts, site of the first game. Its rosters include the names of great players, coaches, referees and people who have contributed significantly to the development of the game. The hall of fame has people who have accomplished many goals in their career in basketball. An upstart organization, the American Basketball Association, emerged in 1967 and briefly threatened the NBA's dominance until the ABA-NBA merger in 1976. Today the NBA is the top professional basketball league in the world in terms of popularity, salaries, talent, and level of competition.
The NBA has featured many famous players, including George Mikan, the first dominating "big man"; ball-handling wizard Bob Cousy and defensive genius Bill Russell of the Boston Celtics; charismatic center Wilt Chamberlain, who originally played for the barnstorming Harlem Globetrotters; all-around stars Oscar Robertson and Jerry West; more recent big men Kareem Abdul-Jabbar, Shaquille O'Neal, Hakeem Olajuwon and Karl Malone; playmakers John Stockton, Isiah Thomas and Steve Nash; crowd-pleasing forwards Julius Erving and Charles Barkley; European stars Dirk Nowitzki, Pau Gasol and Tony Parker; more recent superstars LeBron James, Allen Iverson, Kobe Bryant, and Stephen Curry; and the three players who many credit with ushering the professional game to its highest level of popularity during the 1980s and 1990s: Larry Bird, Earvin "Magic" Johnson, and Michael Jordan.
In 2001, the NBA formed a developmental league, the National Basketball Development League (later known as the NBA D-League and then the NBA G League after a branding deal with Gatorade). As of the 2018–19 season, the G League has 27 teams.
FIBA (International Basketball Federation) was formed in 1932 by eight founding nations: Argentina, Czechoslovakia, Greece, Italy, Latvia, Portugal, Romania and Switzerland. At this time, the organization only oversaw amateur players. Its acronym, derived from the French "Fédération Internationale de Basket-ball Amateur", was thus "FIBA". Men's basketball was first included at the Berlin 1936 Summer Olympics, although a demonstration tournament was held in 1904. The United States defeated Canada in the first final, played outdoors. This competition has usually been dominated by the United States, whose team has won all but three titles. The first of these came in a controversial final game in Munich in 1972 against the Soviet Union, in which the ending of the game was replayed three times until the Soviet Union finally came out on top. In 1950 the first FIBA World Championship for men, now known as the FIBA Basketball World Cup, was held in Argentina. Three years later, the first FIBA World Championship for women, now known as the FIBA Women's Basketball World Cup, was held in Chile. Women's basketball was added to the Olympics in 1976, which were held in Montreal, Quebec, Canada with teams such as the Soviet Union, Brazil and Australia rivaling the American squads.
In 1989, FIBA allowed professional NBA players to participate in the Olympics for the first time. Prior to the 1992 Summer Olympics, only European and South American teams were allowed to field professionals in the Olympics. The United States' dominance continued with the introduction of the original Dream Team. In the 2004 Athens Olympics, the United States suffered its first Olympic loss while using professional players, falling to Puerto Rico (in a 19-point loss) and Lithuania in group games, and being eliminated in the semifinals by Argentina. It eventually won the bronze medal defeating Lithuania, finishing behind Argentina and Italy. The Redeem Team, won gold at the 2008 Olympics, and the B-Team, won gold at the 2010 FIBA World Championship in Turkey despite featuring no players from the 2008 squad. The United States continued its dominance as they won gold at the 2012 Olympics, 2014 FIBA World Cup and the 2016 Olympics.
Worldwide, basketball tournaments are held for boys and girls of all age levels. The global popularity of the sport is reflected in the nationalities represented in the NBA. Players from all six inhabited continents currently play in the NBA. Top international players began coming into the NBA in the mid-1990s, including Croatians Dražen Petrović and Toni Kukoč, Serbian Vlade Divac, Lithuanians Arvydas Sabonis and Šarūnas Marčiulionis, Dutchman Rik Smits and German Detlef Schrempf.
In the Philippines, the Philippine Basketball Association's first game was played on April 9, 1975, at the Araneta Coliseum in Cubao, Quezon City, Philippines. It was founded as a "rebellion" of several teams from the now-defunct Manila Industrial and Commercial Athletic Association, which was tightly controlled by the Basketball Association of the Philippines (now defunct), the then-FIBA recognized national association. Nine teams from the MICAA participated in the league's first season that opened on April 9, 1975. The NBL is Australia's pre-eminent men's professional basketball league. The league commenced in 1979, playing a winter season (April–September) and did so until the completion of the 20th season in 1998. The 1998–99 season, which commenced only months later, was the first season after the shift to the current summer season format (October–April). This shift was an attempt to avoid competing directly against Australia's various football codes. It features 8 teams from around Australia and one in New Zealand. A few players including Luc Longley, Andrew Gaze, Shane Heal, Chris Anstey and Andrew Bogut made it big internationally, becoming poster figures for the sport in Australia. The Women's National Basketball League began in 1981."#;
    #[test]
    fn test_basketball() {
        let expected = expect![[r#"
            QWERTY:    13.76% (ok)
            Print:     14.22% (ok)
            Encode:    12.78% (ok)
            Decode:    0.01% (ok)
            Double:    0.00% (ok)
            Roundtrip: 0.14% (ok)
        "#]];
        expected.assert_eq(&m(WIKI_BASKETBALL));
    }
}
