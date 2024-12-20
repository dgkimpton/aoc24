
fn {REPLACE_benchname}(b: &mut Bencher) {
	let filename = "{REPLACE_filename}";
	benchmark_part({REPLACE_part}, &load_file(filename), b);
}