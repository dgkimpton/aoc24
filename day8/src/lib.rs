// public modules for bencher
pub mod files;
pub mod input_finder;
pub mod misc;

use std::ops::Add;

use misc::AResult;

pub fn run(filename: &str, part: u8) -> AResult<i64> {
    let input = files::load_full_input_as_string(filename)?;
    run_on_string(&input, part)
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Point {
    row: i32,
    col: i32,
}
impl Point {
    fn from(row: usize, col: usize) -> Self {
        Self::new(row as i32, col as i32)
    }

    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

#[derive(Debug)]
struct Grid {
    antennas: std::collections::HashMap<char, Vec<Point>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut partial = Self {
            antennas: std::collections::HashMap::new(),
            width: 0,
            height: 0,
        };

        partial.height = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                partial.width = line
                    .chars()
                    .enumerate()
                    .map(|(col, c)| {
                        if c != '.' && c != '#' {
                            partial
                                .antennas
                                .entry(c)
                                .or_insert_with(|| Vec::new())
                                .push(Point::from(row, col));
                        }
                    })
                    .count() as i32;
            })
            .count() as i32;

        partial
    }
}

pub fn run_on_string(input: &str, part: u8) -> AResult<i64> {
    let grid = Grid::new(input);

    let mut antinodes: std::collections::HashSet<Point> = std::collections::HashSet::new();

    for (_, antennas) in &grid.antennas {
        for (first, second) in generate_pairs(antennas, part) {
            let diff = Point::new(first.row - second.row, first.col - second.col);
            let mut antinode = *first;

            loop {
                antinode = antinode + diff;

                if antinode.row >= 0
                    && antinode.row < grid.height
                    && antinode.col >= 0
                    && antinode.col < grid.width
                {
                    antinodes.insert(antinode);
                    if part == 1 || diff.row == 0 && diff.col == 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
    print(&grid, &antinodes);

    Ok(antinodes.len() as i64)
}

fn generate_pairs(items: &Vec<Point>, part: u8) -> Vec<(&Point, &Point)> {
    let mut result = Vec::<(&Point, &Point)>::new();
    for a in items {
        for b in items {
            if *b != *a || part == 2 {
                result.push((a, b));
            }
        }
    }
    result
}

fn print(grid: &Grid, antinodes: &std::collections::HashSet<Point>) {
    for row in 0..grid.height {
        for col in 0..grid.width {
            let p = Point::new(row, col);
            let antenna = grid.antennas.iter().find(|(_, points)| points.contains(&p));
            let antinode = antinodes.iter().find(|point| **point == p);
            if antinode.is_some() {
                print!("\x1b[7m");
            }
            if let Some(antenna) = antenna {
                print!("{}", antenna.0);
            } else {
                print!(".");
            }
            if antinode.is_some() {
                print!("\x1b[27m");
            }
        }
        print!("\n");
    }
}
