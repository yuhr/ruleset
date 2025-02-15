mod prelude;
use prelude::*;
use ruleset::*;

#[test]
fn lifetime_0() {
	enum Symbol<'src> {
		Word(&'src str),
	}
	use Symbol::*;
	let rule = ruleset! {
		Word(s), Word(t) if s == t => Word("ok");
	};
}

#[test]
fn subrules_0() {
	enum Symbol {
		Word(&'static str),
		Number(u32),
	}
	let reductum = |value: u32| {
		Reductum::from(Consequence::new([Some((1, Symbol::Number(value)).try_into().unwrap())]))
	};
	let reductum = Reductum::from(Subrule::new(
		move |input: &Symbol| match input {
			&Symbol::Word(s) => s.parse().map(reductum).into(),
			_ => Default::default(),
		},
		None::<&str>,
	));
}

#[test]
fn negation_0() {
	let ruleset = ruleset! {
		!"A" => "A";
	};
	let grammar = [&ruleset];
	let mut parser = Parser::new();
	let inputs = "X".split_whitespace().collect::<Vec<_>>();
	let inputs = inputs.iter().copied().collect::<Vec<_>>();
	for input in inputs.iter().copied() {
		parser.parse(input, &grammar);
	}

	println!("{parser}");
	assert!(parser.interpret().any(by_matches!("A")));
}

#[test]
fn context_sensitivity_left_0() {
	let ruleset = ruleset! {
		"A", "B" => _, "C";
		"A", "C" => "S";
	};
	let grammar = [&ruleset];
	let mut parser = Parser::new();
	let inputs = "A B".split_whitespace();
	for input in inputs {
		parser.parse(input, &grammar);
	}
	assert!(parser.interpret().all(by_matches!("S")));
	assert_eq!(parser.interpret().count(), 1);
}

#[test]
fn context_sensitivity_right_0() {
	let ruleset = ruleset! {
		"A", "B" => "C", _;
		"C", "B" => "S";
	};
	let grammar = [&ruleset];
	let mut parser = Parser::new();
	let inputs = "A B".split_whitespace();
	for input in inputs {
		parser.parse(input, &grammar);
	}
	assert!(parser.interpret().all(by_matches!("S")));
	assert_eq!(parser.interpret().count(), 1);
}

#[test]
fn context_sensitivity_0() {
	let ruleset = ruleset! {
		"A", "B", "C" => _, "P", _;
		"A", "P", "C" => _, "Q", _;
		"A", "Q", "C" => _, "R", _;
		"A", "R", "C" => "S";
	};
	let grammar = [&ruleset];
	let mut parser = Parser::new();
	let inputs = "A B C".split_whitespace();
	for input in inputs {
		parser.parse(input, &grammar);
	}
	assert!(parser.interpret().all(by_matches!("S")));
	assert_eq!(parser.interpret().count(), 1);
}

#[test]
fn context_sensitivity_1() {
	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	enum Symbol {
		A(usize),
		B(usize),
		C(usize),
		Input(&'static str),
		Accepted,
	}
	impl Display for Symbol {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{self:?}") }
	}
	use Symbol::*;
	let ruleset = ruleset! {
		Input("A") => A(1);
		Input("A"), A(a) => A(a + 1);
		Input("B") => B(1);
		Input("B"), B(b) => B(b + 1);
		Input("C") => C(1);
		Input("C"), C(c) => C(c + 1);
		A(_), B(b) => C(b), _;
		C(b), B(b) => Accepted;
	};
	let grammar = [&ruleset];
	let mut parser = Parser::new();
	let inputs = "A B B".split_whitespace().map(Input);
	for input in inputs {
		println!("{parser}");
		parser.parse(input, &grammar);
	}
	println!("{parser}");
	assert!(parser.interpret().all(by_matches!(Accepted)));
	assert_eq!(parser.interpret().count(), 1);
}

#[test]
fn usize_0() {
	#[derive(Debug, Hash, Clone)]
	enum Symbol {
		Word(&'static str),
		Usize(usize),
	}
	use Symbol::*;
	let ruleset = ruleset! {
		Word(str) if let n = str.parse()? => Usize(n);
		Word(str) => Usize(str.parse()?);
	};
	let grammar = [&ruleset];
	let mut parser = Parser::new();
	let inputs = "42".split_whitespace().map(Word).collect::<Vec<_>>();
	for input in inputs {
		parser.parse(input, &grammar);
	}
	let result = parser.interpret().filter(by_matches!(Usize(42)));
	assert!(result.count() == 2);
}

#[test]
fn derefs_0() {
	enum Symbol {
		Word(Box<String>),
		Number(Box<usize>),
	}
	use Symbol::*;
	let ruleset = ruleset! {
		Word(&&"hello") => Word(Default::default());
		Number(&(42..43)) if let s = true && s == true => Word(Default::default());
		Number(&n @ 0..42) => Number(Box::new(n * 2));
		Number(&n), Number(&n) => Number(Box::new(n * n));
	};
}

#[test]
fn dynamic_0() {
	let test_id_output = test_id_output!();
	let mut inputs = "A B C".split_ascii_whitespace();
	let ruleset_ab = ruleset! {
		"A" => "B";
	};
	let ruleset_bc = ruleset! {
		"B" => "C";
	};
	let ruleset_cd = ruleset! {
		"C" => "D";
	};
	let mut parser = Parser::new();
	parser.parse(inputs.next().unwrap(), &[&ruleset_ab]);
	assert!(parser
		.interpret_within(parser.chart().len() - 1..parser.chart().len())
		.any(by_matches!("B")));
	parser.parse(inputs.next().unwrap(), &[&ruleset_bc]);
	assert!(parser
		.interpret_within(parser.chart().len() - 1..parser.chart().len())
		.any(by_matches!("C")));
	parser.parse(inputs.next().unwrap(), &[&ruleset_cd]);
	assert!(parser
		.interpret_within(parser.chart().len() - 1..parser.chart().len())
		.any(by_matches!("D")));

	println!("{}", &parser);
	let interpretation = parser
		.interpret_within(parser.chart().len() - 1..parser.chart().len())
		.find(by_matches!("D"))
		.unwrap();
	let tree = Tree::from(interpretation);
	insta::assert_snapshot!(tree);
}