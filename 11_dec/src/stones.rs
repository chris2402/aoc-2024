pub mod alignment;
pub mod parse;

pub type Stone = i64;
pub trait StoneSplit {
    fn split(&self) -> Vec<Stone>;
}

impl StoneSplit for Stone {
    fn split(&self) -> Vec<Stone> {
        let stringed_self = self.to_string();

        let result = match stringed_self.as_str() {
            "0" => vec![1],
            x if x.len() % 2 == 1 => {
                vec![self * 2024]
            },
            x if x.len() % 2 == 0 => {
                let (first, last) = x.split_at(x.len() / 2);
                let first = first.parse::<Stone>().unwrap();
                let last = last.parse::<Stone>().unwrap();
                vec![first, last]
            },
            _ => panic!("Invalid input")
        };
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_0() {
        assert_eq!(0.split(), vec![1]);
    }
    #[test]
    fn test_split_1000() {
        assert_eq!(1000.split(), vec![10, 0]);
    }
    #[test]
    fn test_split_1() {
        assert_eq!(1.split(), vec![2024]);
    }
}