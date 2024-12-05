use std::{collections::{HashMap, HashSet}, fs};


fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let (mut left,mut right) = parse_contents(contents);

    left.sort();
    right.sort();

    let sim_count = compute_similarity_count(&left, &right);

    let result: i32 = compute_diff(left, right);


    println!("{}", sim_count);
}

fn compute_similarity_count(left: &Vec<i32>, right: &Vec<i32>) -> i32{
    let right_map = count_occurrences(right);

    let mut similarity_count = 0;
    for l in left {
        let count = right_map.get(l).unwrap_or(&0);
        similarity_count += (l * count);
    }

    similarity_count
}

fn count_occurrences(list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for val in list.iter() {
        let count = count_map.entry(*val).or_insert(0);
        *count += 1;
    }

    count_map
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

    #[test]
    fn test_count_occurences() {
        let right = vec![1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let result = count_occurrences(&right);

        assert_eq!(result.get(&1), Some(&3));
        assert_eq!(result.get(&2), Some(&3));
        assert_eq!(result.get(&3), Some(&3));
        assert_eq!(result.get(&4), Some(&3));
    }

    #[test]
    fn test_compute_similarity_count() {
        let left = vec![3 ,4 ,2 ,1 ,3 ,3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let result = compute_similarity_count(&left, &right);

        assert_eq!(result, 31);
    }
}