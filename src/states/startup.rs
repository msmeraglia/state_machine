use crate::{
    state_machine::{StateFns, StateMachine},
    Engine, GameData, GameStates,
};

#[derive(Debug)]
pub struct StartUp;

impl<'game_data, 'machine, const NUM_STATES: usize> StateFns<'game_data, NUM_STATES> for StartUp
where
    'game_data: 'machine,
    Self: 'static,
{
    type StateEnum = GameStates;
    type StateData = (&'game_data mut GameData, &'game_data mut Engine);

    fn on_enter(
        &self,
        state_data: &mut Self::StateData,
        state_machine: &mut StateMachine<
            'game_data,
            '_,
            Self::StateData,
            Self::StateEnum,
            NUM_STATES,
        >,
    ) {
        let (_game, _engine) = state_data;
        println!("Entering {:?}", self);
    }

    fn on_exit(
        &self,
        state_data: &mut Self::StateData,
        state_machine: &mut StateMachine<
            'game_data,
            '_,
            Self::StateData,
            Self::StateEnum,
            NUM_STATES,
        >,
    ) {
        let (_game, _engine) = state_data;
        println!("Exiting {:?}", self);
    }

    fn update(
        &self,
        state_data: &mut Self::StateData,
        state_machine: &mut StateMachine<
            'game_data,
            '_,
            Self::StateData,
            Self::StateEnum,
            NUM_STATES,
        >,
    ) {
        let (game, _engine) = state_data;

        println!("Updating {:?}. Score: {}", self, game.score);

        game.score += 1;

        if game.score >= 5 {
            state_machine.set_state(GameStates::Gameplay);
        }
    }
}
