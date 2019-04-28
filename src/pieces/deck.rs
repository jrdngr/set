use crate::prelude::*;
use crate::utils::EnumValues;
use rand::{seq::SliceRandom, thread_rng};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn num_remaining(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let deck = Deck::default();
        assert_eq!(deck.num_remaining(), 81);
    }

    #[test]
    fn test_draw() {
        let mut deck = Deck::default();
        deck.shuffle();
        assert_eq!(deck.num_remaining(), 81);
        let _ = deck.draw();
        assert_eq!(deck.num_remaining(), 80);
        let _ = deck.draw();
        assert_eq!(deck.num_remaining(), 79);
        let _ = deck.draw();
    }
}
