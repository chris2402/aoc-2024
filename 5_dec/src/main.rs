mod rules {
    pub mod model;
    pub mod validation;
    pub mod parser;
}

mod updates {
    pub mod model;
}

use load_input::read_file_contents;
use rules::parser::RuleParser;
use rules::model::Rule;
use updates::model::{
    Updates,
    UpdateValidator,
    RuleSolver
};

fn parse_content(content: &str) -> Result<(Vec<Rule>, Vec<Updates>), &str> {
    let mut content_iter = content.split("\r\n\r\n").flat_map(|s| s.split("\n\n"));
    
    let rules = match content_iter.next() {
        Some(rule_content) => match rule_content.parse_rules(){
            Some(rules) => rules,
            _ => return Err("Error parsing rules")
        },
        _ => return Err("Error finding rule content")   
    };
    
    let updates: Vec<Updates> = match content_iter.next(){

        Some(updates) => {
            updates
                .lines()
                .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
                .collect()
        },
        _ => return Err("Failed to find updates")
    };

    Ok((rules, updates))
}

fn get_middle (updates: &Updates) -> usize {
    updates[updates.len()/2]
}


fn main() {
    let contents = read_file_contents("input.txt").unwrap();
    let (rules, updates) = parse_content(&contents).unwrap();

    let valid_result = task_1(&rules, &updates);
    let soved_result = task_2(&rules, &updates);
    
    println!("Valid mid-sum Result: {}", valid_result.iter().map(get_middle).sum::<usize>());
    println!("Solved mid-sum Result: {}", soved_result.iter().map(get_middle).sum::<usize>());
}

fn task_2(rules: &Vec<Rule>, updates: &Vec<Updates>) -> Vec<Vec<usize>> {
    updates.iter().filter(|ue| !ue.validate(rules).is_valid())
        .map(|ue| ue.solve(rules))
        .collect()
}

fn task_1(rules: &Vec<Rule>, updates: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let validated = updates.iter().map(|update_entry| {
        let is_valid = rules.iter().all(|r| r.test_compliance(update_entry));
        (is_valid, update_entry)
    });
    
    let valids = validated.clone().filter(|(x,_)| *x);
    let valid_result = valids.clone()
        .map(|(_,y)| y)
        .cloned()
        .collect::<Vec<_>>();

    valid_result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn content_parse() {
        let content = 
"1|2
3|4
5|6

1,2,3,4,5
5,4,3,2,1";

        let (rule, updates) = parse_content(content).unwrap();

        assert_eq!(rule, vec![Rule::new(1, 2), Rule::new(3, 4), Rule::new(5, 6)]);
        assert_eq!(updates, vec![vec![1,2,3,4,5], vec![5,4,3,2,1]]);
    }

    #[test]
    fn task1_test() {
        let content = 
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let (rules, updates) = parse_content(content).unwrap();
        let valid_result = task_1(&rules, &updates);

        
        assert_eq!(valid_result.iter().map(get_middle).sum::<usize>(), 143);
        assert_eq!(valid_result.len(), 3);
    }

    #[test]
    fn task2_test() {
        let content = 
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,97,47,61,53
61,13,29
97,13,75,29,47";

        let (rules, updates) = parse_content(content).unwrap();
        let valid_result = task_2(&rules, &updates);

        
        assert_eq!(valid_result.iter().map(get_middle).sum::<usize>(), 123);
        assert_eq!(valid_result.len(), 3);
    }
    
}