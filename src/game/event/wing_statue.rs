use crate::{
    card::PlayEffect,
    game::{Choice, choice::EventAction, Game, event::EventRoom},
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WingStatue;

fn has_10_attack_card(game: &Game) -> bool {
    let attack_threshold = 10;
    for card in &game.base_deck {
        for action in card.actions() {
            match action {
                PlayEffect::Attack(x) => {
                    if (*x + game.bonus_attack(card)) > attack_threshold {
                        return true;
                    }
                }
                PlayEffect::AttackAll(x) => {
                    if *x > attack_threshold {
                        return true;
                    }
                }
                PlayEffect::AttackFiendFire(x) => {
                    if *x > attack_threshold {
                        return true;
                    }
                }
                PlayEffect::AttackLethalEffect(x, _) => {
                    if *x > attack_threshold {
                        return true;
                    }
                }
                //TODO - add Skewer+ when implemented.
                _ => {}
            }
        }
    }
    false
}

impl EventRoom for WingStatue {
    fn get_actions(&self, game: &Game) -> Vec<EventAction> {
        let mut res = Vec::new();
        res.push(EventAction(0));
        if has_10_attack_card(game) {
            res.push(EventAction(1));
        }
        res.push(EventAction(2));
        res
    }

    fn take_action(self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => {
                game.player_lose_hp(7, false);
                game.goto_remove_card()
            }
            1 => {
                let amount = game.rng.sample_i32_inclusive(50, 80);
                game.gain_gold(amount);
                game.goto_map()
            }
            2 => game.goto_map(),
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, _game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => {
                format!("Lose 7 hp. Remove a card from your deck.")
            }
            1 => {
                format!("Gain 50-80 gold.")
            }
            2 => {
                format!("Leave.")
            }
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "Wing Statue"
    }

    fn new(rng: &mut Rng) -> Self {
        WingStatue
    }
}
