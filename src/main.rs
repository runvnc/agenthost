use rhai::{Engine, EvalAltResult};
use std::fs;

struct ScriptRunner {
    engine: Engine,
}

impl ScriptRunner {
    fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    fn run(&self, fname: &str) -> Result<String, Box<EvalAltResult>> {
        let script = fs::read_to_string(fname)
            .expect("Unable to read the Rhai script file.");

        self.engine.eval::<String>(&script)
    }
}

fn main() {
    let runner = ScriptRunner::new();
    
    match runner.run("test.rhai") {
        Ok(result) => println!("Answer: {}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

