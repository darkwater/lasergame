use avian3d::prelude::*;
use bevy::prelude::*;

use crate::team::Team;

pub const LOCKED_AXES: LockedAxes = LockedAxes::new()
    .lock_rotation_x()
    .lock_rotation_y()
    .lock_translation_z();

#[derive(PhysicsLayer, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Bullet,
    MapGeometry,
}

impl GameLayer {
    pub fn team(&self) -> Team {
        match self {
            GameLayer::Player => Team::Player,
            GameLayer::Enemy => Team::Enemy,
            _ => Team::Unassigned,
        }
    }
}

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
            offset: Vec3::new(0., -10., 80.),
            look_offset: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
pub struct DebugVisibility;
// TODO: implement
