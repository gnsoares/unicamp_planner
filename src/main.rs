mod cache;
mod cli;
mod scraping;
mod solving;
mod unicamp;

use std::collections::HashMap;
use time::OffsetDateTime;

use crate::scraping::build_timesheet;
use crate::solving::solve;
use crate::unicamp::{Schedule, Semester, Timesheet};

fn get_timesheets_and_credits(
    scrape: bool,
    data_dir: String,
    subjects: Vec<(String, String)>,
    semester: &Semester,
) -> (Timesheet, Timesheet, HashMap<String, u8>) {
    let semester_prev = semester.previous();
    if scrape {
        let mut credits_map: HashMap<String, u8> = HashMap::new();
        let ts1 = build_timesheet(&subjects, &semester_prev, &mut credits_map);
        let ts2 = build_timesheet(&subjects, semester, &mut credits_map);
        cache::save_timesheet(data_dir.as_str(), &ts1, &semester_prev);
        cache::save_timesheet(data_dir.as_str(), &ts2, semester);
        cache::save_credits(data_dir.as_str(), &credits_map);
        return (ts1, ts2, credits_map);
    }
    (
        cache::load_timesheet(data_dir.as_str(), &subjects, &semester_prev),
        cache::load_timesheet(data_dir.as_str(), &subjects, semester),
        cache::load_credits(data_dir.as_str()),
    )
}

fn main() {
    let (scrape, data_dir, subjects, max_cr) = cli::parse();
    let mut semester = Semester::from(OffsetDateTime::now_utc().date());
    let (ts1, ts2, cmap) = get_timesheets_and_credits(scrape, data_dir, subjects, &semester);
    let solution = solve(&ts1, &ts2, &cmap, max_cr, Vec::new());
    for schedule in solution.0 {
        semester = semester.next();
        println!("{}", semester);
        println!("{}", Schedule::from(&schedule));
    }
}
