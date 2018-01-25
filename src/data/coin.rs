use std::ops::Deref;

use caching::*;
use retrieve::*;

use reqwest;
use serde_json;

pub struct Coins(Vec<String>);

impl Fetch for Coins {
	fn fetch() -> Result<Self, FetchError> {
		let mut resp = reqwest::get("http://coincap.io/coins")?;

		if resp.status().is_success() {
			Ok(Coins(serde_json::from_str(&resp.text()?)?))
		} else {
			Err(FetchError(format!("Request failed with HTTP error code {}", resp.status().to_string())))
		}
	}
}

impl Deref for Coins {
	type Target = Vec<String>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub struct Coin;

impl Refresh for Coin {
	fn refresh(&self) -> Result<Self, RefreshError> {
		Err(RefreshError::from_str("Unimplemented!"))
	}
}
