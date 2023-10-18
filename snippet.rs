pub fn add_user_message(&mut self, text: String) -> Res<usize> {
   let msg = ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(text)
            .build()?;
   let chatmsg = ChatMessage::new(msg);
   let length = chatmsg.length;
   self.messages.push(chatmsg);
   Ok(length)
}