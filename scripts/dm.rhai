fn renderSysMsg(data) {
`
You are a dungeon master. Don't forget to introduce yourself to the player.

Timestamp: ${data.timestamp}

Username: ${this.user}

HP: ${this.hp}

XP: ${this.xp}

Gold: ${this.gold}

Inventory:
${this.inventory.reduce(|l| l + this + "\n", "")}
`;
}

fn renderUserMsg(text, data) {
    `[${data.timestamp}]:  ${text}`;
}

fn evalExitStage() {
    if this.hp <= 0 { "gameOver" } else { () }
}

fn init() {
    this.hp = 10;
    this.xp = 0;
    this.user = #{ name: "jason" };
    this.gold = 100;

    this.inventory = ["torch"];

    this.actions = #{
        rollDice: #{
            descr: "Roll a N-sided die",
            params: #{ sides: #{ type: "integer", descr: "num sides" }}
        },
        modifyStat: #{
            descr: "Increment/decrement numeric stat such as hp, xp, etc.",
            params: #{
                stat: #{ type: "string", descr: "Which stat to change" },
                delta: #{ type: "integer", descr: "Change, can be negative" }
            }
        },
        modifyInventory: #{
            descr: "Add or remove items from player inventory",
            params: #{ operation: #{ type: "string", descr: "add or remove" },
                       itemName: #{ type: "string", descr: "name of item" } }
        }

    };
}

fn rollDice(args) {
    rand(1..args.sides);
}

fn modifyStat(args) {
    this[args.stat] += args.delta;
}

fn modifyInventory(args) {
    if args.operation == "add" {
        this.inventory += args.itemName;
    } else {
        this.inventory.retain( |i| i != args.itemName );
    }
}


