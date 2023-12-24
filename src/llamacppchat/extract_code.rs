use regex::Regex;
pub fn check_for_code(s: &str) -> bool {
    let re = Regex::new(r"```Rhai_host\n.*?```").unwrap();
    re.is_match(s)
}

pub fn extract_code(s: &str) -> Option<&str> {
    let re = Regex::new(r"```Rhai_host\n(.*?)```").unwrap();
    re.captures(s).and_then(|cap| cap.get(1).map(|m| m.as_str()))
}
/*
pub fn check_for_code(s: &str) -> bool {
    let re = Regex::new(r"```(?:Rhai\n)?HOST\n.*\n```").unwrap();
    let found = re.is_match(s);
    //println!("check_for_code input = {:?}  result = {:?}", s, found);
    found
}

pub fn extract_code(s: &str) -> Option<&str> {
    let re = Regex::new(r"```(?:Rhai\n)?HOST\n(?s)(.*?)\n```").unwrap();
    let result = re.captures(s).and_then(|cap| cap.get(1).map(|m| m.as_str()));

    println!("extract_code input = {:?}  result = {:?}", s, result);
    result
}*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_for_code() {
        assert_eq!(check_for_code("Here we calculate the sum: ```Rhai_host\n33+156\n``` ..."), true);
        assert_eq!(check_for_code("Here we calculate the sum: ```Rhai_host\nrollDice(sides=20, num=4)\n``` ..."), true);
        assert_eq!(check_for_code("This string does not contain any code."), false);
    }

    #[test]
    fn test_extract_code() {
        assert_eq!(extract_code("Here we calculate the sum: ```Rhai_host\n33+156\n``` ..."), Some("33+156"));
        assert_eq!(extract_code("Here we calculate the sum: ```Rhai_host\nrollDice(sides=20, num=4)\n``` ..."), Some("rollDice(sides=20, num=4)"));
        assert_eq!(extract_code("This string does not contain any code."), None);
    }

    // Additional tests for multiline and Rhai prefixed code blocks can be added here
    #[test]
    fn test_extract_code_multiline() {
        let multiline_input = "This is a multiline string\n\
                               with a code snippet: ```Rhai_host\n33+156\n```\n\
                               in the middle.";
        assert_eq!(extract_code(multiline_input), Some("33+156"));
    }
}
