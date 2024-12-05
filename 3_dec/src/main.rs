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
    let re = Regex::new(r"mul\((?<left>[0-9]+(\.?[0-9])*),(?<right>[0-9]+(\.?[0-9])*)\)").unwrap();
    
    let sum: f64 = re.captures_iter(&contents)
        .map(|cap| {
            let left = cap["left"].parse::<f64>().unwrap();
            let right = cap["right"].parse::<f64>().unwrap();
            left * right
        })
        .sum();

    println!("Sum: {}", sum);
}
