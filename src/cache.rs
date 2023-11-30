use serde::de::DeserializeOwned;

use crate::unicamp::{Class, Semester, Subject};
use std::cmp::Eq;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub fn load_classes(dir: &str, subject: &Subject, semester: &Semester) -> Option<Vec<Class>> {
    let path = Path::new(dir).join(Path::new((subject.code.to_owned() + ".yaml").as_str()));
    let cached: HashMap<String, Vec<Class>> = load_yaml(&path).unwrap_or(HashMap::new());
    cached
        .get(&semester.to_string())
        .map(|classes| classes.to_vec())
}

pub fn load_credits(dir: &str, subject: &Subject) -> Option<u8> {
    let path = Path::new(dir).join(Path::new("credits.yaml"));
    let cached: HashMap<String, u8> = load_yaml(&path).unwrap_or(HashMap::new());
    cached.get(subject.code).copied()
}

fn load_yaml<K: DeserializeOwned + Eq + Hash, V: DeserializeOwned>(
    path: &PathBuf,
) -> Option<HashMap<K, V>> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            return None;
        }
    };
    let mut text = String::new();
    if file.read_to_string(&mut text).is_err() {
        return None;
    };
    serde_yaml::from_str(&text).unwrap_or(None)
}

pub fn save_credits(dir: &str, subject: &Subject, credits: u8) {
    let path = Path::new(dir).join(Path::new("credits.yaml"));
    let mut cached: HashMap<String, u8> = load_yaml(&path).unwrap_or(HashMap::new());
    if !cached.contains_key(subject.code) {
        cached.insert(subject.code.to_string(), credits);
        File::create(path)
            .unwrap()
            .write_all(serde_yaml::to_string(&cached).unwrap().as_bytes())
            .unwrap();
    }
}

pub fn save_classes(dir: &str, subject: &Subject, semester: &Semester, classes: &[Class]) {
    let path = Path::new(dir).join(Path::new((subject.code.to_owned() + ".yaml").as_str()));
    let mut cached: HashMap<String, Vec<Class>> = load_yaml(&path).unwrap_or(HashMap::new());
    cached.insert(semester.to_string(), classes.to_owned());
    File::create(&path)
        .unwrap()
        .write_all(serde_yaml::to_string(&cached).unwrap().as_bytes())
        .unwrap();
}
