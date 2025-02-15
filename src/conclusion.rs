use crate::*;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::Range;
use std::rc::Rc;

/// A consequent symbol yielded from a reduction.
#[derive(Debug, AsRef)]
pub struct Conclusion<T>(Rc<Content<T>>);

#[derive(Debug)]
struct Content<T> {
	reduction: Reduction<T>,
	index: usize,
	antecedents: Box<[Reducer<T>]>,
	start: usize,
	end: usize,
}

impl<T> Conclusion<T> {
	pub(crate) fn new(
		reduction: Reduction<T>,
		index: usize,
		antecedents: Box<[Reducer<T>]>,
	) -> Self {
		let start = antecedents.first().unwrap().range().start;
		let end = antecedents.last().unwrap().range().end;
		Self(Rc::new(Content { reduction, index, antecedents, start, end }))
	}

	/// Returns a reference to the value.
	pub fn value(&self) -> &T {
		self.0.reduction.reductum().consequence().unwrap()[self.0.index].as_ref().unwrap().value()
	}

	/// Returns a reference to the ruleset this is originated from.
	pub fn origin(&self) -> &Ruleset<T> { self.0.reduction.origin() }

	/// Returns a reference to the reduction this is originated from.
	pub fn reduction(&self) -> &Reduction<T> { &self.0.reduction }

	/// Enumerates the antecedents this conclusion covers.
	pub fn antecedents(&self) -> &[Reducer<T>] { &self.0.antecedents }
}

impl<T> AsRef<T> for Conclusion<T> {
	fn as_ref(&self) -> &T { self.value() }
}

impl<T> Borrow<T> for Conclusion<T> {
	fn borrow(&self) -> &T { self.value() }
}

impl<T> Deref for Conclusion<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target { self.value() }
}

impl<T> Addressable for Conclusion<T> {
	fn range(&self) -> Range<usize> { self.0.start..self.0.end }
}

impl<T> Child for Conclusion<T> {
	type Mother = Reducible<T>;
	type Father = Reducer<T>;
	fn mother(&self) -> &Self::Mother { self.0.reduction.mother() }
	fn father(&self) -> &Self::Father { self.0.reduction.father() }
}

impl<T: Display> Display for Conclusion<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let precision = f.precision();
		let string = Digest::from(&self.0.reduction).to_string();
		let string = &string[0..(precision.unwrap_or(string.len()))];
		write!(f, "{}({string})", self.value())
	}
}

impl<T> Hash for Conclusion<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		std::ptr::hash(Rc::as_ptr(&self.0), state)
	}
}

impl<T> Clone for Conclusion<T> {
	fn clone(&self) -> Self { Self(Rc::clone(&self.0)) }
}
