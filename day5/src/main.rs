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

    let day = std::env::var("CARGO_PKG_NAME").unwrap();
    let parts = read_config(&day).expect("a config file in the form <dayX.config>");

    for part in &parts {
        match generate_result(&part) {
            Ok(result) => println!(
                "{} part {}-{} result: {}",
                day, part.part, part.mode, result
            ),
            Err(e) => println!("{} {}", day, e),
        }
    }
}

fn generate_result(part: &Part) -> AResult<i32> {
    lib::run(&part.filename, part.part)
        .map_err(|e| format!("Failed for part {} <{}> :: {}", part.part, part.filename, e))
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
    fn part2_test() {
        test(2, Mode::Test);
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
