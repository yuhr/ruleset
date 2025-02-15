#![allow(unused_imports)]

pub use ruleset::prelude::*;
pub use ruleset::Chart;
pub use std::fmt::Debug;
pub use std::fmt::Display;
pub use std::fs::File;
pub use std::io::Write;

#[macro_export]
macro_rules! test_id_output {
	() => {{
		let dir_manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
		let dir_tests = format!("{dir_manifest}/tests");
		let name_file =
			std::path::Path::new(std::file!()).file_stem().and_then(|s| s.to_str()).unwrap();
		let name_test = stdext::function_name!()
			.trim_end_matches(format!("::{name_file}").as_str())
			.trim_end_matches("::test")
			.replace("::", ".");
		format!("{dir_tests}/.{name_test}")
	}};
}

#[macro_export]
macro_rules! test_id_input {
	() => {{
		let dir_manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
		let dir_tests = format!("{dir_manifest}/tests");
		let name_file =
			std::path::Path::new(std::file!()).file_stem().and_then(|s| s.to_str()).unwrap();
		let name_test = stdext::function_name!()
			.trim_end_matches(format!("::{name_file}").as_str())
			.trim_end_matches("::test")
			.replace("::", ".");
		format!("{dir_tests}/{name_test}")
	}};
}
