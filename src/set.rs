use crate::card::Card;

pub struct Set(Card, Card, Card);

impl Set {
    pub fn new(card1: Card, card2: Card, card3: Card) -> Option<Self> {
        if card1 == card2 || card1 == card3 || card2 == card3 {
            None
        } else {
            Some(Self(card1, card2, card3))
        }
    }

    pub fn is_valid_set(&self) -> bool {
        self.element_valid(Card::color)
            && self.element_valid(Card::shape)
            && self.element_valid(Card::fill)
            && self.element_valid(Card::count)
    }

    fn element_valid<T, F>(&self, extractor: F) -> bool
    where
        F: Fn(&Card) -> T + Clone,
        T: Eq,
    {
        self.element_same(extractor.clone()) || self.element_different(extractor)
    }

    fn element_same<T, F>(&self, extractor: F) -> bool
    where
        F: Fn(&Card) -> T,
        T: Eq,
    {
        let t1 = extractor(&self.0);
        let t2 = extractor(&self.1);
        let t3 = extractor(&self.2);

        threequal(t1, t2, t3)
    }

    fn element_different<T, F>(&self, extractor: F) -> bool
    where
        F: Fn(&Card) -> T,
        T: Eq,
    {
        let t1 = extractor(&self.0);
        let t2 = extractor(&self.1);
        let t3 = extractor(&self.2);

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
    use crate::card::{Color, Count, Fill, Shape};

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
        Set::new(card_1(), card_2(), card_3()).unwrap()
    }
    fn bad_set() -> Set {
        Set::new(card_1(), card_2(), card_3_bad()).unwrap()
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
        assert!(good_set().element_same(Card::shape));
    }

    #[test]
    fn test_element_different() {
        assert!(good_set().element_different(Card::color));
    }

    #[test]
    fn test_element_not_same() {
        assert!(!good_set().element_same(Card::color));
    }

    #[test]
    fn test_element_not_different() {
        assert!(!good_set().element_different(Card::count));
    }

    #[test]
    fn test_validation_good() {
        assert!(good_set().is_valid_set());
    }

    #[test]
    fn test_validation_bad() {
        assert!(!bad_set().is_valid_set());
    }

    #[test]
    fn test_good_construction() {
        assert!(Set::new(card_1(), card_2(), card_3()).is_some())
    }

    #[test]
    fn test_bad_construction() {
        assert!(Set::new(card_1(), card_1(), card_2()).is_none())
    }
}
