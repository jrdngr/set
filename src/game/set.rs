use crate::pieces::Card;

pub struct Set(pub Card, pub Card, pub Card);

type Triple<'a> = (&'a Card, &'a Card, &'a Card);

impl Set {
    pub fn is_valid(card_1: &Card, card_2: &Card, card_3: &Card) -> bool {
        let cards = (card_1, card_2, card_3);

        Self::element_valid(cards, Card::color)
            && Self::element_valid(cards, Card::shape)
            && Self::element_valid(cards, Card::fill)
            && Self::element_valid(cards, Card::count)
    }

    pub fn into_tuple(self) -> (Card, Card, Card) {
        (self.0, self.1, self.2)
    }

    pub fn to_tuple_ref(&self) -> (&Card, &Card, &Card) {
        (&self.0, &self.1, &self.2)
    }

    pub fn is_valid_set(&self) -> bool {
        if self.0 == self.1 || self.0 == self.2 || self.1 == self.2 {
            return false;
        }

        Self::is_valid(&self.0, &self.1, &self.2)
    }

    fn element_valid<T, F>(cards: Triple, extractor: F) -> bool
    where
        F: Fn(&Card) -> T + Clone,
        T: Eq,
    {
        Self::element_same(cards, extractor.clone()) || Self::element_different(cards, extractor)
    }

    fn element_same<T, F>(cards: Triple, extractor: F) -> bool
    where
        F: Fn(&Card) -> T,
        T: Eq,
    {
        let t1 = extractor(&cards.0);
        let t2 = extractor(&cards.1);
        let t3 = extractor(&cards.2);

        threequal(t1, t2, t3)
    }

    fn element_different<T, F>(cards: Triple, extractor: F) -> bool
    where
        F: Fn(&Card) -> T,
        T: Eq,
    {
        let t1 = extractor(&cards.0);
        let t2 = extractor(&cards.1);
        let t3 = extractor(&cards.2);

        thrifferent(t1, t2, t3)
    }
}

fn threequal<T>(t1: T, t2: T, t3: T) -> bool
where
    T: Eq,
{
    (t1 == t2) && (t1 == t3)
}

fn thrifferent<T>(t1: T, t2: T, t3: T) -> bool
where
    T: Eq,
{
    (t1 != t2) && (t1 != t3) && (t2 != t3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::{Color, Count, Fill, Shape};

    fn card_1() -> Card {
        Card::new(Color::Red, Shape::Squiggle, Fill::Solid, Count::One)
    }
    fn card_2() -> Card {
        Card::new(Color::Green, Shape::Squiggle, Fill::Solid, Count::One)
    }
    fn card_3() -> Card {
        Card::new(Color::Purple, Shape::Squiggle, Fill::Solid, Count::One)
    }
    fn card_3_bad() -> Card {
        Card::new(Color::Red, Shape::Rectangle, Fill::Solid, Count::One)
    }

    fn good_set() -> Set {
        Set(card_1(), card_2(), card_3())
    }
    fn bad_set() -> Set {
        Set(card_1(), card_2(), card_3_bad())
    }

    #[test]
    fn test_threequal_good() {
        assert!(threequal(3, 3, 3));
    }

    #[test]
    fn test_threequal_bad() {
        assert!(!threequal(3, 2, 1));
    }

    #[test]
    fn test_thrifferent_good() {
        assert!(thrifferent(1, 2, 3));
    }

    #[test]
    fn test_thrifferent_bad() {
        assert!(!thrifferent(1, 1, 1));
    }

    #[test]
    fn test_element_same() {
        assert!(Set::element_same(good_set().to_tuple_ref(), Card::shape));
    }

    #[test]
    fn test_element_different() {
        assert!(Set::element_different(
            good_set().to_tuple_ref(),
            Card::color
        ));
    }

    #[test]
    fn test_element_not_same() {
        assert!(!Set::element_same(good_set().to_tuple_ref(), Card::color));
    }

    #[test]
    fn test_element_not_different() {
        assert!(!Set::element_different(
            good_set().to_tuple_ref(),
            Card::count
        ));
    }

    #[test]
    fn test_validation_good() {
        assert!(good_set().is_valid_set());
    }

    #[test]
    fn test_validation_bad() {
        assert!(!bad_set().is_valid_set());
    }
}
