use crate::*;
use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Range;

/// A parsed syntax tree.
///
/// This is a concise and detached version of the original derivation tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, AsRef)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tree<T> {
	value: T,
	range: Range<usize>,
	subtrees: Box<[Tree<T>]>,
}

impl<T: Hash + Clone, R: AsRef<Reducer<T>>> From<R> for Tree<T> {
	fn from(reducer: R) -> Self {
		let reducer = reducer.as_ref();
		let value = reducer.value().clone();
		let range = reducer.range();
		let subtrees = reducer.antecedents_flattened().collect_trees().into_boxed_slice();
		Self { value, range, subtrees }
	}
}

impl<T> Tree<T> {
	/// Whether this tree has no subtrees.
	pub fn is_leaf(&self) -> bool { self.subtrees.is_empty() }

	/// Whether this tree has subtrees.
	pub fn is_branch(&self) -> bool { !self.subtrees.is_empty() }

	/// Returns a reference to the syntactic category this tree stands for.
	pub fn value(&self) -> &T { &self.value }

	/// Returns the range over the input words that this tree covers.
	pub fn range(&self) -> Range<usize> { self.range.clone() }

	/// Returns a slice of the subtrees of this tree.
	pub fn subtrees(&self) -> &[Tree<T>] { &self.subtrees }
}

impl<T> AsRef<T> for Tree<T> {
	fn as_ref(&self) -> &T { self.value() }
}

impl<T> Borrow<T> for Tree<T> {
	fn borrow(&self) -> &T { self.value() }
}

impl<T> Deref for Tree<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target { &self.value }
}

impl<T> DerefMut for Tree<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl<T: Display> Display for Tree<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&DisplaySubtree { subtree: self, pre: &[], is_last_sibling: true }, f)
	}
}

struct DisplaySubtree<'a, T> {
	subtree: &'a Tree<T>,
	pre: &'a [&'static str],
	is_last_sibling: bool,
}

impl<T> Deref for DisplaySubtree<'_, T> {
	type Target = Tree<T>;
	fn deref(&self) -> &Self::Target { &self.subtree }
}

impl<T: Display> Display for DisplaySubtree<'_, T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let is_root = self.pre.is_empty();
		if !is_root {
			write!(f, "\n")?;
			for str in self.pre.iter() {
				write!(f, "{str}")?;
			}
			if self.is_last_sibling {
				write!(f, "└─")?;
			} else {
				write!(f, "├─")?;
			};
		}
		Display::fmt(&self.value, f)?;
		write!(f, "@{:?}", self.range)?;
		let pre: &Box<[&'static str]> = &self
			.pre
			.iter()
			.cloned()
			.chain([if is_root {
				""
			} else if self.is_last_sibling {
				"  "
			} else {
				"│ "
			}])
			.collect();
		let len = self.subtrees.len();
		for (index, subtree) in self.subtrees.iter().enumerate() {
			let is_last_sibling = index == len - 1;
			Display::fmt(&DisplaySubtree { subtree, pre, is_last_sibling }, f)?;
		}
		Ok(())
	}
}
