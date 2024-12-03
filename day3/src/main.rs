// Computer Issues - Mull it over

mod calculator;
mod parser;

use std::io::Read;

pub type FileReader = std::io::BufReader<std::fs::File>;
pub type AResult<T> = Result<T, String>;

fn main() {
    println!(
        "Part 1 Test result: {}",
        run_calculations("day3-test.txt", 1).expect("a result")
    );
    println!(
        "Part 2 Test result: {}",
        run_calculations("day3-test2.txt", 2).expect("a result")
    );

    println!();

    println!(
        "Part 1 result: {}",
        run_calculations("day3.txt", 1).expect("a result")
    );
    println!(
        "Part 2 result: {}",
        run_calculations("day3.txt", 2).expect("a result")
    );
}

fn run_calculations(filename: &str, part: u8) -> AResult<i32> {
    let mut calc = calculator::ElvishCalculator::new();

    let mut machine = parser::ElvishMachineLanguageParser::new(&mut calc, part == 2);
    machine.load_string(load_full_input_as_string(filename)?.as_str());

    Ok(calc.result())
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
        .open(path)
        .map_err(|e| e.to_string())?;
    Ok(std::io::BufReader::new(file))
}
