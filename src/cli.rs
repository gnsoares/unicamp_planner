use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    scrape: bool,
    #[arg(short, long)]
    data_dir: String,
    #[arg(long)]
    subjects_file: String,
    #[arg(long)]
    max_cr: u8,
}

pub fn parse() -> (bool, String, Vec<(String, String)>, u8) {
    let args = Args::parse();
    let path = Path::new(args.subjects_file.as_str());
    let mut subjects = Vec::new();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(path).unwrap();
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    for subject in text.split('\n') {
        let fc = subject.split(':').take(2).collect::<Vec<&str>>();
        if fc.len() == 2 {
            subjects.push((fc[0].to_string(), fc[1].to_string()));
        }
    }

    (args.scrape, args.data_dir, subjects, args.max_cr)
}
