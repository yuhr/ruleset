use crate::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Index;
use std::rc::Rc;

/// A zero-antecedent suffix of a syntactic rule.
///
/// When a reduction has reached to the consequence of a rule, each consequent will be plotted at the cell of the corresponding antecedent in the chart. For example, when we have rules `A, B => _, C` and `A, C => D`, and as inputs `A, B` is given, we have a successful reduction that reaches to the consequence `=> _, C`; this causes the parser to “overlay” the consequent `C` at the cell of `B`, and now we have another successful reduction that reaches to `=> D`.
#[derive(AsRef)]
pub struct Consequence<T> {
	consequents: Rc<[Option<Consequent<T>>]>,
	debug_text: Option<String>,
}

impl<T> Consequence<T> {
	/// Creates a new `Consequence` from a sequence of `Consequent`s.
	pub fn new(consequents: impl IntoIterator<Item = Option<Consequent<T>>>) -> Self {
		Self { consequents: consequents.into_iter().collect(), debug_text: None }
	}

	/// Creates a new `Consequence` from a sequence of `Consequent`s with a debug text which is usually expected to be a textual representation of how the consequence would be written in the `ruleset!` macro, without the `=>`.
	pub fn new_with_debug_text(
		consequents: impl IntoIterator<Item = Option<Consequent<T>>>,
		debug_text: impl AsRef<str>,
	) -> Self {
		Self {
			consequents: consequents.into_iter().collect(),
			debug_text: Some(debug_text.as_ref().to_string()),
		}
	}

	/// Enumerates the contents of this consequence.
	pub fn consequents(&self) -> &[Option<Consequent<T>>] { &self.consequents }

	fn debug_text(&self) -> String {
		self.debug_text.as_ref().map(|debug_text| format!("=> {debug_text}")).unwrap_or_else(|| {
			let digest: Digest<*const [Option<Consequent<T>>]> =
				Digest::from(&(self.consequents.as_ref() as _));
			format!("<dynamic:{digest}>")
		})
	}
}

impl<T> Index<usize> for Consequence<T> {
	type Output = Option<Consequent<T>>;
	fn index(&self, index: usize) -> &Self::Output { &self.consequents[index] }
}

impl<T> Debug for Consequence<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Consequence({})", self.debug_text())
	}
}

impl<T> Display for Consequence<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.debug_text())
	}
}

impl<T> Clone for Consequence<T> {
	fn clone(&self) -> Self {
		Self { consequents: Rc::clone(&self.consequents), debug_text: self.debug_text.clone() }
	}
}