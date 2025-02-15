use crate::Child;

/// Types that stand for a node of a derivation tree.
///
/// Such nodes may have a reducee (mother) and a reducer (father) i.e. the two premises of modus ponens, `A -> B` and `A` respectively.
pub(crate) trait ChildMaybe {
	type Mother;
	type Father;

	/// Returns the reducee of this node, if exists.
	fn mother_maybe(&self) -> Option<&Self::Mother>;

	/// Returns the reducer of this node, if exists.
	fn father_maybe(&self) -> Option<&Self::Father>;
}

impl<T: Child> ChildMaybe for T {
	type Mother = T::Mother;
	type Father = T::Father;
	fn mother_maybe(&self) -> Option<&Self::Mother> { Some(Child::mother(self)) }
	fn father_maybe(&self) -> Option<&Self::Father> { Some(Child::father(self)) }
}