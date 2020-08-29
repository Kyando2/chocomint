pub trait RobloxId {
	/// Returns the value as a String
	fn repr(&self) -> String;
}

impl RobloxId for str {
	fn repr(&self) -> String {
		self.to_owned()
	}
}

impl RobloxId for String {
	fn repr(&self) -> Self {
		self.to_owned()
	}
}

impl RobloxId for u32 {
	fn repr(&self) -> String {
		self.to_string()
	}
}

impl RobloxId for u64 {
	fn repr(&self) -> String {
		self.to_string()
	}
}

impl RobloxId for i32 {
	fn repr(&self) -> String {
		self.to_string()
	}
}

impl RobloxId for i64 {
	fn repr(&self) -> String {
		self.to_string()
	}
}

