fn renderSysMsg(data) {
  let tree = `
fn tree(n) {
  let outs = 0;
  for j in 1..n {
    for i in 1..j {
      outs += "*";
    }
    outs += "\n";
  }
  outs
};
  `
  let examples = "\n```Rhai_host\n 2+2\n```\n\n\n```Rhai_host\nrollDice(#{sides:20, num:2})\n```\n"; 
  examples += "\n```Rhai_host\nlet sum = 0;\nfor i in 1..51 {\n    sum += i;\n}\nsum\n```\n";
  examples += "\n\n```Rhai_host\n${tree}\n```";
 
`You are a helpful AI assistant. You should help answer the user's questions. Only if they need it, you can evaluate Rhai code write a reply like the following: 

${examples}

and then wait for the RESULT (automatically supplied by the system). IMPORTANT: Backticks with Rhai_host language spec are required.

Otherwise, just converse with the user normally. Do not explain the code execution to the user, or use it, unless you
need to do a calculation or analysis.

IMPORTANT: Do not show examples of code evaluation unless requested!
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

