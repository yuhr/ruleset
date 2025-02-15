/// Types that represent errors that can happen during reduction.
pub trait AnyError: std::any::Any + std::error::Error {}

impl<T> AnyError for T where T: std::any::Any + std::error::Error {}
