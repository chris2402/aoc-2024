#[derive(PartialEq, Hash, Eq, Clone, Debug)]
pub struct Rule{
    pub x: usize,
    pub y: usize
}

impl Rule {
    pub fn new(x: usize, y: usize) -> Self {
        assert_ne!(x, y);
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rule = Rule::new(1, 2);
        assert_eq!(rule.x, 1);
        assert_eq!(rule.y, 2);
    }
}