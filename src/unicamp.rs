use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use time::{Date, Month};

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

impl fmt::Display for Semester {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}s{}", self.semester, self.year)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Timesheet(pub HashMap<String, Vec<Class>>);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Class(pub Vec<Slot>);

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schedule(pub [[String; 7]; 15]);
impl From<&HashMap<String, Class>> for Schedule {
    fn from(value: &HashMap<String, Class>) -> Self {
        let mut arr: [[String; 7]; 15] = [
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
            [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
        ];
        for weekday in 1..8 {
            for hour in 8..23 {
                for (code, class) in value.iter() {
                    for slot in &class.0 {
                        if slot.weekday == weekday
                            && slot.start / 100 <= hour
                            && slot.finish / 100 > hour
                        {
                            arr[(hour - 8) as usize][(weekday - 1) as usize] = code.clone();
                            break;
                        }
                    }
                }
            }
        }
        Schedule(arr)
    }
}
impl fmt::Display for Schedule {
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
                        && self.0[(hour - 8) as usize][(weekday - 1) as usize]
                            == self.0[(hour - 9) as usize][(weekday - 1) as usize]
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
                        self.0[(hour - 8) as usize][(weekday - 1) as usize]
                    )?;
                    if weekday < 7
                        && self.0[(hour - 8) as usize][(weekday - 1) as usize]
                            == self.0[(hour - 8) as usize][(weekday) as usize]
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
impl Schedule {
    fn get_first_hour(&self) -> Option<u8> {
        for hour in 0..15 {
            if self.0[hour].iter().any(|s| !s.is_empty()) {
                return Some((hour + 8) as u8);
            }
        }
        None
    }
    fn get_last_hour(&self) -> Option<u8> {
        for hour in (0..15).rev() {
            if self.0[hour].iter().any(|s| !s.is_empty()) {
                return Some((hour + 9) as u8);
            }
        }
        None
    }
}
