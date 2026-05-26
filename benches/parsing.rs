use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use ruleset::prelude::*;
use ruleset::Parser;
use ruleset::Ruleset;
use ruleset::SyntacticCategory;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum JsonSymbol {
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

impl Display for JsonSymbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{self:?}") }
}

impl SyntacticCategory<char> for JsonSymbol {
	fn lexical(char: char) -> Self { Self::Char(char) }
}

fn build_json_ruleset() -> Ruleset<JsonSymbol> {
	use JsonSymbol::*;
	ruleset! {
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
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AbcSymbol {
	S,
	A(usize),
	B(usize),
	C(usize),
	Char(char),
}

impl From<char> for AbcSymbol {
	fn from(char: char) -> Self { Self::Char(char) }
}

impl Display for AbcSymbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Debug::fmt(self, f) }
}

fn build_anbncn_ruleset() -> Ruleset<AbcSymbol> {
	use AbcSymbol::*;
	ruleset! {
		A(n), B(n), C(n) => S;
		Char('a') => A(0);
		A(n), Char('a') => A(n+1);
		Char('b') => B(0);
		B(n), Char('b') => B(n+1);
		Char('c') => C(0);
		C(n), Char('c') => C(n+1);
	}
}

fn bench_json(c: &mut Criterion) {
	let input_path = format!("{}/tests/json.json", env!("CARGO_MANIFEST_DIR"));
	let json = std::fs::read_to_string(input_path).expect("failed to load tests/json.json");
	let ruleset = build_json_ruleset();
	let grammar = [&ruleset];

	c.bench_function("parse_json_full", |b| {
		b.iter(|| {
			let mut parser = Parser::new();
			parser.parse_all(black_box(json.chars()), black_box(&grammar));
			let trees = parser.interpret_as(black_box(&JsonSymbol::Json)).collect_trees();
			black_box(trees.len())
		})
	});
}

fn bench_anbncn(c: &mut Criterion) {
	let input = "aaaaaaaaaabbbbbbbbbbcccccccccc";
	let ruleset = build_anbncn_ruleset();
	let grammar = [&ruleset];

	c.bench_function("parse_anbncn_10", |b| {
		b.iter(|| {
			let mut parser = Parser::new();
			parser.parse_all(black_box(input.chars()), black_box(&grammar));
			let trees = parser.interpret().filter(by_matches!(AbcSymbol::S)).collect_trees();
			black_box(trees.len())
		})
	});
}

criterion_group!(benches, bench_json, bench_anbncn);
criterion_main!(benches);