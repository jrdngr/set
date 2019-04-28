use crate::card::Card;

pub struct Set(Card, Card, Card);

impl Set {
    pub fn build(card1: Card, card2: Card, card3: Card) -> Option<Self> {
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
