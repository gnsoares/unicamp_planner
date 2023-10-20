use crate::unicamp::{Class, Semester, Slot, Timesheet};
use select::document::Document;
use select::predicate;
use std::collections::HashMap;
use std::{thread, time};

pub fn build_timesheet(
    subjects: &Vec<(String, String)>,
    semester: &Semester,
    credits_map: &mut HashMap<String, u8>,
) -> Timesheet {
    let mut timesheet: HashMap<String, Vec<Class>> = HashMap::new();
    for subject in subjects {
        timesheet.insert(subject.1.clone(), Vec::new());
        thread::sleep(time::Duration::from_millis(500));
        let resp = reqwest::get(
            format!(
                "https://www.dac.unicamp.br/portal/caderno-de-horarios/{}/{}/S/G/{}/{}",
                semester.year, semester.semester, subject.0, subject.1,
            )
            .as_str(),
        )
        .unwrap();
        if !resp.status().is_success() {
            continue;
        }
        let document = Document::from_read(resp).unwrap();
        if !credits_map.contains_key(&subject.1) {
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
            credits_map.insert(subject.1.clone(), credits);
        }
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
            timesheet.get_mut(&subject.1).unwrap().push(slots);
        }
    }
    Timesheet(timesheet)
}
