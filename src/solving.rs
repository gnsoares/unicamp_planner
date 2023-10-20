use crate::unicamp::{Class, Slot, Timesheet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Solution(pub Vec<HashMap<String, Class>>);

pub fn solve(
    ts1: &Timesheet,
    ts2: &Timesheet,
    cmap: &HashMap<String, u8>,
    max_cr: u8,
    ignore: Vec<String>,
) -> Solution {
    let mut cr_count = 0;
    let mut ts = ts1.clone();
    let mut solution = HashMap::new();
    let mut impossible = ignore.clone();

    while let Some(subject) = get_next_subject(&ts, &solution, &impossible) {
        if cr_count >= max_cr {
            break;
        }
        if cr_count + cmap.get(&subject).unwrap() > max_cr {
            impossible.push(subject.clone());
        }
        if let Some(class) =
            ts.0.get(&subject)
                .unwrap()
                .iter()
                .find(|&c| c.0.iter().all(|slot| !does_slot_conflict(slot, &solution)))
        {
            solution.insert(subject.clone(), class.clone());
            cr_count += cmap.get(&subject).unwrap();
        } else {
            impossible.push(subject.clone());
        }
    }
    let mut satisfied = solution.clone().into_keys().collect::<Vec<_>>();
    satisfied.extend(ignore);
    let finished =
        ts.0.keys()
            .filter(|&k| !satisfied.contains(k))
            .collect::<Vec<_>>()
            .is_empty();
    if finished {
        return Solution(vec![solution]);
    }
    let mut semesters = vec![solution];
    semesters.extend(solve(ts2, ts1, cmap, max_cr, satisfied).0);
    Solution(semesters)
}

fn get_next_subject(
    ts: &Timesheet,
    current: &HashMap<String, Class>,
    impossible: &[String],
) -> Option<String> {
    let mut min_values = 999;
    let mut chosen = String::from("");
    for (subject, classes) in ts.0.iter() {
        if classes.is_empty() || current.contains_key(subject) || impossible.contains(subject) {
            continue;
        }
        if classes.len() < min_values {
            min_values = classes.len();
            chosen = subject.clone();
        }
    }
    if chosen.is_empty() {
        return None;
    }
    Some(chosen)
}

fn does_slot_conflict(slot: &Slot, current: &HashMap<String, Class>) -> bool {
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
