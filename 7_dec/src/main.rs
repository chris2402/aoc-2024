use std::collections::VecDeque;

use load_input::read_file_contents;

fn parse(input: &str) -> Vec<(usize, VecDeque<usize>)> {
    input.lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value: usize = parts.next().unwrap().trim().parse().unwrap();
            let factors_and_terms = parts.next().unwrap().trim().split(" ")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<usize>>()
                .into_iter()
                .collect::<std::collections::VecDeque<_>>();
            (test_value, factors_and_terms)
        })
        .collect()
}

fn main() {
    let input = read_file_contents("input.txt").unwrap();
    
    let equations = parse(&input);

    let result = solve_1(equations);

    println!("Result: {}", result);
}

fn solve_recursive(test_val: usize, stack: &mut VecDeque<usize>) -> Option<usize> {
    let first = stack.pop_front()?;
    
    if first == test_val && stack.is_empty() {
        return Some(test_val);
    } 
    
    let second = stack.pop_front()?;

    if first * second <= test_val {
        let mut stack2 = stack.clone();
        stack2.push_front(first * second);
        match solve_recursive(test_val, &mut stack2) {
            Some(_) => return Some(test_val),
            None => {}
        }
    }

    if first + second <= test_val {
        let mut stack2 = stack.clone();
        stack2.push_front(first + second);
        match solve_recursive(test_val, &mut stack2) {
            Some(_) => return Some(test_val),
            None => {}
        }
    }

    None
}

fn solve_1(input: Vec<(usize, VecDeque<usize>)>) -> usize {
    input.iter()
        .filter_map(|(test_val, stack)|{
            solve_recursive(*test_val, &mut stack.clone())
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = 
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn parse_test() {

        let actual = parse(INPUT);

        let expected = vec![
            (190, VecDeque::from(vec![10, 19])),
            (3267, VecDeque::from(vec![81, 40, 27])),
            (83, VecDeque::from(vec![17, 5])),
            (156, VecDeque::from(vec![15, 6])),
            (7290, VecDeque::from(vec![6, 8, 6, 15])),
            (161011, VecDeque::from(vec![16, 10, 13])),
            (192, VecDeque::from(vec![17, 8, 14])),
            (21037, VecDeque::from(vec![9, 7, 18, 13])),
            (292, VecDeque::from(vec![11, 6, 16, 20])),
        ];

        assert_eq!(expected, actual);
    }

    
    #[test]
    fn deque_test() {

        let mut deque = VecDeque::from(vec![81, 40, 27]);
        assert_eq!(*deque.front().unwrap(), 81);
        assert_eq!(deque.pop_front().unwrap(), 81);
        assert_eq!(deque.pop_front().unwrap(), 40);
        assert_eq!(deque.pop_front().unwrap(), 27);

        let mut deque = VecDeque::from(vec![81, 40, 27]);
        assert_eq!(deque.pop_back().unwrap(), 27);
        assert_eq!(deque.pop_back().unwrap(), 40);
        assert_eq!(deque.pop_back().unwrap(), 81);

        deque.push_back(10);
        deque.push_front(100);
        assert_eq!(deque.pop_back().unwrap(), 10);
        assert_eq!(deque.pop_front().unwrap(), 100);
        
    }

    #[test]
    fn assignment_test() {
        let equations = parse(INPUT);
        let actual = solve_1(equations);

        assert_eq!(3749, actual);
    }
    
}