#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod caching;
pub mod error;
pub mod retrieve;
pub mod stats;

pub use caching::*;
use stats::GlobalStats;

use std::collections::HashMap;
use std::sync::Mutex;

pub struct Coin;

impl Refresh for Coin {
	fn refresh(&self) -> Result<Self, RefreshError> {
		Err(RefreshError::from_str("Unimplemented!"))
	}
}

lazy_static! {
	static ref GLOBAL_STATS: Mutex<Option<Cached<GlobalStats>>> = Mutex::new(None);
	pub static ref COINS: Mutex<HashMap<String, Cached<Coin>>> = Mutex::new(HashMap::new());
}

/// Trait for accessing the cache conveniently
pub trait CoinCapCache {
	/// A Vector of coin symbols which's lifetime is dependent on that of COINS
	fn symbols<'a>(&'a self) -> Vec<&'a String>;
}

/// This is an implementation for the cache that makes it as easy to access as possible
impl CoinCapCache for HashMap<String, Coin> {
	/// Returns a vector of coin symbols that are supported, making api calls if necessary.
	fn symbols<'a>(&'a self) -> Vec<&'a String> {
		let mut symbols = Vec::with_capacity(self.len());
		for (symbol, _) in self.iter() {
			symbols.push(symbol);
		}

		return symbols;
	}
}
