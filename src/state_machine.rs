pub trait StateFns<'a, const NUM_STATES: usize> {
    type StateEnum: Into<usize> + From<usize>;
    type StateData: 'a;

    fn on_enter(
        &self,
        state_data: &mut Self::StateData,
        state_machine: &mut StateMachine<'a, Self::StateData, Self::StateEnum, NUM_STATES>,
    );

    fn on_exit(
        &self,
        state_data: &mut Self::StateData,
        state_machine: &mut StateMachine<'a, Self::StateData, Self::StateEnum, NUM_STATES>,
    );

    fn update(
        &self,
        state_data: &mut Self::StateData,
        state_machine: &mut StateMachine<'a, Self::StateData, Self::StateEnum, NUM_STATES>,
    );
}

pub struct StateMachine<'a, T: 'a, S: 'a, const NUM_STATES: usize>
where
    S: Into<usize> + From<usize>,
{
    states: [&'a dyn StateFns<'a, NUM_STATES, StateEnum = S, StateData = T>; NUM_STATES],
    current_state: usize,
    next_state: Option<usize>,
}

impl<'a, T, S, const NUM_STATES: usize> StateMachine<'a, T, S, NUM_STATES>
where
    S: Into<usize> + From<usize>,
{
    pub fn new(
        states: [&'a dyn StateFns<'a, NUM_STATES, StateEnum = S, StateData = T>; NUM_STATES],
    ) -> Self {
        Self {
            states,
            current_state: S::from(0).into(),
            next_state: None,
        }
    }

    pub fn set_state(&mut self, state: S) {
        self.next_state = Some(state.into());
    }

    pub fn update(&mut self, state_data: &mut T) {
        if self.next_state.is_some() {
            self.states[self.current_state].on_exit(state_data, self);
            self.current_state = self.next_state.take().unwrap();
            self.states[self.current_state].on_enter(state_data, self);
        }

        self.states[self.current_state].update(state_data, self);
    }

    pub fn init(&mut self, state_data: &mut T) {
        self.states[self.current_state].on_enter(state_data, self);
    }
}
