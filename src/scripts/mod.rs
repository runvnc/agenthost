use rhai::{CallFnOptions, Dynamic, Engine, EvalAltResult, Map, Position, Scope, AST};
use serde_json::{self, json};
use std::fs;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};

use rhai::packages::Package;
use rhai_rand::RandomPackage;

use anyhow::{anyhow, Result};

use crate::cat::cat_files;
use crate::{dyn_map, dyn_str};
use rhai_fs::FilesystemPackage;

#[derive(Debug)]
pub struct Handler {
    pub engine: Engine,
    pub scope: Scope<'static>,
    pub states: Dynamic,
    pub ast: AST,
    pub script: String,
    pub dir: String,
    pub session_id: usize,
    pub username: String, // Add username field
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


fn esprint_error(input: &str, err: EvalAltResult) -> String {
    fn esprint_line(lines: &[&str], pos: Position, err_msg: &str) -> String {
        let mut output = String::new();

        let line = pos.line().unwrap();
        let line_no = format!("{line}: ");

        output += &format!("{line_no}{}\n", lines[line - 1]);

        for (i, err_line) in err_msg.to_string().lines().enumerate() {
            // Append position marker
            output += &format!(
                "{0:>1$}{err_line}\n",
                if i > 0 { "| " } else { "^ " },
                line_no.len() + pos.position().unwrap() + 1,
            );
        }
        output += "\n";
        output
    }

    let mut output = String::new();
    let lines: Vec<_> = input.lines().collect();

    // Append error to output
    let pos = err.position();

    //if let Some(p) = pos {
        output += &esprint_line(&lines, pos, &err.to_string());
    //} else {
    //    output += "No position found in error.\n";
    //    output += &format!("{err}\n");
    //}

    output
}



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
pub fn init(path: &str, session_id: usize, username: &str) -> Result<Handler> {
    // Add username parameter
    let dir = get_directory(path);
    let username = username.to_owned(); // Clone the username to store in the Handler

    stdout().flush().expect("flush stdout");

    let mut engine = Engine::new();
    engine.set_max_map_size(200);

    engine.register_global_module(RandomPackage::new().as_shared_module());

    let package = FilesystemPackage::new();
    package.register_into_engine(&mut engine);

    //std::env::set_current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("examples")).unwrap();

    engine.register_fn("path", sandboxed_path);

    let states_file = format!("data/sessions/{username}/states-{}.json", session_id); // Include username in the path
                                                                                      // Create the directory for the user if it doesn't exist
    let user_dir = Path::new("data/sessions").join(&username);
    fs::create_dir_all(&user_dir)?;

    let mut states = match fs::read_to_string(&states_file) {
        Ok(data) => {
            let states_map: Map = serde_json::from_str(&data)?;
            Dynamic::from_map(states_map).into_shared()
        }
        Err(_) => {
            let states_map = Map::new();
            Dynamic::from_map(states_map).into_shared()
        }
    };
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
        dir,
        session_id,
        username, // Add username to the handler
    };

    Ok(handler)
}

pub fn goto_stage(handler: &mut Handler, stage: &str) -> Result<()> {
    let path = handler.dir.as_str().to_owned() + "/" + stage + ".rhai";
    println!("> Loading script file: {path} with utils.rhai appended");

    let with_utils = cat_files(path.as_str(), "scripts/utils.rhai")?;

    let ast = match handler
        .engine
        .compile_with_scope(&handler.scope, with_utils.as_str())
    {
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

    let result =
        handler
            .engine
            .call_fn_with_options::<()>(options, &mut handler.scope, &ast, "init", ());

    if let Err(err) = result {
        eprintln!("Script init() error: {err}")
    }

    handler.ast = ast;
    Ok(())
}

pub fn get_actions(handler: &mut Handler) -> Result<rhai::Map> {
    let states_map = dyn_map!(handler.states, "Could not access states as map.")?;
    let actions = states_map
        .get("actions")
        .ok_or(anyhow!("Could not read actions"))?;
    dyn_map!(actions, "Could not read actions as map")
}

pub fn get_sys_msg(handler: &mut Handler) -> Result<String> {
    let states_map = dyn_map!(handler.states, "Could not access states as map.")?;
    dyn_str!(states_map, "sys")
}

pub fn get_relevant_state(handler: &mut Handler) -> Result<String> {
    call_function(handler, "get_relevant", "{}")
}

pub fn call_function(handler: &mut Handler, func: &str, args_json: &str) -> Result<String> {
    let argmap = handler
        .engine
        .parse_json(&args_json, true)
        .unwrap_or(Map::new());
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
        Ok(result) => {
            save_states(handler)?; // Update the save_states function call
            format!("{:?}", result)
        }
        Err(err) => {
            eprint_error(&handler.script, *err);
            "Error".to_string()
        }
    };
    Ok(output)
}

pub fn eval_expr(handler: &mut Handler, expr: &str) -> Result<String> {
    let engine = &handler.engine;
    let scope = &mut handler.scope; 
    let result = engine.eval_with_scope::<Dynamic>(scope, expr);
    let output = match result {
        Ok(result) => {
            save_states(handler)?;
            format!("{:?}", result)
        }
        Err(err) => {
            println!("Error in eval expr: {:?}", err);
            esprint_error(&expr, *err)
        }
    };
 
    Ok( output )
}
 

fn sandboxed_path(str_path: &str) -> Result<PathBuf, Box<EvalAltResult>> {
    let root_path = PathBuf::from("sandbox").canonicalize().unwrap();
    let mut path = PathBuf::from(str_path);

    if path.is_relative() {
        path = root_path.join(path);
    }

    match path.canonicalize() {
        Ok(p) => p.starts_with(root_path).then(|| path),
        Err(e) => return Err(e.to_string().into()),
    }
    .ok_or_else(|| "Path out of bounds".into())
}

fn save_states(handler: &Handler) -> Result<()> {
    // Remove session_id parameter as it's now part of the handler
    let states_map = dyn_map!(handler.states, "Could not access states as map.")?;
    let data = serde_json::to_string(&states_map)?;
    let states_file = format!(
        "data/sessions/{}/states-{}.json",
        handler.username, handler.session_id
    ); // Include username in the path
    fs::write(&states_file, data)?;
    Ok(())
}
