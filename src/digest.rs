use derive_where::derive_where;
use ruleset_macros::AsRef;
use std::any::type_name;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::num::NonZeroU64;
use xxhash_rust::xxh3::Xxh3;

#[derive_where(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(AsRef)]
#[repr(transparent)]
pub(crate) struct Digest<T: ?Sized>(NonZeroU64, PhantomData<T>);

impl<T: ?Sized + Hash> From<&T> for Digest<T> {
	fn from(value: &T) -> Self {
		let mut hasher = Xxh3::new();
		Hash::hash(value, &mut hasher);
		let hash = hasher.finish();
		Self(NonZeroU64::new(hash).unwrap(), PhantomData)
	}
}

impl<T: ?Sized> Digest<T> {
	pub fn to_inner(&self) -> NonZeroU64 { self.0 }
}

impl<T: ?Sized> Debug for Digest<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let type_name = type_name::<T>();
		write!(f, "Digest<{type_name}>")?;
		match f.precision() {
			Some(precision) if precision == 0 => {}
			_ => {
				write!(f, "(")?;
				Display::fmt(self, f)?;
				write!(f, ")")?;
			}
		}
		Ok(())
	}
}

impl<T: ?Sized> Display for Digest<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let len = 2 * std::mem::size_of::<u64>();
		let string = format!("{:0>len$x}", self.0).to_ascii_uppercase();
		let precision = f.precision().unwrap_or(len);
		write!(f, "{string:0<precision$.precision$}")
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use k9::snapshot;
// 	use std::mem::size_of;

// 	#[test]
// 	fn digest() {
// 		assert_eq!(size_of::<Option<Digest<()>>>(), size_of::<Digest<()>>());
// 		snapshot!(format!("{:?}", Digest::from(&())), "Digest<()>(2d06800538d394c2)");
// 		snapshot!(format!("{:.4?}", Digest::from(&())), "Digest<()>(2d06)");
// 		snapshot!(format!("{:.4}", Digest::from(&())), "2d06");
// 	}
// }