fn init() {
    this.sys = "You are a dungeon master.";
    this.msgHeader = "Relevant state:\n\n";

    this.relevant = ["location", "health"];

    this.health = "fine";
    this.score = 0;

    this.actions = #{
        rollDice: #{
            type: "object",
            description: "Roll a N-sided die",
            properties: #{ 
                sides: #{ type: "integer",
                          description: "num sides" }
            }
        }
    };
}

fn rollDice(args) {
    rand(1..args.sides);
}


