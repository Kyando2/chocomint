use reqwest::header::HeaderName;
use reqwest::header::HeaderMap;

/// Authorizations and Creditentials used for building headers
pub struct Creditentials {
	pub token: String,
	pub security: String
}

impl Creditentials {
	/// Factory, creates a new Creditentials with the given 
	/// token(X-CSRF-TOKEN) and security(.ROBLOSECURITY cookie)
	pub fn new(token: String, security: String) -> Creditentials {
		Creditentials {
			token,
			security
		}
	}
	/// Returns a complete header map with the stored Creditentials
	/// `"content-type = "application/json"`
	/// `"x-csrf-token" = self.token` note that self.token is borrowed here
	/// `"cookie" = ".ROBLOSECURITY="` + `self.security` note that self.security is borrowed here
	pub fn headers(&self) -> HeaderMap {
		let mut headers = HeaderMap::new();
		let content_type = HeaderName::from_static("content-type");
		let csrf = HeaderName::from_static("x-csrf-token");
		let cookie = HeaderName::from_static("cookie");
		let token = &self.token;
		let mut cookie_value = ".ROBLOSECURITY=".to_owned();
		cookie_value.push_str(&self.security);
		headers.insert(csrf, token.parse().unwrap());
		headers.insert(content_type, "application/json".parse().unwrap());
		headers.insert(cookie, cookie_value.parse().unwrap());
		return headers
	}
}