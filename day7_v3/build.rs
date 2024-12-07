fn main() {
    let package_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let alias_code = format!("pub use {pkg} as lib;", pkg = package_name);

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let alias_path = std::path::Path::new(&out_dir).join("lib_alias.rs");

    std::fs::write(alias_path, alias_code).unwrap();
}
