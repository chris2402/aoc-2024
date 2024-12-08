use itertools::Itertools;
use std::collections::{HashSet, HashMap};
use super::{
    AntennaMap,
    AntennaType,
    Coordinates,
};

use super::point_calculations::calculate_distance;

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
            .flat_map(|(from, to)| {
                let mut result: Vec<(usize, usize)> = Vec::new();

                result.push(to.clone());
                let distance = calculate_distance(*from, *to);
                let mut current = distance.relative_to(to);
                while self.is_within_bounds(&current) {
                    result.push((current.0 as usize, current.1 as usize));
                    current = distance.relative_to(&(current.0 as usize, current.1 as usize));
                } 

                result.into_iter()
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