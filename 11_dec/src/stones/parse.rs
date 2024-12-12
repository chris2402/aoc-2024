use crate::stones::alignment::StoneAlignment;
use crate::stones::Stone;

pub trait StoneAlignmentParse {
    fn parse_stones(&self) -> StoneAlignment;
}

impl StoneAlignmentParse for str {
    fn parse_stones(&self) -> StoneAlignment {
        StoneAlignment::from(self.split_whitespace()
            .map(|s| s.parse::<Stone>().unwrap())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stones() {
        let stone_alignment = "0 1000 1".parse_stones();
        assert_eq!(stone_alignment.as_vec(), vec![0, 1000, 1]);
    }
}