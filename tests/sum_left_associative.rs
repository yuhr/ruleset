mod prelude;
use prelude::*;

#[test]
fn sum_left_associative() {
	let test_id_output = test_id_output!();

	#[derive(Debug, PartialEq, Eq, Hash, Clone)]
	enum Symbol {
		Sum(usize, usize),
		Usize(usize),
	}
	use Symbol::*;
	impl From<usize> for Symbol {
		fn from(value: usize) -> Self { Usize(value) }
	}
	impl Display for Symbol {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Debug::fmt(&self, f) }
	}
	let ruleset = ruleset! {
		Sum(l, n), Usize(m) => Sum(l + 1, n + m);
		Usize(n), Usize(m) => Sum(0, n + m);
	};
	let mut parser = Parser::new();
	parser.parse_all([0, 1, 2], &[&ruleset]);
	let trees = parser.interpret().filter(by_matches!(Sum(_, 3))).collect_trees();
	assert!(trees.len() == 1);
	for tree in trees {
		println!("{tree}");
	}

	println!("{parser}");

	let mut file = File::create(format!("{test_id_output}.table")).unwrap();
	file.write_all(&parser.to_string().into_bytes()).unwrap();
}