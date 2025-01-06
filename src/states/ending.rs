use crate::{
    state_machine::{StateFns, StateMachine},
    Engine, GameData, GameStates,
};

#[derive(Debug)]
pub struct Ending;
impl<'a, const NUM_STATES: usize> StateFns<'a, NUM_STATES> for Ending {
    type StateEnum = GameStates;
    type StateData = (&'a mut GameData, &'a mut Engine);

    fn on_enter(
        &self,
        state_data: &mut Self::StateData,
        _state_machine: &mut StateMachine<'a, Self::StateData, Self::StateEnum, NUM_STATES>,
    ) {
        let (_game, _engine) = state_data;
        println!("Entering {:?}", self);
    }

    fn on_exit(
        &self,
        state_data: &mut Self::StateData,
        _state_machine: &mut StateMachine<'a, Self::StateData, Self::StateEnum, NUM_STATES>,
    ) {
        let (_game, _engine) = state_data;
        println!("Exiting {:?}", self);
    }

    fn update(
        &self,
        state_data: &mut Self::StateData,
        _state_machine: &mut StateMachine<'a, Self::StateData, Self::StateEnum, NUM_STATES>,
    ) {
        let (game, _engine) = state_data;

        println!("Updating {:?}. The Game is over.", self);

        game.should_exit = true;
    }
}
