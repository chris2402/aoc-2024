mod topological_map;
use topological_map::locator::{TrailHeadLocator, TrailLocator};
use topological_map::parser::TopologicalMapParser;


// use topological_map::parser::TopologicalMapParser;
use load_input::read_file_contents;
fn main() {
    let input = read_file_contents("input.txt").unwrap();
    let result = assignment(input);
    println!("Result: {}", result);
}

fn assignment(input: String) -> usize {
    let map = input.parse_topology().unwrap();
    let trail_heads = map.get_trail_heads();
    
    trail_heads.into_iter().map(|trail_head| {
        map.find_tops_from(trail_head).len()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use topological_map::TopologicalMap;

    // TODO: Memoize the nodes in the map when finding the paths

    pub static INPUT: &str = 
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    pub fn get_map() -> TopologicalMap {  
        INPUT.to_string().parse_topology().unwrap()
    }


    #[test]
    fn test_assignment() {
        assert_eq!(assignment(INPUT.to_string()), 36);
    }
}