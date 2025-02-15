use crate::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;

/// A syntactic rule.
///
/// This is actually an owning shared reference to a [`Subrule`] in a [`Ruleset`].
#[derive(AsRef)]
pub struct Rule<T> {
	pub(crate) container: Ruleset<T>,
	pub(crate) index: usize,
}

impl<T> Rule<T> {
	/// Returns a reference to the `Ruleset` this rule belongs to.
	pub fn ruleset(&self) -> &Ruleset<T> { &self.container }

	/// Returns a reference to the `Subrule` this rule points to.
	pub fn subrule(&self) -> &Subrule<T> { &self.container[self.index] }
}

impl<T> Deref for Rule<T> {
	type Target = Subrule<T>;
	fn deref(&self) -> &Self::Target { self.subrule() }
}

impl<T> Debug for Rule<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Rule(")?;
		Debug::fmt(self.subrule(), f)?;
		write!(f, ")")?;
		Ok(())
	}
}

impl<T> Display for Rule<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(self.subrule(), f)
	}
}

impl<T> Hash for Rule<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		Hash::hash(&self.container, state);
		Hash::hash(&self.index, state);
	}
}

impl<T> Clone for Rule<T> {
	fn clone(&self) -> Self { Self { container: self.container.clone(), index: self.index } }
}