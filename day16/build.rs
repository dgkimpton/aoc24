use std::io::Read;

pub type FileReader = std::io::BufReader<std::fs::File>;

fn main() {
    let day = std::env::var("CARGO_PKG_NAME").unwrap();

    println!("cargo::rerun-if-changed=../input/{day}.config");
    println!("cargo::rerun-if-changed=../input/");
    println!("cargo::rerun-if-changed=./src_templates/");

    let alias_code = format!("pub use {pkg} as lib;", pkg = day);
    write_file("lib_alias.rs", &alias_code);

    let parts: Vec<Part> = read_config(&day).expect("a config file in the form <dayX.config>");

    if parts.len() == 0 {
        println!("no active lines found in the config file");
    }
    create_tests(&parts, "tests.rs");
    create_benchmarks(&day, &parts, "benchmarks.rs");
}

fn create_tests(parts: &Vec<Part>, filename: &str) {
    let tests_template = read_template("tests");
    let test_template = read_template("test");

    let mut found_tests = String::new();

    for (n, part) in parts.iter().filter(|p| p.mode == Mode::Test).enumerate() {
        found_tests = format!(
            "{}\n{}",
            found_tests,
            test_template
                .replace("{REPLACE_n}", &(n + 1).to_string())
                .replace("{REPLACE_part}", &part.part.to_string())
                .replace("{REPLACE_filename}", &part.filename)
                .replace("{REPLACE_expected}", &part.expected.to_string())
        );
    }

    let tests = tests_template.replace("{REPLACE_tests}", &found_tests);
    write_file(filename, if found_tests.is_empty() { "" } else { &tests });
}

fn create_benchmarks(day: &str, parts: &Vec<Part>, filename: &str) {
    let benchmarks_template = read_template("benchmarks");
    let benchmark_template = read_template("benchmark");

    let mut found_benches = String::new();
    let mut bench_list = String::new();

    for (n, part) in parts.iter().filter(|p| p.mode == Mode::Real).enumerate() {
        let bench_name = format!("bench{}", n + 1);
        found_benches = format!(
            "{}\n{}",
            found_benches,
            benchmark_template
                .replace("{REPLACE_benchname}", &bench_name)
                .replace("{REPLACE_part}", &part.part.to_string())
                .replace("{REPLACE_filename}", &part.filename)
        );

        bench_list = format!("{}, {}", bench_list, bench_name)
    }

    let benchmarks = benchmarks_template
        .replace("{REPLACE_day}", &day)
        .replace("{REPLACE_benches}", &found_benches)
        .replace("{REPLACE_benchlist}", &bench_list);

    write_file(
        filename,
        &if found_benches.is_empty() {
            format!("pub fn main() {{ println!(\"no real configurations specified in {day}.config\"); }}")
        } else {
            benchmarks
        },
    );
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
    let input = load_full_input_as_string(make_input_filename(&filename))?;
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

fn make_input_filename(input_relative_filename: &str) -> std::path::PathBuf {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("input")
        .join(input_relative_filename);
    path
}

fn read_template(name: &str) -> String {
    let filename = format!("{name}.rst");
    load_full_input_as_string(make_template_filename(&filename))
        .expect(&format!("failed to read template {filename}"))
}

fn make_template_filename(template_relative_filename: &str) -> std::path::PathBuf {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src_templates")
        .join(template_relative_filename);
    path
}

fn make_out_path(filename: &str) -> std::path::PathBuf {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::path::Path::new(&out_dir).join(filename)
}

fn write_file(filename: &str, code: &str) {
    std::fs::write(make_out_path(filename), code).unwrap();
}

pub fn load_full_input_as_string(path: std::path::PathBuf) -> Result<String, String> {
    let mut file = open_file(path)?;
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

pub fn open_file(path: std::path::PathBuf) -> Result<FileReader, String> {
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
