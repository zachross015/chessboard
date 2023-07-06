pub use fen::*;
pub use num_traits::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};
use bevy::prelude::{Resource, Component, Vec3};

#[derive(Resource)]
pub struct Board(BoardState);

impl Board {
    pub fn from_fen<'a>(fen: &'a str) -> FenResult<'a, Self> {
        match BoardState::from_fen(fen) {
            Ok(res) => Ok(Board(res)),
            Err(err) => Err(err),
        }
    }

    pub fn to_fen(&self) -> String {
        self.state().to_fen()
    }

    pub fn move_piece(&mut self, original_space: Space, new_space: Space) {
        let space = original_space as usize;
        let og = &self.0.pieces[space];
        self.0.pieces[new_space as usize] = og.clone();
        self.0.pieces[space] = None;
    }

    pub fn state(&self) -> &BoardState {
        &self.0
    }
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, Component, PartialEq, Eq)]
pub enum Space {
    #[num_traits]
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Space {
    pub fn physical_position(&self) -> Vec3 {
        let i = self.to_i32().unwrap();
        let xf = ((i % 8) - 4) as f32;
        let yf = ((i / 8) - 4) as f32;
        Vec3::new(xf, yf, 0.0)
    }
}
