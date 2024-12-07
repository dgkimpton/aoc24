// public modules for bencher
pub mod files;
pub mod input_finder;
pub mod misc;
use misc::AResult;
use rayon::prelude::*;

pub fn run(filename: &str, part: u8) -> AResult<i64> {
    let input = files::load_full_input_as_string(filename)?;
    run_on_string(&input, part)
}

#[derive(Debug, Default, Clone)]
struct Input {
    line_number: usize,
    result: i64,
    args: Vec<Token>,
}

#[derive(Debug, Clone, Copy)]
struct Token {
    value: i64,
    offset: i64,
}

pub fn run_on_string(input: &str, part: u8) -> AResult<i64> {
    const GROUP_COUNT: usize = 4;
    let mut groups: Vec<Vec<Input>> = vec![Vec::new(); GROUP_COUNT];

    input
        .lines()
        .enumerate()
        .map(|(line_number, line)| -> AResult<(usize, Input)> {
            let mut calc = Input::default();
            calc.line_number = line_number;
            let mut parts = line.split(": ");

            if let Some(result) = parts.next() {
                calc.result = parse_int(result)?;
            }

            if let Some(args) = parts.next() {
                args.split(' ').try_for_each(|arg| -> AResult<()> {
                    calc.args.push(Token {
                        value: parse_int(arg)?,
                        offset: (10 as i64).pow(arg.len() as u32),
                    });
                    Ok(())
                })?;
            }

            Ok((line_number, calc))
        })
        .try_for_each(|f| {
            if let Ok((line_number, input)) = f {
                groups[line_number % GROUP_COUNT].push(input);
                Ok(())
            } else {
                Err(f.err().unwrap())
            }
        })?;

    groups
        .into_par_iter()
        .map(|calc_set| {
            calc_set
                .iter()
                .map(|calc: &Input| -> AResult<i64> {
                    let result = calc.result;
                    Ok(if could_be_true(calc, 0, 0, part) {
                        result
                    } else {
                        0
                    })
                })
                .sum::<Result<i64, String>>()
        })
        .sum::<Result<i64, String>>()
}

fn parse_int(value: &str) -> AResult<i64> {
    value.parse::<i64>().map_err(|e| e.to_string())
}

fn could_be_true(calc: &Input, partial_result: i64, index: usize, part: u8) -> bool {
    if partial_result > calc.result {
        // answers only ever grow, so we can short circuit here
        return false;
    }

    if index >= calc.args.len() {
        return partial_result == calc.result;
    }

    let token = calc.args[index];

    return could_be_true(calc, partial_result + token.value, index + 1, part)
        || could_be_true(calc, partial_result * token.value, index + 1, part)
        || (part == 2
            && could_be_true(
                calc,
                partial_result * token.offset + token.value,
                index + 1,
                part,
            ));
}
