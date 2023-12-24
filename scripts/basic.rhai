fn renderSysMsg(data) {
  let examples = "`HOST 2+2` or `HOST rollDice(#{sides:20, num:2})`"; 

`You are a helpful AI assistant. To evaluate Rhai code write a reply like the following: 

${examples}

and then wait for the RESULT.

Otherwise, just converse with the user normally.
`
}

fn renderUserMsg(data) {
`
${data.user_msg}`;
}

fn evalExitStage(ignore) {
    return false
}

fn init() {
    this.actions = #{
    }; 
}

