pub trait States<'game_data, 'machine, T: 'game_data, S, const NUM_STATES: usize>
where
    'game_data: 'machine,
    S: Into<usize> + From<usize>,
    Self: 'static,
{
    fn get_states(
    ) -> &'machine [&'machine dyn StateFns<'game_data, NUM_STATES, StateData = T, StateEnum = S>;
                     NUM_STATES];
    fn num_states() -> usize;
}

pub trait StateFns<'game_data, const NUM_STATES: usize>: Sync {
    type StateEnum: Into<usize> + From<usize>;
    type StateData: 'game_data;

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
    );

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
    );

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
    );
}

pub struct StateMachine<'game_data, 'machine, T: 'game_data, S, const NUM_STATES: usize>
where
    'game_data: 'machine,
    S: Into<usize> + From<usize>,
{
    states: &'machine [&'machine dyn StateFns<'game_data, NUM_STATES, StateData = T, StateEnum = S>;
                  NUM_STATES],
    current_state: usize,
    next_state: Option<usize>,
}

impl<'game_data, 'machine, T: 'game_data, S, const NUM_STATES: usize>
    StateMachine<'game_data, 'machine, T, S, NUM_STATES>
where
    'game_data: 'machine,
    S: Into<usize> + From<usize>,
{
    pub fn new<StateEnum>() -> Self
    where
        StateEnum: States<'game_data, 'machine, T, S, NUM_STATES>,
    {
        Self {
            states: StateEnum::get_states(),
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
