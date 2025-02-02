use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{prelude::*, Actionlike};

use super::PlayerAimTarget;
use crate::{
    misc::{CameraOffset, MovementSpeed, TargetMovement},
    weapon::{ActiveWeapon, ShootActiveWeapon},
};

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    Look,
    Shoot,
    #[actionlike(Axis)]
    Zoom,
}

pub fn input_map() -> InputMap<Action> {
    InputMap::default()
        // keyboard+mouse
        .with_dual_axis(Action::Move, VirtualDPad::wasd())
        .with_dual_axis(Action::Move, VirtualDPad::arrow_keys())
        .with(Action::Shoot, MouseButton::Left)
        .with_axis(Action::Zoom, MouseScrollAxis::Y)
        // gamepad
        .with_dual_axis(Action::Move, GamepadStick::LEFT)
        .with_dual_axis(Action::Look, GamepadStick::RIGHT)
        .with(Action::Shoot, GamepadButton::RightTrigger2)
}

pub fn update_movement(
    mut query: Query<(&ActionState<Action>, &MovementSpeed, &mut TargetMovement)>,
) {
    for (action_state, movement_speed, mut target_movement) in query.iter_mut() {
        target_movement.0 =
            action_state.axis_pair(&Action::Move).clamp_length_max(1.) * movement_speed.max_speed;
    }
}

pub fn update_zoom(mut query: Query<(&mut CameraOffset, &ActionState<Action>)>) {
    for (mut cam_offset, action_state) in query.iter_mut() {
        if let Some(zoom) = action_state.axis_data(&Action::Zoom) {
            cam_offset.offset.z += zoom.value * -5.;

            // let min = CameraOffset::default().offset.z;
            // if cam_offset.offset.z < min {
            //     cam_offset.offset.z = min;
            // }
        }
    }
}

pub fn update_target_pos(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut player_aim_target: Query<&mut Transform, With<PlayerAimTarget>>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    let (camera, camera_transform) = camera.single();
    let mut player_aim_target = player_aim_target.single_mut();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z)) else {
        return;
    };

    let position = ray.get_point(distance);

    if player_aim_target.translation != position {
        player_aim_target.translation = position;
    }
}

pub fn try_shoot(
    query: Query<(Entity, &ActionState<Action>), With<ActiveWeapon>>,
    mut events: EventWriter<ShootActiveWeapon>,
) {
    for (entity, action_state) in query.iter() {
        if action_state.pressed(&Action::Shoot) {
            events.send(ShootActiveWeapon(entity));
        }
    }
}
