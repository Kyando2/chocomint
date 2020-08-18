//! Returns a string matching the predefined constant regex

use regex::Regex;
/* Possible other regex: 
"(?<=Roblox.XrsfToken.setToken\\(')(.+?)(?='\\))" => Rust version 
"(?<=Roblox\.XsrfToken\.setToken\(')(.+?)(?='\))" => Javascript version
*/
/// Finds the x-csrf-token in the body and returns it as an owned string
pub fn match_token(text: &str) -> String {
    let re = Regex::new("Roblox.XsrfToken.setToken\\('(.+?)'\\)") 
        .unwrap();
    let re2 = Regex::new("'(.+?)'")
        .unwrap();
    let capture = re.captures(text)
        .unwrap();
    let text2 = capture.get(0)
        .unwrap()
        .as_str();
    let capture2 = re2.captures(text2)
        .unwrap();
    return capture2.get(0).unwrap().as_str().replace("'", "");
}
