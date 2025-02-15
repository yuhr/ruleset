mod prelude;
use prelude::*;

// cf. <https://www.json.org/json-en.html>

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Symbol {
	Json,
	Value,
	Object,
	Members,
	Member,
	Array,
	Elements,
	Element,
	String,
	Characters,
	Character,
	Escape,
	Hex,
	Number,
	Integer,
	Digits,
	Digit,
	Onenine,
	Fraction,
	Exponent,
	Sign,
	Whitespace,
	True,
	False,
	Null,
	Char(char),
}

impl Display for Symbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{self:?}") }
}

impl SyntacticCategory<char> for Symbol {
	fn lexical(char: char) -> Self { Self::Char(char) }
}

pub struct ParserJson {
	pub parser: Parser<char, Symbol>,
	pub ruleset: Ruleset<Symbol>,
}

impl ParserJson {
	pub fn new() -> Self {
		use Symbol::*;
		let ruleset = ruleset! {
			Element => Json;

			Object | Array | String | Number => Value;
			Char('t'), Char('r'), Char('u'), Char('e') => True;
			Char('f'), Char('a'), Char('l'), Char('s'), Char('e') => False;
			Char('n'), Char('u'), Char('l'), Char('l') => Null;
			True | False | Null => Value;

			Char('{'), Char('}') => Object;
			Char('{'), Whitespace | Members, Char('}') => Object;

			Member => Members;
			Member, Char(','), Members => Members;

			String, Char(':'), Element => Member;
			Whitespace, String, Char(':'), Element => Member;
			String, Whitespace, Char(':'), Element => Member;
			Whitespace, String, Whitespace, Char(':'), Element => Member;

			Char('['), Char(']') => Array;
			Char('['), Whitespace | Elements, Char(']') => Array;

			Element => Elements;
			Element, Char(','), Elements => Elements;

			Value => Element;
			Whitespace, Value => Element;
			Value, Whitespace => Element;
			Whitespace, Value, Whitespace => Element;

			Char('\"'), Char('\"') => String;
			Char('\"'), Characters, Char('\"') => String;

			Character => Characters;
			Character, Characters => Characters;

			Char(c @ '\u{20}'..='\u{10FFFF}') if c != '\"' && c != '\\' => Character;
			Char('\\'), Escape => Character;

			Char('\"') => Escape;
			Char('\\') => Escape;
			Char('/') => Escape;
			Char('b') => Escape;
			Char('f') => Escape;
			Char('n') => Escape;
			Char('r') => Escape;
			Char('t') => Escape;
			Char('u'), Hex, Hex, Hex, Hex => Escape;

			Digit => Hex;
			Char(('A'..='F') | ('a'..='f')) => Hex;

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

			Char('\u{20}') => Whitespace;
			Char('\u{20}'), Whitespace => Whitespace;
			Char('\u{0A}') => Whitespace;
			Char('\u{0A}'), Whitespace => Whitespace;
			Char('\u{0D}') => Whitespace;
			Char('\u{0D}'), Whitespace => Whitespace;
			Char('\u{09}') => Whitespace;
			Char('\u{09}'), Whitespace => Whitespace;
		};

		let parser = Parser::new();
		Self { parser, ruleset }
	}
}

#[test]
fn json() {
	let test_id_input = test_id_input!();
	let test_id_output = test_id_output!();

	let ParserJson { mut parser, ruleset } = ParserJson::new();
	let json = std::fs::read_to_string(format!("{test_id_input}.json")).unwrap();
	parser.parse_all(json.chars(), &[&ruleset]);
	let trees = parser.interpret_as(&Symbol::Json).collect_trees();
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