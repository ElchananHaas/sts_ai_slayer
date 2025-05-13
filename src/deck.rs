//This file simulates a deck. It allows for lazy sampling.
use crate::{card::Card, rng::Rng};

#[derive(Clone, Debug, PartialEq, Eq)]
enum DeckSegment {
    //This is a segment of the deck that is shuffled. It is
    //not in any particular order, cards will be drawn from it using an RNG.
    Shuffled(Vec<Card>),
    //This is a segment of the deck that is known.
    Known(Vec<Card>),
    //This is a segment of the deck made up of other segments.
    Composite(Vec<Deck>),
    //This segment is created when cards are shuffled into the deck
    ShuffleInto {
        primary: Box<Deck>,
        shuffled: Box<Deck>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Deck {
    num_cards: usize,
    segment: DeckSegment,
}

impl Deck {
    //This tries to draw from a deck, removing the last card in the process.
    //The deck must be nonempty or this will panic. This should only be called by
    //the game which will reshuffle the deck if it is empty.
    pub fn draw(&mut self, rng: &mut Rng) -> Card {
        self.num_cards -= 1;
        match &mut self.segment {
            DeckSegment::Shuffled(cards) => {
                debug_assert!(self.num_cards + 1 == cards.len());
                let idx = rng.sample(cards.len());
                cards.swap_remove(idx)
            }
            DeckSegment::Known(cards) => {
                debug_assert!(self.num_cards + 1 == cards.len());
                cards.pop().expect("Cards are nonempty")
            }
            DeckSegment::Composite(decks) => {
                debug_assert!(
                    self.num_cards + 1 == decks.into_iter().map(|deck| deck.num_cards).sum()
                );
                while let Some(_) = decks.pop_if(|deck| deck.num_cards == 0) {}
                decks
                    .last_mut()
                    .expect("There is a nonempty deck")
                    .draw(rng)
            }
            DeckSegment::ShuffleInto { primary, shuffled } => {
                debug_assert!(self.num_cards + 1 == primary.num_cards + shuffled.num_cards);
                let total_cards = primary.num_cards + shuffled.num_cards;
                let sample = rng.sample(total_cards);
                if sample > primary.num_cards {
                    shuffled.draw(rng)
                } else {
                    primary.draw(rng)
                }
            }
        }
    }

    pub fn shuffle(cards: Vec<Card>) -> Deck {
        Deck {
            num_cards: cards.len(),
            segment: DeckSegment::Shuffled(cards),
        }
    }
}
