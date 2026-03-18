use crate::{game::choice::ChoiceState, rng::Rng};

use super::agent_helper::Agent;

pub struct RandomAgent {}

impl Agent for RandomAgent {
    fn action(&mut self, state: &ChoiceState, rng: &mut Rng) -> usize {
        rng.sample(state.num_actions())
    }
}
