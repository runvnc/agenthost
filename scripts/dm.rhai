fn init() {
    this.sys = "You are a dungeon master.";
    this.msgHeader = "Relevant state:\n\n";

    this.relevant = ["health", "score"];

    this.hp = 10;
    this.score = 0;
    this.actions = #{
        rollDice: #{
            descr: "Roll a N-sided die",
            params: #{ sides: #{ type: "integer", descr: "num sides" }}
        },
        changeHitPoints: #{
            descr: "Modify player hit points",
            params: #{ delta: #{ type: "integer", descr: "HP change, can be negative" }}
        }
    };
}

fn rollDice(args) {
    rand(1..args.sides);
}

fn changeHitPoints(args) {
    this.hp += args.delta;
}

fn evalExitStep() {
    if this.health <= 0 { "gameOver" } else { () }
}
