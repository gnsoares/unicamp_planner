use crate::unicamp::{Class, Slot, Timesheet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Solution<'a> {
    #[serde(borrow)]
    pub schedules: Vec<HashMap<&'a str, Class>>,
}

pub fn solve<'a>(
    ts1: &Timesheet<'a>,
    ts2: &Timesheet<'a>,
    cmap: &HashMap<&'a str, u8>,
    max_cr: u8,
    ignore: Vec<&'a str>,
) -> Solution<'a> {
    let mut cr_count = 0;
    let ts = ts1.clone();
    let mut solution = HashMap::new();
    let mut impossible = ignore.clone();

    while let Some(subject) = get_next_subject(&ts, &solution, &impossible) {
        if cr_count >= max_cr {
            break;
        }
        if cr_count + cmap.get(&subject).unwrap() > max_cr {
            impossible.push(subject);
        }
        if let Some(class) = ts
            .table
            .get(&subject)
            .unwrap()
            .iter()
            .find(|&c| c.0.iter().all(|slot| !does_slot_conflict(slot, &solution)))
        {
            solution.insert(subject, class.clone());
            cr_count += cmap.get(&subject).unwrap();
        } else {
            impossible.push(subject);
        }
    }
    let mut satisfied = solution.clone().into_keys().collect::<Vec<_>>();
    satisfied.extend(ignore);
    let finished = ts
        .table
        .keys()
        .filter(|&k| !satisfied.contains(k))
        .collect::<Vec<_>>()
        .is_empty();
    if finished {
        return Solution {
            schedules: vec![solution],
        };
    }
    let mut semesters = vec![solution];
    semesters.extend(solve(ts2, ts1, cmap, max_cr, satisfied).schedules);
    Solution {
        schedules: semesters,
    }
}

fn get_next_subject<'a>(
    ts: &Timesheet<'a>,
    current: &HashMap<&'a str, Class>,
    impossible: &[&'a str],
) -> Option<&'a str> {
    let mut min_values = 999;
    let mut chosen = "";
    for (subject, classes) in ts.table.iter() {
        if classes.is_empty() || current.contains_key(*subject) || impossible.contains(subject) {
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
