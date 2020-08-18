use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::blocking::Response;
use reqwest::Method;

/// HttpClient handling low abstractions such as direct API requests
/// and caching
pub struct HttpClient {
	pub r_client: Client
}

impl HttpClient {
	/// Basic request
	/// Sends a request to the API
	/// Will not panic even if an error occurs
	pub fn request(&self, meth: Method, url: &str, headers: HeaderMap) -> Response {
		let resp = self.r_client.request(meth, url)
			.headers(headers)
			.send()
			.unwrap();
		resp
	}
}