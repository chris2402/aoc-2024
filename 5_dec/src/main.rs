use load_input::read_file_contents;

type Updates = Vec<usize>;
type Rule = (usize, usize);
struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn from_contents(contents: Vec<&str>) -> Self {
        let rules = contents
            .iter()
            .map(|line| {
                let c: Vec<usize> = line.split("|").map(|x| x.trim().parse().unwrap()).collect();
                assert!(c.len() == 2);
                    
                (c[0], c[1])    

            })
            .collect::<Vec<Rule>>();

        Self { rules }
    }
    
    fn validate(&self, updates: &Updates) -> bool {
        self.broken_rules(updates).len() == 0
    }

    fn broken_rules(&self, updates: &Updates) -> Vec<(usize, bool, Rule)> {
        let mut broken_rules = vec![];

        for (i, elem) in updates.iter().enumerate() {
            let mut iter = updates.iter();

            let before: Vec<usize> = iter
                        .by_ref().take_while(|u| u.ne(&elem)).cloned()
                        .collect();
            
            let after: Vec<usize> = iter.cloned().collect();

            for rule in self.rules.clone()
            {
                match rule {
                    (x,y) if x == *elem => {
                        if before.contains(&&y) {
                            broken_rules.push((i,false,(x,y)));
                        }
                        else {
                            continue;
                        }
                    },
                    
                    (x,y) if *elem == y => {
                        if after.contains(&&x) {
                            broken_rules.push((i, true, (x,y)));
                        }
                        else {
                            continue;
                        }
                    },
                    _ => continue
                }
            }
        }
        
        broken_rules
    }
}

fn get_middle (updates: &Updates) -> usize {
    updates[updates.len()/2]
}


fn main() {
    let contents = read_file_contents("input.txt").unwrap();
    let mut lines = contents.lines();
    let rules: Vec<&str> = lines.by_ref().take_while(|l| l.contains('|')).collect();
    let rules = Rules::from_contents(rules);

    let updates: Vec<Vec<usize>> = lines
        .map(|x| 
            x.split(",").map(|f| 
                f.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect();
    
    let validated = updates.iter().map(|x| (rules.validate(&x), x));
    
    
    let valids = validated.clone().filter(|(x,_)| *x);
    let valid_result = valids.clone()
        .map(|(_,y)| y)
        .cloned()
        .collect::<Vec<_>>();

    let invalids = validated.clone().filter(|(x,_)| !x);

    println!();
    println!("Valid mid-sum Result: {}", valid_result.iter().map(get_middle).sum::<usize>());
    println!();
    println!("Invalids: {}\nValids: {}", invalids.count(), valids.count());
    println!();
}
