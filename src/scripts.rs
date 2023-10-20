use rhai::{CallFnOptions, Dynamic, Engine, 
           Map, Scope, AST, format_map_as_json};
use std::io::{stdin, stdout, Write};
use rhai::packages::Package;
use rhai_rand::RandomPackage;
use std::error::Error;
use serde;
use serde_json;
use anyhow::{Result, anyhow};
use std::sync::RwLock;
use std::sync::Arc;

use crate::{s, dyn_map};

#[derive(Debug)]
pub struct Handler {
    pub engine: Engine,
    pub scope: Scope<'static>,
    pub states: Dynamic,
    pub ast: AST,
}

pub fn print_scope_ex(scope: &Scope) {
    println!("Hello from print_scope_ex!");
    /*
    for (i, (name, constant, value)) in scope.iter_raw().enumerate() {
        #[cfg(not(feature = "no_closure"))]
        let value_is_shared = if value.is_shared() { " (shared)" } else { "" };
        #[cfg(feature = "no_closure")]
        let value_is_shared = "";
        println!("Name = {}", name);
        println!(
            "[{}] {}{}{} = {:?}",
            i + 1,
            if constant { "const " } else { "" },
            name,
            value_is_shared,
            *value.read_lock::<Dynamic>().unwrap(),
        )
    } */
    println!();
}


#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
pub fn init(path: &str) -> Result<Handler>  {
    print!("Script file [{}]: ", path);
    stdout().flush().expect("flush stdout");

    let mut engine = Engine::new();

    engine.register_global_module(RandomPackage::new().as_shared_module());

    let mut states_map = Map::new();
    let mut states_dyn: Dynamic = states_map.into();
    let mut states = (states_dyn.into_shared());
    let mut scope = Scope::new();

    println!("> Loading script file: {path}");

    let ast = match engine.compile_file_with_scope(&scope, path.into()) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("! Error: {err}");
            println!("Cannot continue. Bye!");
            return Err(anyhow!("Compilation failed."));
        }
    };

    let options = CallFnOptions::new()
        .eval_ast(false)
        .bind_this_ptr(&mut states);

    let result = engine.call_fn_with_options::<()>(options, &mut scope, &ast, "init", ());

    if let Err(err) = result {
        eprintln!("! {err}")
    }

    let handler = Handler {
        engine,
        scope,
        states,
        ast,
    };

    Ok(handler)
}

pub fn get_actions(handler: &mut Handler) -> Result<rhai::Map> {
    let states_map = dyn_map!(handler.states, "Could not access states as map.")?;
    let actions = states_map.get("actions").ok_or(anyhow!("Could not read actions"))?;
    dyn_map!(actions, "Could not read actions as map")
}

pub fn call_function(handler: &mut Handler, func: &str, args_json: &str) {
    let argmap = handler.engine.parse_json(&args_json, true).unwrap_or(Map::new());
    let arg = Dynamic::from_map(argmap);

    //println!("{:?}", handler.states);

    let engine = &handler.engine;
    let scope = &mut handler.scope;
    let ast = &handler.ast;
    let options = CallFnOptions::new()
        .eval_ast(false)
        .bind_this_ptr(&mut handler.states);

    let result = engine.call_fn_with_options::<i64>(options, scope, ast, func, (arg,));
        
    match result {
        Ok(value) => println!("{value}"),
        Err(err) => eprintln!("! {err}")
    }
}
