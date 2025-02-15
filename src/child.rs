/// Types that stand for a non-leaf node of a derivation tree.
///
/// Such nodes shall always have a reducee (mother) and a reducer (father) i.e. the two premises of modus ponens, `A -> B` and `A` respectively.
pub(crate) trait Child {
	type Mother;
	type Father;

	/// Returns the reducee of this node.
	fn mother(&self) -> &Self::Mother;

	/// Returns the reducer of this node.
	fn father(&self) -> &Self::Father;
}