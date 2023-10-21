fn init() {
    this.sys = "You are an AI assistant with Rhai script execution abilities.";
    this.msgHeader = "Relevant state:\n\n";

    this.relevant = ["location", "health"];

    this.health = "fine";
    this.count = 0;
    this.score = 0;
    this.actions = #{
        runCode: #{
            descr: "Evaluate some Rhai code",
            params: #{ script: #{ type: "string", 
                       descr: "rhai code (cannot contain function definitions; remember rhai uses double quotes only)" }}
        }
    };
}

fn runCode(args) {
    return eval(args.script);
}


