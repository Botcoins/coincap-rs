use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct CacheError(pub String);

impl Error for CacheError {
	fn description<'a>(&'a self) -> &'a str {
		&self.0
	}
}

impl Display for CacheError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		f.write_str(self.description())
	}
}
