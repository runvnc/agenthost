use async_openai::types::{ChatCompletionRequestMessage};

trait Model {
    fn type(&self) -> &str {
      self.type_name
    }

    fn file_path(&self) -> &str {
        "models/" + self.model_file
    }

    fn download_url(&self) -> &str {
        self.url
    }

    fn context_len(&self) -> &str {
        self.max_context
    }

    to_instruct_string(msgs: &Vec<ChatCompletionRequestMessage>) -> String
}
