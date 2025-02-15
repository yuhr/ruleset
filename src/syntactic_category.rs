use std::hash::Hash;

/// Types that formulate [syntactic categories](https://en.wikipedia.org/wiki/Syntactic_category) of a grammar.
///
/// The supertrait [`Hash`] is required to prune the search space and to flatten resulting syntax trees.
pub trait SyntacticCategory<W>: Sized + Hash {
	/// Returns a variant of `Self` that injects the input word into `Self`. Those variants will become the label of leaf nodes of parsed syntax trees.
	///
	/// This would usually be a single variant that simply wraps the input word, but there's nothing to stop you from returning variants that represent lexical categories (i.e. part-of-speech-like categorization of words) directly.
	fn lexical(word: W) -> Self;

	/// Returns a variant of `Self` that can be used as a fallback when the value returned by [`SyntacticCategory::lexical`] didn't cause any reductions.
	///
	/// This is to handle unknown words and error recovery.
	fn unknown(word: W) -> Option<Self> { None }
}

impl<W, T: Sized + Hash + From<W>> SyntacticCategory<W> for T {
	fn lexical(word: W) -> Self { Self::from(word) }
}
