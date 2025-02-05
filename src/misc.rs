use avian3d::prelude::*;
use bevy::{prelude::*, utils::Duration};

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
#[require(TargetMovement)]
pub struct MovementSpeed {
    pub max_speed: f32,
    /// 1 / accel = time to near max speed
    pub acceleration: f32,
}

#[derive(Component, Default)]
pub struct TargetMovement(pub Vec2);
pub fn target_movement(
    mut query: Query<(&mut LinearVelocity, &TargetMovement, &MovementSpeed)>,
    time: Res<Time>,
) {
    for (mut velocity, target_movement, movement_speed) in query.iter_mut() {
        let mut delta = target_movement.0 - velocity.0.xy();
        delta *= time.delta().as_secs_f32() / (1. / movement_speed.acceleration);

        velocity.0 += delta.extend(0.0);
    }
}

#[derive(Component, Reflect)]
pub struct CameraOffset {
    pub offset: Vec3,
    pub look_offset: Vec3,
}

impl Default for CameraOffset {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0., -10., 110.),
            look_offset: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
pub struct DebugVisibility;
// TODO: implement

#[derive(Component)]
pub struct Expire {
    pub deadline: Duration,
}
pub fn expire(mut query: Query<(Entity, &Expire)>, time: Res<Time>, mut commands: Commands) {
    for (entity, timeout) in query.iter_mut() {
        if time.elapsed() >= timeout.deadline {
            commands.entity(entity).despawn();
        }
    }
}
