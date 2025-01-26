use avian3d::prelude::*;
use bevy::prelude::*;

pub const LOCKED_AXES: LockedAxes = LockedAxes::new()
    .lock_rotation_x()
    .lock_rotation_y()
    .lock_translation_z();

#[derive(Component)]
pub struct MovementSpeed {
    pub max_speed: f32,
    /// 1 / accel = time to near max speed
    pub acceleration: f32,
}

#[derive(Component, Reflect)]
pub struct CameraOffset {
    pub offset: Vec3,
    pub look_offset: Vec3,
}

impl Default for CameraOffset {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0., -5., 60.),
            look_offset: Vec3::ZERO,
        }
    }
}
