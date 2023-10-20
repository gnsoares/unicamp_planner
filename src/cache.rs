use crate::unicamp::{Class, Semester, Timesheet};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn load_credits(dir: &str) -> HashMap<String, u8> {
    let path = Path::new(dir).join(Path::new("credits.yaml"));
    let mut file = File::open(path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    serde_yaml::from_str(&text).unwrap()
}

pub fn load_timesheet(
    dir: &str,
    subjects: &Vec<(String, String)>,
    semester: &Semester,
) -> Timesheet {
    let path = Path::new(dir);
    let key = &semester.to_string();
    let mut map = HashMap::new();
    for (_, subject) in subjects {
        let subject_path = path.join(Path::new((subject.clone() + ".yaml").as_str()));
        let saved: HashMap<String, Vec<Class>> = {
            let mut file = File::open(&subject_path).unwrap();
            let mut text = String::new();
            file.read_to_string(&mut text).unwrap();
            serde_yaml::from_str(&text).unwrap()
        };
        if let Some(classes) = saved.get(key) {
            map.insert(subject.clone(), classes.clone());
        }
    }
    Timesheet(map)
}

pub fn save_credits(dir: &str, credits: &HashMap<String, u8>) {
    let path = Path::new(dir).join(Path::new("credits.yaml"));
    let mut saved: HashMap<String, u8> = {
        let mut file = File::open(&path).unwrap();
        let mut text = String::new();
        file.read_to_string(&mut text).unwrap();
        serde_yaml::from_str(&text).unwrap()
    };
    for (code, subject_credits) in credits.iter() {
        if !saved.contains_key(code) {
            saved.insert(code.clone(), *subject_credits);
        }
    }
    File::create(path)
        .unwrap()
        .write_all(serde_yaml::to_string(&saved).unwrap().as_bytes())
        .unwrap();
}

pub fn save_timesheet(dir: &str, timesheet: &Timesheet, semester: &Semester) {
    let path = Path::new(dir);
    let map = &timesheet.0;
    for (subject, classes) in map.iter() {
        let subject_path = path.join(Path::new((subject.clone() + ".yaml").as_str()));
        let mut saved: HashMap<String, Vec<Class>> = if subject_path.exists() {
            let mut file = File::open(&subject_path).unwrap();
            let mut text = String::new();
            file.read_to_string(&mut text).unwrap();
            serde_yaml::from_str(&text).unwrap()
        } else {
            HashMap::new()
        };
        saved.insert(semester.to_string().clone(), classes.clone());
        File::create(&subject_path)
            .unwrap()
            .write_all(serde_yaml::to_string(&saved).unwrap().as_bytes())
            .unwrap();
    }
}
