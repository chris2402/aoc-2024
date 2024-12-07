use crate::rules::model::Rule;
use crate::rules::validation::{RuleIndex, RuleIndexer};
use std::collections::{HashMap, HashSet};

pub type Updates = Vec<usize>;

#[derive(PartialEq, Debug)]
enum ValidationSample {
    Valid((Rule, RuleIndex)),
    Invalid((Rule, RuleIndex)),
}

pub struct UpdateValidationResult {
    rule_map: Vec<ValidationSample>
}

impl UpdateValidationResult {
    pub fn is_valid(&self) -> bool {
        self.rule_map.iter().all(|v| match v {
            ValidationSample::Valid(_) => true,
            _ => false
        })
    }

    pub fn rules(&self) -> Vec<Rule> {
        self.rule_map.iter().map(|v| match v {
            ValidationSample::Valid((r, _)) => r.clone(),
            ValidationSample::Invalid((r, _)) => r.clone()
        }).collect()
    }

    pub fn valid_iter(&self) -> impl Iterator<Item = &ValidationSample> {
        self.rule_map.iter().filter(|v| match v {
            ValidationSample::Valid(_) => true,
            _ => false
        })
    }

    pub fn invalid_iter(&self) -> impl Iterator<Item = &ValidationSample> {
        self.rule_map.iter().filter(|v| match v {
            ValidationSample::Invalid(_) => true,
            _ => false
        })
    }
}

pub trait UpdateValidator {
    fn validate(&self, rules: &Vec<Rule>) -> UpdateValidationResult;
}

impl UpdateValidator for Updates {
    fn validate(&self, rules: &Vec<Rule>) -> UpdateValidationResult {
        let mut rule_map = Vec::new();

        for r in rules {
            match self.rule_indexes(r){
                RuleIndex::Both(x, y) => {
                    rule_map.push(match x < y {
                        true => ValidationSample::Valid((r.clone(), RuleIndex::Both(x, y))),
                        false => ValidationSample::Invalid((r.clone(), RuleIndex::Both(x, y)))
                    });
                },
                _ => continue
            }
        }

        UpdateValidationResult { rule_map }
    }
}

pub trait RuleSolver {
    fn solve(&self, rules: &Vec<Rule>) -> Vec<usize>;
}

impl RuleSolver for Updates {
    fn solve(&self, rules: &Vec<Rule>) -> Vec<usize> {
        let mut solved: Vec<usize> = Vec::new();
         
        let mut rules = self.validate(rules).rules();
        let ys: HashSet<usize> = rules.iter().map(|r| r.y).collect();
        while rules.len() > 0 {
            let xs: HashSet<usize> = rules.iter().map(|r| r.x).collect();
        
            let y_counts: HashMap<usize, usize> = xs.iter().map(|x| {
                let count = rules.iter().filter(|r| r.y == *x).count();
                (*x, count)
            }).collect();
            
            let x_only= *y_counts.iter().find(|(_, count)| **count==0).unwrap().0;
            solved.push(x_only);
            rules = rules.iter().filter(|r| r.x != x_only).cloned().collect();
        }
        
        let solved_set: HashSet<usize> = solved.iter().cloned().collect();

        let remaining_y = ys.difference(&solved_set).next().unwrap();
        solved.push(*remaining_y);
        
        solved
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_validator() {
        let updates = vec![1, 2, 3];
        let rule = &vec![
            Rule::new(1, 2),
            Rule::new(3, 2),
        ];

        let result = updates.validate(rule);

        let actual = result.valid_iter().next().unwrap();
        let expected = ValidationSample::Valid((Rule::new(1, 2), RuleIndex::Both(0, 1)));
        assert_eq!(actual, &expected);
    }

    #[test]
    fn test_validation_result() {
        let updates = vec![1, 2, 3];
        let rule = &vec![
            Rule::new(1, 2),
            Rule::new(3, 2),
        ];

        let result = updates.validate(rule);

        assert!(!result.is_valid());
    }

    
    #[test]
    fn test_get_active_rules() {
        let updates = vec![1, 2, 3];
        let rule = &vec![
            Rule::new(1, 2),
            Rule::new(3, 2),
            Rule::new(0, 2),
            Rule::new(3, 7),
        ];

        let result = updates.validate(rule).rules();

        assert!(result.contains(&Rule::new(1, 2)));
        assert!(result.contains(&Rule::new(3, 2)));
    }

    #[test]
    fn test_rules_solver() {
        let updates = vec![1, 2, 3, 4, 5];
        let rules = &vec![
            Rule::new(1, 2),
            Rule::new(1, 3),
            Rule::new(3, 2),
            Rule::new(2, 4),
            Rule::new(3, 7),
            Rule::new(4, 5),
        ];

        let result = updates.solve(&rules);

        assert_eq!(result, vec![1, 3, 2, 4, 5]);
    }
}