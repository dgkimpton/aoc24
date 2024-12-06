// public modules for bencher
pub mod files;
pub mod input_finder;
pub mod misc;

use std::fmt::Write;

use misc::AResult;

pub fn run(filename: &str, part: u8) -> AResult<i32> {
    let input = files::load_full_input_as_string(filename)?;
    run_on_string(&input, part)
}

pub fn run_on_string(input: &str, part: u8) -> AResult<i32> {
    // Note: Part 3 is part 1 with visualisation, part 4 is part 2 with visualisation
    let visualise: bool = part == 3;

    let mut world = World::from_string(input)?;

    print_board(visualise, &world, 0, 1000)?;
    while world.guard.state == GuardState::StillWalking {
        world.guard.step(&mut world.map, false);
        print_board(visualise, &world, 0, 100)?;
    }

    if visualise {
        print!("{world}");
        println!("loops found: 0");
    }

    let mut result = world.guard.visited_count;

    if part == 2 || part == 4 {
        let visualise: bool = part == 4;

        let risk_map = world.map.clone();
        let mut loopable_positions = 0;

        for row in 0..risk_map.height {
            for col in 0..risk_map.width {
                let p = Position::new(row, col);
                if !risk_map.sensible_place_for_obstruction(p) {
                    continue;
                }

                world.reset();
                world.map.place_obstruction_at(p);
                print_board(visualise, &world, loopable_positions, 500)?;

                while world.guard.state == GuardState::StillWalking {
                    world.guard.step(&mut world.map, true);

                    print_board(visualise, &world, loopable_positions, 50)?;

                    match world.guard.state {
                        GuardState::StillWalking => {}
                        GuardState::StuckInLoop => {
                            loopable_positions += 1;
                            break;
                        }
                        GuardState::LeftTheMap => break,
                    }
                }
            }
        }

        print_board(visualise, &world, loopable_positions, 0)?;

        result = loopable_positions;
    }
    Ok(result)
}

fn print_board(visualise: bool, world: &World, loopable_positions: i32, delay: u64) -> AResult<()> {
    if !visualise {
        return Ok(());
    }

    print!("\x1b[s");
    print!("{world}");
    println!("loops found: {loopable_positions}");
    print!("\x1b[u");
    wait(delay)
}

struct World {
    map: TimeMap,
    initial_map: TimeMap,
    guard: Guard,
    initial_guard: Guard,
}

