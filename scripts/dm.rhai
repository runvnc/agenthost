fn init() {
    this.sys = "You are a dungeon master.";
    this.msgHeader = "Relevant state:\n\n";

    this.relevant = ["health", "score"];

    this.health = "fine";
    this.score = 0;
    this.actions = #{
        rollDice: #{
            descr: "Roll a N-sided die",
            params: #{ sides: #{ type: "integer", descr: "num sides" }}
        }
    };
}

fn rollDice(args) {
    rand(1..args.sides);
}


