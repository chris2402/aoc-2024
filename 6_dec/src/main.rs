mod guard;
mod map;

use std::collections::HashSet;

use map::Map;


use load_input::read_file_contents;

fn main() {
    let input = read_file_contents("input.txt").unwrap();

    let mut map: Map = Map::parse(&input);
    let no_distinct_places     = map.solve().into_iter().collect::<HashSet<_>>().len();
    println!("{}", no_distinct_places);
}
