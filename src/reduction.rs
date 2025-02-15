use crate::*;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Range;
use std::rc::Rc;

/// A triple of a [`Reducible`], a [`Reducer`], and a [`Reductum`].
///
/// This represents a single step of derivation.
#[derive(Debug, AsRef)]
pub struct Reduction<T>(Rc<Content<T>>);

#[derive(Debug)]
struct Content<T> {
	/// The reducible this reduction was performed upon.
	pub(crate) reducible: Reducible<T>,

	/// The reducer that caused this reduction.
	pub(crate) reducer: Reducer<T>,

	/// The result of this reduction.
	pub(crate) reductum: Reductum<T>,
}

impl<T> Reduction<T> {
	pub(crate) fn new(reducible: Reducible<T>, reducer: Reducer<T>, reductum: Reductum<T>) -> Self {
		Self(Rc::new(Content { reducible, reducer, reductum }))
	}

	/// Enumerates the antecedents this reduction was performed upon.
	pub fn antecedents(&self) -> Box<dyn '_ + Iterator<Item = Reducer<T>>> {
		Box::new(
			self.mother()
				.reduction()
				.into_iter()
				.flat_map(Reduction::antecedents)
				.chain([self.father().clone()]),
		)
	}

	/// Returns a reference to the reducible, which is the first parent part of this derivation.
	pub fn reducible(&self) -> &Reducible<T> { &self.0.reducible }

	/// Returns a reference to the reducer, which is the second parent part of this derivation.
	pub fn reducer(&self) -> &Reducer<T> { &self.0.reducer }

	/// Returns a reference to the reductum, which is the child part of this derivation.
	pub fn reductum(&self) -> &Reductum<T> { &self.0.reductum }

	/// Returns a reference to the ruleset this reduction originates from.
	pub fn origin(&self) -> &Ruleset<T> { self.0.reducible.origin() }
}

impl<T> Addressable for Reduction<T> {
	fn range(&self) -> Range<usize> {
		match &self.0.reducible {
			Reducible::Rule(rule) => self.0.reducer.range(),
			Reducible::Reduction(reduction) => reduction.range().start..self.0.reducer.range().end,
		}
	}
}

impl<T> Child for Reduction<T> {
	type Mother = Reducible<T>;
	type Father = Reducer<T>;
	fn mother(&self) -> &Self::Mother { &self.0.reducible }
	fn father(&self) -> &Self::Father { &self.0.reducer }
}

impl<T> Display for Reduction<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let precision = f.precision();
		let string = Digest::from(self).to_string();
		let string = &string[0..(precision.unwrap_or(string.len()))];
		write!(f, "{string}")?;
		Ok(())
	}
}

impl<T> Hash for Reduction<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		Hash::hash(&self.0.reducible, state);
		Hash::hash(&self.0.reducer, state);
	}
}

impl<T> Clone for Reduction<T> {
	fn clone(&self) -> Self { Self(Rc::clone(&self.0)) }
}