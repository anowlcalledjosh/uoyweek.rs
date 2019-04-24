use chrono::prelude::*;
use chrono_tz::{Europe::London, Tz};

macro_rules! term {
    // "to" the day on which term ends (usually a Friday).
    ($name:ident, $sy:literal-$sm:literal-$sd:literal to $ey:literal-$em:literal-$ed:literal) => {{
        let start = London.ymd($sy, $sm, $sd).and_hms(0, 0, 0);
        let end = London.ymd($ey, $em, $ed).succ().and_hms(0, 0, 0);
        Term {
            name: $name,
            start,
            end,
        }
    }};
}

macro_rules! terms {
    ($($name:ident: $sy:literal-$sm:literal-$sd:literal to $ey:literal-$em:literal-$ed:literal),+$(,)?) => {
        [
            $(
                term!($name, $sy-$sm-$sd to $ey-$em-$ed)
            ),+
        ]
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TermName {
    Autumn,
    Spring,
    Summer,
}
use TermName::*;

impl TermName {
    fn shortname(&self) -> &'static str {
        match self {
            Autumn => "Aut",
            Spring => "Spr",
            Summer => "Sum",
        }
    }
    fn longname(&self) -> &'static str {
        match self {
            Autumn => "Autumn",
            Spring => "Spring",
            Summer => "Summer",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Term {
    name: TermName,
    /// The first instant of the term.
    start: DateTime<Tz>,
    /// The instant after the term ends.
    end: DateTime<Tz>,
}

impl Term {
    fn name(&self) -> TermName {
        self.name
    }
    fn start(&self) -> DateTime<Tz> {
        self.start
    }
    fn end(&self) -> DateTime<Tz> {
        self.end
    }
    /// Returns `s`, where `s <= start && s.weekday() == Mon`.
    fn loose_start(&self) -> DateTime<Tz> {
        let mut s = self.start.date();
        while s.weekday() != Weekday::Mon {
            s = s.pred();
        }
        s.and_hms(0, 0, 0)
    }
    /// Returns `e`, where `e >= end && e.weekday() == Mon`.
    fn loose_end(&self) -> DateTime<Tz> {
        let mut e = self.end.date();
        while e.weekday() != Weekday::Mon {
            e = e.succ();
        }
        e.and_hms(0, 0, 0)
    }
}

fn get_term(terms: &[Term], now: DateTime<Tz>) -> Option<&Term> {
    terms
        .iter()
        .filter(|&term| term.loose_start() <= now && now <= term.loose_end())
        .last()
}

fn get_strict_term(terms: &[Term], now: DateTime<Tz>) -> Option<&Term> {
    terms
        .iter()
        .filter(|&term| term.start() <= now && now <= term.end())
        .last()
}

fn main() {
    // <https://www.york.ac.uk/about/term-dates/>
    let mut terms = terms!(
        // 2018-19
        Autumn: 2018-09-24 to 2018-11-30,
        Spring: 2019-01-07 to 2019-03-15,
        Summer: 2019-04-15 to 2019-06-21,
        // 2019-20
        Autumn: 2019-09-30 to 2019-12-06,
        Spring: 2020-01-06 to 2020-03-13,
        Summer: 2020-04-14 to 2020-06-19,
        // 2020-21
        Autumn: 2020-09-28 to 2020-12-03,
        Spring: 2021-01-11 to 2021-03-19,
        Summer: 2021-04-19 to 2021-06-25,
        // 2021-22
        Autumn: 2021-09-27 to 2021-12-03,
        Spring: 2022-01-10 to 2022-03-18,
        Summer: 2022-04-19 to 2022-06-24,
        // 2022-23
        Autumn: 2022-09-26 to 2022-12-02,
        Spring: 2023-01-09 to 2023-03-17,
        Summer: 2023-04-17 to 2023-06-23,
        // 2023-24
        Autumn: 2023-09-25 to 2023-12-01,
        Spring: 2024-01-08 to 2024-03-15,
        Summer: 2024-04-15 to 2024-06-21,
        // 2024-25
        Autumn: 2024-09-23 to 2024-11-29,
        Spring: 2025-01-06 to 2024-03-14,
        Summer: 2025-04-22 to 2025-06-27,
        // 2025-26
        Autumn: 2025-09-29 to 2025-12-05,
        Spring: 2026-01-12 to 2026-03-20,
        Summer: 2026-04-20 to 2026-06-26,
        // 2026-27
        Autumn: 2026-09-28 to 2026-12-04,
        Spring: 2027-01-11 to 2027-03-19,
        Summer: 2027-04-19 to 2027-06-25,
        // 2027-28
        Autumn: 2027-09-27 to 2027-12-03,
        Spring: 2028-01-10 to 2028-03-17,
        Summer: 2028-04-24 to 2028-06-30,
    );
    terms.sort_unstable_by_key(|term| term.start());
    let terms = terms;
    let now = London.from_utc_datetime(&Utc::now().naive_utc());
    if let Some(term) = get_term(&terms, now) {
        let termname = term.name().shortname();
        // FIXME: this is broken when going between years (e.g. before the first term of the year
        // starts) -- fix when euclidean_division is stable
        // (<https://github.com/rust-lang/rust/issues/49048>)
        // OTOH, it shouldn't actually matter -- there's no term that runs between two years.
        let weeknum = now.iso_week().week() as i32 - term.start().iso_week().week() as i32 + 1;
        let day = now.format("%a");
        if let Some(strict_term) = get_strict_term(&terms, now) {
            assert_eq!(strict_term, term);
            // We're in real term.
            println!("{}/{}/{}", termname, weeknum, day);
        } else {
            // We're not in real term (i.e. another part of this week is real term).
            println!("({}/{}/{})", termname, weeknum, day);
        }
    } else {
        // Giving a term date would be nonsensical.
        println!("n/a");
    }
}
