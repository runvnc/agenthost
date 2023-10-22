use rhai::{CallFnOptions, Dynamic, Engine,
           EvalAltResult, Position, Map, Scope, AST};
use std::io::{stdout, Write};
use rhai::packages::Package;
use rhai_rand::RandomPackage;

use anyhow::{Result, anyhow};

use crate::{dyn_map, dyn_str};
use crate::cat::{cat_files};

#[derive(Debug)]
pub struct Handler {
    pub engine: Engine,
    pub scope: Scope<'static>,
    pub states: Dynamic,
    pub ast: AST,
    pub script: String,
    pub dir: String
}

use regex::Regex;

fn extract_line_and_position(s: &str) -> Option<rhai::Position> {
    let re = Regex::new(r"\(line (\d+), position (\d+)\)").unwrap();
    if let Some(caps) = re.captures(s) {
        let line = caps.get(1)?.as_str().parse::<u16>().ok()?;
        let pos = caps.get(2)?.as_str().parse::<u16>().ok()?;
        Some(rhai::Position::new(line, pos))
    } else {
        None
    }
}

fn eprint_error(input: &str, mut err: EvalAltResult) {
    fn eprint_line(lines: &[&str], pos: Position, err_msg: &str) {
        let line = pos.line().unwrap();
        let line_no = format!("{line}: ");

        eprintln!("{line_no}{}", lines[line - 1]);

        for (i, err_line) in err_msg.to_string().lines().enumerate() {
            // Display position marker
            println!(
                "{0:>1$}{err_line}",
                if i > 0 { "| " } else { "^ " },
                line_no.len() + pos.position().unwrap() + 1,
            );
        }
        eprintln!();
    }

    let lines: Vec<_> = input.lines().collect();

    // Print error
    let pos = err.position();

    if pos.is_none() {
        let temp = format!("{err}");
        let pos_str = temp.as_str();
        if let Some(p) = extract_line_and_position(pos_str) {
            eprint_line(&lines, p, &err.to_string());
        } else {
            eprintln!("No position found in error.");
            eprintln!("{err}");
        }
    } else {
        eprint_line(&lines, pos, &err.to_string())
    }
}

use std::path::Path;
use std::ffi::OsStr;

fn get_directory(file_path: &str) -> String {
    let path = Path::new(file_path);
    match path.parent() {
        Some(dir_path) => dir_path.to_str().unwrap_or("scripts").to_string(),
        None => String::from("scripts"),
    }
}

#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
pub fn init(path: &str) -> Result<Handler>  {
    let dir = get_directory(path);

    print!("Script file: {}  directory: {}", path, dir);
    stdout().flush().expect("flush stdout");

    let mut engine = Engine::new();
    engine.set_max_map_size(200);

    engine.register_global_module(RandomPackage::new().as_shared_module());

    let states_map = Map::new();
    let states_dyn: Dynamic = states_map.into();
    let mut states = states_dyn.into_shared();
    let mut scope = Scope::new();

    println!("> Loading script file: {path} with utils.rhai appended");
   
    let with_utils = cat_files(path, "scripts/utils.rhai")?;

    let ast = match engine.compile_with_scope(&scope, with_utils.as_str()) {
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
        eprintln!("Script init() error: {err}")
    }

    let handler = Handler {
        engine,
        scope,
        states,
        ast,
        script: with_utils,
        dir
    };

    Ok(handler)
}

pub fn goto_stage(handler: &mut Handler, stage: &str) -> Result<()> {
    let path = handler.dir.as_str().to_owned() + "/" + stage+".rhai";
    println!("> Loading script file: {path} with utils.rhai appended");
   
    let with_utils = cat_files(path.as_str(), "scripts/utils.rhai")?;

    let ast = match handler.engine.compile_with_scope(&handler.scope, with_utils.as_str()) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("! Error: {err}");
            println!("Cannot continue. Bye!");
            return Err(anyhow!("Compilation failed."));
        }
    };

    let options = CallFnOptions::new()
        .eval_ast(false)
        .bind_this_ptr(&mut handler.states);

    let result = handler.engine.call_fn_with_options::<()>(options, &mut handler.scope, &ast, "init", ());

    if let Err(err) = result {
        eprintln!("Script init() error: {err}")
    }

    handler.ast = ast;
    Ok( () )
} 

pub fn get_actions(handler: &mut Handler) -> Result<rhai::Map> {
    let states_map = dyn_map!(handler.states, "Could not access states as map.")?;
    let actions = states_map.get("actions").ok_or(anyhow!("Could not read actions"))?;
    dyn_map!(actions, "Could not read actions as map")
}

pub fn get_sys_msg(handler: &mut Handler) -> Result<String> {
    let states_map = dyn_map!(handler.states, "Could not access states as map.")?;
    dyn_str!(states_map, "sys")     
}

pub fn get_relevant_state(handler: &mut Handler) -> Result<String> {
    call_function(handler, "get_relevant", "{}")
}

pub fn call_function(handler: &mut Handler, func: &str, args_json: &str) ->
        Result<String> {
    let argmap = handler.engine.parse_json(&args_json, true).unwrap_or(Map::new());
    let arg = Dynamic::from_map(argmap);

    //println!("{:?}", handler.states);

    let engine = &handler.engine;
    let scope = &mut handler.scope;
    let ast = &handler.ast;
    let options = CallFnOptions::new()
        .eval_ast(false)
        .bind_this_ptr(&mut handler.states);

    let result = engine.call_fn_with_options::<Dynamic>(options, scope, ast, func, (arg,));
        
    let output = match result {
        Ok(result) => format!("{:?}", result),
        Err(err) => {
            eprint_error(&handler.script, *err);
            "Error".to_string()
            //format!("{:?}", err)
        }
    };
    Ok( output )
}
