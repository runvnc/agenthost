use rhai::{Engine, EvalAltResult};
use std::fs;
use std::time::{Instant};

struct ScriptRunner {
    engine: Engine,
    script: String,
}

impl ScriptRunner {
    fn new() -> Self {
        Self {
            engine: Engine::new(),
            script: String::new() 
        }
    }

    fn load(&mut self, fname: &str) {
       self.script = fs::read_to_string(fname)
            .expect("Unable to read the Rhai script file.");
    }

    fn run(&self) -> Result<String, Box<EvalAltResult>> {
        self.engine.eval::<String>(&self.script)
    }
}

fn main() {
    let mut runner = ScriptRunner::new();
    runner.load("test.rhai");

    let start_time = Instant::now();
    let mut cnt = 0;
    loop {
        match runner.run() {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {:?}", e),
        }
        cnt += 1;
        if cnt == 1000 {
            break;
        }
    }
    let duration = start_time.elapsed();
    println!("Total time: {} ms", duration.as_millis());
}
