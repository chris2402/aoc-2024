use std::isize;

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
pub struct Map {
    objects: Vec<Vec<Object>>,
    guard_pos: Position,
}

impl Map {
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

    fn set (&mut self, position: Position, object: Object) {
        let x = if position.0 >= 0 { 
            position.0 as usize
        } else {
            return;
        };
        let y = if position.1 >= 0 { 
            position.1 as usize
        } else {
            return;
        };

        self.objects[x][y] = object;
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


    fn check_valid_position(&self, position: &Position) -> bool {
        let line_range = (0 as isize, self.objects.len() as isize);
        let char_range = (0 as isize, self.objects[0].len() as isize);

        let x = position.0;
        let y = position.1;

        x >= line_range.0 && char_range.0 >= 0 && x < line_range.1 && y < char_range.1
    }

    fn move_guard(&mut self, new_pos: Position) {
        let guard_tile = Object::Guard(self.get_guard().clone());
        let guard_post = self.guard_pos;
        
        self.set(new_pos, guard_tile);
        self.set(guard_post, Object::Empty);
        self.guard_pos = new_pos;
    }

    fn is_valid_guard_position(&self) -> bool {
        self.check_valid_position(&self.guard_pos)
    }
    
    pub fn solve(&mut self) -> Vec<Position> {
        let mut path: Vec<Position> = vec![];
        
        path.push(self.guard_pos);
        let mut guard = self.get_guard().clone();
        while self.is_valid_guard_position() {

            let guard_pos: Position = self.guard_pos;
            
            let new_pos = guard.move_ahead(&guard_pos);
            match self.get(&new_pos) {
                Some(Object::Wall) => {
                    guard.turn_right();
                },
                Some(Object::Empty) => {
                    self.move_guard(if self.check_valid_position(&new_pos) {new_pos } else {panic!("Guard not in ")});
                    path.push(new_pos);
                    // Check if guard has been at this position before - maybe task 2 
                },
                Some(Object::Guard(_)) => panic!("Guard already at new position!"), // Other guards should be handled here - guessing thats task 2
                None => break,
            };
        }
        path
    }
}


#[cfg(test)]
mod tests {
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
        let guard_path = map.solve();

        assert_eq!(guard_path, vec![
            (2, 1),
            (1, 1),
            (1, 2),
            (2, 2)
        ]);    
    }
}