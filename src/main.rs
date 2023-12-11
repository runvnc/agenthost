use llama_cpp::LlamaModel;
use tokio::select;
use tokio::time::Instant;
use std::time::Duration;


#[tokio::main]
async fn main() {

    // Create a model from anything that implements `AsRef<Path>`:
    let model = LlamaModel::load_from_file_async("path_to_model.gguf").await.expect("Could not load model");

    // A `LlamaModel` holds the weights shared across many _sessions_; while your model may be
    // several gigabytes large, a session is typically a few dozen to a hundred megabytes!
    let mut ctx = model.create_session();

    // You can feed anything that implements `AsRef<[u8]>` into the model's context.
    ctx.advance_context("This is the story of a man named Stanley.").unwrap();

    // LLMs are typically used to predict the next word in a sequence. Let's generate some tokens!
    let max_tokens = 1024;
    let mut decoded_tokens = 0;
    let timeout_by = Instant::now() + Duration::from_secs(5);

    // `ctx.get_completions` creates a worker thread that generates tokens. When the completion
    // handle is dropped, tokens stop generating!
    let mut completions = ctx.start_completing();

    loop {
        select! {
            _ = tokio::time::sleep_until(timeout_by) => {
                break;
            }
            _completion = completions.next_token_async() => {
                continue;
            }
        }
    }
}
