use piechart::{Chart, Color, Data};
use serde::{Deserialize, Serialize};
use std::io::{self, BufReader};

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    tags: Vec<String>,
    time: f64, // in seconds
}

#[derive(Debug, Serialize, Deserialize)]
struct Projects {
    projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeSpan {
    from: String,
    to: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Report {
    projects: Vec<Project>,
    time: f64, // overall time
    timespan: TimeSpan,
}
impl Report {
    pub fn sort_projects(&mut self) {
        self.projects
            .sort_unstable_by(|a, b| b.time.partial_cmp(&a.time).unwrap());
    }
}

fn main() {
    let stdin = io::stdin();
    let f = BufReader::new(stdin);
    let mut report: Report = serde_json::from_reader(f).unwrap();

    report.sort_projects();

    let colors = vec![
        Color::Red,
        Color::Green,
        Color::Purple,
        Color::Yellow,
        Color::Blue,
        Color::Cyan,
    ];

    let chars_to_use = vec!['*', '.', ',', '•', '▪'];

    let mut char_loop = chars_to_use.iter().cycle();

    let mut data = Vec::new();
    let mut last_index = 0;
    for (index, color) in colors.iter().enumerate() {
        if let Some(project) = report.projects.get(index) {
            last_index = index;
            data.push(Data {
                label: project.name.clone().into(),
                value: project.time as f32,
                color: Some(color.clone()),
                fill: char_loop.next().unwrap().clone(),
            });
        }
    }

    // if there is only one remaining project, we can give it white
    if (last_index + 1) == report.projects.len() - 1 {
        let last_project = report.projects.last().unwrap();
        data.push(Data {
            label: last_project.name.clone().into(),
            value: last_project.time as f32,
            color: Some(Color::White),
            fill: char_loop.next().unwrap().clone(),
        });
    } else {
        let remaining_total: f64 = report.projects[(last_index + 1)..]
            .iter()
            .map(|r| r.time)
            .sum();

        data.push(Data {
            label: "other".into(),
            value: remaining_total as f32,
            color: Some(Color::White),
            fill: char_loop.next().unwrap().clone(),
        });
    }

    Chart::new()
        .radius(12)
        .aspect_ratio(4)
        .legend(true)
        .draw(&data);
    println!();
}
