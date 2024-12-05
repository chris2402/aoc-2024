use std::fs::File;
use std::io::{Read, Result};
use regex::Regex;



fn read_file_contents(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


fn main() {
    let contents = read_file_contents("input.txt").unwrap();
    let re = Regex::new(r"(?ms)(?:\A|do\(\))(?:.*?)(?:don't\(\)|\z)").unwrap();

    let sum: f64 = re.captures_iter(&contents)
        .map(|cap| {
            let re = Regex::new(r"(?ms)mul\((?P<left>[0-9]+),(?P<right>[0-9]+)\)").unwrap();    
            let inner_cap = cap.get(0).unwrap().as_str();
            re.captures_iter(inner_cap)
                .map(|cap| {
                    let left = cap["left"].parse::<f64>().unwrap();
                    let right = cap["right"].parse::<f64>().unwrap();
                    left * right
                })
                .sum::<f64>()      
        }).sum();

    println!("Sum: {}", sum);
}
