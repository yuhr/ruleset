use crate::*;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Range;

/// Either a [`Reducer`] or a [`Reduction`].
#[derive(Debug, Enum, AsRef)]
pub enum Item<T> {
	Reducer(Reducer<T>),
	Reduction(Reduction<T>),
}

impl<T> From<Reducer<T>> for Item<T> {
	fn from(reducer: Reducer<T>) -> Self { Self::Reducer(reducer) }
}

impl<T> From<Reduction<T>> for Item<T> {
	fn from(reduction: Reduction<T>) -> Self { Self::Reduction(reduction) }
}

impl<T> Item<T> {
	/// Returns a reference to the ruleset this is originated from, if any.
	pub fn origin(&self) -> Option<&Ruleset<T>> {
		match self {
			Self::Reduction(reduction) => Some(reduction.origin()),
			Self::Reducer(reducer) => reducer.origin(),
		}
	}
}

impl<T> Addressable for Item<T> {
	fn range(&self) -> Range<usize> {
		match self {
			Self::Reduction(reduction) => reduction.range(),
			Self::Reducer(reducer) => reducer.range(),
		}
	}
}

impl<T> Hash for Item<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		match self {
			Self::Reduction(reduction) => Hash::hash(reduction, state),
			Self::Reducer(reducer) => Hash::hash(reducer, state),
		}
	}
}

impl<T> Clone for Item<T> {
	fn clone(&self) -> Self {
		match self {
			Self::Reduction(reduction) => Self::Reduction(reduction.clone()),
			Self::Reducer(reducer) => Self::Reducer(reducer.clone()),
		}
	}
}

impl<T: Display> Display for Item<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let precision = f.precision();
		let string = Digest::from(self).to_string();
		let string = &string[0..(precision.unwrap_or(string.len()))];
		match self {
			Self::Reduction(reduction) => {
				write!(f, "Δ{string}∵")?;
				Display::fmt(reduction.reducible(), f)?;
				if reduction.reducible().is_rule() {
					write!(f, "⋅")?;
				} else {
					write!(f, ":")?;
				}
				Display::fmt(reduction.reducer(), f)?;
				write!(f, "\n└─")?;
				Display::fmt(reduction.reductum(), f)?;
			}
			Self::Reducer(reducer) => {
				match reducer {
					Reducer::Input(input) => {
						write!(f, "ϛ{string}")?;
					}
					Reducer::Conclusion(conclusion) => {
						write!(f, "ϛ{string}∵")?;
						Display::fmt(conclusion.reduction(), f)?;
					}
				}
				write!(f, "\n└─")?;
				Display::fmt(reducer.value(), f)?;
			}
		}
		Ok(())
	}
}