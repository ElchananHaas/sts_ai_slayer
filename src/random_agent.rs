use crate::{game::ChoiceState, rng::Rng};

pub struct RandomAgent {}

impl RandomAgent {
    pub fn take_action<'a>(&self, state: &mut ChoiceState<'a>, rng: &mut Rng) {
        match state.get_choice() {
            crate::game::Choice::PlayCardState(play_card_actions) => {
                play_random(state, sample_random(play_card_actions, rng));
            }
            crate::game::Choice::ChooseEnemyState(choose_enemy_actions, _) => {
                play_random(state, sample_random(choose_enemy_actions, rng));
            }
            crate::game::Choice::Win => unimplemented!("Win state has no actions"),
            crate::game::Choice::Loss => unimplemented!("Loss state has no actions"),
            crate::game::Choice::RewardState(reward_state_actions) => {
                play_random(state, sample_random(reward_state_actions, rng));
            }
        }
    }
}

fn sample_random<T>(actions: &Vec<T>, rng: &mut Rng) -> usize {
    rng.sample(actions.len())
}
fn play_random(state: &mut ChoiceState, idx: usize) {
    println!("{}", state.action_str(idx));
    state.take_action(idx);
}
