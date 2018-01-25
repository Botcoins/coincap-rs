use caching::*;

pub struct Coin;

impl Refresh for Coin {
	fn refresh(&self) -> Result<Self, RefreshError> {
		Err(RefreshError::from_str("Unimplemented!"))
	}
}
