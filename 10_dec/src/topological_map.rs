pub mod parser;
pub mod locator;

use ndarray::prelude::*;

type Tile = (usize, usize);
type Trail = Vec<Tile>;

#[derive(Debug, PartialEq)]
pub struct TrailHead {
    tile: Tile,
}

impl TrailHead {
    fn from(x: usize, y: usize) -> TrailHead {
        TrailHead { tile: (x, y) }
    }
}

#[derive(Debug, PartialEq)]
pub struct TopologicalMap {
    map: Array2<isize>,
}

impl TopologicalMap {
    pub fn new(size: (usize, usize)) -> TopologicalMap {
        TopologicalMap {
            map: Array2::from_elem(size, -1)
        }
    }

    fn to_map_string(&self) -> String {
        self.map
            .rows()
            .into_iter()
            .map(|row| {
            row.iter()
                .map(|&elem| if elem < 0 { ".".to_string() } else { elem.to_string() })
                .collect::<Vec<_>>()
                .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn get_value(&self, tile: Tile) -> Result<isize, String> {
        let (x, y) = tile;
        if x >= self.map.nrows() || y >= self.map.ncols() {
            return Err("Tile out of bounds".to_string());
        }
        Ok(self.map[[x, y]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let map = TopologicalMap::new((2, 2));
        assert_eq!(map.map, array![[-1, -1], [-1, -1]]);
    }

    #[test]
    fn test_to_map_string() {
        const MAP: &str = 
"....
....
....
....";

        let map = TopologicalMap::new((4,4));
        assert_eq!(map.to_map_string(), MAP);
    }

    #[test]
    fn test_get_value() {
        let mut map = TopologicalMap::new((3,3));
        map.map[[1, 2]] = 1;
        assert_eq!(map.get_value((1, 2)).unwrap(), 1);
    }
}