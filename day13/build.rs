use std::io::Read;

pub type FileReader = std::io::BufReader<std::fs::File>;

fn main() {
    let day = std::env::var("CARGO_PKG_NAME").unwrap();
    let alias_code = format!("pub use {pkg} as lib;", pkg = day);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let alias_path = std::path::Path::new(&out_dir).join("lib_alias.rs");

    std::fs::write(alias_path, alias_code).unwrap();

    let parts = read_config(&day).expect("a config file in the form <dayX.config>");

    if parts.len() == 0 {
        println!("no active lines found in the config file");
    }

    let mut tests = String::new();
    tests.push_str(
        r"
	#[cfg(test)]
	mod test {
		use super::*;
	
	",
    );
    let mut n = 0;
    for part in parts.iter().filter(|p| p.mode == Mode::Test) {
        n = n + 1;
        let (part, mode, filename, expected) =
            (part.part, part.mode, part.filename.clone(), part.expected);
        let lmode = if mode == Mode::Test { "test" } else { "real" };

        println!("Running day {day} part {part} using {mode:?} data");

        tests.push_str(
            format!(
                "
		#[test]
		fn test_{n}_part{part}_{lmode}() {{
			let actual = run_test({part}, \"{filename}\");	
			assert_eq!({expected}, actual);		
		}}
		"
            )
            .as_str(),
        );
    }
    tests.push_str(
        format!(
            "
		fn run_test(part:u8, filename:&str) -> i64 {{
			use std::time::Instant;
			let now = Instant::now();

			let result = run(filename,part);

			match result {{
				Ok(actual) =>{{
					let elapsed = now.elapsed();
					println!(\"Elapsed: {{:.2?}}\", elapsed);
					actual
				}}
				Err(e) => {{
					eprintln!(\"TEST FAILED for part {{part}} <{{filename}}> :: {{e}}\");
					const TEST_FAILED:bool = false;
					assert!(TEST_FAILED);
					0
				}}
			}}
			
		}}
	}}
	"
        )
        .as_str(),
    );

    let tests_path = std::path::Path::new(&out_dir).join("tests.rs");

    std::fs::write(tests_path, tests).unwrap();

    println!("cargo::rerun-if-changed=../input/{day}.config");
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Test,
    Real,
}

pub struct Part {
    pub part: u8,
    pub mode: Mode,
    pub filename: String,
    #[allow(dead_code)] // only used by tests
    pub expected: i64,
}

pub fn read_config(day: &str) -> Result<Vec<Part>, String> {
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

pub fn load_full_input_as_string(filename: &str) -> Result<String, String> {
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
        .parent()
        .unwrap()
        .join("input")
        .join(project_relative_filename);

    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(&path)
        .map_err(|e| format!("<{}> :: {}", path.display(), e.to_string()))?;
    Ok(std::io::BufReader::new(file))
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
