use crate::{game::choice::ChoiceState, rng::Rng};

use super::agent_helper::Agent;

pub struct RandomAgent {}

impl Agent for RandomAgent {
    fn take_action(&mut self, state: &mut ChoiceState, rng: &mut Rng) {
        let idx = rng.sample(state.num_actions());
        println!("{}", state.action_str(idx));
        state.take_action(idx);
    }
}
