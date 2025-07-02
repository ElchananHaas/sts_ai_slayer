use crate::game::{Choice, EventAction, Game, event::EventData};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeadAdventurer;

impl EventData for DeadAdventurer {
    fn get_actions(&self, game: &Game) -> Vec<EventAction> {
        todo!()
    }

    fn take_action(&self, game: &mut Game, action: EventAction) -> Choice {
        todo!()
    }

    fn action_str(&self, game: &Game, action: EventAction) -> String {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }
}
