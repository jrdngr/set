pub mod game;
pub mod pieces;
pub mod utils;

pub mod prelude {
    pub use crate::game::{Board, Set};
    pub use crate::pieces::{Card, Color, Count, Deck, Fill, Shape};
}
