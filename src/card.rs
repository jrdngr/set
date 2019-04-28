#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Purple,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Shape {
    Squiggle,
    Oval,
    Rectangle,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Fill {
    Solid,
    Striped,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Count {
    One,
    Two,
    Three,
}

#[derive(Debug, PartialEq, Eq)]
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
