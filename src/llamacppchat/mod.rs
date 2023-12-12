use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use std::env;
use std::io::{self, Write};

Struct LlamaCppChat {
  model_options: ModelOptions,
  model_file: String,
  llama: &Llama
}

impl LlamaCppChat {
    fn new(model_file: String) {
        let model_options = ModelOptions {
            n_gpu_layers: 1000,
            ..Default::default()
        };
        let llama = LLama::new(model_file, &model_options).unwrap();
    }

    async fn generate(self: &LlamaCppChat, 
                messages: Vec<ChatCompletionRequestMessage>,
                reply_sender: flume::Sender<ChatUIMessage>,
                token: CancellationToken,
       ) {
        let args: Vec<String> = env::args().collect();

        let mut layers = 1000;
        if args.len() > 2 {
            layers = args[2].parse::<i32>().unwrap().into()
        }

        let predict_options = PredictOptions {
            tokens: 0,
            threads: 10,
            temperature: 0.001,
            //top_k: 90,
            //top_p: 0.86,
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

}
