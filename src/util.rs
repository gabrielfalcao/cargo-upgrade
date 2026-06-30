use regex::Regex;
use std::string::ToString;

pub fn slugify<T: ToString>(value: T) -> String {
    let value = value.to_string();
    let regex = Regex::new(r"[^a-zA-Z0-9_.-]+").unwrap();
    let replaced = regex.replace_all(&value, "-").to_string();
    replaced.trim_start_matches("-").trim_end_matches("-").to_string()
}
