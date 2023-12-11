use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut layers = 1000;
    if args.len() > 2 {
        layers = args[2].parse::<i32>().unwrap().into()
    }

    let model_options = ModelOptions {
        n_gpu_layers: layers,
        ..Default::default()
    };
    let model_file = args[1].clone();
    let llama = LLama::new(model_file, &model_options).unwrap();

    let predict_options = PredictOptions {
        tokens: 0,
        threads: 40,
        temperature: 0.001,
        top_k: 90,
        top_p: 0.86,
        token_callback: Some(Box::new(|token| {
            print!("{}", token);
            io::stdout().flush().unwrap();
            true
        })),
        ..Default::default()
    };

    llama
        .predict(
            "what are the national animals of india".into(),
            predict_options,
        )
        .unwrap();
}
