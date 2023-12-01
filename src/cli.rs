use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::unicamp::Subject;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    semester: String,
    #[arg(long)]
    subjects_file: String,
    #[arg(long)]
    cr_max: u8,
}

pub fn parse() -> (String, Vec<Subject>, u8, PathBuf) {
    let args = Args::parse();
    let path = Path::new(args.subjects_file.as_str());

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(path).unwrap();
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let subjects = text
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|subject| {
            let mut fc = subject.split(':').take(2).map(|x| x.to_string());
            Subject {
                institute: Box::leak(fc.next().unwrap().into_boxed_str()),
                code: Box::leak(fc.next().unwrap().into_boxed_str()),
            }
        })
        .collect::<Vec<_>>();

    let out_dir = Path::new("data")
        .join("solutions")
        .join(Path::new(args.subjects_file.as_str()).file_stem().unwrap());

    (args.semester, subjects, args.cr_max, out_dir)
}
