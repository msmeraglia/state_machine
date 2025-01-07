use state_machine::{StateFns, StateMachine, States};
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

// below to be generated
static STARTUP: StartUp = StartUp;
static GAMEPLAY: GamePlay = GamePlay;
static ENDING: Ending = Ending;

// Specify a fixed length for the states manually.
static LIST: &'static [&'static dyn for<'game_data, 'machine> StateFns<
    'game_data,
    { GameStates::len() },
    StateData = (&'game_data mut GameData, &'game_data mut Engine),
    StateEnum = GameStates,
>; GameStates::len()] = &[&STARTUP, &GAMEPLAY, &ENDING];

impl<'game_data, 'machine>
    States<
        'game_data,
        'machine,
        (&'game_data mut GameData, &'game_data mut Engine),
        GameStates,
        { GameStates::len() },
    > for GameStates
where
    'game_data: 'machine,
{
    fn get_states() -> &'machine [&'machine dyn StateFns<
        'game_data,
        { GameStates::len() },
        StateData = (&'game_data mut GameData, &'game_data mut Engine),
        StateEnum = GameStates,
    >; GameStates::len()] {
        LIST
    }

    fn num_states() -> usize {
        GameStates::len()
    }
}

fn main() {
    let game_data = &mut GameData {
        player_name: String::from("Cool Main Character"),
        score: 0,
        should_exit: false,
    };

    let engine = &mut Engine { dt: 1.0 / 60.0 };
    let state_data = &mut (game_data, engine);

    let mut game = StateMachine::new::<GameStates>();

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
