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
pub mod data;

pub use caching::*;
pub use data::*;

use std::collections::HashMap;

/// The cache object, the end user can cache this themselves
pub struct CoinCapCache {
	pub global_stats: GlobalStats,
	map: HashMap<String, Cached<Coin>>,
}

/// This is an implementation for the cache that makes it as easy to access as possible
impl CoinCapCache  {
	/// Returns a vector of coin symbols that are supported, making api calls if necessary.
	fn symbols<'a>(&'a self) -> Vec<&'a String> {
		let mut symbols = Vec::with_capacity(self.map.len());
		for (symbol, _) in self.map.iter() {
			symbols.push(symbol);
		}

		return symbols;
	}
}
