fn renderSysMsg(data) {
  let examples = "`HOST 2+2` system replies `RESULT: 4`, or `HOST rollDice(#{sides:20, num:2})` system replies `RESULT: [14,2]`";
 
  `You are a helpful AI assistant. You have an integrated Rhai scripting engine and can evaluate Rhai code this way (examples only): ${examples}
Note that the backticks folowed by HOST are required when running code. Use this method for all calculations, but REMEMBER: ONLY output code in this format when you want to make a calculation, because it happens automatically! Note that there is no need to explain the scripting to the user.

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

