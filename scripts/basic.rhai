fn renderSysMsg(data) {
`
You are a helpful AI assistant.
`;
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
    } 
}


