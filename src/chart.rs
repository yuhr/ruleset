use crate::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Range;
use std::ops::RangeBounds;

/// A well-formed substring table.
///
/// This is the low-level part of our parsing algorithm. In most cases you would instead want to use [`Parser`], a featureful wrapper around this struct.
#[derive(Debug, Clone, AsRef)]
pub struct Chart<T> {
	table: Vec<Vec<Vec<Item<T>>>>,
	history: Vec<Vec<Item<T>>>,
}

impl<T> Default for Chart<T> {
	fn default() -> Self { Self::new() }
}

impl<T> Chart<T> {
	/// Creates an empty chart.
	pub fn new() -> Self { Self { table: Vec::new(), history: Vec::new() } }

	/// Converts bounds into a range. Ranges converted from end-open bounds are only valid for the lifetime of `self`.
	pub(crate) fn range_from(&self, bounds: impl RangeBounds<usize>) -> Range<usize> {
		use std::ops::Bound::*;
		let len = self.table.len();
		let start = match bounds.start_bound() {
			Included(&start) => start,
			Excluded(&start) => start + 1,
			Unbounded => 0,
		};
		let end = match bounds.end_bound() {
			Included(&end) => end + 1,
			Excluded(&end) => end,
			Unbounded => len,
		};
		usize::min(start, len)..usize::min(end, len)
	}

	/// Returns a reference to the table.
	pub fn rows(&self) -> &Vec<Vec<Vec<Item<T>>>> { &self.table }

	/// Returns a mutable reference to the table.
	fn rows_mut(&mut self) -> &mut Vec<Vec<Vec<Item<T>>>> { &mut self.table }

	/// Returns a reference to the row.
	pub fn row(&self, cursor: usize) -> Option<&Vec<Vec<Item<T>>>> { self.table.get(cursor) }

	/// Returns a mutable reference to the row.
	fn row_mut(&mut self, cursor: usize) -> Option<&mut Vec<Vec<Item<T>>>> {
		self.table.get_mut(cursor)
	}

	/// Returns a reference to the cell.
	pub fn cell(&self, cursor: usize, offset: usize) -> Option<&Vec<Item<T>>> {
		self.row(cursor).and_then(|row| row.get(offset))
	}

	/// Returns a mutable reference to the cell.
	fn cell_mut(&mut self, cursor: usize, offset: usize) -> Option<&mut Vec<Item<T>>> {
		self.row_mut(cursor).and_then(|row| {
			if offset < cursor + 1 {
				if row.len() <= offset {
					row.resize(offset + 1, Default::default())
				}
				Some(row.get_mut(offset).unwrap())
			} else {
				None
			}
		})
	}

	/// Enumerates items on the row at the cursor.
	pub fn items_horizontally_on(
		&self,
		cursor: usize,
	) -> impl Iterator<Item = &Item<T>> + DoubleEndedIterator {
		self.row(cursor).into_iter().flatten().flatten()
	}

	/// Enumerates items diagonally, starting from the first cell of the row at the start of the range, until the row at the end of the range.
	pub fn items_diagonally_within(
		&self,
		bounds: impl RangeBounds<usize>,
	) -> impl Iterator<Item = &Item<T>> + DoubleEndedIterator {
		self.range_from(bounds)
			.into_iter()
			.enumerate()
			.filter_map(|(offset, cursor)| self.cell(cursor, offset))
			.flatten()
	}

	/// Removes the unused tail of the row at the cursor.
	fn shrink_to_fit(&mut self, cursor: usize) {
		if let Some(row) = self.row_mut(cursor) {
			while let Some(cell_last) = row.last() {
				if cell_last.is_empty() {
					row.pop();
				} else {
					break;
				}
			}
			row.shrink_to_fit();
		}
	}

