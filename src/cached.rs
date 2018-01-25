use error::CacheError;
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

impl<T> Cached<T> {
	/// Initialize the cache with supplied inner data and timeout
	pub fn new(inner: T, timeout: u64) -> Self {
		Cached { last_update: Instant::now(), timeout, inner }
	}

	/// Safely dereference this pointer, taking into account the age
	pub fn checked_deref(&self) -> Result<&T, CacheError> {
		if self.timeout != 0 && self.ttl() > 0 {
			Err(CacheError(format!("Cache timed out. Age: {}, Timeout: {}", self.age().as_secs(), self.timeout)))
		} else {
			Ok(self.deref())
		}
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

impl<T> Deref for Cached<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.inner
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::thread::sleep;
	use std::time::Duration;

	fn deref_check() {
		let ch = Cached::new("ato".to_string(), 1);
		assert_eq!("pot".to_string() + &*ch, "potato");
	}

	fn time_check() {
		let cached_data = Cached::new(String::new(), 1);
		assert!(cached_data.checked_deref().is_ok());
		sleep(Duration::from_secs(1));
		assert!(cached_data.checked_deref().is_err());
	}

	fn neverexpire_check() {
		let cached_data = Cached::new(String::new(), 0);
		assert!(cached_data.checked_deref().is_ok());
		sleep(Duration::from_secs(1));
		assert!(cached_data.checked_deref().is_ok());
	}
}