impl ChatLog {
    pub fn to_request_msgs(&self) -> Vec<ChatCompletionRequestMessage> {
        self.messages.iter().map(|msg| msg.message.clone()).collect()
    }
}