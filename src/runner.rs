use rhai::{Engine, RegisterFn, Dynamic};
use std::fs;

#[derive(Debug, Clone)]
struct GameRunner {
    engine: Engine,
    state: Dynamic,
    stages: Dynamic,
    current_stage: String,
}

impl GameRunner {
    fn new() -> Self {
        Self {
            engine: Engine::new(),
            state: Dynamic::from(()),
            stages: Dynamic::from(()),
            current_stage: "mainLoop".to_string(),  // Starting stage
        }
    }

    fn load_script(&self, filename: &str) -> String {
        fs::read_to_string(filename).expect("Unable to read the Rhai script file.")
    }

    fn load_stages(&mut self, script: &str) -> rhai::Result<()> {
        self.stages = self.engine.eval::<Dynamic>(script)?;
        Ok(())
    }

    fn execute_current_stage(&mut self) -> rhai::Result<()> {
        let current_stage = self.stages.get_mut(&self.current_stage).unwrap().clone();
        
        // Init
        let init: rhai::FnPtr = current_stage.get("init").unwrap();
        init.call_dynamic((&mut self.engine, &mut Default::default()), (self.state.clone(), Dynamic::from(())))?;

        // TODO: Here you'll handle actions based on user inputs.

        // EvalExit
        let evalExit: rhai::FnPtr = current_stage.get("evalExit").unwrap();
        let next_stage: String = evalExit.call_dynamic((&mut self.engine, &mut Default::default()), (self.state.clone(), ))?.try_into()?;

        // Transition to the next stage if evalExit returns a valid stage name
        if !next_stage.is_empty() {
            self.current_stage = next_stage;
        }

        println!("Current State: {:?}", self.state);
        println!("Current Stage: {:?}", self.current_stage);

        Ok(())
    }

    fn run(&mut self, filename: &str) -> rhai::Result<()> {
        let script = self.load_script(filename);
        self.load_stages(&script)?;

        // Example of running the stages. In the real application, 
        // this will be in response to user actions/input.
        self.execute_current_stage()?;  
        self.execute_current_stage()?;  // Sample: Executing stages multiple times for demonstration

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game_runner = GameRunner::new();

    // Register Rhai's built-in 'rand' function for rolling dice
    game_runner.engine.register_global_module(rhai::module_std::rand());

    // Register a custom function for database selection (mocked)
    game_runner.engine.register_fn("db_select", |name: String| {
        "Void Foyer".to_string() // Mocking DB interaction
    });

    // Run the game with the provided Rhai script
    game_runner.run("game_logic.rhai")?;

    Ok(())
}

