use crate::*;
use either::Either;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::Range;

/// Either an [`Input`] or a [`Conclusion`].
///
/// This can be viewed as a parsed syntax tree. You can get a tree representation by [`Tree::from`].
#[derive(Debug, Enum, AsRef)]
pub enum Reducer<T> {
	Input(Input<T>),
	Conclusion(Conclusion<T>),
}

impl<T> From<Input<T>> for Reducer<T> {
	fn from(input: Input<T>) -> Self { Self::Input(input) }
}

impl<T> From<Conclusion<T>> for Reducer<T> {
	fn from(conclusion: Conclusion<T>) -> Self { Self::Conclusion(conclusion) }
}

impl<T> Reducer<T> {
	pub(crate) fn overlaps(&self, other: &Self) -> bool {
		let range_a = self.range();
		let range_b = other.range();
		range_a.start < range_b.end && range_a.end > range_b.start
	}

	/// Accesses the value in this variant transparently.
	pub fn value(&self) -> &T {
		match self {
			Self::Input(input) => &input.value(),
			Self::Conclusion(conclusion) => conclusion.value(),
		}
	}

	/// Enumerates the antecedents this reducer covers.
	pub fn antecedents(&self) -> impl Iterator<Item = &Reducer<T>> + DoubleEndedIterator {
		self.conclusion().into_iter().flat_map(Conclusion::antecedents)
	}

	/// Enumerates the antecedents this reducer covers. For each antecedent reducer with the value being hash-equal to the value of this reducer, the antecedents of it are flattened, recursively.
	pub fn antecedents_flattened(&self) -> Box<dyn '_ + Iterator<Item = &Reducer<T>>>
	where T: Hash {
		Box::new(self.antecedents().flat_map(|antecedent| {
			if Digest::from(antecedent.value()) == Digest::from(self.value()) {
				Either::Left(antecedent.antecedents_flattened())
			} else {
				Either::Right(std::iter::once(antecedent))
			}
		}))
	}

	/// Returns a reference to the ruleset this is originated from, if any.
	pub fn origin(&self) -> Option<&Ruleset<T>> {
		match self {
			Self::Input(input) => None,
			Self::Conclusion(conclusion) => Some(conclusion.origin()),
		}
	}
}

impl<T> AsRef<T> for Reducer<T> {
	fn as_ref(&self) -> &T { self.value() }
}

impl<T> Borrow<T> for Reducer<T> {
	fn borrow(&self) -> &T { self.value() }
}

impl<T> Deref for Reducer<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target { self.value() }
}

impl<T> Addressable for Reducer<T> {
	fn range(&self) -> Range<usize> {
		match self {
			Self::Input(input) => input.range(),
			Self::Conclusion(conclusion) => conclusion.range(),
		}
	}
}

impl<T> ChildMaybe for Reducer<T> {
	type Mother = Reducible<T>;
	type Father = Reducer<T>;
	fn mother_maybe(&self) -> Option<&Self::Mother> { self.conclusion().map(Child::mother) }
	fn father_maybe(&self) -> Option<&Self::Father> { self.conclusion().map(Child::father) }
}

impl<T> Display for Reducer<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let precision = f.precision();
		let string = Digest::from(self).to_string();
		let string = &string[0..(precision.unwrap_or(string.len()))];
		write!(f, "{string}")?;
		Ok(())
	}
}

impl<T> Hash for Reducer<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Input(input) => Hash::hash(input, state),
			Self::Conclusion(conclusion) => Hash::hash(conclusion, state),
		}
	}
}

impl<T> Clone for Reducer<T> {
	fn clone(&self) -> Self {
		match self {
			Self::Input(input) => Self::Input(input.clone()),
			Self::Conclusion(conclusion) => Self::Conclusion(conclusion.clone()),
		}
	}
}