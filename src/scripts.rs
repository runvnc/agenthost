use rhai::{CallFnOptions, Dynamic, Engine, Map, Scope, AST};
use std::io::{stdin, stdout, Write};
use rhai::packages::Package;
use rhai_rand::RandomPackage;

type Res<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
struct Handler {
    pub engine: Engine,
    pub scope: Scope<'static>,
    pub states: Dynamic,
    pub ast: AST,
}

fn print_scope(scope: &Scope) {
    for (i, (name, constant, value)) in scope.iter_raw().enumerate() {
        #[cfg(not(feature = "no_closure"))]
        let value_is_shared = if value.is_shared() { " (shared)" } else { "" };
        #[cfg(feature = "no_closure")]
        let value_is_shared = "";

        println!(
            "[{}] {}{}{} = {:?}",
            i + 1,
            if constant { "const " } else { "" },
            name,
            value_is_shared,
            *value.read_lock::<Dynamic>().unwrap(),
        )
    }
    println!();
}


#[cfg(not(feature = "no_function"))]
#[cfg(not(feature = "no_object"))]
pub fn init(path: &str) -> Res<&Handler>  {
    print!("Script file [{}]: ", path);
    stdout().flush().expect("flush stdout");

    let mut engine = Engine::new();

    engine.register_global_module(RandomPackage::new().as_shared_module());

    let mut states = Map::new();
    let mut states: Dynamic = states.into();

    let mut scope = Scope::new();

    println!("> Loading script file: {path}");

    let ast = match engine.compile_file_with_scope(&scope, path.into()) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("! Error: {err}");
            println!("Cannot continue. Bye!");
            return;
        }
    };

    let options = CallFnOptions::new()
        .eval_ast(false)
        .bind_this_ptr(&mut states);

    let result = engine.call_fn_with_options::<()>(options, &mut scope, &ast, "init", ());

    if let Err(err) = result {
        eprintln!("! {err}")
    }

    let mut handler = Handler {
        engine,
        scope,
        states,
        ast,
    };

    handler
}

pub fn call_function(handler: &Handler, func: &str, args_json: &str) {
    let argmap = handler.engine.parse_json(&args_json, true).unwrap_or(Map::new());
    let arg = Dynamic::from_map(argmap);

    println!("{:?}", handler.states);

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
