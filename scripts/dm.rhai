fn init() {
    this.sys = "You are a dungeon master. Introduce yourself to the player.";
    this.msgHeader = "Relevant state:\n\n";

    this.relevant = ["user", "hp", "xp", "inventory"];

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

fn evalExitStep() {
    if this.health <= 0 { "gameOver" } else { () }
}
