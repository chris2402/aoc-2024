mod stones;

use load_input::read_file_contents;
use stones::alignment::StoneAlignment;
use stones::parse::StoneAlignmentParse;

fn main() {
    let input = read_file_contents("input.txt").unwrap();
    let mut aligment = input.parse_stones();
    let result = assignment(&mut aligment);

    println!("Result: {}", result);
}

fn assignment(aligment: &mut StoneAlignment) -> usize {
    for _ in 0..25{
        aligment.blink();
    }

    aligment.as_vec().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_assignment() {
        let mut aligment = INPUT.parse_stones();
        let result = assignment(&mut aligment);
        assert_eq!(result, 55312);
    }
}