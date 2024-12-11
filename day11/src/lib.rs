// public modules for bencher
pub mod files;
pub mod input_finder;
pub mod misc;

use misc::AResult; // Result<T, String>
use std::collections::HashMap;

pub fn run(filename: &str, part: u8) -> AResult<i64> {
    let input = files::load_full_input_as_string(filename)?;
    run_on_string(&input, part, false)
}

pub fn run_on_string(input: &str, part: u8, _is_benchmark: bool) -> AResult<i64> {
    let blink_count = if part == 1 { 25 } else { 75 };

    let stones = input
        .split(" ")
        .map(|v| {
            v.parse::<u64>()
                .map_err(|_| format!("couldn't parse {v} as number"))
        })
        .collect::<AResult<Vec<u64>>>()?;

    let mut stone_line = StoneLine::default();

    let mut count = 0;
    for stone in stones {
        count += stone_line.simulate_blinks((stone, blink_count));
    }

    Ok(count as i64)
}

#[derive(Default)]
struct StoneLine {
    known_results: HashMap<(u64, u32), u64>,
}

impl StoneLine {
    fn simulate_blinks(&mut self, pair: (u64, u32)) -> u64 {
        let (stone, iterations) = pair;

        if iterations == 0 {
            return 1;
        }

        if let Some(result) = self.known_results.get(&pair) {
            return *result;
        }

        let remaining_iterations = iterations - 1;

        let result = match stone {
            0 => self.simulate_blinks((1u64, remaining_iterations)),
            v if stone.ilog10() % 2 == 0 => {
                self.simulate_blinks((v * 2024u64, remaining_iterations))
            }
            v => {
                let (left, right) = split_value(v);

                self.simulate_blinks((left, remaining_iterations))
                    + self.simulate_blinks((right, remaining_iterations))
            }
        };

        self.known_results.insert(pair, result);

        result
    }
}

fn split_value(value: u64) -> (u64, u64) {
    let digit_count = value.ilog10() + 1;
    let half_digit_count = digit_count / 2;
    let new_left = value / (10u64).pow(half_digit_count as u32);
    let new_right = value - (new_left * (10u64).pow(half_digit_count as u32));
    (new_left, new_right)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let (left, right) = split_value(10);
        assert_eq!(1, left);
        assert_eq!(0, right);
    }

    #[test]
    fn test2() {
        let (left, right) = split_value(3456);
        assert_eq!(34, left);
        assert_eq!(56, right);
    }
}
