use crate::Reducer;
use crate::Tree;
use std::hash::Hash;

pub(crate) mod private {
	pub trait Sealed<T> {}
}

/// A sealed trait to add convenient methods to iterators of reducers.
pub trait IteratorReducerExt<T>: Iterator + private::Sealed<T>
where Self::Item: AsRef<Reducer<T>> {
	fn collect_trees(self) -> Vec<Tree<T>>;
}

impl<T, R, I> IteratorReducerExt<T> for I
where
	T: Hash + Clone,
	R: AsRef<Reducer<T>>,
	I: Iterator<Item = R> + private::Sealed<T>,
{
	fn collect_trees(self) -> Vec<Tree<T>> { self.map(Tree::from).collect() }
}

impl<T, R, I> private::Sealed<T> for I
where
	T: Hash + Clone,
	R: AsRef<Reducer<T>>,
	I: Iterator<Item = R>,
{
}