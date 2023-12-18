use async_openai::types::{ChatCompletionRequestMessage};

struct ModelInfo {
    type_name: &str,
    model_file: &str,
    url: &str,
    max_context: 2048
}

pub trait Model {
    fn model_info(&self) -> &ModelInfo;

    fn to_instruct_string(msgs: &Vec<ChatCompletionRequestMessage>) -> String;
}
