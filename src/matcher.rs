use crate::*;

/// Types that can be implementations of [`Subrule`]s.
pub trait Matcher<T>: Sync + Send + for<'a> Fn(&'a T) -> Match<T> {}

impl<T, M> Matcher<T> for M where M: Sync + Send + for<'a> Fn(&'a T) -> Match<T> {}
