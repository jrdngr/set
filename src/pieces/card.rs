use crate::utils::EnumValues;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Purple,
}

impl EnumValues for Color {
    fn values() -> Vec<Self> {
        vec![Color::Red, Color::Green, Color::Purple]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Shape {
    Squiggle,
    Oval,
    Rectangle,
}

impl EnumValues for Shape {
    fn values() -> Vec<Self> {
        vec![Shape::Squiggle, Shape::Oval, Shape::Rectangle]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Fill {
    Solid,
    Striped,
    Empty,
}

impl EnumValues for Fill {
    fn values() -> Vec<Self> {
        vec![Fill::Solid, Fill::Striped, Fill::Empty]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Count {
    One,
    Two,
    Three,
}

impl EnumValues for Count {
    fn values() -> Vec<Self> {
        vec![Count::One, Count::Two, Count::Three]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Card {
    color: Color,
    shape: Shape,
    fill: Fill,
    count: Count,
}

impl Card {
    pub fn new(color: Color, shape: Shape, fill: Fill, count: Count) -> Self {
        Self {
            color,
            shape,
            fill,
            count,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn fill(&self) -> Fill {
        self.fill
    }

    pub fn count(&self) -> Count {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        let card = Card::new(Color::Green, Shape::Oval, Fill::Empty, Count::Three);
        assert_eq!(card.color(), Color::Green);
        assert_eq!(card.shape(), Shape::Oval);
        assert_eq!(card.fill(), Fill::Empty);
        assert_eq!(card.count(), Count::Three);
    }
}
