mod string_windows;

use string_windows::StringWindowExt;
use load_input::read_file_contents;

fn main() {
    let content = read_file_contents("input.txt").unwrap();

    let mas_x_count = content.as_str().windows((3,3))
        .into_iter()
        .filter(|window| {
            window[1][1] == 'A' &&
            (window[0][0] == 'M' && window[2][2] == 'S' || window[0][0] == 'S' && window[2][2] == 'M') &&
            (window[2][0] == 'M' && window[0][2] == 'S' || window[2][0] == 'S' && window[0][2] == 'M')
        })
        .count();

    println!("MAS-X count: {}", mas_x_count);
}
