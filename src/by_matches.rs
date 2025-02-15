#[allow(unused_imports)] // Used in doc comments.
use crate::*;

/// A convenient utility to filter on an iterator of [`Reducer`]s.
///
/// You can simply replace `|x| matches!(x.value(), &<pattern>)` with `by_matches!(<pattern>)`.
#[macro_export]
macro_rules! by_matches {
	($pattern:pat) => {
		|x| std::matches!(x.value(), &$pattern)
	};
}