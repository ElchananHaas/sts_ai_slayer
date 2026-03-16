use crate::{game::choice::ChoiceState, rng::Rng};

pub trait Agent {
    fn action(&mut self, state: &mut ChoiceState, rng: &mut Rng) -> usize;
}

pub struct SkipSingleChoiceAgent<T> {
    pub agent: T,
}

impl<T> Agent for SkipSingleChoiceAgent<T>
where
    T: Agent,
{
    fn action(&mut self, state: &mut ChoiceState, rng: &mut Rng) -> usize {
        if state.num_actions() == 1 {
            0
        } else {
            self.agent.action(state, rng)
        }
    }
}
