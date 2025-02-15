use crate::*;
use stable_try_trait_v2::*;
use std::convert::Infallible;

/// A trial of a reduction.
#[derive(Debug, AsRef)]
pub enum Match<T> {
	Hit { reductum: Reductum<T> },
	Miss,
	Error { reason: Box<dyn AnyError> },
}

impl<T> Default for Match<T> {
	fn default() -> Self { Self::Miss }
}

impl<T> From<Option<Reductum<T>>> for Match<T> {
	fn from(option: Option<Reductum<T>>) -> Self {
		match option {
			Some(reductum) => Self::Hit { reductum },
			None => Default::default(),
		}
	}
}

impl<T, E: AnyError> From<Result<Reductum<T>, E>> for Match<T> {
	fn from(result: Result<Reductum<T>, E>) -> Self {
		match result {
			Ok(reductum) => Self::Hit { reductum },
			Err(content) => Self::Error { reason: Box::new(content) },
		}
	}
}

impl<T> FromResidual<Option<Infallible>> for Match<T> {
	fn from_residual(residual: Option<Infallible>) -> Self {
		match residual {
			Some(_) => unreachable!(),
			None => Default::default(),
		}
	}
}

impl<T, E: AnyError> FromResidual<Result<Infallible, E>> for Match<T> {
	fn from_residual(residual: Result<Infallible, E>) -> Self {
		match residual {
			Ok(_) => unreachable!(),
			Err(content) => Self::Error { reason: Box::new(content) },
		}
	}
}