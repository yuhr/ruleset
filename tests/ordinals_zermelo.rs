mod prelude;
use prelude::*;

#[test]
fn ordinals_zermelo() {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
	enum Symbol {
		Number(usize),
		Char(char),
	}
	impl Display for Symbol {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Debug::fmt(&self, f) }
	}
	use Symbol::*;
	let ruleset = ruleset! {
		Char('{'), Char('}') => Number(0);
		Char('{'), Number(n), Char('}') => Number(n+1);
	};
	let grammar = [&ruleset];
	let mut parser = Parser::new();
	let inputs = "{{{{{{{{}}}}}}}}";
	for input in inputs.chars().filter(|c| !c.is_ascii_whitespace()).map(Char) {
		parser.parse(input, &grammar);
	}
	let trees = parser.interpret().filter(by_matches!(Number(7))).collect_trees();
	assert!(trees.len() == 1);
	let tree = &trees[0];
	insta::assert_snapshot!(tree);
	let serialized = serde_yaml::to_string(&tree).unwrap();
	insta::assert_snapshot!(serialized);
}