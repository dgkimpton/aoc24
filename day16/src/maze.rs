use crate::grid::{Grid, GridRow};
use crate::xy::XY;

#[derive(Debug)]
pub struct Maze {
    map: Grid<Kind>,
    start: XY,
    end: XY,
}

#[derive(Debug)]
pub enum Kind {
    Wall,
    Floor,
}

impl Maze {
    pub fn new_from_string(input: &str) -> Self {
        read(input)
    }

    pub fn start(&self) -> XY {
        self.start
    }
    pub fn end(&self) -> XY {
        self.end
    }

    pub fn row_count(&self) -> usize {
        self.map.row_count()
    }

    pub fn col_count(&self) -> usize {
        self.map.col_count()
    }

    pub fn width(&self) -> i64 {
        self.map.width()
    }

    pub fn height(&self) -> i64 {
        self.map.height()
    }

    pub fn at(&self, pos: XY) -> &Kind {
        self.map.at(pos)
    }

    pub fn print(&self) {
        for row in 0..self.row_count() {
            for col in 0..self.col_count() {
                match XY::from_rc(row, col) {
                    p if p == self.start => print!("S"),
                    p if p == self.end => print!("E"),
                    p => match *self.at(p) {
                        Kind::Wall => print!("█"),
                        Kind::Floor => print!("."),
                    },
                }
            }
            println!()
        }
        println!()
    }
}

fn read(input: &str) -> Maze {
    let mut start: Option<XY> = None;
    let mut end: Option<XY> = None;

    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => Kind::Wall,
                    '.' => Kind::Floor,
                    'S' => {
                        start = Some(XY::from_rc(row, col));
                        Kind::Floor
                    }
                    'E' => {
                        end = Some(XY::from_rc(row, col));
                        Kind::Floor
                    }
                    _ => panic!("unknown input"),
                })
                .collect::<GridRow<Kind>>()
        })
        .collect::<Vec<GridRow<Kind>>>();

    Maze {
        map: Grid::from_cells(map),
        start: start.expect("should have a starting position"),
        end: end.expect("should have a starting position"),
    }
}
