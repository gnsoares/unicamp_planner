use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use time::{Date, Month};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    pub code: &'static str,
    pub institute: &'static str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Semester {
    pub year: u16,
    pub semester: u8,
}

impl Semester {
    pub const fn next(&self) -> Self {
        if self.semester == 1 {
            Semester {
                year: self.year,
                semester: 2,
            }
        } else {
            Semester {
                year: self.year + 1,
                semester: 1,
            }
        }
    }

    pub const fn previous(&self) -> Self {
        if self.semester == 1 {
            Semester {
                year: self.year - 1,
                semester: 2,
            }
        } else {
            Semester {
                year: self.year,
                semester: 1,
            }
        }
    }
}

impl From<Date> for Semester {
    fn from(dt: Date) -> Self {
        Semester {
            year: dt.year() as u16,
            semester: if (dt.month() as u8) < (Month::August as u8) {
                1
            } else {
                2
            },
        }
    }
}

impl From<&str> for Semester {
    fn from(s: &str) -> Self {
        let mut ss = s.split('s');
        Semester {
            semester: ss.next().unwrap().parse().unwrap(),
            year: ss.next().unwrap().parse().unwrap(),
        }
    }
}

impl fmt::Display for Semester {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}s{}", self.semester, self.year)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Timesheet<'a> {
    #[serde(borrow)]
    pub table: HashMap<&'a str, Vec<Class>>,
}

impl<'a> Timesheet<'a> {
    pub fn remove_duplicates(&mut self) {
        let mut to_remove;
        for (_, classes) in self.table.iter_mut() {
            to_remove = vec![];
            for idxs in (0..classes.len()).combinations(2) {
                let i = idxs[0];
                let j = idxs[1];
                if to_remove.contains(&j) {
                    continue;
                }
                if classes[i] == classes[j] {
                    to_remove.push(j);
                }
            }
            to_remove.sort();
            to_remove.reverse();
            for i in to_remove {
                classes.remove(i);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Class(pub Vec<Slot>);

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        let mut slots_self = self.0.clone();
        let mut slots_other = other.0.clone();
        if slots_self.len() != slots_other.len() {
            return false;
        }
        slots_self.sort();
        slots_other.sort();
        for (si, sj) in std::iter::zip(slots_self, slots_other) {
            if si != sj {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Slot {
    pub weekday: u8,
    pub start: u16,
    pub finish: u16,
}

impl Slot {
    pub fn new(weekday_pt: String, duration: String) -> Self {
        let duration_parsed = duration
            .split('-')
            .map(|s| {
                s.split(':')
                    .map(|s| s.trim().parse::<u16>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Slot {
            weekday: match weekday_pt.as_str() {
                "Domingo" => 1,
                "Segunda" => 2,
                "Terça" => 3,
                "Quarta" => 4,
                "Quinta" => 5,
                "Sexta" => 6,
                "Sábado" => 7,
                s => panic!("Unexpected portuguese weekday {}", s),
            },
            start: duration_parsed[0][0] * 100 + duration_parsed[0][1],
            finish: duration_parsed[1][0] * 100 + duration_parsed[1][1],
        }
    }
}

impl PartialEq for Slot {
    fn eq(&self, other: &Self) -> bool {
        self.weekday == other.weekday && self.start == other.start && self.finish == other.finish
    }
}

impl Eq for Slot {}

impl PartialOrd for Slot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Slot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.weekday.cmp(&other.weekday) {
            std::cmp::Ordering::Equal => match self.start.cmp(&other.start) {
                std::cmp::Ordering::Equal => self.finish.cmp(&other.finish),
                x => x,
            },
            x => x,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schedule<'a> {
    #[serde(borrow)]
    pub table: [[&'a str; 7]; 15],
}
impl<'a> From<&HashMap<&'a str, Class>> for Schedule<'a> {
    fn from(value: &HashMap<&'a str, Class>) -> Self {
        let mut table: [[&str; 7]; 15] = [
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
            ["", "", "", "", "", "", ""],
        ];
        for weekday in 1..8 {
            for hour in 8..23 {
                for (code, class) in value.iter() {
                    for slot in &class.0 {
                        if slot.weekday == weekday
                            && slot.start / 100 <= hour
                            && slot.finish / 100 > hour
                        {
                            table[(hour - 8) as usize][(weekday - 1) as usize] = code;
                            break;
                        }
                    }
                }
            }
        }
        Schedule { table }
    }
}
impl<'a> fmt::Display for Schedule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "+-------------+---------+---------+---------+---------+---------+---------+---------+"
        )?;
        writeln!(
            f,
            "|             | Domingo | Segunda |  Terça  | Quarta  | Quinta  |  Sexta  | Sábado  |"
        )?;
        if let Some(first_hour) = self.get_first_hour() {
            for hour in first_hour..self.get_last_hour().unwrap() {
                write!(f, "+-------------+")?;
                for weekday in 1..8 {
                    if hour > first_hour
                        && self.table[(hour - 8) as usize][(weekday - 1) as usize]
                            == self.table[(hour - 9) as usize][(weekday - 1) as usize]
                    {
                        write!(f, "         +")?;
                    } else {
                        write!(f, "---------+")?;
                    }
                }
                writeln!(f)?;
                write!(f, "| {:0>2}:00-{:0>2}:00 |", hour, hour + 1)?;
                for weekday in 1..8 {
                    write!(
                        f,
                        "{: ^9}",
                        self.table[(hour - 8) as usize][(weekday - 1) as usize]
                    )?;
                    if weekday < 7
                        && self.table[(hour - 8) as usize][(weekday - 1) as usize]
                            == self.table[(hour - 8) as usize][(weekday) as usize]
                    {
                        write!(f, " ")?;
                    } else {
                        write!(f, "|")?;
                    }
                }
                writeln!(f)?;
            }
        }
        writeln!(
            f,
            "+-------------+---------+---------+---------+---------+---------+---------+---------+"
        )
    }
}
impl<'a> Schedule<'a> {
    fn get_first_hour(&self) -> Option<u8> {
        for hour in 0..15 {
            if self.table[hour].iter().any(|s| !s.is_empty()) {
                return Some((hour + 8) as u8);
            }
        }
        None
    }
    fn get_last_hour(&self) -> Option<u8> {
        for hour in (0..15).rev() {
            if self.table[hour].iter().any(|s| !s.is_empty()) {
                return Some((hour + 9) as u8);
            }
        }
        None
    }
}