impl World {
    fn from_string(input: &str) -> AResult<Self> {
        let mut guard: Option<Guard> = None;

        let data = input
            .lines()
            .enumerate()
            .map(|line| {
                line.1
                    .chars()
                    .enumerate()
                    .map(|c| match c.1 {
                        '#' => Ok(Square::with_access(Accessibility::Obstructed(
                            ObstructionType::Original,
                        ))),
                        '0' => Ok(Square::with_access(Accessibility::Obstructed(
                            ObstructionType::Introduced,
                        ))),
                        '>' => init_guard(&mut guard, line.0, c.0, Direction::Right),
                        '<' => init_guard(&mut guard, line.0, c.0, Direction::Left),
                        '^' => init_guard(&mut guard, line.0, c.0, Direction::Up),
                        'v' => init_guard(&mut guard, line.0, c.0, Direction::Down),
                        '.' => Ok(Square::with_access(Accessibility::Free)),
                        invalid => {
                            Err(format!("failed to understand input - found {invalid}").to_string())
                        }
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<Square>>, String>>()?;

        let map = TimeMap::from(data);
        let guard = guard.ok_or("missing guard".to_string())?;

        Ok(Self {
            map: map.clone(),
            guard: guard.clone(),
            initial_guard: guard,
            initial_map: map,
        })
    }

    fn reset(&mut self) {
        self.guard = self.initial_guard.clone();
        self.map = self.initial_map.clone();
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rotated_right(self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Self {
        Self { row: row, col: col }
    }
    fn from(row: usize, col: usize) -> Self {
        Self::new(row as i32, col as i32)
    }

    fn offset_by(&self, row_offset: i32, col_offset: i32) -> Self {
        Self {
            row: self.row + row_offset,
            col: self.col + col_offset,
        }
    }

    fn moved(&self, direction: Direction) -> Self {
        match direction {
            Direction::Left => self.offset_by(0, -1),
            Direction::Right => self.offset_by(0, 1),
            Direction::Up => self.offset_by(-1, 0),
            Direction::Down => self.offset_by(1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    position: Position,
    direction: Direction,
    state: GuardState,
    visited_count: i32,
}

fn init_guard(
    guard: &mut Option<Guard>,
    row: usize,
    col: usize,
    direction: Direction,
) -> AResult<Square> {
    match guard {
        Some(_) => Err("found two guards?".to_string()),
        None => {
            guard.replace(Guard {
                position: Position::from(row, col),
                direction,
                state: GuardState::StillWalking,
                visited_count: 1,
            });
            Ok(Square {
                accessibility: Accessibility::Free,
                visited: Visited::new(direction),
            })
        }
    }
}

impl Guard {
    fn is_at(&self, row: usize, col: usize) -> bool {
        self.position == Position::from(row, col)
    }

    fn as_char(&self) -> char {
        match self.direction {
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }

    fn step(&mut self, map: &mut TimeMap, check_loops: bool) {
        loop {
            let square = map.at(self.position.moved(self.direction));
            match square {
                Some(square) => match square.accessibility {
                    Accessibility::Free => {
                        if !square.visited.is_visited() {
                            self.visited_count += 1;
                        }

                        if !square.visited.is_first_visit(self.direction) && check_loops {
                            self.state = GuardState::StuckInLoop;
                        }
                        break;
                    }
                    Accessibility::Obstructed(_) => self.direction = self.direction.rotated_right(),
                },
                None => {
                    self.state = GuardState::LeftTheMap;
                    return;
                }
            }
        }

        self.position = self.position.moved(self.direction);
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GuardState {
    StillWalking,
    StuckInLoop,
    LeftTheMap,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct Visited {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl Visited {
    fn is_visited(&self) -> bool {
        self.left || self.right || self.up || self.down
    }

    fn is_first_visit(&mut self, direction: Direction) -> bool {
        if match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
            Direction::Up => self.up,
            Direction::Down => self.down,
        } {
            return false;
        }

        match direction {
            Direction::Left => self.left = true,
            Direction::Right => self.right = true,
            Direction::Up => self.up = true,
            Direction::Down => self.down = true,
        };

        return true;
    }

    fn new(direction: Direction) -> Self {
        let mut result = Self::default();
        match direction {
            Direction::Left => result.left = true,
            Direction::Right => result.right = true,
            Direction::Up => result.up = true,
            Direction::Down => result.down = true,
        };
        result
    }
}

#[derive(Debug, Clone)]
struct Square {
    accessibility: Accessibility,
    visited: Visited,
}

impl Square {
    fn with_access(access: Accessibility) -> Self {
        Self {
            accessibility: access,
            visited: Visited::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ObstructionType {
    Original,
    Introduced,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Accessibility {
    Free,
    Obstructed(ObstructionType),
}

#[derive(Debug, Clone)]
struct TimeMap {
    data: Vec<Vec<Square>>,
    width: i32,
    height: i32,
}

impl TimeMap {
    fn from(data: Vec<Vec<Square>>) -> Self {
        Self {
            height: data.len() as i32,
            width: if data.len() > 0 { data[0].len() } else { 0 } as i32,
            data,
        }
    }

    fn at(&mut self, pos: Position) -> Option<&mut Square> {
        if pos.row < 0 || pos.row >= self.height {
            return None;
        }
        if pos.col < 0 || pos.col >= self.width {
            return None;
        }
        Some(&mut self.data[pos.row as usize][pos.col as usize])
    }

    fn read(&self, pos: Position) -> &Square {
        &self.data[pos.row as usize][pos.col as usize]
    }

    fn sensible_place_for_obstruction(&self, p: Position) -> bool {
        // no point placing an obstruction where the guard never passes
        let square = self.read(p);
        return match square.accessibility {
            Accessibility::Obstructed(_) => false,
            Accessibility::Free if !square.visited.is_visited() => false,
            _ => true,
        };
    }

    fn place_obstruction_at(&mut self, p: Position) {
        self.at(p).unwrap().accessibility = Accessibility::Obstructed(ObstructionType::Introduced)
    }

    fn display(&self, f: &mut std::fmt::Formatter<'_>, guard: &Guard) -> std::fmt::Result {
        f.write_char('╔')?;
        for _ in 0..self.width {
            f.write_char('═')?
        }
        f.write_char('╗')?;
        f.write_char('\n')?;

        self.data.iter().enumerate().try_for_each(|row| {
            f.write_char('║')?;
            row.1.iter().enumerate().try_for_each(|point| {
                let c = match point.1.accessibility {
                    Accessibility::Free if guard.is_at(row.0, point.0) => guard.as_char(),
                    Accessibility::Free if point.1.visited.is_visited() => 'X',
                    Accessibility::Free => '·',
                    Accessibility::Obstructed(ObstructionType::Original) => '█',
                    Accessibility::Obstructed(ObstructionType::Introduced) => 'Ø',
                };
                f.write_char(c)
            })?;
            f.write_char('║')?;
            f.write_char('\n')
        })?;

        f.write_char('╚')?;
        for _ in 0..self.width {
            f.write_char('═')?
        }
        f.write_char('╝')?;
        f.write_char('\n')?;
        f.write_str(format!("Visited: {}", guard.visited_count).as_str())?;
        f.write_char('\n')?;
        Ok(())
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.display(f, &self.guard)
    }
}

fn wait(milis: u64) -> AResult<()> {
    std::io::Write::flush(&mut std::io::stdout()).map_err(|e| e.to_string())?;
    std::thread::sleep(std::time::Duration::from_millis(milis));
    Ok(())
}
