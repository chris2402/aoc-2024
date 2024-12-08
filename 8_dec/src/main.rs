mod antenna_map;

use antenna_map::AntennaMap;
use antenna_map::antinode_locator::AntiNodeLocator;

use load_input::read_file_contents;

fn main() {
    let input = read_file_contents("input.txt").unwrap();
    let map = AntennaMap::parse(&input).unwrap();
    
    let no_antinodes = map.find_antinodes_all().len();

    println!("No. anti-nodes: '{}'", no_antinodes);
}
