// public modules for bencher
pub mod files;
pub mod input_finder;
pub mod misc;
use misc::AResult;

pub fn run(filename: &str, part: u8) -> AResult<i64> {
    let input = files::load_full_input_as_string(filename)?;
    run_on_string(&input, part)
}

#[derive(Debug, Default, Clone)]
struct Input {
    result: i64,
    input: String,
    args: Vec<Token>,
}

#[derive(Debug, Clone, Copy)]
struct Token {
    value: i64,
    offset: i64,
}

pub fn run_on_string(input: &str, part: u8) -> AResult<i64> {
    input
        .lines()
        .map(|line| -> AResult<i64> {
            let mut calc = Input::default();
            let mut parts = line.split(": ");

            if let Some(result) = parts.next() {
                calc.result = parse_int(result)?;
            }

            if let Some(args) = parts.next() {
                calc.input = args.to_string();
                args.split(' ').try_for_each(|arg| -> AResult<()> {
                    calc.args.push(Token {
                        value: parse_int(arg)?,
                        offset: (10 as i64).pow(arg.len() as u32),
                    });
                    Ok(())
                })?;
            }

            let result = could_be_true(&calc, calc.result, calc.args.len() as i32 - 1, part);

            Ok(if result { calc.result } else { 0 })
        })
        .sum::<Result<i64, String>>()
}

fn parse_int(value: &str) -> AResult<i64> {
    value.parse::<i64>().map_err(|e| e.to_string())
}

fn could_be_true(calc: &Input, partial_result: i64, index: i32, part: u8) -> bool {
    if index < 0 {
        // ran out of possible values to operate on
        // if we are now at exactly zero then this branch passed

        return partial_result == 0;
    }

    let token = calc.args[index as usize];

    if partial_result < 0 {
        // never have negative answers
        return false;
    }

    let mut result = (partial_result % token.value == 0
        && could_be_true(calc, partial_result / token.value, index - 1, part))
        || could_be_true(calc, partial_result - token.value, index - 1, part);

    if part == 2 {
        let stripped_numerator = partial_result - token.value;
        if stripped_numerator > 0 && stripped_numerator % token.offset == 0 {
            result =
                result || could_be_true(calc, stripped_numerator / token.offset, index - 1, part);
        }
    }

    result
}
