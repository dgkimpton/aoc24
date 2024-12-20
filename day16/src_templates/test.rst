	#[test]
	fn test_{REPLACE_n}_part{REPLACE_part}() {
		let actual = run_test({REPLACE_part}, "{REPLACE_filename}");	
		assert_eq!({REPLACE_expected}, actual);		
	}