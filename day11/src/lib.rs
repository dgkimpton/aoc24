use std::collections::HashMap;

pub fn run_on_string(input: &str, part: u8) -> Result<i64, String> {
    let blink_count = if part == 1 { 25 } else { 75 };

    let mut cache = HashMap::new();

    input
        .split(' ')
        .map(parse_to::<Stone>)
        .map_ok(|stone| simulate_blinks(stone, blink_count, &mut cache))
        .sum_ok()
}

fn simulate_blinks(
    stone: Stone,
    iterations: u32,
    cache: &mut HashMap<(Stone, Iterations), Count>,
) -> Count {
    if iterations == 0 {
        return 1;
    }

    if let Some(&count) = cache.get(&(stone, iterations)) {
        return count;
    }

    let simulate_next = &mut |s| simulate_blinks(s, iterations - 1, cache);

    let count = match stone {
        0 => simulate_next(1 as Stone),
        value => {
            let pow10 = value.ilog10();

            match pow10 % 2 {
                0 => simulate_next(value * 2024 as Stone),
                _ => split(value, pow10 + 1).apply(simulate_next).sum(),
            }
        }
    };

    cache.insert((stone, iterations), count);

    count
}

fn split(value: Stone, digit_count: u32) -> (Stone, Stone) {
    let half_digit_count = digit_count / 2;
    let new_left = value / (10u64).pow(half_digit_count as u32);
    let new_right = value - (new_left * (10u64).pow(half_digit_count as u32));
    (new_left, new_right)
}

type Stone = u64;
type Count = i64;
type Iterations = u32;

fn parse_to<T>(v: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    v.parse::<T>().map_err(|_| format!("couldn't parse {v}"))
}

trait TupleFunctions<T> {
    fn apply<F, TOut>(self, f: &mut F) -> (TOut, TOut)
    where
        F: FnMut(T) -> TOut;

    fn sum(self) -> T
    where
        T: std::ops::Add<Output = T>;
}

impl<T> TupleFunctions<T> for (T, T) {
    fn apply<F, TOut>(self, f: &mut F) -> (TOut, TOut)
    where
        F: FnMut(T) -> TOut,
    {
        let (a, b) = self;
        (f(a), f(b))
    }

    fn sum(self) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        let (a, b) = self;
        a + b
    }
}

impl<I, T, E> IteratorHelpers<T, E> for I where I: Iterator<Item = Result<T, E>> {}

trait IteratorHelpers<TInput, E>: Iterator<Item = Result<TInput, E>> + Sized {
    fn map_ok<TOutput, FMutate>(self, f: FMutate) -> MapOkIterator<Self, FMutate>
    where
        FMutate: FnMut(TInput) -> TOutput,
    {
        MapOkIterator {
            iter: self,
            func: f,
        }
    }

    fn sum_ok(&mut self) -> Result<TInput, E>
    where
        TInput: std::ops::Add<Output = TInput>,
        TInput: Default,
    {
        self.try_fold(TInput::default(), |acc, res| Ok(acc + res?))
    }
}

struct MapOkIterator<I, F> {
    iter: I,
    func: F,
}

impl<I, FMutate, TInput, TOutput, E> Iterator for MapOkIterator<I, FMutate>
where
    I: Iterator<Item = Result<TInput, E>>,
    FMutate: FnMut(TInput) -> TOutput,
{
    type Item = Result<TOutput, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|result| Ok((self.func)(result?)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let value = 10;
        let (left, right) = split(value, value.ilog10() + 1);
        assert_eq!(1, left);
        assert_eq!(0, right);
    }

    #[test]
    fn test2() {
        let value = 3456;
        let (left, right) = split(value, value.ilog10() + 1);
        assert_eq!(34, left);
        assert_eq!(56, right);
    }
}

// public modules for bencher
pub mod files;
pub mod input_finder;
pub mod misc;
