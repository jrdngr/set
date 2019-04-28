use crate::prelude::*;

pub struct Board {
    cards: Vec<Card>,
}

impl Default for Board {
    fn default() -> Self {
        Self { cards: Vec::new() }
    }
}

impl Board {
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn view_cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn get_set_by_index(&mut self, card_1: usize, card_2: usize, card_3: usize) -> Option<Set> {
        let card_1 = self.cards.remove(card_1);
        let card_2 = self.cards.remove(card_2);
        let card_3 = self.cards.remove(card_3);

        let set = Set(card_1, card_2, card_3);

        if set.is_valid() {
            Some(set)
        } else {
            None
        }
    }

    pub fn contains_valid_set(&self) -> bool {
        if self.cards.len() < 3 {
            return false;
        }

        for i in 0..self.cards.len() {
            for j in 1..self.cards.len() {
                for k in 2..self.cards.len() {
                    if i == j || i == k || j == k {
                        continue;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
