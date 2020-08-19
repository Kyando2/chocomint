//! First function you'll used with any code related to the library
//! a `Mint` is absolutely required and lower level abstracted should
//! never be used.I

pub mod mint;
use crate::http;
use crate::structs::creditentials::Creditentials;

/// Creates a `Mint` a `Creditentials` and a `HttpClient`
pub fn new(security: &str) -> mint::Mint {
	let requests = http::new();
	let auth: Creditentials = Creditentials {
	token: "".to_owned(),
	security: security.to_owned()
	};

	mint::Mint{
		requests,
		auth,
		__connected: false
	}
}