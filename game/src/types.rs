#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn is_opposite(&self, other: &Direction) -> bool {
        match self {
            Direction::Up => *other == Direction::Down,
            Direction::Right => *other == Direction::Left,
            Direction::Down => *other == Direction::Up,
            Direction::Left => *other == Direction::Right,
        }
    }
}