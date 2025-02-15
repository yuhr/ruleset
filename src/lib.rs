mod digest;
pub(self) use self::digest::*;

mod matcher;
pub use self::matcher::*;

mod rule;
pub use self::rule::*;

mod subrule;
pub use self::subrule::*;

mod consequence;
pub use self::consequence::*;

mod consequent;
pub use self::consequent::*;

mod reducible;
pub use reducible::*;

mod reducer;
pub use reducer::*;

mod reductum;
pub use self::reductum::*;

mod ruleset;
pub use self::ruleset::*;

mod chart;
pub use self::chart::*;

mod reduction;
pub use reduction::*;

mod input;
pub use input::*;

mod conclusion;
pub use self::conclusion::*;

mod item;
pub use item::*;

mod addressable;
pub(self) use addressable::*;

mod child;
pub(self) use child::*;

mod child_maybe;
pub(self) use child_maybe::*;

mod r#match;
pub use r#match::*;

mod any_error;
pub use any_error::*;

mod parser;
pub use self::parser::*;

mod syntactic_category;
pub use self::syntactic_category::*;

mod tree;
pub use self::tree::*;

mod iterator_reducer_ext;
pub use self::iterator_reducer_ext::*;

mod by_matches;

pub mod prelude;

#[allow(rustdoc::invalid_rust_codeblocks)]
/// A macro for defining a ruleset.
///
/// A single ruleset is to model a [language construct](https://en.wikipedia.org/wiki/Language_construct) which is typically expressed as disjunction of some alternatives.
///
/// This macro currently doesn't support disjunctions whose alternatives have different lengths, so if you want to express a Kleene star or plus, you have to write at least two rules.
///
/// # Usage
///
/// The basic syntax of the macro body is a sequence of `r` rules, each being terminated by a semicolon:
///
/// ```rust
/// ruleset::ruleset! {
/// 	<rule 0>;
/// 	⋮
/// 	<rule r-1>;
/// }
/// ```
///
/// Each `<rule i>` consists of a sequence of `a` antecedents, a `=>`, and `c` consequents in this order, with the restriction that either `c == 1` (i.e. context-free rules) or `a == c` (i.e. context-sensitive rules) must hold:
///
/// ```rust
/// <antecedent 0>, …, <antecedent a-1> => <consequent 0>, …, <consequent c-1>
/// ```
///
/// Each `<antecedent i>` is a pattern, and each `<consequent i>` is an expression, in the sense of Rust's syntax.
///
/// In the following sections, we premise that the symbol type is defined as:
///
/// ```rust
/// enum Symbol {
/// 	Str(&'static str),
/// 	String(String),
/// 	BoxString(Box<String>),
/// 	Usize(usize),
/// 	BoxUsize(Box<usize>),
/// }
/// ```
///
/// ## Miscellaneous Tips
///
/// ### Underscores
///
/// You can use underscores not only in antecedents but also in consequents in context-sensitive rules:
///
/// ```rust
/// ruleset::ruleset! {
/// 	Str("foo"), Str("bar") => _, Str("baz");
/// 	Str("foo"), Str("bar") => Str("foo"), Str("baz");
/// }
/// ```
///
/// The above two rules are theoretically equivalent, but the former is rather efficient when it comes to computing.
///
/// ### Underscore-Dots
///
/// Context-free rules are actually a special case of context-sensitive rules where all consequents except the last one are underscore-dots:
///
/// ```rust
/// ruleset::ruleset! {
/// 	Str("foo"), Str("bar"), Str("baz") => Str("qux");
/// 	Str("foo"), Str("bar"), Str("baz") => _. _. Str("qux");
/// }
/// ```
///
/// The former is the shorthand for the latter. An underscore-dot means that the corresponding antecedent will belong to the next non-underscore-dot consequent.
///
/// Use of underscore-dots is sometimes necessary to avoid ambiguity of context-sensitive rules:
///
/// ```rust
/// ruleset::ruleset! {
/// 	// Str("foo"), Str("bar"), Str("baz") => Str("qux"), Str("quux");
/// 	Str("foo"), Str("bar"), Str("baz") => Str("qux"), _. Str("quux");
/// 	Str("foo"), Str("bar"), Str("baz") => _. Str("qux"), Str("quux");
/// }
/// ```
///
/// The first rule (commented out) is ambiguous because it's unclear whether the second antecedent corresponds to the first consequent or the second one, thus results in an error. One of the latter two rules must be used instead.
///
/// ### Early Return
///
/// Each rule is internally implemented as a closure whose return type is fallible (i.e. implements `FromResidual<Option<Infallible>>` and `FromResidual<Result<Infallible, E>>`), so you can use the `?` operator inside guards and consequent expressions:
///
/// ```rust
/// ruleset::ruleset! {
/// 	Str(str) if let n = str.parse()? => Usize(n);
/// 	Str(str) => Usize(str.parse()?);
/// }
/// ```
///
/// Above two rules are effectively equivalent, but it's a good habit to bail out of pattern matching as soon as possible; consider the following example:
///
/// ```rust
/// ruleset::ruleset! {
/// 	Str(str0), Str(str1), Str(str2) => Usize(str0.parse()? * str1.parse()? * str2.parse()?);
/// }
/// ```
///
/// In this case, validation of the first input is deferred until all the three inputs has been matched and captured. This will obviously lead to wasted computation. However, most likely you'd want to integrate error reporting mechanisms to the parser, and there may be cases where validations can't be performed until all informations to report have been collected from the inputs, so you should consider what suits your case.
///
/// ### Error Reporting
///
/// As stated in the previous section, you can return `Err(content)` from within a rule. Any value of types that implement `ruleset::AnyError` trait can be the content of an error. The returned errors are just discarded for now. The API surface around this is under consideration.
///
/// `ruleset::AnyError` is blanket-implemented when a type implements `std::any::Any + std::error::Error`.
///
/// ## Extensions to Pattern Matching Syntax and Semantics
///
/// Basically, the syntax of rules is designed to be similar to stable Rust's pattern matching syntax, but there are some extensions mimicking nightly features and proposals for improved ergonomics. These are all handled by the macro, so you can use them in stable Rust.
///
/// ### `if_let_guard`
///
/// You can use `if let` guards in antecedents, which is behind the [`if_let_guard`](https://github.com/rust-lang/rust/issues/51114) feature gate at the time of writing. Like in antecedent patterns, captured portions of guard patterns are also available for later use:
///
/// ```rust
/// ruleset::ruleset! {
/// 	Str(s) if let Ok(n) = s.parse() => Usize(n);
/// }
/// ```
///
/// The macro (ab)uses the `if let` syntax not only for guards but also for arbitrary local variable bindings that can be used for later guards or consequent values, thus `#[allow(irrefutable_let_patterns)]` is always annotated internally to suppress the warnings for trivial bindings like `if let a = 42`.
///
/// ### `let_chains`
///
/// You can conjoint or disjoint `let` bindings and normal boolean expressions in guards, which is behind the [`let_chains`](https://github.com/rust-lang/rust/issues/53667) feature gate at the time of writing:
///
/// ```rust
/// ruleset::ruleset! {
/// 	Str(s) if let Ok(n) = s.parse() && n % 42 == 0 => Usize(n);
/// }
/// ```
///
/// ### Deref Patterns
///
/// For ease of writing destructuring patterns against your nested `Deref` types, this macro supports [deref patterns](https://github.com/rust-lang/rfcs/issues/2099), so you can use `&` to dereference to the target type amidst patterns:
///
/// ```rust
/// ruleset::ruleset! {
/// 	String(&"hello") => String("world".into());
/// 	BoxString(&&"hello") => BoxString(Box::new("world".into()));
/// }
/// ```
///
/// This allows you to avoid using guards in certain cases, for example:
///
/// - `String(ref s) if s == "hello"` can be replaced with `String(&"hello")`
/// - `BoxUsize(ref n) if (0..42).contains(&**n)` can be replaced with `BoxUsize(&(0..42))`
///
/// ### Equal Constraint
///
/// It's sometimes the case that multiple fields in patterns are required to be equal, but writing such trivial constraints using guards looks verbose. You can instead repeat the same identifier for such fields:
///
/// ```rust
/// ruleset::ruleset! {
/// 	Usize(n), Usize(n) => Str("same");
/// 	Usize(n), Usize(m) if n == m => Str("same");
/// }
/// ```
///
/// Above two rules are equivalent; it internally uses the `PartialEq` equality.
///
/// This semantics won't be incorporated into the compiler at least in the near future due to [several difficulties](https://internals.rust-lang.org/t/vibe-check-equality-patterns/21775), but for us it's perfectly fine as we don't care for exhaustiveness and mutability of antecedents.
pub use ruleset_macros::ruleset;

pub(crate) use ruleset_macros::*;

#[doc(hidden)]
pub use stable_try_trait_v2::*;
