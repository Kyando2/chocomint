use std::fs;
use fancy_regex::Regex;

fn main() {
    let text = fs::read_to_string("text.txt")
        .expect("Something went wrong with the file reading");
    let re = Regex::new("(?<=Roblox.XsrfToken.setToken\\(')(.+?)(?='\\))")
        .unwrap();
    let capture = re.captures(&text)
        .unwrap()
        .unwrap();
    println!("{}", capture.get(0).unwrap().as_str());
}

