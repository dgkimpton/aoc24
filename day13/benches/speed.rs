extern crate bencher;
include!(concat!(env!("OUT_DIR"), "/lib_alias.rs"));

use lib::files::*;
use lib::input_finder::*;

use bencher::Bencher;

fn benchmark_part(part_number: u8, b: &mut Bencher) {
    let config = read_test_io(part_number, Mode::Real).expect("benchmark configuration");
    let input = load_full_input_as_string(&config.filename).expect("an input");

    b.iter(|| std::hint::black_box(lib::run_on_string(&input, config.part, true).unwrap()))
}

fn bench1(b: &mut Bencher) {
    benchmark_part(1, b);
}

fn bench2(b: &mut Bencher) {
    benchmark_part(2, b);
}

bencher::benchmark_group!(benches, bench1, bench2);
bencher::benchmark_main!(benches);
