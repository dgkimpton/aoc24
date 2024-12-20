use crate::xy::XY;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(C)]
pub enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}
impl Direction {
    pub fn mirror(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    pub fn counter_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    pub fn as_offset(&self) -> XY {
        match self {
            Direction::North => XY::new(0, -1),
            Direction::South => XY::new(0, 1),
            Direction::East => XY::new(1, 0),
            Direction::West => XY::new(-1, 0),
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }

    pub fn as_letter(&self) -> char {
        match self {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
        }
    }
}
