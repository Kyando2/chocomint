pub mod client;
use reqwest::blocking::Client;

/// Creates a chocomint `HttpClient` and a `reqwest::blocking::Client`
pub fn new() -> client::HttpClient {
	let r_client = Client::new();
	client::HttpClient{ 
		r_client
	}
}