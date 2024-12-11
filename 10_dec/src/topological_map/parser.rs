use super::TopologicalMap;

type TopologicalMapInput = String;
pub trait TopologicalMapParser {
    fn parse_topology(&self) -> Result<TopologicalMap, &'static str>;
}

impl TopologicalMapParser for TopologicalMapInput {
    fn parse_topology(&self) -> Result<TopologicalMap, &'static str> {
        
        let no_lines = self.lines().count()    ;
        let no_columns = self.lines().next().unwrap().chars().count();
        
        let mut map = TopologicalMap::new((no_lines, no_columns));
        for (x, line) in self.lines().enumerate() {
            for (y, chr) in line.chars().enumerate() {
                if chr == '.' { continue; }
                map.map[[x, y]] = chr.to_digit(10).unwrap() as isize;
            }
        }

        Ok(map)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn parse_topology_test() {
        let map = INPUT.to_string().parse_topology().unwrap();
        assert_eq!(map.to_map_string(), INPUT.to_string());
    }
    
    #[test]
    fn parse_topology_with_value_test() {
        let map = INPUT.to_string().parse_topology().unwrap();
        assert_eq!(map.get_value((0,1)).unwrap(), 9);
        assert_eq!(map.get_value((0,2)).unwrap(), 0);
        assert_eq!(map.get_value((0,3)).unwrap(), 1);
        assert_eq!(map.get_value((1,2)).unwrap(), 1);
    }
}