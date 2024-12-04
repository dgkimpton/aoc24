use std::io::Read;

pub type FileReader = std::io::BufReader<std::fs::File>;
pub type AResult<T> = Result<T, String>;

fn main() {
    println!(
        "Part 1 result: {}",
        part1(
            load_full_input_as_string("day4.txt")
                .expect("an input")
                .as_str()
        )
        .expect("a result")
    );
}

pub fn part1(input: &str) -> AResult<i32> {
    let mut count: Count = 0;

    let source = Source::new(input.lines().map(|r| r.chars().collect()).collect());

    for col in 0..source.width {
        for row in 0..source.height {
            if source.rows[row][col] == 'X' {
                count += count_xmases(&source, Point(col as i32, row as i32))
            }
        }
    }

    Ok(count)
}

fn count_xmases(source: &Source, p: Point) -> Count {
    return count_xmas(source, p, Offset(1, 0), Offset(2, 0), Offset(3, 0))
        + count_xmas(source, p, Offset(0, 1), Offset(0, 2), Offset(0, 3))
        + count_xmas(source, p, Offset(1, 1), Offset(2, 2), Offset(3, 3))
        + count_xmas(source, p, Offset(1, -1), Offset(2, -2), Offset(3, -3));
}

fn count_xmas(source: &Source, p1: Point, o2: Offset, o3: Offset, o4: Offset) -> Count {
    is_xmas(source, p1 + o2, p1 + o3, p1 + o4) + is_xmas(source, p1 + -o2, p1 + -o3, p1 + -o4)
}

fn is_xmas(source: &Source, p2: Point, p3: Point, p4: Point) -> Count {
    (source.is(p2, 'M') && source.is(p3, 'A') && source.is(p4, 'S')) as Count
}

impl Source {
    pub fn new(input: Vec<Vec<char>>) -> Self {
        let height = input.len();
        let width = if height > 0 { input[0].len() } else { 0 };

        Self {
            rows: input,
            width,
            height,
        }
    }

    pub fn at(&self, p: Point) -> Option<char> {
        if p.1 >= self.height as i32 || p.1 < 0 || p.0 >= self.width as i32 || p.0 < 0 {
            None
        } else {
            Some(self.rows[p.1 as usize][p.0 as usize])
        }
    }

    pub fn is(&self, p: Point, c: char) -> bool {
        self.at(p) == Some(c)
    }
}

type Count = i32;

#[derive(Debug, Clone, Copy)]
pub struct Point(i32, i32);

#[derive(Debug, Copy, Clone)]
struct Offset(i32, i32);

struct Source {
    pub rows: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl std::ops::Neg for Offset {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl std::ops::Add<Offset> for Point {
    type Output = Self;
    fn add(self, other: Offset) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

pub fn load_full_input_as_string(filename: &str) -> AResult<String> {
    let mut file = open_file(filename)?;
    let mut buffer = String::new();
    buffer.reserve(4096);

    let char_count = file
        .read_to_string(&mut buffer)
        .map_err(|e| format!("failed to read bytes {e:?}"))?;

    if char_count == 0 {
        return Err("no data found".to_string());
    }

    Ok(buffer)
}

pub fn open_file(project_relative_filename: &str) -> Result<FileReader, String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join(project_relative_filename);

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(&path)
        .map_err(|e| format!("<{}> :: {}", path.display(), e.to_string()))?;
    Ok(std::io::BufReader::new(file))
}
