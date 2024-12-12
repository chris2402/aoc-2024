use crate::stones::{
    Stone,
    StoneSplit
};

pub struct StoneAlignment
{
    stones: Vec<Stone>
}

impl StoneAlignment{
    pub fn from(stones: Vec<Stone>) -> Self{
        StoneAlignment{
            stones
        }
    }

    pub fn as_vec(&self) -> Vec<Stone> {
        self.stones.clone()
    }

    pub fn blink(&mut self) {
        self.stones = self.stones.iter().map(|s| s.split()).flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        let mut stone_alignment = StoneAlignment::from(vec![0, 1000, 1]);
        stone_alignment.blink();
        assert_eq!(stone_alignment.as_vec(), vec![1, 10, 0, 2024]);
    }
}