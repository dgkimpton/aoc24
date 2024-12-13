pub fn run_on_string(input: &str, part: u8, _is_benchmark: bool) -> Result<i64, String> {
    let prize_offset = if part == 1 { 0f64 } else { 10000000000000f64 };

    Ok(input
        .split("\n\n")
        .map(|machine_lines| {
            let mut lines = machine_lines.lines();

            let a = xy_extract(&mut lines, "Button A: X+", ", Y+");
            let b = xy_extract(&mut lines, "Button B: X+", ", Y+");
            let prize = xy_extract(&mut lines, "Prize: X=", ", Y=");

            let p = XY {
                x: prize.x + prize_offset,
                y: prize.y + prize_offset,
            };

            let presses_a = (p.x * b.y - b.x * p.y) / (b.y * a.x - a.y * b.x);
            let presses_b = (p.y * a.x - p.x * a.y) / (a.x * b.y - b.x * a.y);

            if presses_a.fract() > 0.0 || presses_b.fract() > 0.0 {
                return 0;
            }

            3 * (presses_a as i64) + presses_b as i64
        })
        .sum())
}

#[derive(Debug)]
struct XY {
    x: f64,
    y: f64,
}

fn xy_extract<'a>(lines: &mut impl Iterator<Item = &'a str>, prefix: &str, sep: &str) -> XY {
    lines
        .next()
        .map(|line| {
            let parts = line
                .trim_start_matches(prefix)
                .split(sep)
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            XY {
                x: parts[0] as f64,
                y: parts[1] as f64,
            }
        })
        .unwrap()
}

// public modules for bencher
pub mod files;
pub mod input_finder;
pub mod misc;
