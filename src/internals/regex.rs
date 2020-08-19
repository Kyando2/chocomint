//! Returns a string matching the predefined constant regex
use regex::Regex;
/* 
Possible other regex: 
# 1, Lookarounds;
"(?<=Roblox.XrsfToken.setToken\\(')(.+?)(?='\\))" => Rust version 
"(?<=Roblox\.XsrfToken\.setToken\(')(.+?)(?='\))" => Javascript version
*/

/// Finds the x-csrf-token in the body and returns it as an owned string
pub fn match_token(text: &str) -> String {
    let re = Regex::new("(Roblox.XsrfToken.setToken\\(')(.+?)('\\))")
        .unwrap();
    let text2 = re.captures(text)
        .unwrap()
        .get(2)
        .unwrap()
        .as_str();
    return text2.to_string();
}
