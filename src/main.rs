use rhai::{Engine, EvalAltResult};
use std::fs;

pub fn main() -> Result<(), Box<EvalAltResult>>
//                          ^^^^^^^^^^^^^^^^^^
//                          Rhai API error type
{
    // Create an 'Engine'
    let engine = Engine::new();
    let script = fs::read_to_string("test.rhai").expect("Unable to read the Rhai script file.");

    // Your first Rhai Script
    //let script = "print(40 + 2);";

    // Run the script - prints "42"
    let result = engine.eval::<String>(&script)?;
    
    println!("Answer {result}");

    // Done!
    Ok(())
}
