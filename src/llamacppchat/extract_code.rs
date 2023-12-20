use regex::Regex;

pub fn check_for_code(s: &str) -> bool {
    let re = Regex::new(r"`HOST .*; `").unwrap();
    re.is_match(s)
}

pub fn extract_code(s: &str) -> Option<&str> {
    let re = Regex::new(r"`HOST (.*); `").unwrap();
    re.captures(s).and_then(|cap| cap.get(1).map(|m| m.as_str()))
}


