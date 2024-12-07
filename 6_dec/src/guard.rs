type Position = (isize, isize);


#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Guard {
    direction: Direction,
}

impl Guard {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
        }
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn move_ahead(&self, position: &Position) -> Position {
        match self.direction {
            Direction::Up => (position.0 - 1, position.1),
            Direction::Right => (position.0, position.1 + 1),
            Direction::Down => (position.0 + 1, position.1),
            Direction::Left => (position.0, position.1 - 1),
        }
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_ahead() {
        let guard = Guard::new(Direction::Up);
        let position = (0, 0);
        assert_eq!(guard.move_ahead(&position), (-1, 0));

        let guard = Guard::new(Direction::Right);
        let position = (0, 0);
        assert_eq!(guard.move_ahead(&position), (0, 1));

        let guard = Guard::new(Direction::Down);
        let position = (0, 0);
        assert_eq!(guard.move_ahead(&position), (1, 0));

        let guard = Guard::new(Direction::Left);
        let position = (0, 0);
        assert_eq!(guard.move_ahead(&position), (0, -1));
    }

    #[test]
    fn test_turn_right() {
        let mut guard = Guard::new(Direction::Up);
        assert_eq!(guard.direction(), &Direction::Up);
        guard.turn_right();
        assert_eq!(guard.direction(), &Direction::Right);
        guard.turn_right();
        assert_eq!(guard.direction(), &Direction::Down);
        guard.turn_right();
        assert_eq!(guard.direction(), &Direction::Left);
        guard.turn_right();
        assert_eq!(guard.direction(), &Direction::Up);
    }
}