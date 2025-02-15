mod prelude;
use prelude::*;

#[test]
fn arithmetic() {
	let test_id_output = test_id_output!();

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
	enum Symbol {
		Expression,
		Atomic,
		Composite,
		Additive(usize),
		Multiplicative(usize),
		Exponentiation(usize),
		Number,
		Integer,
		Digits,
		Digit,
		Onenine,
		Fraction,
		Exponent,
		Sign,
		Char(char),
	}
	impl Display for Symbol {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Debug::fmt(&self, f) }
	}
	use Symbol::*;
	let ruleset = ruleset! {
		Atomic | Composite => Expression;

		Number => Atomic;
		Char('('), Atomic | Composite, Char(')') => Atomic;
		Additive(_) | Multiplicative(_) | Exponentiation(_) => Composite;

		Additive(n), Char('+' | '-'), Atomic | Multiplicative(_) => Additive(n+1);
		Multiplicative(_) | Atomic, Char('+' | '-'), Atomic | Multiplicative(_) => Additive(0);
		Multiplicative(n), Char('*' | '/'), Atomic | Exponentiation(_) => Multiplicative(n+1);
		Exponentiation(_) | Atomic, Char('*' | '/'), Atomic | Exponentiation(_) => Multiplicative(0);
		Atomic, Char('^'), Exponentiation(n) => Exponentiation(n+1);
		Atomic, Char('^'), Atomic => Exponentiation(0);

		Integer => Number;
		Integer, Fraction | Exponent => Number;
		Integer, Fraction, Exponent => Number;

		Digit => Integer;
		Onenine, Digits => Integer;
		Char('-'), Digit => Integer;
		Char('-'), Onenine, Digits => Integer;

		Digit => Digits;
		Digit, Digits => Digits;

		Char('0') | Onenine => Digit;

		Char('1'..='9') => Onenine;

		Char('.'), Digits => Fraction;

		Char('E' | 'e'), Digits => Exponent;
		Char('E' | 'e'), Sign, Digits => Exponent;

		Char('+' | '-') => Sign;
	};
	let grammar = [&ruleset];
	let mut parser = Parser::new();
	let inputs = "0 + 1 * 2 ^ 3 ^ 4 / 5 - 6";
	for input in inputs.chars().filter(|c| !c.is_ascii_whitespace()).map(Char) {
		parser.parse(input, &grammar);
	}
	let trees = parser.interpret().filter(by_matches!(Expression)).collect_trees();
	assert!(trees.len() == 1);
	let tree = &trees[0];

	insta::assert_snapshot!(tree);
	let mut file = File::create(format!("{test_id_output}.tree")).unwrap();
	file.write_all(&tree.to_string().into_bytes()).unwrap();

	let serialized = serde_yaml::to_string(&tree).unwrap();
	insta::assert_snapshot!(serialized);
	let mut file = File::create(format!("{test_id_output}.yaml")).unwrap();
	serde_yaml::to_writer(&mut file, &tree).unwrap();
}