use crate::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;

/// A non-zero-antecedent suffix of a syntactic rule.
#[derive(AsRef)]
pub struct Subrule<T> {
	matcher: Rc<dyn Matcher<T>>,
	debug_text: Option<String>,
}

impl<T> Subrule<T> {
	/// Creates a new [`Subrule`] from a [`Matcher`].
	///
	/// The debug text is typically a textual representation of what the subrule looks like when written in the [`ruleset!`] macro.
	pub fn new(matcher: impl 'static + Matcher<T>, debug_text: Option<impl AsRef<str>>) -> Self {
		Self {
			matcher: Rc::new(matcher),
			debug_text: debug_text.map(|debug_text| debug_text.as_ref().to_string()),
		}
	}

	/// Performs a reduction based on this [`Subrule`] with an input.
	pub fn reduce(&self, input: &T) -> Match<T> { (self.matcher)(input) }

	fn debug_text(&self) -> String {
		self.debug_text.clone().unwrap_or_else(|| {
			let digest: Digest<*const dyn Matcher<T>> = Digest::from(&(self.matcher.as_ref() as _));
			format!("<dynamic:{digest}>")
		})
	}
}

impl<T> Deref for Subrule<T> {
	type Target = Rc<dyn Matcher<T>>;
	fn deref(&self) -> &Self::Target { &self.matcher }
}

impl<T> Debug for Subrule<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Subrule({})", self.debug_text())
	}
}

impl<T> Display for Subrule<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.debug_text())
	}
}

impl<T> Clone for Subrule<T> {
	fn clone(&self) -> Self {
		Self { matcher: Rc::clone(&self.matcher), debug_text: self.debug_text.clone() }
	}
}