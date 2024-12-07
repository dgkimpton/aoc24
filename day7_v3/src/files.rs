use crate::misc::AResult;
use std::io::Read;

pub type FileReader = std::io::BufReader<std::fs::File>;

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
