use crate::unicamp::{Class, Slot, Timesheet};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Schedule<'a> {
    pub table: HashMap<&'a str, Class>,
    pub impossible: Vec<&'a str>,
    pub finished: bool,
    pub cr_count: u8,
}

#[derive(Clone, Debug)]
pub struct Solution<'a> {
    pub schedules: Vec<Schedule<'a>>,
    // pub satisfied: Vec<&'a str>,
}

impl<'a> Solution<'a> {
    pub fn finished(&self) -> bool {
        self.schedules.iter().all(|sc| sc.finished)
    }
    pub fn satisfied(&self) -> Vec<&'a str> {
        let mut codes: Vec<&str> = vec![];
        for sc in self.schedules.iter() {
            for code in sc.table.keys() {
                if !codes.contains(code) {
                    codes.push(*code);
                }
            }
        }
        codes
    }
}

// pub fn solve_all<'a>(
//     ts1: &Timesheet<'a>,
//     ts2: &Timesheet<'a>,
//     cr_map: &HashMap<&'a str, u8>,
//     cr_max: u8,
//     ignore: Vec<&'a str>,
// ) -> Vec<Solution<'a>> {
// }

pub fn solve_semester<'a>(
    ts: &Timesheet<'a>,
    solutions: &mut Vec<Solution<'a>>,
    schedule_idx: usize,
    cr_map: &HashMap<&'a str, u8>,
    cr_max: u8,
) {
    if solutions.is_empty() {
        let fsub = get_first_subject(ts).unwrap();
        for c in ts.table.get(fsub).unwrap() {
            solutions.push(Solution {
                schedules: vec![Schedule {
                    table: HashMap::from([(fsub, c.clone())]),
                    impossible: vec![fsub],
                    cr_count: *cr_map.get(fsub).unwrap(),
                    finished: false,
                }],
            });
        }
    } else if solutions[0].schedules.len() == schedule_idx {
        for sol in solutions.iter_mut() {
            let schedule = Schedule {
                table: HashMap::new(),
                impossible: sol.satisfied(),
                cr_count: 0,
                finished: false,
            };
            sol.schedules.push(schedule);
        }
    }
    if solutions.iter().all(|sol| sol.finished()) {
        return;
    }
    let mut copies = vec![];
    for sol in solutions.iter_mut().filter(|s| !s.finished()) {
        let sc = &mut sol.schedules[schedule_idx];
        if let Some(subject) = get_next_subject(ts, &sc.table, cr_map, sc.cr_count, cr_max) {
            let classes = ts
                .table
                .get(subject)
                .unwrap()
                .iter()
                .filter(|c| c.0.iter().all(|slot| !does_slot_conflict(slot, &sc.table)))
                .collect::<Vec<_>>();
            sc.cr_count += cr_map.get(subject).unwrap();
            sc.table.insert(subject, classes[0].clone());
            for c in classes.iter().skip(1) {
                let mut sol_copy = sol.clone();
                let sc_copy = &mut sol_copy.schedules[schedule_idx];
                sc_copy.table.insert(subject, (*c).clone());
                copies.push(sol_copy);
            }
        } else {
            sc.finished = true;
        }
    }
    for copy in copies {
        solutions.push(copy);
    }
    solve_semester(ts, solutions, schedule_idx, cr_map, cr_max);
}

// pub fn solve_greedy<'a>(
//     ts1: &Timesheet<'a>,
//     ts2: &Timesheet<'a>,
//     cr_map: &HashMap<&'a str, u8>,
//     cr_max: u8,
//     ignore: Vec<&'a str>,
// ) -> Solution<'a> {
//     let mut cr_count = 0;
//     let ts = ts1.clone();
//     let mut solution = HashMap::new();
//     let mut impossible = ignore.clone();

//     while let Some(subject) = get_next_subject(&ts, &solution, &impossible) {
//         if cr_count >= cr_max {
//             break;
//         }
//         if cr_count + cr_map.get(&subject).unwrap() > cr_max {
//             impossible.push(subject);
//         }
//         if let Some(class) = ts
//             .table
//             .get(&subject)
//             .unwrap()
//             .iter()
//             .find(|&c| c.0.iter().all(|slot| !does_slot_conflict(slot, &solution)))
//         {
//             solution.insert(subject, class.clone());
//             cr_count += cr_map.get(&subject).unwrap();
//         } else {
//             impossible.push(subject);
//         }
//     }
//     let mut satisfied = solution.clone().into_keys().collect::<Vec<_>>();
//     satisfied.extend(ignore);
//     let finished = ts
//         .table
//         .keys()
//         .filter(|&k| !satisfied.contains(k))
//         .collect::<Vec<_>>()
//         .is_empty();
//     if finished {
//         return Solution {
//             schedules: vec![solution],
//         };
//     }
//     let mut semesters = vec![solution];
//     semesters.extend(solve_greedy(ts2, ts1, cr_map, cr_max, satisfied).schedules);
//     Solution {
//         schedules: semesters,
//     }
// }

fn get_first_subject<'a>(ts: &Timesheet<'a>) -> Option<&'a str> {
    let mut min_values = 999;
    let mut chosen = "";
    for (subject, classes) in ts.table.iter() {
        if classes.is_empty() {
            continue;
        }
        if classes.len() < min_values {
            min_values = classes.len();
            chosen = subject;
        }
    }
    if chosen.is_empty() {
        return None;
    }
    Some(chosen)
}

fn get_next_subject<'a>(
    ts: &Timesheet<'a>,
    current: &HashMap<&'a str, Class>,
    cr_map: &HashMap<&'a str, u8>,
    cr_count: u8,
    cr_max: u8,
) -> Option<&'a str> {
    let mut min_values = 999;
    let mut chosen = "";
    for (subject, classes) in ts.table.iter() {
        if cr_count + cr_map.get(subject).unwrap() > cr_max {
            continue;
        }
        let classes_filt = classes
            .iter()
            .filter(|c| c.0.iter().all(|slot| !does_slot_conflict(slot, current)))
            .collect::<Vec<_>>();
        if classes_filt.is_empty() || current.contains_key(*subject) {
            continue;
        }
        if classes_filt.len() < min_values {
            min_values = classes_filt.len();
            chosen = subject;
        }
    }
    if chosen.is_empty() {
        return None;
    }
    Some(chosen)
}

fn does_slot_conflict(slot: &Slot, current: &HashMap<&str, Class>) -> bool {
    for (_, class) in current.iter() {
        for slot_other in &class.0 {
            if slot.weekday == slot_other.weekday
                && slot.start < slot_other.finish
                && slot.finish > slot_other.start
            {
                return true;
            }
        }
    }
    false
}
