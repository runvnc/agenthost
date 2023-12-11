use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

fn main() {
    let model_options = ModelOptions {
        n_gpu_layers: 1000,
        ..Default::default()
    };

    let llama = LLama::new("models/starling-lm.gguf".into(), &model_options).unwrap();

    let predict_options = PredictOptions {
        tokens: 0,
        threads: 10,
        temperature: 0.001,
        top_k: 90,
        top_p: 0.86,
        token_callback: Some(Box::new(|token| {
            println!("token1: {}", token);

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
