
#[cfg(test)]
mod test {
	use super::*;

{REPLACE_tests}
	
	fn run_test(part:u8, filename:&str) -> i64 {
		use std::time::Instant;
		let now = Instant::now();

		let result = run(filename,part, false);

		match result {
			Ok(actual) =>{
				let elapsed = now.elapsed();
				println!("Elapsed: {:.2?}", elapsed);
				actual
			}
			Err(e) => {
				eprintln!("TEST FAILED for part {part} <{filename}> :: {e}");
				const TEST_FAILED:bool = false;
				assert!(TEST_FAILED);
				0
			}
		}
	}
}