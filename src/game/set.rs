use crate::pieces::Card;

pub struct Set(pub Card, pub Card, pub Card);

impl Set {
    pub fn is_valid(&self) -> bool {
        if self.0 == self.1 || self.0 == self.2 || self.1 == self.2 {
            return false;
        }

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
        assert!(good_set().is_valid());
    }

    #[test]
    fn test_validation_bad() {
        assert!(!bad_set().is_valid());
    }
}
