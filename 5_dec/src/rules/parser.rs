use crate::rules::model::Rule;

pub trait RuleParser {
    fn parse_rules(&self) -> Option<Vec<Rule>>;
    fn parse_rule(&self) -> Option<Rule>;
}

impl RuleParser for &str {
    fn parse_rules(&self) -> Option<Vec<Rule>> {
        let rule_candidates = self.lines().map(|x: &str| x.parse_rule());
        
        let mut rules: Vec<Rule> = vec![];
        for maybe_rule in rule_candidates {
            match maybe_rule {
                None => return None,
                Some(r) => rules.push(r)
                
            } 
        }

        Some(rules)
    }

    fn parse_rule(&self) -> Option<Rule> {
        let mut x_y = self
            .split("|")
            .map(|number| number.trim().parse::<usize>());

        let x = match x_y.next(){
            Some(Ok(x)) => x,
            _ => return None
        };
        let y = match x_y.next(){
            Some(Ok(y)) => y,
            _ => return None
        };
        
        // Should be empty now!
        assert_eq!(x_y.next(), None);

        Some(Rule::new(x, y))
    }
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let rule = "1|2".parse_rule().unwrap();
        assert_eq!(rule, Rule::new(1, 2));
    }

    #[test]
    fn test_parse_rules() {
        let input = format!("{}\n{}\n{}", "1|2", "3|4", "5|6");
        
        match input.as_str().parse_rules(){
            None => panic!("Failed to parse rules"),
            Some(rules) => {
                assert_eq!(rules.len(), 3, "Input should have 3 rules, but found {}", rules.len());
                assert_eq!(rules[0], Rule::new(1, 2), "First rule should be 1|2, but found {:?}", rules[0]);
                assert_eq!(rules[1], Rule::new(3, 4), "Second rule should be 3|4, but found {:?}", rules[1]);
                assert_eq!(rules[2], Rule::new(5, 6), "Third rule should be 5|6, but found {:?}", rules[2]);
            }
        }
    }

    
}