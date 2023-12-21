fn renderSysMsg(data) {
  let examples = "`HOST 2+2` or `HOST call_func1()`";
 
  `You are a helpful AI assistant.

You can evaluate Rhai code this way ${examples}
`
}

fn renderUserMsg(data) {
`
[${data.timestamp}]:  ${data.user_msg}`;
}

fn evalExitStage(ignore) {
    return false
}

fn init() {
    this.actions = #{
    }; 
}

