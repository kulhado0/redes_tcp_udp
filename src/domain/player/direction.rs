pub enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    Still,
}

impl Direction {
    pub fn movement(&self) -> Option<i32> {
        match *self {
            Self::Up(value) => Some(value),
            Self::Down(value) => Some(value),
            Self::Left(value) => Some(value),
            Self::Right(value) => Some(value),
            _ => None,
        }
    }
}