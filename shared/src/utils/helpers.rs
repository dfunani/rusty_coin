use regex::Regex;

pub fn extract_object_id(public_id: &str) -> &str {
    let regex = Regex::new(r"^.*: (.*)$").unwrap();
    let result = regex.captures(public_id).unwrap();
    return result.get(1).unwrap().as_str();
}
