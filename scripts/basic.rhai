fn renderSysMsg(data) {
  let examples = "`HOST 2+2` or `HOST rollDice(sides=20, num=4)`";
 
  `You are a helpful AI assistant. You have an integrated Rhai scripting engine and can evaluate Rhai code this way (examples only): ${examples}
Note that the backticks folowed by HOST are required. Use this method for all calculations.
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

