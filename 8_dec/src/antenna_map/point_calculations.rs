use super::Coordinates;

pub struct Distance {
    from: Coordinates, 
    to: Coordinates
}

type DistanceVector = (isize, isize);

impl Distance {
    pub fn new(from: Coordinates, to: Coordinates) -> Self {
        Self { from, to }
    }

    fn delta(&self) -> DistanceVector {
        let (x1, y1) = (self.from.0 as isize, self.from.1 as isize);
        let (x2, y2) = (self.to.0 as isize, self.to.1 as isize);
        (x2 - x1, y2 - y1)
    }

    pub fn relative_to(&self, point: &Coordinates) -> DistanceVector {
        let (x, y) = (point.0 as isize, point.1 as isize);
        let (dx, dy) = self.delta();
        (x + dx, y + dy)
    }
}

pub fn calculate_distance(from: Coordinates, to: Coordinates) -> Distance {
    Distance::new(from, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let from = (0, 0);
        let to = (0, 2);
        let distance = calculate_distance(from, to);
        assert_eq!(distance.delta(), (0, 2));
    }

    #[test]
    fn test_relative_to() {
        let from = (0, 0);
        let to = (0, 2);
        let distance = calculate_distance(from, to);
        let point = (1,1);
        assert_eq!(distance.relative_to(&point), (1, 3));
    }
}