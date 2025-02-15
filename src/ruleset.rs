use crate::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Index;
use std::rc::Rc;

/// A sequence of syntactic rules.
///
/// This is the semantic unit to abstract a single syntax (or language construct) which is sometimes composed of multiple rules. For example, a Kleene plus must be expressed as two rules like `A => B` and `A, B => B`.
///
/// Cloning a `Ruleset` is cheap because it's backed by an owning shared reference.
#[derive(AsRef)]
pub struct Ruleset<T>(Rc<[Subrule<T>]>);

impl<T> Ruleset<T> {
	/// Creates a new [`Ruleset`] from a name, a doc, and a sequence of [`Subrule`]s.
	pub fn new(subrules: impl IntoIterator<Item = Subrule<T>>) -> Self {
		// TODO: avoid reallocation
		let subrules = subrules.into_iter().collect::<Vec<_>>().into_boxed_slice();
		Self(Rc::from(subrules))
	}

	/// Enumerates rules that belong to this ruleset.
	pub fn rules(&self) -> impl Iterator<Item = Rule<T>> {
		(0..self.0.len())
			.into_iter()
			.zip(std::iter::repeat(self.clone()))
			.map(|(index, container)| Rule { container, index })
	}
}

impl<'a, T> IntoIterator for &'a Ruleset<T> {
	type Item = &'a Subrule<T>;
	type IntoIter = std::slice::Iter<'a, Subrule<T>>;
	fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

impl<T> Index<usize> for Ruleset<T> {
	type Output = Subrule<T>;
	fn index(&self, index: usize) -> &Self::Output { &self.0[index] }
}

impl<T> Debug for Ruleset<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Ruleset ")?;
		Display::fmt(self, f)?;
		Ok(())
	}
}

impl<T> Display for Ruleset<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{ ")?;
		for subrules in self.0.iter() {
			write!(f, "{subrules}; ")?;
		}
		write!(f, "}}")?;
		Ok(())
	}
}

impl<T> Hash for Ruleset<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		std::ptr::hash(Rc::as_ptr(&self.0), state);
	}
}

impl<T> Clone for Ruleset<T> {
	fn clone(&self) -> Self { Self(Rc::clone(&self.0)) }
}