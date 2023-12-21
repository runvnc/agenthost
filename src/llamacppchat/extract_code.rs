use regex::Regex;

// Example: 
//
//   Here we calculate the sum: `HOST 33+156` ...
//
// Example:
//
//   I will roll the dice now: `HOST rollDice(sides=20, num=4)`  ..

pub fn check_for_code(s: &str) -> bool {
    let re = Regex::new(r"`HOST .*; `").unwrap();
    let found = re.is_match(s);
    //println!("check_for_code input = {:?}  result = {:?}", s, found);
    found
}

pub fn extract_code(s: &str) -> Option<&str> {
    let re = Regex::new(r"`HOST (.*); `").unwrap();
    let result = re.captures(s).and_then(|cap| cap.get(1).map(|m| m.as_str()));

    //println!("extract_code input = {:?}  result = {:?}", s, result);
    result
}


