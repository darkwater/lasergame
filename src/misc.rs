use avian3d::prelude::*;
use bevy::prelude::*;

pub const CAMERA_OFFSET: Vec3 = Vec3::new(0., -5., 50.);

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
