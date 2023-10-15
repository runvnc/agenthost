use std::io::Result;

fn main() -> Result<()> {
    let result: Result<i64> = some_function();

    match result {
        Ok(value) => println!("Value: {}", value),
        Err(e) => println!("Error occurred: {}", e),
    }

    Ok(())
}

fn some_function() -> Result<i64> {
    // Some code here...
}