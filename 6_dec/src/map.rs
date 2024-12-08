use std::{collections::{HashMap, HashSet}, isize};

use crate::guard::{
    self, Direction, Guard
};

#[derive(Clone, PartialEq, Debug)]
pub enum Object {
    Guard(Guard),
    Wall,
    Empty,
}

type Position = (isize, isize);

#[derive(Clone, PartialEq, Debug)]
pub struct Map {
    objects: Vec<Vec<Object>>,
    guard_pos: Position,
}

impl Map {
    pub fn parse_and_place(input: &str, position: &Position) -> Map {
        let mut map = Map::parse(input);
        map.set(*position, Object::Wall);
        map
    }

    pub fn parse(input: &str) -> Map {
            let mut guard_pos: Position = (0, 0);
        let objects = input.lines().enumerate().map(|(line_i, line)| {
            line.chars().enumerate().map(|(char_i, char)| {
                const GUARDS: [char;4] = ['v', '^', '<', '>'];
                const WALLS: char = '#';
                const EMPTY: char = '.';
    
                match char {
                    WALLS => Object::Wall,
                    EMPTY => Object::Empty,
                    g if GUARDS.contains(&g) => {
                        let direction = match &g {
                            'v' => Direction::Down,
                            '^' => Direction::Up,
                            '<' => Direction::Left,
                            '>' => Direction::Right,
                            _ => panic!("Invalid guard direction"),
                        };
                        let guard = Guard::new(direction);
                        guard_pos = (line_i as isize, char_i as isize);
                        Object::Guard(guard)
                    },
                    _ => panic!("Invalid character in map"),
                }
            }).collect()
        }).collect();

        Map {
            objects,
            guard_pos
        }
    }

    fn set (&mut self, position: Position, object: Object) -> Result<(), String>  {
        let x = if position.0 >= 0 { 
            position.0 as usize
        } else {
            return Err(format!("Invalid 'x' position: '{}'", position.0));
        };
        let y = if position.1 >= 0 { 
            position.1 as usize
        } else {
            return Err(format!("Invalid 'y' position: '{}'", position.1));
        };

        if x < self.objects.len() && y < self.objects[x].len() {
            self.objects[x][y] = object;
            Ok(())
        } else {
            Err(format!("Position out of bounds: ({}, {})", x, y))
        }
    }

    fn get(&self, position: &Position) -> Option<&Object> {
        let x = if position.0 >= 0 { 
            position.0 as usize
        } else {
            return None;
        };
        let y = if position.1 >= 0 { 
            position.1 as usize
        } else {
            return None;
        };

        let chars = &self.objects.get(x)?;
        let obj = chars.get(y)?;
        Some(obj)
    }
    

    fn get_guard(&self) -> &Guard {
        match self.get(&self.guard_pos) {
            Some(Object::Guard(ref guard)) => guard,
            _ => panic!("No guard at guard assumed position!"),
        }
    }

    fn get_guard_pos(&self) -> Position {
        self.guard_pos
    }


    fn check_valid_position(&self, position: &Position) -> bool {
        let line_range = (0 as isize, self.objects.len() as isize);
        let char_range = (0 as isize, self.objects[0].len() as isize);

        let x = position.0;
        let y = position.1;

        x >= line_range.0 && char_range.0 >= 0 && x < line_range.1 && y < char_range.1
    }

    fn move_guard(&mut self, new_pos: Position) -> Result<(), String> {
        let guard_tile = Object::Guard(self.get_guard().clone());
        let guard_post = self.guard_pos;

        self.set(new_pos, guard_tile)?;
        self.set(guard_post, Object::Empty)?;
        self.guard_pos = new_pos;
        Ok(())
    }

    fn is_valid_guard_position(&self) -> bool {
        self.check_valid_position(&self.guard_pos)
    }
    
    // Returns Err True if guard is stuck in a loop
    pub fn solve(&mut self) -> Result<HashMap<Position, Vec<Direction>>, bool> {
        let mut path: HashMap<Position, Vec<Direction>> = HashMap::new();
        
        let mut guard = self.get_guard().clone();
        path.entry(self.guard_pos).or_insert(vec![]).push(guard.direction().clone());

        while self.is_valid_guard_position() {

            let guard_pos: Position = self.guard_pos;
            
            let new_pos = guard.move_ahead(&guard_pos);
            match self.get(&new_pos) {
                Some(Object::Wall) => {
                    guard.turn_right();
                },
                Some(Object::Empty) => {
                    if !self.check_valid_position(&new_pos) {
                        panic!("Guard not in ");
                    }

                    self.move_guard(new_pos).unwrap();
                },
                Some(Object::Guard(_)) => panic!("Guard already at new position!"), 
                None => break,
            };

            if path.entry(self.guard_pos).or_default().contains(guard.direction()) {
                return Err(true);
            }

            path.entry(self.guard_pos).or_insert(vec![]).push(guard.direction().clone());
        }

        Ok(path)
    }

