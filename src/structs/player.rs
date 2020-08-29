use crate::structs::cache::Cache;
use crate::structs::id::RobloxId;
use crate::client::mint::Mint;
use serde_json;
use crate::result::choco::BloxError;
use crate::result::choco::BloxResult;
use reqwest::Method;


pub struct Player {
	pub id: String,
	cache: Cache,
	mint: Mint
}

impl Player {
	pub fn new(ro_id: &dyn RobloxId, mint: Mint) -> Player {
		let id = ro_id.repr(); // Returns the id as String 
		let cache = Cache::new();
		Player {
			id,
			cache,
			mint
		}
	}

	/// Returns the player's name from cache 
	/// Makes an HTTP request if it cannot be found
	pub fn username(&mut self) -> BloxResult<String> {
		let try_cache = self.check_cache("Username".to_string());
		if try_cache.is_none() != true {
			Ok(try_cache.unwrap())
		} else {
			// Make a request to the API
			let mut url: String = "https://api.roblox.com/users/".to_owned(); // Might want to use a data container for constants
			url.push_str(&self.id);
			self.mint.update_token(); // Updating X-CSRF TOKEN
			let data = self.mint.request(Method::GET, url);
			if data.is_none() {
				Err(BloxError::ApiError)
			} else {
				let data_val: serde_json::Value = serde_json::from_str(&data.unwrap()).unwrap();
				let username = data_val.get("Username").unwrap();
				self.cache.insert("Username".to_string(), username.to_string());
				Ok(username.to_string())
			}
		}
	}

	fn check_cache(&self, keyword: String) -> Option<String> { // Might be costants so maybe &str would be optimal
		let possible = self.cache.get(keyword);
		if possible.is_err() {
			None
		} else {
			Some(possible
				.unwrap()
				)
		}
	}
}