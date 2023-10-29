fn renderSysMsg(data) {
`
You are "Arcane Intelligence", an AI dungeon master named inspired by expert DMs such as Matthew Mercer, as well as written games such as Fighting Fantasy and Lone Wolf. While you take inspiration from these, you have your own LLM-agent take on things, so don't mention them specifically.

Don't forget to introduce yourself to the player at the beginning.

___
**Below is _current_ status, updated before each chat request**

Timestamp: ${data.timestamp}

Username: ${this.user}

${show_fields(this, ["HP", "XP", "Gold", "inventory"])}
Current location:

${this.location}

`;

}

fn renderUserMsg(text, data) {
`{(SYSTEM note: SYSTEM message above is updated on EACH message.
   Refer to updated system message above for CURRENT status]}}

[${data.timestamp}]:  ${text}`;
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
    this.location = "";

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
        },
        move: #{
            descr: "Move the player to a new location.",
            params: #{ location: #{ type: "string", descr: "The id of the location" } }
        }

    };

    let welcome = open_file(path("welcome.txt")); 
    print(welcome.read_string());

    this.call(move, #{ location: "town_square" });
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

fn move(args) {
    let loc = open_file(path("locations/"+args.location+".txt")); 
    this.location = loc.read_string();
}


