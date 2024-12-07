pub type StringWindow = Vec<Vec<char>>;

pub trait StringWindowExt {
    fn windows(&self, window_size: (usize,usize)) -> impl Iterator<Item = StringWindow>;
}

impl StringWindowExt for &str {
    fn windows(&self, window_size: (usize,usize)) -> impl Iterator<Item = StringWindow> {
        let lines: Vec<String> = self.lines().map(|line| line.to_string()).collect();
        let line_windows: Vec<Vec<String>> = lines.windows(window_size.0).map(|w| w.to_vec()).collect();

        line_windows.into_iter().flat_map(move |lw| {
            let mut result = Vec::new();

            let char_windows: Vec<_> = lw.iter().map(|l| {
                l.chars().collect::<Vec<_>>()
                .windows(window_size.1)
                .map(|x| x.to_vec())
                .collect::<Vec<_>>()
            }).collect();

            let mut iterators: Vec<_> = char_windows.iter().map(|cw| cw.iter().peekable()).collect();

            while iterators.iter().all(|it| it.clone().peek().is_some()) {
                let mut line_window = Vec::new();
                for it in iterators.iter_mut() {
                    if let Some(window) = it.next() {
                        line_window.push(window.clone());
                    }
                }
                result.push(line_window);
            }

            result.into_iter()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_22() {
        let input = "abc\ndef\nghi\n";

        let result: Vec<_> = input.windows((2,2)).collect();

        assert_eq!(result, vec![
            vec![vec!['a','b'], vec!['d','e']],
            vec![vec!['b','c'], vec!['e','f']],
            vec![vec!['d','e'], vec!['g','h']],
            vec![vec!['e', 'f'], vec!['h', 'i']],
        ]);
    }

    #[test]
    fn test_windows_33() {
        let input = "abcd\nefgh\nijkl\nmnop";

        let result: Vec<_> = input.windows((3,3)).collect();

        assert_eq!(result, vec![
            vec![vec!['a','b','c'], vec!['e', 'f','g'], vec!['i','j', 'k']],
            vec![vec!['b','c','d'], vec!['f','g','h'], vec!['j','k','l']],
            vec![vec!['e', 'f','g'], vec!['i','j', 'k'], vec!['m','n','o']],
            vec![vec!['f','g','h'], vec!['j','k','l'], vec!['n','o','p']]
        ]);
    }
}