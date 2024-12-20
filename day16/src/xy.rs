#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct XY {
    x: i64,
    y: i64,
}

impl XY {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn from_rc(row: usize, col: usize) -> Self {
        Self::new(col as i64, row as i64)
    }

    pub fn x(&self) -> i64 {
        self.y
    }
    pub fn y(&self) -> i64 {
        self.x
    }

    pub fn row(&self) -> usize {
        self.y as usize
    }
    pub fn col(&self) -> usize {
        self.x as usize
    }

    pub fn mut_move_to(&mut self, target: XY) {
        self.x = target.x;
        self.y = target.y;
    }

    pub fn offset(&self, x: i64, y: i64) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn mut_offset(&mut self, x: i64, y: i64) {
        self.x += x;
        self.y += y;
    }

    pub fn offset_with(&self, offset: XY) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }

    pub fn mut_offset_with(&mut self, offset: XY) {
        self.x += offset.x;
        self.y += offset.y;
    }
}
