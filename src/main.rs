mod cache;
mod cli;
mod scraping;
mod solving;
mod unicamp;

use std::collections::HashMap;
// use time::OffsetDateTime;

use crate::scraping::build_timesheet;
use crate::solving::solve;
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
    let ts1 = build_timesheet(&subjects, semester, &mut credits_map, data_dir);
    let ts2 = build_timesheet(&subjects, &semester_prev, &mut credits_map, data_dir);
    (ts1, ts2, credits_map)
}

fn main() {
    let data_dir: &'static str = "data";
    let (semester_str, subjects, max_cr) = cli::parse();
    // let mut semester = Semester::from(OffsetDateTime::now_utc().date());
    let mut semester = Semester::from(semester_str.as_str());
    let (ts1, ts2, cmap) = get_timesheets_and_credits(data_dir, subjects, &semester);
    let solution = solve(&ts1, &ts2, &cmap, max_cr, Vec::new());
    semester = semester.previous();
    for schedule in solution.schedules {
        semester = semester.next();
        println!("{}", semester);
        println!("{}", Schedule::from(&schedule));
    }
}
