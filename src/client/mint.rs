use crate::http::client::HttpClient;
use reqwest::header::HeaderName;
use reqwest::header::HeaderMap;
use reqwest::Method;
use crate::internals::regex;

use crate::structs::creditentials::Creditentials;

/// A Mint client handling abstract connections with roblox
/// by using a chocomint::http::client::HttpClient
/// Must be constructed as a mutable or the `connect` meth won't work
pub struct Mint {
	pub requests: HttpClient,
	pub auth: Creditentials,
	pub __connected: bool
}

impl Mint {
	/// Obtains the valid X-CSRF-TOKEN for the first time.
	/// Then calls the private meth `verify_connection` and panics on error
	/// Turns `Mint.__conected` to true
	/// 
	/// This should always be called first
	///
	pub fn connect(mut self) {
		let headers: HeaderMap = self.base_headers();
		let res = self.requests.request(Method::GET, "https://roblox.com/home", headers)
			.text()
			.unwrap();
		let token = regex::match_token(&res);
		self.auth.token = token;
		let operation = self.verify_connection();
		if operation == false {
			panic!("Something went wrong");
		}
		self.__connected = true;
	}
	/// Returns the following basic HeaderMap;
	/// `"content-type" = "application/json"`
	fn base_headers(&self) -> HeaderMap {
		let content_type = HeaderName::from_static("content-type");
		let mut headers = HeaderMap::new();
		headers.insert(content_type, "application/json".parse().unwrap());
		headers
	}

	/// Sends a request to find the friend count of the authenticated user
	/// And returns true or false depending on the success of the request
	fn verify_connection(&self) -> bool {
		let headers = self.auth
			.headers();
		let res = self.requests.request(Method::GET, "https://friends.roblox.com/v1/my/friends/count", headers)
			.text();
		if res.is_ok() {
			return true
		}
		return false
	}
}