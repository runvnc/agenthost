use async_openai::types::ChatCompletionRequestMessage;

#[derive(Clone, Default, Debug)]
pub struct ModelInfo {
    pub type_name: String,
    pub model_file: String,
    pub url: String,
    pub max_context: i32,
}

pub trait Model: Send + Sync {
    fn model_info(&self) -> ModelInfo;

    fn to_instruct_string(&self, msgs: &Vec<ChatCompletionRequestMessage>) -> String;
}
