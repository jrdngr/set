use crate::enum_values::EnumValues;
use crate::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::new();

        for color in Color::values() {
            for shape in Shape::values() {
                for fill in Fill::values() {
                    for count in Count::values() {
                        cards.push(Card::new(color, shape, fill, count));
                    }
                }
            }
        }

        Self { cards }
    }
}
