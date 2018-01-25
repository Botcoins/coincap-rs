pub use error::{FetchError, FindError};

pub trait Fetch where Self: Sized {
	fn fetch() -> Result<Self, FetchError>;
}

pub trait Find where Self: Sized {
	fn find(code: &str) -> Result<Self, FindError>;
}
