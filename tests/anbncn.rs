mod prelude;
use prelude::*;

#[test]
fn anbncn() {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
	enum Symbol {
		S,
		A(usize),
		B(usize),
		C(usize),
		Char(char),
	}
	impl From<char> for Symbol {
		fn from(char: char) -> Self { Char(char) }
	}
	impl Display for Symbol {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Debug::fmt(&self, f) }
	}
	use Symbol::*;
	let ruleset = ruleset! {
		A(n), B(n), C(n) => S;
		Char('a') => A(0);
		A(n), Char('a') => A(n+1);
		Char('b') => B(0);
		B(n), Char('b') => B(n+1);
		Char('c') => C(0);
		C(n), Char('c') => C(n+1);
	};
	let grammar = [&ruleset];
	{
		let mut parser = Parser::new();
		let inputs = "aaabbbccc";
		for input in inputs.chars() {
			parser.parse(input, &grammar);
		}
		let trees = parser.interpret().filter(by_matches!(S)).collect_trees();
		assert!(trees.len() == 1);
		let tree = &trees[0];
		insta::assert_snapshot!(tree);
		let serialized = serde_yaml::to_string(&tree).unwrap();
		insta::assert_snapshot!(serialized);
	}
	{
		let mut parser = Parser::new();
		let inputs = "aaabccc";
		for input in inputs.chars().filter(|c| !c.is_ascii_whitespace()).map(Char) {
			parser.parse(input, &grammar);
		}
		let trees = parser.interpret().filter(by_matches!(S)).collect_trees();
		assert!(trees.len() == 0);
	}
}