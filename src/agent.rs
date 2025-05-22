use crate::{game::ChoiceState, rng::Rng};

pub struct RandomAgent {}

impl RandomAgent {
    pub fn take_action<'a>(&self, choice: ChoiceState<'a>, rng: &mut Rng) -> ChoiceState<'a> {
        match choice {
            ChoiceState::PlayCardState(play_card_state) => {
                let actions = play_card_state.available_actions();
                let idx = rng.sample(actions.len());
                println!("{}", play_card_state.action_str(actions[idx]));
                play_card_state.take_action(actions[idx])
            }
            ChoiceState::ChooseEnemyState(choose_enemy_state) => {
                let actions = choose_enemy_state.available_actions();
                let idx = rng.sample(actions.len());
                println!("{}", choose_enemy_state.action_str(actions[idx]));
                choose_enemy_state.take_action(actions[idx])
            }
            ChoiceState::WinState(_) => {
                panic!("Game is already won!");
            }
            ChoiceState::LossState(_) => {
                panic!("Game is already lost!")
            }
            ChoiceState::MapState(_) => {
                todo!("Multiple floors aren't implemented yet!")
            }
        }
    }
}
