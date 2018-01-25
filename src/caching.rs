pub use error::{CacheError, RefreshError};

use std::time::{Duration, Instant};
use std::ops::Deref;

/// A smart pointer that stores data that may age
#[derive(Clone, Copy, Debug)]
pub struct Cached<T> {
	/// Rust instant since the data updated
	last_update: Instant,
	/// The maximum age of the data, in seconds, 0 is never expiring data
	timeout: u64,
	/// Actual data
	inner: T,
}

/// All cached data must implement this so data can be refreshed
pub trait Refresh: Sized {
	fn refresh(&self) -> Result<Self, RefreshError>;
}

impl<T> Cached<T> {
	/// Initialize the cache with supplied inner data and timeout
	pub fn new(inner: T, timeout: u64) -> Self {
		Cached { last_update: Instant::now(), timeout, inner }
	}

	/// Check if the cached object has timed out
	pub fn has_timed_out(&self) -> bool {
		self.timeout != 0 && self.ttl() < 0
	}

	/// Time left to live of the currently held data, negative is probably expired data, in seconds
	pub fn ttl(&self) -> i64 {
		self.timeout as i64 - self.age().as_secs() as i64
	}

	/// Age of the cached data
	pub fn age(&self) -> Duration {
		Instant::now().duration_since(self.last_update)
	}
}

impl<T: Refresh> Cached<T> {
	/// Safely dereference this pointer, taking into account the age for automatic updates
	pub fn checked_deref<'a>(&'a mut self) -> Result<&'a T, CacheError> {
		if self.has_timed_out() {
			debug!("Detected cache timeout, refreshing... (Age past timeout: {})", -self.ttl());
			self.inner = self.refresh()?;
		}
		Ok(&self.inner)
	}
}

impl<T> Deref for Cached<T> {
	type Target = T;

	fn deref<'a>(&'a self) -> &'a T {
		&self.inner
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::thread::sleep;
	use std::time::Duration;

	pub struct MockObj(pub usize);

	impl Refresh for MockObj {
		fn refresh(&self) -> Result<Self, RefreshError> {
			Ok(MockObj(self.0 + 1))
		}
	}

	#[test]
	fn refreshed_deref() {
		let mut ch = Cached::new(MockObj(0), 1);
		sleep(Duration::from_secs(2));
		assert_eq!(ch.checked_deref().unwrap().0, 1);
	}

	#[test]
	fn deref_check() {
		let ch = Cached::new("ato".to_string(), 1);
		assert_eq!("pot".to_string() + &*ch, "potato");
	}

	#[test]
	fn time_check() {
		let cached_data = Cached::new(String::new(), 1);
		assert!(!cached_data.has_timed_out());
		sleep(Duration::from_secs(2));
		assert!(cached_data.has_timed_out());
	}

	#[test]
	fn neverexpire_check() {
		let cached_data = Cached::new(String::new(), 0);
		assert!(!cached_data.has_timed_out());
		sleep(Duration::from_secs(2));
		assert!(!cached_data.has_timed_out());
	}
}