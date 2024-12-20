// public modules for bencher
pub mod config;
mod day;
pub mod direction;
pub mod files;
pub mod grid;
pub mod maze;
pub mod maze_graph;
pub mod xy;

pub fn run_on_string(input: &str, part: u8, is_benchmark: bool) -> Result<i64, String> {
    day::run_on_string(input, part, is_benchmark)
}
