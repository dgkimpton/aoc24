use crate::files::load_full_input_as_string;

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
pub fn read_config(day: &str) -> Result<Vec<Part>, String> {
    let filename = format!("{day}.config");
    let input = load_full_input_as_string(filename.as_str())?;
    Ok(input
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0 && !l.starts_with("//"))
        .map(|l| l.split(',').map(|p| p.trim()).collect::<Vec<&str>>())
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