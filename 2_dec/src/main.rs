use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

type Level = i32;
type LevelStep = i32;
type Report = Vec<Level>;


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction { Inc, Dec, None }

trait LevelValidity {
    fn direction(&self) -> Direction;
    fn bounded(&self) -> bool;
    fn validate(&self, direction: Direction) -> bool;
}

impl LevelValidity for i32 {
    fn direction(&self) -> Direction {
        match self {
            x if *x > 0 => Direction::Inc,
            x if *x < 0 => Direction::Dec,
            _ => Direction::None,
        }
    }

    fn bounded(&self) -> bool {
        self.abs() > 0 && self.abs() < 4
    }

    fn validate(&self, direction: Direction) -> bool {
        self.bounded() && self.direction() == direction
    }
}

fn read_file_contents(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_reports() -> Result<Vec<Report>, io::Error> {
    let contents = read_file_contents("input.txt")?;
    let reports: Vec<Report> = contents
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.parse::<Level>().unwrap())
                .collect::<Report>()
        })
        .collect();
    Ok(reports)
}

fn find_general_direction(level_steps: &Vec<LevelStep>) -> Direction {
    let mut direction_count_map: HashMap<Direction, usize> = HashMap::new();
    for s in level_steps.iter() {
        let direction = s.direction();
        let count = direction_count_map.entry(direction).or_insert(0);
        *count += 1;
    }
    
    *direction_count_map.iter().max_by_key(|x| x.1).unwrap().0
}

fn validate_report(report: &Report) -> bool {
    let maybe_invalid_step = try_find_invalid_step(report);

    match maybe_invalid_step {
        Some(invalid_step_index) => validate_with_problem_dampening(report, invalid_step_index),
        None => true
    }
}

fn try_find_invalid_step(report: &Vec<i32>) -> Option<usize> {
    let level_steps: Vec<i32> = report.windows(2)
        .map(|window| window[0] - window[1])
        .collect();

    let main_direction = find_general_direction(&level_steps);
    
    let maybe_invalid_step = level_steps
        .iter()
        .position(|x| !x.validate(main_direction));
    
    maybe_invalid_step
}

fn validate_with_problem_dampening(report: &Report, index: usize) -> bool {
    let mut dampened_report = report.clone();
    dampened_report.remove(index);
    
    match try_find_invalid_step(&dampened_report) {
        None => true,
        Some(_) => {
            let mut dampened_report = report.clone();
            dampened_report.remove(index + 1);
            match try_find_invalid_step(&dampened_report){
                None => true,
                Some(_) => false
            }
        },
    }
}

fn main() -> io::Result<()> {
    let reports = get_reports()?;

    let valid_reports  = 
        reports.iter()
            .map(validate_report)
            .filter(|x| *x)
            .count();
    
    println!("Valid Reports {}", valid_reports);
    
    Ok(())
}
