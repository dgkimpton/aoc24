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
    line_number: usize,
    result: i64,
    args: Vec<Token>,
    operators: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Unknown,
    Add,
    Multiply,
    Concat,
    Value(i64, i64),
}

fn parse_int(value: &str) -> AResult<i64> {
    value.parse::<i64>().map_err(|e| e.to_string())
}

pub fn run_on_string(input: &str, part: u8) -> AResult<i64> {
    input
        .lines()
        .enumerate()
        .map(|line| {
            let mut calc = Input::default();
            calc.line_number = line.0;
            let mut parts = line.1.split(": ");

            if let Some(result) = parts.next() {
                calc.result = parse_int(result)?;
            }

            if let Some(args) = parts.next() {
                args.split(' ').try_for_each(|arg| -> AResult<()> {
                    if !calc.args.is_empty() {
                        let position = calc.args.len();
                        calc.operators.push(position);
                        calc.args.push(Token::Unknown);
                    }
                    calc.args.push(Token::Value(
                        parse_int(arg)?,
                        (10 as i64).pow(arg.len() as u32),
                    ));
                    Ok(())
                })?;
            }

            Ok(calc)
        })
        .filter_map(|calc| match calc {
            Ok(calc) if could_be_true(calc.clone(), part) => Some(Ok(calc.result as i64)),
            Ok(_) => None,
            Err(e) => Some(Err(e)),
        })
        .sum::<Result<i64, String>>()
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for arg in &self.args {
            match arg {
                Token::Unknown => f.write_str(" ? ")?,
                Token::Add => f.write_str(" + ")?,
                Token::Multiply => f.write_str(" * ")?,
                Token::Concat => f.write_str("|")?,
                Token::Value(v, _) => write!(f, "{v}")?,
            }
        }
        f.write_str(" should be ")?;
        write!(f, "{}", self.result)
    }
}

#[derive(Debug, Default)]
struct Permutation {
    ops: Vec<u8>,
    limit: u8,
}

impl Permutation {
    fn new(count: usize, part: u8) -> Self {
        Self {
            ops: vec![0; count + 1],
            limit: part,
        }
    }

    fn increment_permutation(&mut self) -> bool {
        let len = self.ops.len();

        for i in 0..len {
            self.ops[i] += 1;
            if self.ops[i] <= self.limit {
                break;
            } else {
                self.ops[i] = 0;
            }
        }

        self.ops[len - 1] == 0
    }

    fn operator_at(&self, index: usize) -> u8 {
        self.ops[index]
    }
}

fn could_be_true(mut calc: Input, part: u8) -> bool {
    let operator_count = calc.operators.len();
    let mut p = Permutation::new(operator_count, part);
    loop {
        for index in 0..operator_count {
            match p.operator_at(index) {
                0 => calc.args[calc.operators[index]] = Token::Add,
                1 => calc.args[calc.operators[index]] = Token::Multiply,
                2 => calc.args[calc.operators[index]] = Token::Concat,
                _ => panic!(),
            }
        }

        let mut result: i64 = 0;
        let mut arg_index = 0;
        while arg_index < calc.args.len() {
            match calc.args[arg_index] {
                Token::Unknown => panic!(),
                Token::Add => {
                    if let Token::Value(value, _) = calc.args[arg_index + 1] {
                        arg_index += 1;
                        result += value
                    }
                }
                Token::Multiply => {
                    if let Token::Value(value, _) = calc.args[arg_index + 1] {
                        arg_index += 1;
                        result *= value
                    }
                }
                Token::Concat => {
                    if let Token::Value(value, offset) = calc.args[arg_index + 1] {
                        arg_index += 1;
                        result *= offset;
                        result += value;
                    }
                }
                Token::Value(value, _) => result = value,
            }
            arg_index += 1;
        }

        if result == calc.result {
            return true;
        }

        if !p.increment_permutation() {
            break;
        }
    }

    false
}
