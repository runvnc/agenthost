// Main loop stage handler
let mainLoop = map();

mainLoop.init = |state, db| {
    state.health = "fine";
    state.score = state.score || 0;
    state.location = db_select("Void Foyer");  // Mocked DB select
};

mainLoop.actions = map();
mainLoop.actions.rollDice = |sides| { rand() % sides + 1 };
mainLoop.actions.setHealth = |health, state| { state.health = health };
mainLoop.actions.addScore = |points, state| { state.score += points };  // Saving mechanism can be added

mainLoop.evalExit = |state| {
    if state.health == "dead" {
        "gameOver"
    } else {
        ""
    }
};

// Game over stage handler
let gameOver = map();

gameOver.init = |state, db| {};  // No specific init actions for game over stage

gameOver.actions = map();
gameOver.actions.restart = |ignore, state| { state.retry = true };

gameOver.evalExit = |state| {
    if state.retry {
        "mainLoop"
    } else {
        ""
    }
};

// Combine all stage handlers into one map for easy access
let stages = map();
stages.mainLoop = mainLoop;
stages.gameOver = gameOver;

stages   // Return the stages map

