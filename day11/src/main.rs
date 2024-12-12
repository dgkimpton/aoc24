mod files;
mod input_finder;
mod misc;

use input_finder::*;
use misc::AResult;

include!(concat!(env!("OUT_DIR"), "/lib_alias.rs"));

fn main() {
    // ASSUMPTION
    // there is a folder called <input> in the root of your Cargo project
    // it contains input and configuration files
    // For each day there is a config file
    // <day1.config>
    // This file specifies the [part, mode, input filename, expected result] one per line
    // e.g. 1,t,day1-test.txt,17
    // would say that part <1> <Test> input in file <day1-test.txt> expects a result of <17>
    // to disable running a line prefix it with //s

    // NOTE: the expected result is only used in cargo test

    let day = std::env::var("CARGO_PKG_NAME").unwrap();

    let parts = read_config(&day).expect("a config file in the form <dayX.config>");

    if parts.len() == 0 {
        println!("no active lines found in the config file");
    }

    for part in &parts {
        println!(
            "Running day {} part {} using {} data",
            day, part.part, part.mode
        );
        match generate_result(&part) {
            Ok(result) => println!("result: {result}",),
            Err(e) => println!("{} {}", day, e),
        }
    }
}

fn generate_result(part: &Part) -> AResult<i64> {
    use std::time::Instant;
    let now = Instant::now();
    let result = run(&part.filename, part.part)
        .map_err(|e| format!("Failed for part {} <{}> :: {}", part.part, part.filename, e))?;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(result)
}

fn run(filename: &str, part: u8) -> Result<i64, String> {
    let input = files::load_full_input_as_string(filename)?;
    lib::run_on_string(&input, part)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        test(1, Mode::Test);
    }

    #[test]
    fn part1_real() {
        test(1, Mode::Real);
    }

    #[test]
    fn part2_real() {
        test(2, Mode::Real);
    }

    fn test(part: u8, mode: Mode) {
        let config = input_finder::read_test_io(part, mode).expect("test configuration");

        let result = generate_result(&config).expect("a result");

        assert_eq!(config.expected, result);
    }
}
