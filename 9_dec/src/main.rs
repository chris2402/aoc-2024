
use load_input::read_file_contents;


fn main() {
    let input = read_file_contents("input.txt").unwrap();
    let flat = DiskMap::parse(input.as_str()).compress().flatten();
    
    let chekc_sum:usize = flat.iter().enumerate().flat_map(|(i, entry)| {
        match entry {
            DiskMapEntry::File(f) => Some(i * f.id),
            DiskMapEntry::Empty(_) => None,
        }
    }).sum();

    println!("Checksum: {}", chekc_sum);
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct File {
    id: usize,
    size: usize,
}

impl File {
    fn reduce_size(&mut self, size: usize) -> File {
        self.size = self.size.saturating_sub(size);
        File { id: self.id, size }
    }
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

    fn size(&self) -> usize {
        match self {
            DiskMapEntry::File(f) => f.size,
            DiskMapEntry::Empty(size) => *size,
        }
    }

    fn is_file(&self) -> Option<DiskMapEntry> {
        match self {
            DiskMapEntry::File(_) => Some(self.clone()),
            DiskMapEntry::Empty(_) => None,
        }
    }
    
    fn as_file_ref(&self) -> Option<&File> {
        match self {
            DiskMapEntry::File(f) => Some(f),
            DiskMapEntry::Empty(_) => None,
        }
    }
    
    fn as_file_mut(&mut self) -> Option<&mut File> {
        match self {
            DiskMapEntry::File(f) => Some(f),
            DiskMapEntry::Empty(_) => None,
        }
    }
    
    fn is_empty(&self) -> Option<DiskMapEntry> {
        match self {
            DiskMapEntry::Empty(_) => Some(self.clone()),
             DiskMapEntry::File(_) => None,
        }
    }
}

type DiskMap = Vec<DiskMapEntry>;
trait DiskMapCounter {
    fn count_files_and_empty(&self) -> (usize, usize);
}

impl DiskMapCounter for DiskMap {
    fn count_files_and_empty(&self) -> (usize, usize) {
        let mut file_count = 0;
        let mut empty_count = 0;

        for entry in self.iter() {
            match entry {
                DiskMapEntry::File(f) => file_count += f.size,
                DiskMapEntry::Empty(size) => empty_count += size,
            }
        }

        (file_count, empty_count)
    }
}

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
    fn compress(&mut self) -> DiskMap;
}

impl DiskMapCompressable for DiskMap {
    fn compress(&mut self) -> DiskMap {
        
        let (total_file_size, total_empty_space): (usize, usize) = self.count_files_and_empty();

        let self_clone = self.clone();
        let file_rev = self.iter_mut()
            .rev().filter_map(|entry| entry.as_file_mut())
            .peekable();
        
        let mut remaining_file_space = total_file_size;
        let mut result: DiskMap = self_clone.iter().scan(file_rev, |state, entry| {
            let mut fill_files: DiskMap = Vec::new();

            match entry {
                DiskMapEntry::Empty(mut empty_space) => {
                    while empty_space > 0 {
                        match state.peek_mut() {
                            Some(file) if (file.size + empty_space) > remaining_file_space => {
                                fill_files.push(DiskMapEntry::File(file.clone()));
                                state.next();
                                empty_space = 0;
                             }
                            Some(file) if file.size <= empty_space => {
                                    empty_space -= file.size;
                                fill_files.push(DiskMapEntry::File(file.clone()));
                                state.next();
                            },
                            Some(file) if file.size > empty_space => {
                                let new_file = file.reduce_size(empty_space);
                                fill_files.push(DiskMapEntry::File(new_file));
                                empty_space = 0;
                            }
                            _ => {
                                state.next();
                                empty_space = 0;
                            }
                        };
                    }
                    
                },
                _ => {
                    match state.next_back() {
                        None => return None,
                        Some(_) => fill_files.push(entry.clone()),
                    }; 
                }
            }
            remaining_file_space -= fill_files.iter().map(|f| f.size()).sum::<usize>();
            Some(fill_files)
        }).flatten().collect();
        
        result.push(DiskMapEntry::new_empty(total_empty_space));
        
        result
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
    fn test_count_files_and_empty() {
        let disk_map = get_expected_disk_map();
        let (file_count, empty_count) = disk_map.count_files_and_empty();

        assert_eq!(file_count, 28);
        assert_eq!(empty_count, 14);
    }

    #[test]
    fn test_compress() {   
        let compressed = get_expected_disk_map().compress();

        assert_eq!(compressed, vec![
            DiskMapEntry::new_file(0, 2),
            DiskMapEntry::new_file(9, 2),
            DiskMapEntry::new_file(8, 1),
            DiskMapEntry::new_file(1, 3),
            DiskMapEntry::new_file(8, 3),
            DiskMapEntry::new_file(2, 1),
            DiskMapEntry::new_file(7, 3),
            DiskMapEntry::new_file(3, 3),
            DiskMapEntry::new_file(6, 1),
            DiskMapEntry::new_file(4, 2),
            DiskMapEntry::new_file(6, 1),
            DiskMapEntry::new_file(5, 4),
            DiskMapEntry::new_file(6, 2),
            DiskMapEntry::new_empty(14),
        ]);
    }

    #[test]
    fn test_parse() {
        let result = DiskMap::parse(INPUT);
        assert_eq!(result, get_expected_disk_map());   
    }

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

    #[test]
    fn test_split_file() {
        let mut old_file = File { id: 0, size: 5 };
        let new_file:File  = old_file.reduce_size(3);
        assert_eq!(old_file.size, 2);
        assert_eq!(new_file.size, 3);
    }

    
    #[test]
    fn test_split_in_iterator() {
        let mut diskmap: DiskMap = DiskMap::from(vec![
            DiskMapEntry::new_file( 0, 5 )
        ]);

        let mut diskmap_iter = diskmap.iter_mut().peekable();
        
        let old_file = diskmap_iter.peek_mut();
        let old_file = old_file.unwrap().as_file_mut().unwrap();
        _  = old_file.reduce_size(3);
        
        assert_eq!(diskmap[0].size(), 2);
    }

    #[test]
    fn test_dmp_as_file_mut() {
        let mut diskmap: DiskMap = DiskMap::from(vec![
            DiskMapEntry::new_file( 0, 5 )
        ]);

        let mut file = diskmap[0].as_file_mut().unwrap();
        file.size = 3;

        assert_eq!(diskmap[0].size(), 3);
    }
}
