use crate::*;
use std::fmt::Debug;
use std::fmt::Display;

/// Either a [`Subrule`] or a [`Consequence`].
///
/// This contains no information about the original reduction, but only the resulting part of it.
#[derive(Debug, Enum, AsRef)]
pub enum Reductum<T> {
	Subrule(Subrule<T>),
	Consequence(Consequence<T>),
}

impl<T> From<Subrule<T>> for Reductum<T> {
	fn from(subrule: Subrule<T>) -> Self { Self::Subrule(subrule) }
}

impl<T> From<Consequence<T>> for Reductum<T> {
	fn from(consequence: Consequence<T>) -> Self { Self::Consequence(consequence) }
}

impl<T> TryFrom<Reductum<T>> for Subrule<T> {
	type Error = ();
	fn try_from(reductum: Reductum<T>) -> std::result::Result<Self, Self::Error> {
		reductum.into_subrule().ok_or(())
	}
}

impl<T> TryFrom<Reductum<T>> for Consequence<T> {
	type Error = ();
	fn try_from(reductum: Reductum<T>) -> std::result::Result<Self, Self::Error> {
		reductum.into_consequence().ok_or(())
	}
}

impl<T> Display for Reductum<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Subrule(subrule) => Display::fmt(subrule, f),
			Self::Consequence(consequence) => Display::fmt(consequence, f),
		}
	}
}

impl<T> Clone for Reductum<T> {
	fn clone(&self) -> Self {
		match self {
			Self::Subrule(subrule) => Self::Subrule(subrule.clone()),
			Self::Consequence(consequence) => Self::Consequence(consequence.clone()),
		}
	}
}