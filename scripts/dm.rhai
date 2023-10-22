fn renderSysMsg(data) {
`
You are a dungeon master. Don't forget to introduce yourself to the player.

___
**Below is _current_ status, updated before each chat request**

Timestamp: ${data.timestamp}

Username: ${this.user}

${show_fields(this, ["HP", "XP", "Gold", "inventory"])}
`;

}

fn renderUserMsg(text, data) {
    `[${data.timestamp}]:  ${text}`;
}

fn evalExitStage(ignore) {
    if this.HP <= 0 { "gameover" } else { () }
}

fn init() {
    this.HP = 10;
    this.XP = 0;
    this.user = #{ name: "jason" };
    this.Gold = 100;

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
    let s = `${args.stat} updated to ${this[args.stat]}`;
    s;
}

fn modifyInventory(args) {
    if args.operation == "add" {
        this.inventory += args.itemName;
    } else {
        this.inventory.retain( |i| i != args.itemName );
    }
}



