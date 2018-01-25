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
pub use error::InitError;
pub use retrieve::*;

use std::collections::HashMap;

/// The cache object, the end user can cache this themselves
pub struct CoinCapCache {
	pub global_stats: Cached<GlobalStats>,
	pub coin_symbols: Cached<Coins>,
	map: HashMap<String, Cached<Coin>>,
}

/// This is an implementation for the cache that makes it as easy to access as possible
impl CoinCapCache {
	/// Initialize coincap data fetcher & cache with specified timeout for data - 0 for no delay
	/// Calling it will immediately dispatch a fetch for global data and a list of all coins
	pub fn new(timeout: u64) -> Result<Self, InitError> {
		let global_stats = Cached::new(GlobalStats::fetch()?, timeout);
		let coin_symbols = Cached::new(Coins::fetch()?, 86400); // Force it to update only every 24 hours as there aren't new coins added that often

		Ok(CoinCapCache {
			global_stats,
			coin_symbols,
			map: HashMap::new(),
		})
	}
}
