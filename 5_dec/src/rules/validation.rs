use crate::rules::model::Rule;


#[derive(PartialEq, Debug)]
pub enum RuleIndex{
    None,
    X(usize),
    Y(usize),
    Both(usize, usize)
}

pub trait RuleIndexer {
    fn rule_indexes(&self, rule: &Rule) -> RuleIndex;
}

impl RuleIndexer for Vec<usize> {
    fn rule_indexes(&self, rule: &Rule) -> RuleIndex {
        match (self.iter().position(|&r| r == rule.x), self.iter().position(|&r| r == rule.y)) {
            (Some(x_i), Some(y_i)) => RuleIndex::Both(x_i, y_i),
            (Some(x_i), None) => RuleIndex::X(x_i),
            (None, Some(y_i)) => RuleIndex::Y(y_i),
            _ => RuleIndex::None
        }
    }
}

impl Rule {

    pub fn test_compliance(&self, updates: &Vec<usize>) -> bool {
        match updates.rule_indexes(self) {
            RuleIndex::Both(x_i, y_i) if x_i > y_i => false,
            _ => true
        }
    }

    // pub fn try_find_index(&self, updates: &Vec<usize>) -> bool {
    //     match updates.rule_indexes(self) {
    //         RuleIndex::Both(x_i, y_i) if x_i < y_i => true,
    //         _ => false
    //     }
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_index() {
        let rule = Rule::new(3, 2);
        let updates = vec![1, 2, 3];
        assert_eq!(updates.rule_indexes(&rule), RuleIndex::Both(2, 1));
    }

    #[test]
    fn test_rule_complies() {
        let rule = Rule::new(1, 2);
        let updates = vec![1, 2, 3];
        assert!(rule.test_compliance(&updates));
    }
    
    #[test]
    fn test_rule_not_complies() {
        let rule = Rule::new(3, 2);
        let updates = vec![1, 2, 3];
        assert!(!rule.test_compliance(&updates));
    }
}