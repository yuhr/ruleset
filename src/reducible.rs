use crate::*;
use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;

/// Either a [`Rule`] or a [`Reduction`] with the reductum being a [`Subrule`].
#[derive(Debug, Enum, AsRef)]
pub enum Reducible<T> {
	Rule(Rule<T>),
	Reduction(Reduction<T>),
}

impl<T> Reducible<T> {
	/// Returns a reference to the ruleset this is originated from.
	pub fn origin(&self) -> &Ruleset<T> {
		match self {
			Self::Rule(rule) => rule.ruleset(),
			Self::Reduction(reduction) => reduction.origin(),
		}
	}
}

impl<T> From<Rule<T>> for Reducible<T> {
	fn from(rule: Rule<T>) -> Self { Self::Rule(rule) }
}

impl<T> TryFrom<Reduction<T>> for Reducible<T> {
	type Error = ();
	fn try_from(reduction: Reduction<T>) -> Result<Self, Self::Error> {
		if reduction.reductum().is_subrule() {
			Ok(Self::Reduction(reduction))
		} else {
			Err(())
		}
	}
}

impl<T> Reducible<T> {
	/// Accesses a reference to the subrule in this variant transparently.
	pub fn subrule(&self) -> &Subrule<T> {
		match self {
			Self::Rule(rule) => rule.subrule(),
			Self::Reduction(reduction) => reduction.reductum().subrule().unwrap(),
		}
	}
}

impl<T> AsRef<Subrule<T>> for Reducible<T> {
	fn as_ref(&self) -> &Subrule<T> { self.subrule() }
}

impl<T> Borrow<Subrule<T>> for Reducible<T> {
	fn borrow(&self) -> &Subrule<T> { self.subrule() }
}

impl<T> Deref for Reducible<T> {
	type Target = Subrule<T>;
	fn deref(&self) -> &Self::Target { self.subrule() }
}

impl<T> Display for Reducible<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let precision = f.precision();
		let string = Digest::from(self).to_string();
		let string = &string[0..(precision.unwrap_or(string.len()))];
		write!(f, "{string}")?;
		Ok(())
	}
}

impl<T> Hash for Reducible<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Rule(rule) => Hash::hash(rule, state),
			Self::Reduction(reduction) => Hash::hash(reduction, state),
		}
	}
}

impl<T> Clone for Reducible<T> {
	fn clone(&self) -> Self {
		match self {
			Self::Rule(rule) => Self::Rule(rule.clone()),
			Self::Reduction(reduction) => Self::Reduction(reduction.clone()),
		}
	}
}

unsafe impl<T> Send for Reducible<T> {}
unsafe impl<T> Sync for Reducible<T> {}