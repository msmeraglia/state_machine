use state_machine::StateMachine;
use states::*;
use super_enum::SuperEnum;

mod state_machine;
mod states;

#[derive(Debug)]
pub struct GameData {
    pub player_name: String,
    pub score: u32,
    pub should_exit: bool,
}

#[derive(Debug)]
pub struct Engine {
    pub dt: f32,
}

#[derive(Debug, Copy, Clone, SuperEnum)]
pub enum GameStates {
    StartUp,
    Gameplay,
    Ending,
}

static STARTUP: &'static StartUp = &StartUp;
static GAMEPLAY: &'static GamePlay = &GamePlay;
static ENDING: &'static Ending = &Ending;

// for brevity
type Game<'a> =
    StateMachine<'a, (&'a mut GameData, &'a mut Engine), GameStates, { GameStates::len() }>;

fn main() {
    let game_data = &mut GameData {
        player_name: String::from("Cool Main Character"),
        score: 0,
        should_exit: false,
    };

    let engine = &mut Engine { dt: 1.0 / 60.0 };

    let mut game: Game = StateMachine::new([STARTUP, GAMEPLAY, ENDING]);

    let state_data = &mut (game_data, engine);

    // init calls .on_enter(..) for the initial state
    game.init(state_data);

    // simulating a game loop
    loop {
        let (game_data, _) = &state_data;

        if game_data.should_exit {
            break;
        }

        game.update(state_data);
    }
}
