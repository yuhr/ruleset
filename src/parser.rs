use crate::*;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::RangeBounds;

/// A featureful wrapper around a [`Chart`].
#[derive(Debug, Clone, AsRef)]
pub struct Parser<W, P = W> {
	chart: Chart<P>,
	history: Vec<W>,
}

impl<W, P> Default for Parser<W, P> {
	fn default() -> Self { Self::new() }
}

impl<W, P> Parser<W, P> {
	/// Creates an empty parser.
	pub fn new() -> Self { Self { chart: Chart::new(), history: Vec::new() } }

	/// Returns a reference to the chart.
	pub fn chart(&self) -> &Chart<P> { &self.chart }

	/// Returns a slice of the history of inputs.
	pub fn history(&self) -> &[W] { self.history.as_slice() }

	/// Enumerates reducers that covers the entire inputs. This is equivalent to `interpret_within(..)`.
	pub fn interpret(&self) -> impl '_ + Iterator<Item = &Reducer<P>> + DoubleEndedIterator {
		self.interpret_within(..)
	}

	/// Enumerates reducers that covers the given range of inputs.
	pub fn interpret_within(
		&self,
		bounds: impl RangeBounds<usize>,
	) -> impl Iterator<Item = &Reducer<P>> + DoubleEndedIterator {
		let (cursor, offset) = self.chart.range_from(bounds).address();
		self.chart.cell(cursor, offset).into_iter().flatten().filter_map(Item::reducer)
	}

	/// Enumerates reducers that covers the entire inputs, filtering by the hash equality to the given one. This is equivalent to filtering elements after `interpret()`.
	pub fn interpret_as(
		&self,
		value: impl Borrow<P>,
	) -> impl Iterator<Item = &Reducer<P>> + DoubleEndedIterator
	where
		P: Hash,
	{
		let digest = Digest::from(value.borrow());
		self.interpret().filter(move |i| Digest::from(i.value()) == digest)
	}

	/// Enumerates reducible reductions that can cover the entire inputs. This is equivalent to `analyze_within(..)`.
	pub fn analyze(&self) -> impl Iterator<Item = &Reduction<P>> + DoubleEndedIterator {
		self.analyze_within(..)
	}

	/// Enumerates reducible reductions that can cover the given range of inputs.
	pub fn analyze_within(
		&self,
		bounds: impl RangeBounds<usize>,
	) -> impl Iterator<Item = &Reduction<P>> + DoubleEndedIterator {
		let range = self.chart.range_from(bounds);
		self.chart
			.items_diagonally_within(range)
			.filter_map(Item::reduction)
			.filter(|reduction| reduction.reductum().is_subrule())
	}

	/// Enumerates reducible reductions that can cover the entire inputs, filtering by the hash equality of the original ruleset of each reduction to the given ruleset. This is equivalent to filtering elements after `analyze()`.
	pub fn analyze_for(
		&self,
		ruleset: impl Borrow<Ruleset<P>>,
	) -> impl Iterator<Item = &Reduction<P>> + DoubleEndedIterator {
		let digest = Digest::from(ruleset.borrow());
		self.analyze().filter(move |reduction| Digest::from(reduction.origin()) == digest)
	}

	/// Returns whether the last `parse` didn't cause any reductions.
	fn is_rejecting(&self) -> bool {
		self.chart
			.items_horizontally_on(self.chart.len() - 1)
			.all(|item| item.reducer().is_some_and(Reducer::is_input))
	}

	/// Returns whether the last cell on the last row contains no reducer.
	fn is_incomplete(&self) -> bool { self.interpret().count() == 0 }
}

impl<W: Clone, P: SyntacticCategory<W>> Parser<W, P> {
	/// Performs a single parsing step.
	pub fn parse(&mut self, word: W, grammar: &[&Ruleset<P>]) {
		let phrase = SyntacticCategory::lexical(word.clone());
		self.history.push(word.clone());
		self.chart.push(phrase, grammar);
		if self.is_rejecting() {
			if let Some(phrase) = SyntacticCategory::unknown(word) {
				let word = self.unparse().unwrap();
				self.history.push(word);
				self.chart.push(phrase, grammar);
			}
		}
	}

	/// Undoes the last parsing step.
	pub fn unparse(&mut self) -> Option<W> {
		self.chart.pop();
		self.history.pop()
	}

	/// Takes an iterable and performs parsing on each of its elements.
	pub fn parse_all(&mut self, words: impl IntoIterator<Item = W>, grammar: &[&Ruleset<P>]) {
		for word in words {
			self.parse(word, grammar);
		}
	}
}

impl<W, P: Hash + Display> Display for Parser<W, P> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.chart)
	}
}
