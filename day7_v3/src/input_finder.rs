use crate::files::load_full_input_as_string;
use crate::misc::AResult;

#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    pub part: u8,
    pub mode: Mode,
    pub filename: String,
    #[allow(dead_code)] // only used by tests
    pub expected: i64,
}

/// The real test information is relegated to a config file
/// so that the code can be shared without revealing it
/// as per the rules of AoC 24
/// This file specifies the [part, mode, input filename, expected result] one per line
/// e.g. 1,t,day1-test.txt,17
pub fn read_config(day: &str) -> AResult<Vec<Part>> {
    let filename = format!("{day}.config");
    let input = load_full_input_as_string(filename.as_str())?;
    Ok(input
        .lines()
        .filter(|l| !l.starts_with("//"))
        .map(|l| l.split(',').collect::<Vec<&str>>())
        .map(|parts| {
            Ok(Part {
                part: parts[0].parse::<u8>().map_err(|e| e.to_string())?,
                mode: parts[1].parse::<Mode>()?,
                filename: parts[2].to_string(),
                expected: parts[3].parse::<i64>().map_err(|e| e.to_string())?,
            })
        })
        .collect::<Result<Vec<Part>, String>>()
        .map_err(|e| format!("format error in <{filename}> :: {e}"))?)
}

// provided for tests and benchmarks only
#[allow(dead_code)]
pub fn read_test_io(part: u8, mode: Mode) -> AResult<Part> {
    let day = std::env::var("CARGO_PKG_NAME").unwrap();
    read_config(&day)
        .expect("a configuration")
        .iter()
        .filter(|p| p.part == part && p.mode == mode)
        .next()
        .map(|c| Ok(c.clone()))
        .unwrap_or(Err(
            "no relevant entries found in the configuration file".to_string()
        ))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Test,
    Real,
}

impl std::str::FromStr for Mode {
    type Err = String;
    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        match input {
            "r" => Ok(Mode::Real),
            "t" => Ok(Mode::Test),
            _ => Err(format!("unknown mode {input}")),
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
