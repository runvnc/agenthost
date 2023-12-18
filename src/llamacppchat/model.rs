use async_openai::types::ChatCompletionRequestMessage;

#[derive(Default)]
pub struct ModelInfo {
    type_name: String,
    model_file: String,
    url: String,
    max_context: i32,
}

pub trait Model: Send + Sync {
    fn model_info(&self) -> ModelInfo;

    fn to_instruct_string(&self, msgs: &Vec<ChatCompletionRequestMessage>) -> String;
}
