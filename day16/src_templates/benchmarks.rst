extern crate bencher;

use bencher::Bencher;
use ::{REPLACE_day}::files;

fn load_file(filename:&str) -> String {
	files::load_full_input_as_string(filename).expect("an input")
}

fn benchmark_part(part_number: u8, input: &str, b: &mut Bencher) {
    b.iter(|| std::hint::black_box({REPLACE_day}::run_on_string(&input, part_number, true).unwrap()))
}

{REPLACE_benches}

bencher::benchmark_group!(benches {REPLACE_benchlist});
bencher::benchmark_main!(benches);