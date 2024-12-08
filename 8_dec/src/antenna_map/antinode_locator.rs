use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use super::{
    AntennaMap,
    AntennaType,
    Coordinates,
};

use super::point_calculations::{
    Distance,
    calculate_distance
};

pub trait AntiNodeLocator {
    fn find_antinodes(&self, antenna_type: &AntennaType) -> Vec<Coordinates>;
    fn find_antinodes_all(&self) -> HashSet<Coordinates>;
}

impl AntennaMap {
    fn permutate(&self, antenna_type: &AntennaType) -> Vec<(Coordinates, Coordinates)> {
        let locations = match self.locations.get(antenna_type) {
            Some(locations) => locations.clone(),
            None => return Vec::new()
        };

        locations.into_iter()
            .permutations(2)
            .map(|pair| (pair[0], pair[1]))
            .collect::<Vec<(_,_)>>()
    }
}

impl AntiNodeLocator for AntennaMap {
    fn find_antinodes(&self, antenna_type: &AntennaType) -> Vec<Coordinates> {
        self.permutate(antenna_type)
            .iter()
            .filter_map(|(from, to)| {
                let distance = calculate_distance(*from, *to);
                let anti_to = distance.relative_to(to);
                if self.is_within_bounds(&anti_to) {
                    Some((anti_to.0 as usize, anti_to.1 as usize))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn find_antinodes_all(&self) -> HashSet<Coordinates> {
        self.unique_types.iter()
            .flat_map(|antenna_type| self.find_antinodes(antenna_type))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn get_map() -> AntennaMap  {
        AntennaMap::create_with(
            (4, 4),
            HashMap::from([
                ('0', vec![(0, 0), (0, 2), (2, 0)]),
                ('1', vec![(3, 0), (3, 3)])
            ])
        ).unwrap()
    }

    #[test]
    fn test_combine() {
        let map = get_map();        
        let distances = map.permutate(&'0');
        assert_eq!(distances, vec![
            ((0, 0), (0, 2)), ((0, 0), (2, 0)), 
            ((0, 2), (0, 0)), ((0, 2), (2, 0)), 
            ((2, 0), (0, 0)), ((2, 0), (0, 2))
        ]);
    }

}