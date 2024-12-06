use std::usize;

use load_input::read_file_contents;

fn count_occurences(input: &str) -> usize {
    let keyword: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let keyword_rev: Vec<char> = keyword.iter().rev().cloned().collect();

    let lines = input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .filter(|window| *window == keyword || *window == keyword_rev)
        .count();

    lines
}

fn diagonal_map<It>(lines: &mut It, char_offset: usize) -> Option<Vec<char>>
where
    It: Iterator<Item = String> + Clone,
{
    match lines.next() {
        Some(line) => {

            let d = match line.chars().nth(char_offset) {
                Some(c) => c,
                None => return Some(vec![]),
            };

            let ds = match diagonal_map(lines, char_offset + 1) {
                Some(ds) => ds,
                None => return Some(vec![]),
            };

            Some(vec![vec![d], ds].concat())
        }
        None => Some(vec![]),
    }
}


fn main() {
    let content = read_file_contents("input.txt").unwrap();
    let lines = content.lines();
    let sum = xmas_slices(lines)
    .iter()
    .map(|s| count_occurences(s.as_str()))
    .sum::<usize>();

println!("Sum: {}", sum);
}

fn vertical_xmas_slices<'a, T>(lines: T, no_lines: usize) -> Vec<String> 
where 
    T: Iterator<Item = &'a str> + Clone
{
    lines
        .collect::<Vec<&str>>()
        .windows(no_lines)
        .flat_map(|window| {
            (0..no_lines)
                .map(|i| {
                    window
                        .iter()
                        .map(|line| line.chars().nth(i).unwrap())
                        .collect()
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>()
}

fn horizontal_xmas_slices<'a, T>(lines: T) -> Vec<String> 
where 
    T: Iterator<Item = &'a str> + Clone
{
    lines.map(|l| String::from(l)).collect::<Vec<String>>()
}

fn diagonal_xmas_slices<'a, T>(lines: T, no_lines: usize, no_chars: usize) -> Vec<String>
where 
    T: Iterator<Item = &'a str> + Clone
{    
    let line_it = lines.map(|l| String::from(l));
    vec![
        (0..no_chars)
            .map(|i| {
                diagonal_map(&mut line_it.clone(), i)
                    .unwrap()
                    .iter()
                    .collect::<String>()
            })
            .collect::<Vec<String>>(),
        (1..no_lines)
            .map(|i| {
                let line_it = line_it.clone().skip(i);
                diagonal_map(&mut line_it.clone(), 0)
                    .unwrap()
                    .iter()
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
    ].concat()  
}

fn xmas_slices(lines: std::str::Lines<'_>) -> Vec<String> {
    let no_lines: usize = lines.clone().count();
    let no_chars: usize = lines.clone().nth(0).unwrap_or_default().chars().count();
    let vec = vec![
        horizontal_xmas_slices(lines.clone()),
        vertical_xmas_slices(lines.clone(), no_lines),
        diagonal_xmas_slices(lines.clone(), no_lines, no_chars),
        diagonal_xmas_slices(lines.clone().rev().clone(), no_lines, no_chars),
    ]
    .concat();

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occurences() {
        let content = "MMAXMASMASASASSAMXAAAA";

        let lines = count_occurences(content);

        assert_eq!(lines, 2);
    }

    #[test]
    fn test_diagonal_map() {
        let input = format!["{}\n{}\n{}",
            "XOO",
            "OXO",
            "OOX"
        ];

        let lines = input.lines().map(|l| String::from(l));
        let lines = diagonal_map(&mut lines.clone(), 0).unwrap();

        assert_eq!(lines, ['X'; 3]);
    }

    #[test]
    fn test_xmas_slices(){
        let input = format!["{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX"
        ];

        let lines = input.lines();
        let slices = xmas_slices(lines);

        assert_eq!(slices.len(), 58);
    }
    
    #[test]
    fn test_vertical_xmas_slices(){
        let input = format!["{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX"
        ];

        let lines = input.lines();
        let no_lines = lines.clone().count();
        let slices = vertical_xmas_slices(lines, no_lines);

        assert_eq!(slices.len(), 10);
    }

    #[test]
    fn test_horizontal_xmas_slices(){
        let input = format!["{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX"
        ];

        let lines = input.lines();
        let slices = horizontal_xmas_slices(lines);

        assert_eq!(slices.len(), 10);
    }
    
    
    #[test]
    fn test_diagonal_xmas_slices(){
        let input = format!["{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX"
        ];

        let lines = input.lines();
        let no_lines = lines.clone().count();
        let no_chars = lines.clone().nth(0).unwrap_or_default().chars().count();
        let slices = diagonal_xmas_slices(lines, no_lines, no_chars);

        assert_eq!(slices.len(), 19);
    }
    

    #[test]
    fn test_xmas_slice_counting(){
        let input = format!["{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX"
        ];


        let lines = input.lines();
        
        let sum = xmas_slices(lines).iter().map(|s| count_occurences(s.as_str())).sum::<usize>();

        assert_eq!(sum, 18);
    }

    #[test]
    fn test_xmas_sep_slice_counting(){
        let input = format!["{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX"
        ];
        
        let lines = input.lines();
        let no_lines = lines.clone().count();
        let no_chars = lines.clone().nth(0).unwrap_or_default().chars().count();
        
        let hor_sum = horizontal_xmas_slices(lines.clone()).iter().map(|s| count_occurences(s.as_str())).sum::<usize>();
        let ver_sum = vertical_xmas_slices(lines.clone(), no_lines).iter().map(|s| count_occurences(s.as_str())).sum::<usize>();
        let diag_sum = diagonal_xmas_slices(lines.clone(), no_lines, no_chars).iter().map(|s| count_occurences(s.as_str())).sum::<usize>();
        let inv_diag_sum = diagonal_xmas_slices(lines.clone().rev(), no_lines, no_chars).iter().map(|s| count_occurences(s.as_str())).sum::<usize>();

        assert_eq!(hor_sum, 5,  "Horizontal sum: {}", hor_sum);
        assert_eq!(ver_sum, 3, "Vertical sum: {}", ver_sum);
        assert_eq!(diag_sum, 5, "Diagonal sum: {}", diag_sum);
        assert_eq!(inv_diag_sum, 5, "Inverted Diagonal sum: {}", inv_diag_sum);
    }


    
}
