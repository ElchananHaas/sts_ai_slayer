use crate::{game::ChoiceState, rng::Rng};

pub trait Agent {
    fn take_action<'a>(&mut self, state: &mut ChoiceState<'a>, rng: &mut Rng);
}

pub struct SkipSingleChoiceAgent<T> {
    pub agent: T,
}

impl<T> Agent for SkipSingleChoiceAgent<T>
where
    T: Agent,
{
    fn take_action<'a>(&mut self, state: &mut ChoiceState<'a>, rng: &mut Rng) {
        if state.num_actions() == 1 {
            state.take_action(0);
        } else {
            self.agent.take_action(state, rng);
        }
    }
}
