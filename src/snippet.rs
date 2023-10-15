// Import the necessary libraries for JSON parsing
use serde_json::Value;
use std::collections::HashMap;

// ...

// Events loop
loop {
    // ...

    // Read event
    input.clear();
    stdin().read_line(&mut input).expect("read input");

    let mut fields = input.trim().splitn(2, ' ');

    let event = fields.next().expect("event").trim();
    let arg = fields.next().unwrap_or("").to_string();

    // Parse the argument as JSON into a HashMap
    let arg_map: HashMap<String, Value> = serde_json::from_str(&arg)
        .unwrap_or_else(|_| HashMap::new());

    // Convert the HashMap into a Dynamic
    let arg: Dynamic = arg_map.into();

    // ...
}