    pub fn solve_with_loop_placement(&self) -> HashSet<Position> {
        let original_map = self.clone();
        let mut map = original_map.clone();

        let illegal_pos = map.get_guard_pos();
        let guard_path = map.solve().unwrap();

        guard_path.iter().flat_map(|(pos, dirs)|{
            let possible_placements: HashSet<_> = dirs.iter().map(|dir| {
                let new_pos = Guard::new(dir.clone()).move_ahead(&pos);
                if new_pos == illegal_pos {
                    return None;
                }
                Some(new_pos)
            }).filter_map(|x| x).collect();

            possible_placements.iter().map(|new_pos| {
                let mut map = original_map.clone();
                let new_pos = new_pos.clone();

                match map.set(new_pos, Object::Wall){
                    Ok(_) => {},
                    Err(_) => return None,
                }
                
                match map.solve() {
                    Ok(_) => None,
                    Err(x) if x => Some(new_pos),
                    _ => panic!("Unexpected error"),
                }
            }).filter_map(|x| x).collect::<Vec<_>>()
        }).collect::<HashSet<_>>()
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::map;

    use super::*;

    #[test]
    fn test_parse() {
        let input = 
"#..
#.#
#^#";
        let map = Map::parse(input);
        assert_eq!(map.objects.len(), 3);
        assert_eq!(map.objects[0].len(), 3);
        assert_eq!(map.guard_pos, (2, 1));

        assert_eq!(map.objects[0][0], Object::Wall);
        assert_eq!(map.objects[0][1], Object::Empty);
        assert_eq!(map.objects[0][2], Object::Empty);
        assert_eq!(map.objects[1][0], Object::Wall);
        assert_eq!(map.objects[1][1], Object::Empty);
        assert_eq!(map.objects[1][2], Object::Wall);
        assert_eq!(map.objects[2][0], Object::Wall);
        assert_eq!(map.objects[2][1], Object::Guard(Guard::new(Direction::Up)));
        assert_eq!(map.objects[2][2], Object::Wall);
    }

    #[test]
    fn test_get_guard() {
        let input = 
"#..
#.#
#^#";
        let map = Map::parse(input);
        let guard = map.get_guard();

        assert_eq!(guard.direction(), &Direction::Up);
        assert_eq!(map.guard_pos, (2, 1));
    }

    
    #[test]
    fn test_solve() {
        let input = 
"####
#..#
#^.#";
        let mut map = Map::parse(input);
        let guard_path = map.solve().unwrap();

        let mut expected_path = HashMap::new();
        expected_path.insert((2, 1), vec![Direction::Up]);
        expected_path.insert((1, 1), vec![Direction::Up, Direction::Right]);
        expected_path.insert((1, 2), vec![Direction::Right, Direction::Down]);
        expected_path.insert((2, 2), vec![Direction::Down]);

        assert_eq!(guard_path, expected_path);
    }

    
    #[test]
    fn test_loop_placement() {
        let input = 
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let mut map = Map::parse_and_place(input, &(6,3));

        assert!(map.solve().is_err());
    }

    
    // Find all previous positions where:
    // 1. Is in the same direction that the guard is facing now
    // 2. Has same direction 
    // 3. Is not illegal_pos (where the guard was placed)
    
    // Find a place the guard has been 2 times;
    // if the last time he passed, he turns right instead, 
    // then he is then facing the same direction as first time - it is a loop
    #[test]
    fn test_find_placements() {
        let input = 
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let map = Map::parse(input);

        let actual = map.solve_with_loop_placement();
        // assert_eq!(loop_places, 6);
        let expected: HashSet<(isize, isize)> = HashSet::from_iter(vec![
            (6,3),
            (7,6),
            (7,7),
            (8,1),
            (8,3),
            (9,7)
        ]);

        
        assert_eq!(expected, actual);
    }

    
}