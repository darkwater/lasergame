use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, Actionlike};

use crate::misc::MovementSpeed;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    Look,
    Shoot,
}

pub fn input_map() -> InputMap<Action> {
    InputMap::default()
        // keyboard+mouse
        .with_dual_axis(Action::Move, VirtualDPad::wasd())
        .with_dual_axis(Action::Move, VirtualDPad::arrow_keys())
        .with(Action::Shoot, MouseButton::Left)
        // gamepad
        .with_dual_axis(Action::Move, GamepadStick::LEFT)
        .with_dual_axis(Action::Look, GamepadStick::RIGHT)
}

pub fn update_velocity(
    mut query: Query<(&mut LinearVelocity, &ActionState<Action>, &MovementSpeed)>,
    time: Res<Time>,
) {
    for (mut velocity, action_state, movement_speed) in query.iter_mut() {
        let target_velocity = action_state.axis_pair(&Action::Move) * movement_speed.max_speed;

        let mut delta = target_velocity - velocity.0.xy();
        delta *= time.delta().as_secs_f32() / (1. / movement_speed.acceleration);

        velocity.0 += delta.extend(0.0);
    }
}