	/// Enumerates reductions on the row whose index is `cursor - offset - 1`.
	fn reductions_for<'a>(
		&'a self,
		reducer: &Reducer<T>,
		grammar: &'a [&Ruleset<T>],
	) -> impl 'a + Iterator<Item = Reduction<T>> {
		let (cursor, offset) = reducer.address();
		let cursor = (cursor - offset).checked_sub(1);
		cursor
			.into_iter()
			.flat_map(|cursor| {
				self.items_horizontally_on(cursor).filter_map(Item::reduction).filter(|reduction| {
					let origin = reduction.origin();
					grammar.iter().find(|&&ruleset| ruleset == origin).is_some()
				})
			})
			.cloned()
	}

	/// Enumerates reducers on the cells with the row index being `i` that varies from `cursor + 1` to the length of the chart, and the column index being `i - (cursor + 1)`; in other words, starting from the first cell of the next row of the given reduction, towards bottom right diagonally.
	fn reducers_for(&self, reduction: &Reduction<T>) -> impl '_ + Iterator<Item = Reducer<T>> {
		let cursor = reduction.cursor() + 1;
		self.items_diagonally_within(cursor..).filter_map(Item::reducer).cloned()
	}

	/// Creates reductions by the reducer from the grammar.
	fn reduce_from_grammar<'a>(
		&'a self,
		reducer: Reducer<T>,
		grammar: &'a [&Ruleset<T>],
	) -> impl 'a + Iterator<Item = (Reducible<T>, Reducer<T>)> {
		grammar
			.into_iter()
			.flat_map(|ruleset| ruleset.rules())
			.map(Reducible::from)
			.zip(std::iter::repeat(reducer.clone()))
	}

	/// Creates reductions by the reducer from the subrules in the chart.
	fn reduce_from_chart<'a>(
		&'a self,
		reducer: Reducer<T>,
		grammar: &'a [&Ruleset<T>],
	) -> impl 'a + Iterator<Item = (Reducible<T>, Reducer<T>)> {
		self.reductions_for(&reducer, grammar)
			.map(Reducible::try_from)
			.filter_map(std::result::Result::ok)
			.zip(std::iter::repeat(reducer))
	}

	/// Creates reductions by the reducers from the reduction whose reductum is a subrule.
	fn reduce_from_subrule<'a>(
		&'a self,
		reduction: Reduction<T>,
	) -> impl 'a + Iterator<Item = (Reducible<T>, Reducer<T>)> {
		let reducers = self.reducers_for(&reduction);
		Reducible::try_from(reduction)
			.into_iter()
			.flat_map(|reducible| std::iter::repeat(reducible))
			.zip(reducers)
	}

	/// Plots an `Item` into the chart.
	fn plot(&mut self, item: Item<T>) {
		let (cursor, offset) = item.address();
		let cell = self.cell_mut(cursor, offset).unwrap();
		cell.push(item.clone());
		self.history.last_mut().unwrap().push(item);
	}

	/// Plots a `Reducer` as an `Item` into the chart.
	fn plot_reducer(&mut self, reducer: Reducer<T>, grammar: &[&Ruleset<T>]) {
		let item = Item::from(reducer.clone());
		self.plot(item);
		self.seminate_reductions(reducer, grammar);
	}

	/// Plots a `Reduction` as an `Item` into the chart.
	fn plot_reduction(&mut self, reduction: Reduction<T>, grammar: &[&Ruleset<T>]) {
		let item = Item::from(reduction.clone());
		self.plot(item);
		self.seminate_reducers(reduction, grammar);
	}

	/// Performs reductions by the given pairs of parents.
	fn reduce(
		&self,
		parents: impl IntoIterator<Item = (Reducible<T>, Reducer<T>)>,
	) -> Vec<Reduction<T>> {
		let mut reductions = Vec::new();
		for (reducible, reducer) in parents {
			match reducible.reduce(&*reducer) {
				Match::Hit { reductum } => {
					let reduction = Reduction::new(reducible, reducer, reductum);
					reductions.push(reduction);
				}
				Match::Miss => {}
				Match::Error { reason } => {
					// Use errors for something?
				}
			}
		}
		reductions
	}

	/// Plots reductions caused by the reducer into the chart.
	fn seminate_reductions(&mut self, reducer: Reducer<T>, grammar: &[&Ruleset<T>]) {
		let from_grammar = self.reduce_from_grammar(reducer.clone(), grammar);
		let from_chart = self.reduce_from_chart(reducer, grammar);
		let reductions = self.reduce(from_grammar.chain(from_chart));
		for reduction in reductions {
			self.plot_reduction(reduction, grammar);
		}
	}

	/// Plots consequents of the reduction into the chart, or plots reductions.
	fn seminate_reducers(&mut self, reduction: Reduction<T>, grammar: &[&Ruleset<T>]) {
		// If the reductum is a consequence, seminate the consequents into the chart.
		if let Some(consequence) = reduction.reductum().consequence() {
			let mut antecedents = reduction.antecedents();
			let consequents = consequence.consequents();
			for (index, consequent) in consequents.into_iter().enumerate() {
				if let Some(consequent) = consequent {
					let antecedents = consequent.take_relevant(&mut antecedents);
					let conclusion = Conclusion::new(reduction.clone(), index, antecedents);
					self.plot_reducer(conclusion.into(), grammar);
				} else {
					antecedents.next().unwrap();
				}
			}
			debug_assert!(antecedents.count() == 0);
		}
		// Otherwise, the reductum must be a subrule, thus recalculate extra reductions that might be based on the subrule. This does nothing unless the reduction has been plotted *retrospectively* (i.e. on a row that is not the last).
		else {
			let from_subrule = self.reduce_from_subrule(reduction);
			let reductions = self.reduce(from_subrule);
			for reduction in reductions {
				self.plot_reduction(reduction, grammar);
			}
		}
	}

	/// Performs a single parsing step.
	pub fn push(&mut self, input: T, grammar: &[&Ruleset<T>]) {
		let cursor = self.table.len();
		let mut row = Vec::with_capacity(cursor + 1);
		row.resize(cursor + 1, Default::default());
		self.table.push(row);
		self.history.push(Default::default());
		let input = Input::new(input, cursor);
		self.plot_reducer(input.into(), grammar);
		self.shrink_to_fit(cursor);
	}

	/// Undoes the last parsing step, and returns the original input or `None` if the chart was empty.
	pub fn pop(&mut self) -> Option<Input<T>> {
		self.table.pop();
		if let Some(mut items) = self.history.pop() {
			while 1 < items.len() {
				let item = items.pop().unwrap();
				let (cursor, offset) = item.address();
				if let Some(cell) = self.cell_mut(cursor, offset) {
					cell.pop();
				}
			}
			items.pop().unwrap().into_reducer().and_then(Reducer::into_input)
		} else {
			None
		}
	}

	/// Removes from the chart all items covered by and conflicting with the given interpretation.
	///
	/// This reduces search space for futher inputs in certain cases, but shall effectively introduce an irreversible “checkpoint” at the cursor of the given interpretation.
	fn determine(&mut self, interpretation: impl AsRef<Reducer<T>>) {
		let interpretation = interpretation.as_ref();
		let (cursor, offset) = interpretation.address();
		let range = interpretation.range();
		// For the covered rows before the cursor, remove all items except the inputs.
		for index in range.start..cursor {
			let row = self.row_mut(index).unwrap();
			let cell_first = row.first_mut().unwrap();
			cell_first.splice(1.., []);
			cell_first.shrink_to_fit();
			for cell in row.iter_mut().skip(1) {
				cell.clear();
				cell.shrink_to_fit();
			}
			row.shrink_to_fit();
		}
		// For the row at the cursor, remove all items except the input and items after and including the given interpretation.
		let row_last = self.row_mut(cursor).unwrap();
		let cell_first = row_last.first_mut().unwrap();
		cell_first.splice(1.., []);
		cell_first.shrink_to_fit();
		for cell in row_last.iter_mut().skip(1).take(offset - 1) {
			cell.clear();
			cell.shrink_to_fit();
		}
		// For rows after the cursor, remove all items conflicting with the given interpretation.
		let width = offset;
		for cell in self
			.table
			.iter_mut()
			.skip(cursor + 1)
			.enumerate()
			.flat_map(|(offset, row)| row.iter_mut().skip(offset + 1).take(width))
		{
			cell.clear();
			cell.shrink_to_fit();
		}
		// TODO: collapse rows
	}

	/// Returns the number of rows in the chart.
	pub fn len(&self) -> usize { self.table.len() }

	/// Returns whether the chart is empty.
	pub fn is_empty(&self) -> bool { self.table.is_empty() }
}

impl<T: Display> Display for Chart<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use comfy_table::presets::UTF8_FULL;
		use comfy_table::*;
		let mut table = Table::new();
		table.load_preset(UTF8_FULL);
		for (index, row) in self.table.iter().enumerate() {
			let mut row = row
				.iter()
				.map(|items| items.iter().map(|item| format!("{item:.4}\n")).collect::<String>())
				.collect::<Vec<_>>();
			row.resize(index + 1, Default::default());
			table.add_row(row);
		}
		write!(f, "{}", table)?;
		Ok(())
	}
}