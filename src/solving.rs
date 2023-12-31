use itertools::Itertools;

use crate::unicamp::{Class, Slot, Timesheet};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct ScheduleInProgress<'a> {
    pub table: HashMap<&'a str, Class>,
    pub finished: bool,
    pub cr_count: u8,
}

#[derive(Clone, Debug)]
struct SolutionInProgress<'a> {
    pub schedules: Vec<ScheduleInProgress<'a>>,
    pub satisfied: Vec<&'a str>,
    pub goal: u8,
}

impl<'a> SolutionInProgress<'a> {
    pub fn finished(&self) -> bool {
        self.schedules.iter().all(|sc| sc.finished)
    }
    pub fn solved(&self) -> bool {
        self.satisfied.len() as u8 == self.goal
    }
}

#[derive(Clone, Debug)]
pub struct Schedule<'a> {
    pub table: HashMap<&'a str, Class>,
    pub cr_count: u8,
    pub score: f32,
}

#[derive(Clone, Debug)]
pub struct Solution<'a> {
    pub schedules: Vec<Schedule<'a>>,
    pub score: f32,
}

pub fn solve_all<'a>(
    ts1: &Timesheet<'a>,
    ts2: &Timesheet<'a>,
    cr_map: &HashMap<&'a str, u8>,
    cr_max: u8,
) -> Vec<Solution<'a>> {
    let mut solutions: Vec<SolutionInProgress<'_>> = vec![];
    let mut subjects = vec![];
    for ts in [ts1, ts2] {
        for subject in ts.table.keys() {
            if !subjects.contains(subject) {
                subjects.push(*subject);
            }
        }
    }
    let mut schedule_idx = 0;
    loop {
        let in_progress = solutions
            .iter()
            .filter(|sol| !sol.solved())
            .collect::<Vec<_>>();
        if !solutions.is_empty() && in_progress.is_empty() {
            break;
        }
        if schedule_idx % 2 == 0 {
            solve_semester(ts1, &mut solutions, &subjects, cr_map, cr_max, schedule_idx);
        } else {
            solve_semester(ts2, &mut solutions, &subjects, cr_map, cr_max, schedule_idx);
        }
        schedule_idx += 1;
    }
    let mut solutions = solutions
        .iter()
        .map(|sol| Solution {
            schedules: sol
                .schedules
                .iter()
                .map(|sc| Schedule {
                    table: sc.table.to_owned(),
                    score: evaluate_solution_semester(sc),
                    cr_count: sc.cr_count,
                })
                .collect_vec(),
            score: 0f32,
        })
        .collect_vec();
    for sol in solutions.iter_mut() {
        sol.score =
            sol.schedules.iter().map(|sc| sc.score).sum::<f32>() / sol.schedules.len() as f32;
    }
    solutions
}

fn solve_semester<'a>(
    ts: &Timesheet<'a>,
    solutions: &mut Vec<SolutionInProgress<'a>>,
    subjects: &Vec<&'a str>,
    cr_map: &HashMap<&'a str, u8>,
    cr_max: u8,
    schedule_idx: usize,
) {
    if solutions.is_empty() {
        let fsub = get_first_subject(ts).unwrap();
        for c in ts.table.get(fsub).unwrap() {
            solutions.push(SolutionInProgress {
                schedules: vec![ScheduleInProgress {
                    table: HashMap::from([(fsub, c.clone())]),
                    cr_count: *cr_map.get(fsub).unwrap(),
                    finished: false,
                }],
                satisfied: vec![fsub],
                goal: subjects.len() as u8,
            });
            println!("New solution spawned (1 satisfied)");
        }
    } else if solutions[0].schedules.len() == schedule_idx {
        for sol in solutions.iter_mut() {
            let schedule = ScheduleInProgress {
                table: HashMap::new(),
                cr_count: 0,
                finished: false,
            };
            sol.schedules.push(schedule);
        }
    }
    if solutions.iter().all(|sol| sol.finished()) {
        println!(
            "All solutions finished semester {}. {}/{} solved.",
            schedule_idx + 1,
            solutions.iter().filter(|sol| sol.solved()).count(),
            solutions.len()
        );
        return;
    }
    let mut copies = vec![];
    for sol in solutions.iter_mut().filter(|s| !s.finished()) {
        let sc = &mut sol.schedules[schedule_idx];
        if let Some(subject) =
            get_next_subject(ts, &sol.satisfied, &sc.table, cr_map, sc.cr_count, cr_max)
        {
            let classes = ts
                .table
                .get(subject)
                .unwrap()
                .iter()
                .filter(|c| c.0.iter().all(|slot| !does_slot_conflict(slot, &sc.table)))
                .collect::<Vec<_>>();
            sol.satisfied.push(subject);
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
        println!("New solution spawned ({} satisfied)", copy.satisfied.len());
        solutions.push(copy);
    }
    solve_semester(ts, solutions, subjects, cr_map, cr_max, schedule_idx);
}

// pub fn solve_greedy<'a>(
//     ts1: &Timesheet<'a>,
//     ts2: &Timesheet<'a>,
//     cr_map: &HashMap<&'a str, u8>,
//     cr_max: u8,
//     ignore: Vec<&'a str>,
// ) -> SolutionInProgress<'a> {
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
//         return SolutionInProgress {
//             schedules: vec![solution],
//         };
//     }
//     let mut schedules = vec![solution];
//     schedules.extend(solve_greedy(ts2, ts1, cr_map, cr_max, satisfied).schedules);
//     SolutionInProgress {
//         schedules: schedules,
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
    satisfied: &[&'a str],
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
        if classes_filt.is_empty() || current.contains_key(*subject) || satisfied.contains(subject)
        {
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

fn evaluate_solution_semester(semester: &ScheduleInProgress) -> f32 {
    let mut points = vec![];
    for (_, cl) in semester.table.iter() {
        for sl in cl.0.iter() {
            for i in (sl.start / 100)..(sl.finish / 100) {
                points.push((sl.weekday as f32, i as f32));
            }
        }
    }
    let centroid = points.iter().fold((0f32, 0f32), |sum, (x, y)| {
        (
            sum.0 + x / points.len() as f32,
            sum.1 + y / points.len() as f32,
        )
    });
    let score = (points.len() as f32)
        / points
            .iter()
            .fold(0f32, |sum, (x, y)| {
                sum + ((centroid.0 - x).powf(2f32) + (centroid.1 - y).powf(2f32)).sqrt()
                    / points.len() as f32
            })
            .powf(2f32);
    if score.is_finite() {
        return score;
    }
    0f32
}
