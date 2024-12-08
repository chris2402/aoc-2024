pub mod antinode_locator;
pub mod point_calculations;

use std::collections::{
    HashMap,
    HashSet
};

type AntennaType = char;
type Coordinates = (usize, usize);
type Grid = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Antenna(AntennaType)
}

#[derive(Debug)]
pub struct AntennaMap {
    grid: Grid,
    unique_types: HashSet<AntennaType>,
    locations: HashMap<AntennaType, Vec<Coordinates>>
}

impl AntennaMap {
    pub fn parse(input: &str) -> Result<AntennaMap, &'static str> {
        let mut grid: Grid = Vec::new();
        let mut unique_types: HashSet<AntennaType> = HashSet::new();
        let mut locations: HashMap<AntennaType, Vec<Coordinates>> = HashMap::new();

        // Assert all lines are the same length
        input.lines().map(|l| Some(l.len()))
            .reduce(|a, b| {
                if a == b { a } else { None }
            }).ok_or("All lines must be the same length")?;

        for (line_i, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (char_i, char) in line.chars().enumerate() {
                let tile = match char {
                    '.' => Tile::Empty,
                    _ => {
                        unique_types.insert(char);
                        locations.entry(char).or_insert(vec![]).push((line_i, char_i));
                        Tile::Antenna(char)
                    }
                };

                row.push(tile);
            }
            grid.push(row);
        }

        Ok(AntennaMap {
            grid,
            unique_types,
            locations
        })
    }
    
    fn create_with(size: (usize, usize), locations: HashMap<AntennaType, Vec<Coordinates>>) -> Result<AntennaMap, &'static str> {
        let mut grid: Grid = vec![vec![Tile::Empty; size.1]; size.0];
        let unique_types: HashSet<AntennaType> = locations.keys().cloned().collect();

        let (max_x, max_y) = match size {
            (x, y) => (x - 1, y - 1)
        };

        for (antenna_type, locations) in locations.iter() {
            for location in locations {
                let (loc_x, loc_y) = location;
                
                if loc_x > &max_x || loc_y > &max_y {
                    return Err("Location out of bounds");
                }

                grid[location.0][location.1] = Tile::Antenna(*antenna_type);
            }
        }

        Ok(AntennaMap {
            grid,
            unique_types,
            locations
        })
    }

    fn is_within_bounds(&self, point: &(isize, isize)) -> bool {
        let (x, y) = point;
        (*x >= 0 && *y >= 0) && ((*x as usize) < self.grid.len() && (*y as usize) < self.grid[0].len())
    }
}



#[cfg(test)]
mod tests {
    use antinode_locator::AntiNodeLocator;

    use super::*;

    static INPUT : &str =
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse() {
        let map = AntennaMap::parse(INPUT).unwrap();
        assert_eq!(map.grid.len(), 12);
        assert_eq!(map.grid[0].len(), 12);
        assert_eq!(map.unique_types, ['0', 'A'].iter().cloned().collect());
        assert_eq!(map.locations.get(&'0').unwrap(), &[(1, 8), (2, 5), (3, 7), (4, 4)]);
    }

    
    #[test]
    fn test_find_antinodes() {
        let map = AntennaMap::parse(INPUT).unwrap();
        
        let no_antinodes = map.find_antinodes_all();
        

        assert_eq!(no_antinodes, vec![
            (0,6),
            (0,11),
            (1,3),
            (2,4),
            (2,10),
            (3,2),
            (4,9),
            (5,1),
            (5,6),
            (6,3),
            (7,0),
            (7,7),
            (10,10),
            (11,10),
        ].into_iter().collect::<HashSet<_>>());
    }
}