use crate::result::choco::BloxResult;
use crate::result::choco::BloxError;
use std::collections::HashMap;


pub struct Cache{
	pub data: HashMap<String, String>
}

impl Cache {
	/// Returns a Cache object with the given serialized data
	pub fn new() -> Cache {
		let data = HashMap::new();
		Cache {
			data
		}
	}

	pub fn get(&self, k: String) -> BloxResult<String> {
		let v = self.data.get(&k);
		if v.is_none() {
			Err(BloxError::NotFound)
		} else {
			Ok(v.unwrap().to_string())
		}
	}

	pub fn insert(&mut self, k: String, v: String) {
		self.data.insert(k, v);
	}
}