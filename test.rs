// Function to check if a given string starts with any of the strings in a vector
fn starts_with_any(string_to_check: &str, prefixes: Vec<String>) -> bool {
    prefixes.iter().any(|prefix| string_to_check.starts_with(prefix))
}

fn main() {
    let my_string = "hello_world".to_string();
    let prefixes = vec!["hello".to_string(), "world".to_string()];

    if starts_with_any(&my_string, prefixes) {
        println!("The string starts with one of the prefixes.");
    } else {
        println!("The string does not start with any of the prefixes.");
    }
}
