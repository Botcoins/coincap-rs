use caching::Refresh;
pub use error::{FetchError, RefreshError};
use retrieve::Fetch;

use reqwest;
use serde_json;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GlobalStats {
	/// Market cap of altcoins
	#[serde(rename = "altCap")] pub alts_cap: f64,
	/// Trading / transaction volume of altcoins
	#[serde(rename = "volumeAlt")] pub alts_vol: f64,
	/// The number of bitcoin nodes found
	#[serde(rename = "bitnodesCount")] pub btc_nodes_count: usize,
	/// USD price bitcoin
	#[serde(rename = "btcPrice")] pub btc_price: f64,
	/// USD Market cap of bitcoin
	#[serde(rename = "btcCap")] pub btc_cap: f64,
	/// Percentage market dominance of bitcoin
	#[serde(rename = "dom")] pub btc_dom: f64,
	/// Volume of bitcoin transactions / trades, in USD
	#[serde(rename = "volumeBtc")] pub btc_vol: f64,
	/// Market cap of all cryptocurrencies combined
	#[serde(rename = "totalCap")] pub total_cap: f64,
	/// Volume of all crypto transactions / trades, in USD
	#[serde(rename = "volumeTotal")] pub total_vol: f64,
}

impl Refresh for GlobalStats {
	fn refresh(&self) -> Result<Self, RefreshError> {
		Ok(Self::fetch()?)
	}
}

impl Fetch for GlobalStats {
	fn fetch() -> Result<Self, FetchError> {
		let mut resp = reqwest::get("http://coincap.io/global")?;

		if resp.status().is_success() {
			Ok(serde_json::from_str(&resp.text()?)?)
		} else {
			Err(FetchError(format!("Request failed with HTTP error code {}", resp.status().to_string())))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! try_panic {
		($ex:expr) => {
			match $ex {
				Ok(inner) => inner,
				Err(err) => panic!("{:?}", err)
			}
		}
	}

	#[test]
	fn test() {
		let stats = try_panic! {GlobalStats::fetch()};

		assert!((stats.alts_cap + stats.btc_cap - stats.total_cap).abs() < 1.); // Checks the absolute difference between components and total is less than 1
	}
}