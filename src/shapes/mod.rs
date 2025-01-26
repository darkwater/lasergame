use bevy::prelude::*;

use crate::line_material::LineStrip;

pub struct Square {
    length: f32,
}

impl Square {
    pub fn new(length: f32) -> Self {
        Self { length }
    }
}

impl From<Square> for Mesh {
    fn from(value: Square) -> Self {
        let points = vec![
            Vec3::new(-value.length, -value.length, 0.) / 2.,
            Vec3::new(value.length, -value.length, 0.) / 2.,
            Vec3::new(value.length, value.length, 0.) / 2.,
            Vec3::new(-value.length, value.length, 0.) / 2.,
            Vec3::new(-value.length, -value.length, 0.) / 2.,
        ];

        let line_strip = LineStrip { points };

        line_strip.into()
    }
}
