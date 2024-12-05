use std::fs;


fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let (mut left,mut right) = parse_contents(contents);

    left.sort();
    right.sort();

    let result: i32 = compute_diff(left, right);

    println!("{}", result);
}

trait Joinable {
    fn join(&self, sep: &str) -> String;
}

impl Joinable for Vec<i32> {
    fn join(&self, sep: &str) -> String {
        self.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(sep)
    }
}

fn parse_contents(contents: String) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in contents.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }

    (left, right)
}

fn compute_diff(left: Vec<i32>, right: Vec<i32>) -> i32 {
    left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l-r).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;


    #[test]
    fn test_parse_contents() {
        let contents = (
"56208   95668
52621   74203
95252   33335
79799   26047"
        ).to_string();
        
        let (left, right) = parse_contents(contents);

        assert_eq!(left, vec![56208 ,52621 ,95252 ,79799]);
        assert_eq!(right, vec![95668 ,74203 ,33335 ,26047]);
    }



    #[parameterized(left = {
        vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![1, 2, 3, 4]
    }, right = {
        vec![1, 2, 3, 4], vec![4, 3, 2, 1], vec![10, 2, 3, 4]
    }, result = {
        0, 0, 9
    })]
    fn test_compute_diff(left: Vec<i32>, right: Vec<i32>, result: i32) {
        let result = compute_diff(left, right);

        assert_eq!(result, result);
    }

}