use std::collections::HashSet;

use super::{TrailHead, TopologicalMap, Trail, Tile};

pub trait TrailHeadLocator {
    fn get_trail_heads(&self) -> Vec<TrailHead>;
}

impl TrailHeadLocator for TopologicalMap {
    fn get_trail_heads(&self) -> Vec<TrailHead> {
        self.map.indexed_iter()
            .filter(|(_, &elem)| elem == 0)
            .map(|((x,y), _)| TrailHead::from( x,y ))
            .collect()
    }
}

pub trait TrailLocator {
    fn get_adjasent_tiles(&self, tile: Tile) -> Vec<Tile>;
    fn find_trails_from(&self, start: TrailHead) -> Vec<Trail>;
    fn find_tops_from(&self, start: TrailHead) -> HashSet<Tile>;
}

impl TrailLocator for TopologicalMap {
    fn get_adjasent_tiles(&self, tile: Tile) -> Vec<Tile> {
        vec![(-1,0), (1,0), (0,-1), (0,1)].iter().filter_map(|(dx,dy)| {
            let x = tile.0 as isize + dx;
            let y = tile.1 as isize + dy;
            if x < 0 || y < 0 {
                return None;
            }
            let x = x as usize;
            let y = y as usize;

            Some((x, y))
        }).collect()
    }

    fn find_trails_from(&self, start: TrailHead) -> Vec<Trail> {
        match find_trail_recursion(self, start.tile) {
            Option::Some(trails) => trails,
            Option::None => vec![],
        }
    }

    fn find_tops_from(&self, start: TrailHead) -> HashSet<Tile> {
        self.find_trails_from(start)
        .iter_mut().map(|trails| *trails.first().unwrap()).collect::<HashSet<_>>()
    }
}

fn find_trail_recursion(map: &TopologicalMap, current: Tile) -> Option<Vec<Trail>> {
    let current_value: isize = match map.get_value(current) {
        Ok(value) => value,
        Err(_) => return Option::None,
    };
    if current_value == 9 {
        return Some(vec![Trail::from([current])]);
    }

    let result = map.get_adjasent_tiles(current).into_iter().filter_map(|maybe_next_tile| {
        let maybe_next_value = match map.get_value(maybe_next_tile){
            Ok(value) => value,
            Err(_) => return Option::None,
        };
        if (maybe_next_value - current_value) != 1 {
            return Option::None;
        }

        match find_trail_recursion(map, maybe_next_tile) {
            Option::Some(trails) => {
                Some(trails.into_iter().map(|mut trail| {
                    trail.push(current);
                    trail
                }).collect::<Vec<_>>())                
            },
            result@Option::None => {
                result
            }
        }
    }).flatten().collect::<Vec<_>>();

    if result.is_empty() {
        return Option::None;
    }

    Some(result)
}

#[cfg(test)]    
mod tests {
    use super::*;
    use crate::tests::get_map;


    #[test]
    fn test_get_trail_heads() {
        let map = get_map();
        assert_eq!(map.get_trail_heads(), vec![
            TrailHead::from(0, 2),
            TrailHead::from(0, 4),
            TrailHead::from(2, 4),
            TrailHead::from(4, 6),
            TrailHead::from(5, 2),
            TrailHead::from(5, 5),
            TrailHead::from(6, 0),
            TrailHead::from(6, 6),
            TrailHead::from(7, 1),
        ]);
    }

    #[test]
    fn test_find_tops_from() {
        let map = get_map();
        let trails = map.find_tops_from(TrailHead::from(0, 2));

        assert_eq!(trails, HashSet::from([(0, 1), (5, 4), (4, 5), (3, 4), (3, 0)]));
    }

    #[test]
    fn test_adjasent_values() {
        let map = get_map();
        let mut result = map.get_adjasent_tiles(TrailHead::from(0, 2).tile);
        result.sort();
        let actual = result.into_iter().map(|r| map.get_value(r).unwrap()).collect::<Vec<_>>();

        // MAP -
        // 89010123
        // 78121874

        assert_eq!(actual, vec![9,1,1]);
    }
    
    #[test]
    fn test_get_adjasent() {
        let map = get_map();
        let mut result = map.get_adjasent_tiles(TrailHead::from(0, 2).tile);
        let mut expected = vec![(0, 1), (0, 3), (1, 2)];

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_tuple_index(){
        let map = get_map();
        assert_eq!(map.map[(0,1)], 9);
    }

    #[test]
    fn test_stacking(){
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
    }

    
    #[test]
    fn test_sets(){
        let mut items = Vec::new();
        
        items.push((0, 1));
        items.push((3, 0));
        items.push((3, 4));
        items.push((5, 4));
        items.push((4, 5));
        items.push((0, 1));
        items.push((3, 0));

        let items = items.into_iter().collect::<HashSet<_>>();
        
        assert_eq!(items.len(), 5);
    }

    
}