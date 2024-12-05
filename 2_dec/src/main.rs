use std::fs::File;
use std::io::{self, Read};

type Level = i32;
type Report = Vec<Level>;

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

fn validate_report(report: &Report) -> bool {
    let differences: Vec<i32> = report.windows(2)
        .map(|window| window[1] - window[0])
        .collect();
    
    let within_threshold: Vec<bool> = differences.iter().map(|&x| x.abs() > 0 && x.abs() < 4).collect();
    let within_threshold = within_threshold.iter().all(|x| *x);
    
    let all_inc: Vec<bool> = 
        differences.iter().map(|&x| x < 0).collect();
    
    let all_dec: Vec<bool> = 
        differences.iter().map(|&x| x > 0).collect();
    let all_inc_or_dec = all_inc.iter().all(|x| *x) || all_dec.iter().all(|x| *x);
    
    within_threshold && all_inc_or_dec
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
