use crate::*;
use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::Range;
use std::rc::Rc;

/// An input symbol.
#[derive(Debug, AsRef)]
pub struct Input<T>(Rc<Content<T>>);

#[derive(Debug)]
struct Content<T> {
	value: T,
	cursor: usize,
}

impl<T> Input<T> {
	pub(crate) fn new(value: T, cursor: usize) -> Self { Self(Rc::new(Content { value, cursor })) }

	/// Returns a reference to the value.
	pub fn value(&self) -> &T { &self.0.value }
}

impl<T> AsRef<T> for Input<T> {
	fn as_ref(&self) -> &T { self.value() }
}

impl<T> Borrow<T> for Input<T> {
	fn borrow(&self) -> &T { self.value() }
}

impl<T> Deref for Input<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target { self.value() }
}

impl<T> Addressable for Input<T> {
	fn range(&self) -> Range<usize> { self.0.cursor..self.0.cursor + 1 }
}

impl<T: Display> Display for Input<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.value)
	}
}

impl<T> Hash for Input<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		std::ptr::hash(Rc::as_ptr(&self.0), state)
	}
}

impl<T> Clone for Input<T> {
	fn clone(&self) -> Self { Self(Rc::clone(&self.0)) }
}
