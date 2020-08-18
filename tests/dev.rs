use chocomint;
use std::fs;

#[test]
fn dev() {
	let secret = fs::read_to_string("resources/text.txt")
		.expect("Unable to read file");
	let client = chocomint::client::new(&secret);
	client.connect();
}