use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
    game::{ChoiceState, Game},
    rng::Rng,
};

pub struct MctsAgent {}

impl MctsAgent {
    pub fn take_action<'a>(&self, state: ChoiceState<'a>, rng: &mut Rng) -> ChoiceState<'a> {
        let choice = mcts(&state, rng);
        take_indexed_action(state, choice, true)
    }
}

fn take_indexed_action<'a>(state: ChoiceState<'a>, idx: usize, verbose: bool) -> ChoiceState<'a> {
    match state {
        ChoiceState::PlayCardState(play_card_state) => {
            let actions = play_card_state.available_actions();
            if verbose {
                println!("{}", play_card_state.action_str(actions[idx]));
            }
            play_card_state.take_action(actions[idx])
        }
        ChoiceState::ChooseEnemyState(choose_enemy_state) => {
            let actions: Vec<crate::game::ChooseEnemyAction> =
                choose_enemy_state.available_actions();
            if verbose {
                println!("{}", choose_enemy_state.action_str(actions[idx]));
            }
            choose_enemy_state.take_action(actions[idx])
        }
        ChoiceState::WinState(_) => {
            panic!("Game is already won!");
        }
        ChoiceState::LossState(_) => {
            panic!("Game is already lost!")
        }
        ChoiceState::RewardState(reward_state) => {
            let actions = reward_state.available_actions();
            if verbose {
                println!("{}", reward_state.action_str(actions[idx]));
            }
            reward_state.take_action(actions[idx])
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct QEntry {
    taken: f32,
    reward_sum: f32,
}

struct MctsEntry {
    //This could be bad if an action is taken over 2^24 = 16,000,000 times. But thats very big.
    visit_count: f32,
    q_vals: Vec<QEntry>,
}

const EXPLORE_FACTOR: f32 = 0.5;

impl MctsEntry {
    fn ucb(&self, rng: &mut Rng) -> usize {
        let mut zero_taken = 0;
        for i in 0..self.q_vals.len() {
            if self.q_vals[i].taken == 0.0 {
                zero_taken += 1;
            }
        }
        if zero_taken > 0 {
            //MCTS performs better when random actions are taken in an unexplored state.
            let mut count = rng.sample(zero_taken);
            for i in 0..self.q_vals.len() {
                if self.q_vals[i].taken == 0.0 {
                    if count == 0 {
                        return i;
                    } else {
                        count -= 1;
                    }
                }
            }
        }
        let ucb_action = self
            .q_vals
            .iter()
            .map(|q| {
                let mean = q.reward_sum / q.taken;
                let ucb_adjust = f32::sqrt(EXPLORE_FACTOR * f32::ln(self.visit_count) / q.taken);
                mean + ucb_adjust
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .expect("Non-empty list of actions");
        ucb_action
    }

    fn update(&mut self, action: usize, reward: f32) {
        self.visit_count += 1.0;
        self.q_vals[action].taken += 1.0;
        self.q_vals[action].reward_sum += reward;
    }

    fn choice(&self) -> usize {
        let ucb_action = self
            .q_vals
            .iter()
            .map(|q| q.taken)
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .expect("Non-empty list of actions");
        ucb_action
    }
}

fn hash_choice_state(state: &ChoiceState) -> u64 {
    let mut s = DefaultHasher::new();
    state.hash(&mut s);
    s.finish()
}

fn mcts<'a>(state: &ChoiceState<'a>, rng: &mut Rng) -> usize {
    let mut total_reward = 0.0;
    //This should be changed to an identity hasher.
    let mut value_map: HashMap<u64, MctsEntry> = HashMap::new();
    //This will be overwritten.
    let mut temp_game = Game::new(crate::game::Charachter::IRONCLAD);

    for i in 0..20000 {
        const REWARD_PRINT_INTERVAL: i32 = 1000;
        let choice_copy = state.clone_to(&mut temp_game);
        let reward = mcts_rollout(choice_copy, &mut value_map, rng);
        if i > 0 && i % REWARD_PRINT_INTERVAL == 0 {
            println!(
                "Average rewards are {}",
                total_reward / (REWARD_PRINT_INTERVAL as f32)
            );
            total_reward = 0.0;
        }
        //Add to the total reward after the print statement to get accurate average rewards.
        total_reward += reward;
    }
    let state_hash = hash_choice_state(&state);
    value_map
        .get_mut(&state_hash)
        .expect("State found")
        .choice()
}

//This function rolls out a game. It mutatates its input
fn mcts_rollout(
    mut state: ChoiceState,
    value_map: &mut HashMap<u64, MctsEntry>,
    rng: &mut Rng,
) -> f32 {
    let mut state_hashes = Vec::new();
    let mut taken_actions = Vec::new();
    let reward: i32 = loop {
        //Check if the game is over before computing any hashes
        let num_actions = match &state {
            ChoiceState::PlayCardState(play_card_state) => {
                play_card_state.available_actions().len()
            }
            ChoiceState::ChooseEnemyState(choose_enemy_state) => {
                choose_enemy_state.available_actions().len()
            }
            ChoiceState::WinState(game) => {
                break game.floor;
            }
            ChoiceState::LossState(game) => {
                break game.floor;
            }
            ChoiceState::RewardState(reward_state) => reward_state.available_actions().len(),
        };
        let state_hash = hash_choice_state(&state);
        let mcts_entry = value_map.entry(state_hash).or_insert_with(|| MctsEntry {
            visit_count: 0.0,
            q_vals: vec![
                QEntry {
                    taken: 0.0,
                    reward_sum: 0.0
                };
                num_actions
            ],
        });
        let action_idx = mcts_entry.ucb(rng);
        state_hashes.push(state_hash);
        taken_actions.push(action_idx);
        state = take_indexed_action(state, action_idx, false);
    };
    let reward = reward as f32;
    for i in 0..state_hashes.len() {
        value_map
            .get_mut(&state_hashes[i])
            .expect("State found")
            .update(taken_actions[i], reward);
    }
    reward as f32
}
