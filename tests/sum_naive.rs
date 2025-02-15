mod prelude;
use prelude::*;

#[test]
fn sum_naive() {
	let test_id_output = test_id_output!();

	let ruleset = ruleset! {
		n, m => n + m;
	};
	let mut parser = Parser::new();
	parser.parse_all([0, 1, 2], &[&ruleset]);
	let trees = parser.interpret_as(3).collect_trees();
	assert!(trees.len() == 2);
	for tree in trees {
		println!("{tree}");
	}

	let mut file = File::create(format!("{test_id_output}.table")).unwrap();
	file.write_all(&parser.to_string().into_bytes()).unwrap();
}
