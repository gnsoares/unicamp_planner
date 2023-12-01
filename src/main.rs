mod cache;
mod cli;
mod scraping;
mod solving;
mod unicamp;

use std::collections::HashMap;
// use time::OffsetDateTime;

use crate::scraping::build_timesheet;
use crate::solving::solve_semester;
use crate::unicamp::{Schedule, Semester, Subject, Timesheet};

fn get_timesheets_and_credits(
    data_dir: &'static str,
    subjects: Vec<Subject>,
    semester: &Semester,
) -> (
    Timesheet<'static>,
    Timesheet<'static>,
    HashMap<&'static str, u8>,
) {
    let semester_prev = semester.previous();
    let mut credits_map: HashMap<&str, u8> = HashMap::new();
    let mut ts1 = build_timesheet(&subjects, semester, &mut credits_map, data_dir);
    let mut ts2 = build_timesheet(&subjects, &semester_prev, &mut credits_map, data_dir);
    ts1.remove_duplicates();
    ts2.remove_duplicates();
    (ts1, ts2, credits_map)
}

fn main() {
    let data_dir: &'static str = "data";
    let (semester_str, subjects, cr_max) = cli::parse();
    // let mut semester = Semester::from(OffsetDateTime::now_utc().date());
    let mut semester = Semester::from(semester_str.as_str());
    let (ts1, ts2, cr_map) = get_timesheets_and_credits(data_dir, subjects, &semester);
    let mut solutions = vec![];
    solve_semester(&ts1, &mut solutions, 0, &cr_map, cr_max);
    // dbg!(&solutions);
    for (i, solution) in solutions.iter().enumerate() {
        println!("SOLUTION {}", i + 1);
        semester = Semester::from(semester_str.as_str()).previous();
        for schedule in solution.schedules.iter() {
            semester = semester.next();
            println!("{}\n{}", semester, Schedule::from(&schedule.table));
        }
    }
}
