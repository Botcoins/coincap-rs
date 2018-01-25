use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use reqwest::Error as ReqwestErr;
use serde_json::Error as SerdeJsonErr;

macro_rules! str_err {
	($ty:ident) => {
		#[derive(Debug)]
		pub struct $ty(pub String);

		impl Error for $ty {
			fn description<'a>(&'a self) -> &'a str {
				&self.0
			}
		}

		impl Display for $ty {
			fn fmt(&self, f: &mut Formatter) -> FmtResult {
				f.write_str(self.description())
			}
		}

		impl $ty {
			pub fn from_str(errmsg: &str) -> Self {
				$ty(errmsg.to_string())
			}
		}
	}
}

macro_rules! convertible_error {
	($ty:ident: $($from_ty:ident),*) => {
		str_err!($ty);

		$(impl From<$from_ty> for $ty {
			fn from(err: $from_ty) -> Self {
				$ty(format!("{:?}", err))
			}
		})*
	}
}

convertible_error!(CacheError: RefreshError);
convertible_error!(RefreshError: FetchError, FindError);
convertible_error!(FetchError: ReqwestErr, SerdeJsonErr);
convertible_error!(FindError: ReqwestErr, SerdeJsonErr);

