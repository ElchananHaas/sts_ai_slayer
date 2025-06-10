use std::{hash::Hash, mem};

//This file simulates a deck. It allows for lazy sampling.
use crate::{card::Card, rng::Rng};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Deck {
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
                //The order of cards is important, so remove is needed over swap remove.
                //I could use a better data structure, but I'll benchmark first.
                cards.remove(idx)
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
            //There are some cases where the agent can "cheat" if a card that is shuffled into
            //the deck that has the same name as a card already in the deck.
            //Knowing if the card that was shuffled into was drawn could give a player an edge.
            //It would be very tricky to fix this so I will ignore it for now.
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

    pub fn shuffled(mut cards: Vec<Card>) -> Deck {
        cards.sort();
        Deck {
            num_cards: cards.len(),
            segment: DeckSegment::Shuffled(cards),
        }
    }

    //This puts some cards on top of the deck.
    pub fn put_on_top(&mut self, cards: Vec<Card>) {
        let known_bit = Deck {
            num_cards: cards.len(),
            segment: DeckSegment::Known(cards),
        };
        let mut temp_segment = DeckSegment::Known(vec![]);
        mem::swap(&mut temp_segment, &mut self.segment);
        let prior_bit = Deck {
            num_cards: self.num_cards,
            segment: temp_segment,
        };
        self.num_cards = known_bit.len() + prior_bit.len();
        self.segment = DeckSegment::Composite(vec![prior_bit, known_bit]);
    }

    pub fn len(&self) -> usize {
        self.num_cards
    }

    //This could probably be replaced by an IntoIter implementation but that
    //would be complicated due to this being a recursive data structure.
    pub fn count(&self, f: fn(&&Card) -> bool) -> usize {
        match &self.segment {
            DeckSegment::Shuffled(cards) => cards.into_iter().filter(f).count(),
            DeckSegment::Known(cards) => cards.into_iter().filter(f).count(),
            DeckSegment::Composite(decks) => decks.into_iter().map(|deck| deck.count(f)).sum(),
            DeckSegment::ShuffleInto { primary, shuffled } => primary.count(f) + shuffled.count(f),
        }
    }
}
