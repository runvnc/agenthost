use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

use download_model::*;

use std::io::{self, Write};

use std::env;

const AGENTHOST_MODEL: &str = "models/mixtral-8x7b-instruct-v0.1.Q4_K_M.gguf";
const AGENTHOST_MODEL_URL: &str = "https://huggingface.co/TheBloke/Mixtral-8x7B-Instruct-v0.1-GGUF/resolve/main/mixtral-8x7b-instruct-v0.1.Q4_K_M.gguf?download=true"; 

struct LlamaCppChat {
  model_options: ModelOptions,
  model_file: String,
  llama: Llama
}

impl LlamaCppChat {
    fn new(model_file: String) {
        let model_options = ModelOptions {
            n_gpu_layers: 1000,
            ..Default::default()
        };
        let llama = LLama::new(&model_file, &model_options).unwrap();

        LlamaCppChat {
            model_options,
            model_file,
            llama
        }
    } -> LLamaCppChat;

    fn new_default_model() -> LLamaCppChat {
        let model_file = env::var("AGENTHOST_MODEL").unwrap_or(AGENTHOST_MODEL.to_string());
        let model_url = env::var("AGENTHOST_MODEL_URL").unwrap_or(AGENTHOST_MODEL_URL.to_string());
 
        download_model_if_not_exist(&model_url, &model_file);

        LlamaCppChat::new(model_file)
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
