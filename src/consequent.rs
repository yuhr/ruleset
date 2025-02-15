use crate::*;
use std::num::NonZeroUsize;
use std::num::TryFromIntError;

/// A single consequent symbol of a syntactic rule.
#[derive(AsRef)]
pub struct Consequent<T> {
	len: NonZeroUsize,
	value: T,
}

impl<T> TryFrom<(usize, T)> for Consequent<T> {
	type Error = TryFromIntError;
	fn try_from((len, value): (usize, T)) -> std::result::Result<Self, Self::Error> {
		Ok(Self { len: NonZeroUsize::try_from(len)?, value })
	}
}

impl<T> Consequent<T> {
	/// Returns how many antecedents are covered by this consequent.
	pub(crate) fn len(&self) -> NonZeroUsize { self.len }

	/// Returns a reference to the value.
	pub(crate) fn value(&self) -> &T { &self.value }

	pub(crate) fn take_relevant(
		&self,
		targets: &mut impl Iterator<Item = Reducer<T>>,
	) -> Box<[Reducer<T>]> {
		let capacity = self.len().get();
		let mut targets_relevant = Vec::with_capacity(capacity);
		for _ in 0..capacity {
			let target = targets.next().unwrap();
			targets_relevant.push(target);
		}
		targets_relevant.into_boxed_slice()
	}
}