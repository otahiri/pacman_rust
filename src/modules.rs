#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub const fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::South,
        }
    }
    pub const fn values(&self) -> (i32, i32, usize){
        match self {
            Direction::North => (-1, 0, 0),
            Direction::East => (0, 1, 1),
            Direction::South => (1, 0, 2),
            Direction::West => (0, -1, 3),
        }
    }
}
