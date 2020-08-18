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