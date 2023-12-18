mod cache;
mod cli;
mod scraping;
mod solving;
mod unicamp;

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;
// use time::OffsetDateTime;

use crate::scraping::build_timesheet;
use crate::solving::solve_all;
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
    let (semester_str, subjects, cr_max, out_dir) = cli::parse();
    // let mut semester = Semester::from(OffsetDateTime::now_utc().date());
    let mut semester = Semester::from(semester_str.as_str());
    let (ts1, ts2, cr_map) = get_timesheets_and_credits(data_dir, subjects, &semester);
    let mut solutions = solve_all(&ts1, &ts2, &cr_map, cr_max);
    solutions.sort_by(|a, b| a.score.total_cmp(&b.score));
    create_dir_all(&out_dir).unwrap();
    for (i, solution) in solutions.iter().rev().take(5).enumerate() {
        let mut file = File::create(out_dir.join(format!("solution_{}.txt", i + 1)))
            .expect("Could not open solution file");
        semester = Semester::from(semester_str.as_str()).previous();
        file.write_all(format!("Score: {}\n", solution.score).as_bytes())
            .expect("Error while writing solution to file");
        for schedule in solution.schedules.iter() {
            semester = semester.next();
            file.write_all(
                format!(
                    "{} ({} credits)\n{}",
                    semester,
                    schedule.cr_count,
                    Schedule::from(&schedule.table)
                )
                .as_bytes(),
            )
            .expect("Error while writing solution to file");
        }
    }
}
