use std::ops::Add;

use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
    pub const SIZE: f32 = 80.;

    pub fn center(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32) * Self::SIZE
    }
}

impl Add<IVec2> for Cell {
    type Output = Cell;

    fn add(self, rhs: IVec2) -> Self::Output {
        Cell { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
