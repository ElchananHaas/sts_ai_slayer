use std::{
    collections::HashMap,
    fmt::Write,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
    game::{Choice, ChoiceState, Game},
    rng::Rng,
};

use super::agent_helper::Agent;

pub struct MctsAgent {}

impl Agent for MctsAgent {
    fn take_action<'a>(&mut self, state: &mut ChoiceState<'a>, rng: &mut Rng) {
        let choice = mcts(&state, rng);
        println!("{}", state.action_str(choice));
        println!("");
        state.take_action(choice);
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

const EXPLORE_FACTOR: f32 = 10.0;
const MCTS_ITERATIONS: usize = 40000;
const REWARD_PRINT_INTERVAL: usize = 5000;
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

    fn print_values(&self) {
        let mut s: String = String::new();
        write!(&mut s, "[ ").expect("Write OK");
        for entry in &self.q_vals {
            write!(&mut s, " {},", entry.reward_sum / entry.taken).expect("Write OK");
        }
        write!(&mut s, "]").expect("Write OK");
        println!("{}", s);
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
    let state_hash = hash_choice_state(&state);
    for i in 0..MCTS_ITERATIONS {
        let mut choice_copy = state.clone_to(&mut temp_game);
        let reward = mcts_rollout(&mut choice_copy, &mut value_map, rng);
        if false && i > 0 && i % REWARD_PRINT_INTERVAL == 0 {
            println!(
                "Average rewards are {}",
                total_reward / (REWARD_PRINT_INTERVAL as f32)
            );
            value_map
                .get_mut(&state_hash)
                .expect("State found")
                .print_values();
            total_reward = 0.0;
        }
        //Add to the total reward after the print statement to get accurate average rewards.
        total_reward += reward;
    }

    value_map
        .get_mut(&state_hash)
        .expect("State found")
        .choice()
}

//This function rolls out a game. It mutatates its input
fn mcts_rollout(
    state: &mut ChoiceState,
    value_map: &mut HashMap<u64, MctsEntry>,
    rng: &mut Rng,
) -> f32 {
    let mut state_hashes = Vec::new();
    let mut taken_actions = Vec::new();
    let mut in_known = true;
    let reward: i32 = loop {
        //Check if the game is over before computing any hashes
        let num_actions = match &state.get_choice() {
            Choice::Win => {
                break state.get_game().get_floor();
            }
            Choice::Loss => {
                break state.get_game().get_floor();
            }
            _ => state.num_actions(),
        };
        //Once the agent is in an unexplored state, play randomly from there on. There is no
        //point in recording it. This helps reduce bias and speed up the MCTS
        if !in_known {
            state.take_action(rng.sample(num_actions));
            continue;
        }
        let state_hash = hash_choice_state(&state);
        let mcts_entry = value_map.entry(state_hash).or_insert_with(|| {
            in_known = false;
            MctsEntry {
                visit_count: 0.0,
                q_vals: vec![
                    QEntry {
                        taken: 0.0,
                        reward_sum: 0.0
                    };
                    num_actions
                ],
            }
        });
        let action_idx = mcts_entry.ucb(rng);
        state_hashes.push(state_hash);
        taken_actions.push(action_idx);
        state.take_action(action_idx);
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
