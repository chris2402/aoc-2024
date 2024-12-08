mod guard;
mod map;

use std::collections::HashSet;

use map::Map;


use load_input::read_file_contents;

fn main() {
    let input = read_file_contents("input.txt").unwrap();

    let orig_map = Map::parse(&input);
    let mut map: Map = orig_map.clone();
    let super_solver = orig_map.clone();
    
    let no_placements = super_solver.solve_with_loop_placement().len();
    let no_distinct_places     = map.solve().unwrap().into_iter().collect::<HashSet<_>>().len();
    println!("Distinct tiles {}", no_distinct_places);
    println!("Placements {}", no_placements);
}
