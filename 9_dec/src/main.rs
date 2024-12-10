
use load_input::read_file_contents;


fn main() {
    let input = read_file_contents("input.txt").unwrap();
    let mut diskmap = DiskMap::parse(input.as_str());
    diskmap.compress();
    
    let chekc_sum:usize = diskmap.get_checksum();

    println!("Checksum: {}", chekc_sum);
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum DiskMapEntry {
    File(File),
    Empty(usize),
}

impl DiskMapEntry {
    fn new_file(id: usize, size: usize) -> Self {
        DiskMapEntry::File(File { id, size })    
    }

    fn new_empty(size: usize) -> Self {
        DiskMapEntry::Empty(size)
    }
    
    fn as_file_ref(&self) -> Option<&File> {
        match self {
            DiskMapEntry::File(f) => Some(f),
            DiskMapEntry::Empty(_) => None,
        }
    }
}

type DiskMap = Vec<DiskMapEntry>;
trait DiskMapFlatten {
    fn flatten(&self) -> DiskMap;    
}

impl DiskMapFlatten for DiskMap {
    fn flatten(&self) -> DiskMap {
        self.iter().flat_map(|entry| {
            match entry {
                DiskMapEntry::File(f) => {
                    vec![DiskMapEntry::new_file(f.id, 1); f.size as usize].into_iter()
                },
                DiskMapEntry::Empty(size) => {
                    vec![DiskMapEntry::new_empty(1); *size as usize].into_iter()
                }
            }
        }).collect()
    }
}

trait DiskMapCompressable {
    fn files(&self) -> impl DoubleEndedIterator<Item = File>;
    fn get_checksum(&self) -> usize;
    fn as_string(&self) -> String;
    fn compress(&mut self);
}

impl DiskMapCompressable for DiskMap {
    fn files(&self) -> impl DoubleEndedIterator<Item = File> {
        self.iter()
            .filter_map(|entry| entry.as_file_ref().cloned())
    }

    fn as_string(&self) -> String {
        self.iter().map(|entry| {
            match entry {
                DiskMapEntry::File(f) => f.id.to_string().repeat(f.size),
                DiskMapEntry::Empty(size) => ".".to_string().repeat(*size),
            }
        }).collect()
    }
    fn get_checksum(&self) -> usize {
        self.flatten().iter().enumerate().map(|(i, entry)| {
            match entry {
                DiskMapEntry::File(f) => i * f.id,
                DiskMapEntry::Empty(_) => 0,
            }
        }).sum()
    }

    fn compress(&mut self) {
        'move_file: for maybe_move in self.clone().iter().rev() {
            let move_file = match maybe_move {
                DiskMapEntry::Empty(_) => continue,
                DiskMapEntry::File(f) => f
            };

            let from_i = self.iter().position(|dmp| dmp == maybe_move).unwrap();
            'find_space: for (to_i, maybe_space) in self.clone().iter().enumerate() {
                if to_i >= from_i {
                    continue 'move_file;
                }
                match maybe_space {
                    DiskMapEntry::Empty(empty_space) if *empty_space >= move_file.size => {
                        self[to_i] = DiskMapEntry::File(move_file.clone());
                        self[from_i] = DiskMapEntry::new_empty(move_file.size);
                        
                        let remaining_space = empty_space - move_file.size;
                        if remaining_space > 0 {
                            self.insert(to_i + 1, DiskMapEntry::new_empty(remaining_space));
                        }
                        continue 'move_file;
                    },
                    _ => continue 'find_space,
                }
            }

        }
    }
}

trait DiskMapParser {
    fn parse(input: &str) -> DiskMap;
}

impl DiskMapParser for DiskMap {

    fn parse(input: &str) -> DiskMap {
        let mut result = DiskMap::new();
        let mut is_file_cycle = vec![true, false].into_iter().cycle();
        let mut file_id_it= 0..;
    
        for i in input.trim().chars() {
            
            let size = i.to_digit(10).unwrap();
            let is_file = is_file_cycle.next().unwrap();
            
            let next_entry = if is_file { 
                let file_id = file_id_it.next().unwrap();
                DiskMapEntry::new_file(file_id, size as usize)
             } else {
                DiskMapEntry::new_empty(size as usize)
             };
            
            result.push(next_entry);
        }
    
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2333133121414131402
    ";
    fn  get_expected_disk_map() -> Vec<DiskMapEntry> {
        vec![
            DiskMapEntry::new_file(0, 2),
            DiskMapEntry::new_empty(3),
            DiskMapEntry::new_file(1, 3),
            DiskMapEntry::new_empty(3),
            DiskMapEntry::new_file(2, 1),
            DiskMapEntry::new_empty(3),
            DiskMapEntry::new_file(3, 3),
            DiskMapEntry::new_empty(1),
            DiskMapEntry::new_file(4, 2),
            DiskMapEntry::new_empty(1),
            DiskMapEntry::new_file(5, 4),
            DiskMapEntry::new_empty(1),
            DiskMapEntry::new_file(6, 4),
            DiskMapEntry::new_empty(1),
            DiskMapEntry::new_file(7, 3),
            DiskMapEntry::new_empty(1),
            DiskMapEntry::new_file(8, 4),
            DiskMapEntry::new_empty(0),
            DiskMapEntry::new_file(9, 2),
        ]
    }

    #[test]
    fn test_compress() {   
        let mut diskmap = get_expected_disk_map();
        diskmap.compress();

        assert_eq!(diskmap.get_checksum(), 2858);
    }

    #[test]
    fn test_compress_string() {   
        let mut diskmap = get_expected_disk_map();
        diskmap.compress();

        assert_eq!(diskmap.as_string(), "00992111777.44.333....5555.6666.....8888..");
    }

    #[test]
    fn test_parse() {
        let result = DiskMap::parse(INPUT);
        assert_eq!(result, get_expected_disk_map());   
    }

    // TODO: Move in Rust specific test documentation
    #[test]
    fn test_cycle(){
        let mut true_and_false = vec![true, false].into_iter().cycle();

        assert_eq!(true_and_false.next().unwrap(), true);
        assert_eq!(true_and_false.next().unwrap(), false);
        assert_eq!(true_and_false.next().unwrap(), true);
        assert_eq!(true_and_false.next().unwrap(), false);
    }

    #[test]
    fn test_mut_vec(){
        let mut vec = vec![1,2,3,4,5,6];

        
        for (i, x) in vec.iter_mut().enumerate()  {
            *x = *x * i;
        }

        assert_eq!(vec, vec![0,2,6,12,20,30]);
    }

    #[test]
    fn test_scan() {
        let vec = vec![1,1,0,1,2,2,0,3,3,3,3];
        let iter = vec.iter().peekable();
        let fill_value = vec![9,4].into_iter();

        let actual: Vec<_> = iter.scan(fill_value, |state, a| {
            if a == &0 {
                Some(state.next().unwrap())
            } else {
                Some(*a)
            }
        }).collect();

        assert_eq!(actual, vec![1,1,9,1,2,2,4,3,3,3,3]);
    }
}
