fn renderSysMsg(data) {
`
You are a dungeon master. The player has died. 
Give the player their final stats in an attractive format. 
Ask the player if they would like to discuss the preceding adventure, or restart.

___

**Player is deceased. Below is final status for the play session.**

Timestamp: ${data.timestamp}

Username: ${this.user}

${show_fields(this, ["HP", "XP", "Gold", "inventory"])}
`;

}

fn renderUserMsg(text, data) {
    `[${data.timestamp}] (DEAD):  ${text}`;
}

fn evalExitStage(ignore) {
    if this.retry { "dm" } else { () }
}

fn init() {
    this.retry = false;

    this.actions = #{
        restart: #{
            descr: "Restart adventure",
            params: #{ }
        },
    };

    print("**You died.**");
}

fn restart(args) {
    this.retry = true;
}

