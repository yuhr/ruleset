use std::ops::Range;

/// Types that have cursor, offset, and range in a chart.
pub(crate) trait Addressable {
	/// Returns the range over the inputs that is covered by `self`.
	fn range(&self) -> Range<usize>;

	/// Returns the cursor (i.e. the row number in the chart) of `self`.
	fn cursor(&self) -> usize { self.range().end - 1 }

	/// Returns the offset (i.e. the column number in the chart) of `self`.
	fn offset(&self) -> usize { self.range().len() - 1 }

	/// Returns the cursor and offset at once in this order.
	fn address(&self) -> (usize, usize) { (self.cursor(), self.offset()) }
}

impl Addressable for Range<usize> {
	fn range(&self) -> Range<usize> { self.clone() }
}