use crate::cache::{load_classes, load_credits, save_classes, save_credits};
use crate::unicamp::{Class, Semester, Slot, Subject, Timesheet};
use select::document::Document;
use select::predicate;
use std::collections::HashMap;
use std::{thread, time};

pub fn build_timesheet<'a>(
    subjects: &Vec<Subject>,
    semester: &Semester,
    credits_map: &mut HashMap<&'a str, u8>,
    cache_dir: &'a str,
) -> Timesheet<'static> {
    let mut table: HashMap<&str, Vec<Class>> = HashMap::new();
    let mut scrape_classes: bool;
    let mut scrape_credits: bool;
    for subject in subjects {
        scrape_classes = false;
        scrape_credits = false;
        if let Some(classes) = load_classes(cache_dir, subject, semester) {
            table.insert(subject.code, classes);
        } else {
            scrape_classes = true;
            println!(
                "No cached classes for subject {} in semester {}. Scraping...",
                subject.code, semester
            );
        }
        if let Some(credits) = load_credits(cache_dir, subject) {
            credits_map.insert(subject.code, credits);
        } else {
            scrape_credits = true;
            println!(
                "No cached credits for subject {} in semester {}. Scraping...",
                subject.code, semester
            );
        }
        if scrape_classes || scrape_credits {
            thread::sleep(time::Duration::from_millis(500));
            let resp = reqwest::get(
                format!(
                    "https://www.dac.unicamp.br/portal/caderno-de-horarios/{}/{}/S/G/{}/{}",
                    semester.year, semester.semester, subject.institute, subject.code,
                )
                .as_str(),
            )
            .unwrap();
            if !resp.status().is_success() {
                if scrape_classes {
                    table.insert(subject.code, Vec::new());
                    save_classes(
                        cache_dir,
                        subject,
                        semester,
                        table.get(subject.code).unwrap(),
                    );
                }
                continue;
            }
            let document = Document::from_read(resp).unwrap();
            if scrape_classes {
                table.insert(subject.code, Vec::new());
                for turma in document
                    .find(predicate::Class("turma"))
                    .flat_map(|x| x.find(predicate::Class("panel-body")))
                    .flat_map(|x| x.find(predicate::Class("horariosFormatado")))
                {
                    let slots = Class(
                        turma
                            .find(predicate::Name("li"))
                            .map(|x| {
                                Slot::new(
                                    x.find(predicate::Class("diaSemana")).next().unwrap().text(),
                                    x.find(predicate::Class("horarios")).next().unwrap().text(),
                                )
                            })
                            .collect::<Vec<_>>(),
                    );
                    table.get_mut(subject.code).unwrap().push(slots);
                }
                save_classes(
                    cache_dir,
                    subject,
                    semester,
                    table.get(subject.code).unwrap(),
                );
            }
            if scrape_credits && !credits_map.contains_key(subject.code) {
                let credits = document
                    .find(predicate::Class("prop"))
                    .find(|x| x.text() == "Créditos:")
                    .unwrap()
                    .next() // space
                    .unwrap()
                    .next() // span with value
                    .unwrap()
                    .text() // value
                    .trim()
                    .parse::<u8>()
                    .unwrap();
                credits_map.insert(subject.code, credits);
                save_credits(cache_dir, subject, credits);
            }
        }
    }
    Timesheet { table }
}
