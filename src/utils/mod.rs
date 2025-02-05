use bevy::prelude::*;

pub trait LookAt2d {
    fn look_at_2d(&mut self, target: Vec3);
}

impl LookAt2d for Transform {
    fn look_at_2d(&mut self, target: Vec3) {
        let mut direction = target - self.translation;
        direction.z = 0.;
        direction = direction.normalize();
        self.rotation = Quat::from_rotation_z(direction.y.atan2(direction.x));
    }
}

pub trait RoundTo<T> {
    fn round_to(self, multiple: T) -> Self;
    fn round_to_offset(self, multiple: T, offset: T) -> Self;
}

impl RoundTo<f32> for Vec2 {
    fn round_to(self, multiple: f32) -> Self {
        (self / multiple).round() * multiple
    }

    fn round_to_offset(self, multiple: f32, offset: f32) -> Self {
        ((self - offset) / multiple).round() * multiple + offset
    }
}

impl RoundTo<f32> for Vec3 {
    fn round_to(self, multiple: f32) -> Self {
        (self / multiple).round() * multiple
    }

    fn round_to_offset(self, multiple: f32, offset: f32) -> Self {
        ((self - offset) / multiple).round() * multiple + offset
    }
}

impl RoundTo<f32> for f32 {
    fn round_to(self, multiple: f32) -> Self {
        (self / multiple).round() * multiple
    }

    fn round_to_offset(self, multiple: f32, offset: f32) -> Self {
        ((self - offset) / multiple).round() * multiple + offset
    }
}
