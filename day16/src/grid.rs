use crate::xy::XY;

pub type GridRow<T> = Vec<T>;

#[derive(Debug)]
pub struct Grid<T> {
    cells: Vec<GridRow<T>>,
    width: i64,
    height: i64,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn from_cells(cells: Vec<GridRow<T>>) -> Self {
        let height = cells.len();
        let width = if height > 0 { cells[0].len() } else { 0 };

        for row in 0..height {
            assert!(cells[row].len() == width)
        }

        Self {
            cells,
            width: width as i64,
            height: height as i64,
        }
    }

    pub fn push(&mut self, row: GridRow<T>) {
        if self.height == 0 {
            self.width = row.len() as i64;
        } else {
            if row.len() != self.width as usize {
                panic!("{} not equal to {}", row.len(), self.width);
            }
        }
        self.cells.push(row);
        self.height += 1;
    }

    pub fn row_count(&self) -> usize {
        self.height as usize
    }

    pub fn col_count(&self) -> usize {
        self.width as usize
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn at(&self, pos: XY) -> &T {
        self.at_rc(pos.row(), pos.col())
    }

    pub fn at_rc(&self, row: usize, col: usize) -> &T {
        &self.cells[row][col]
    }

    pub fn at_mut(&mut self, pos: XY) -> &mut T {
        self.at_rc_mut(pos.row(), pos.col())
    }

    pub fn at_rc_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.cells[row][col]
    }
}
