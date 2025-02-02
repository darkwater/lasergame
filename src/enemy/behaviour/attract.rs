use bevy::prelude::*;

use crate::{misc::TargetMovement, player::PlayerShip};

#[derive(Component)]
pub struct AttractBehaviour {
    pub activation_range: f32,
    pub tracking: Option<Entity>,
}

impl AttractBehaviour {
    pub fn new(activation_range: f32) -> Self {
        Self { activation_range, tracking: None }
    }
}

pub fn activate(
    mut enemies: Query<(&mut AttractBehaviour, &GlobalTransform)>,
    players: Query<(Entity, &GlobalTransform), (With<PlayerShip>, Changed<GlobalTransform>)>,
) {
    for (mut behaviour, transform) in enemies.iter_mut() {
        if behaviour.tracking.is_some() {
            continue;
        }

        let Some((player, player_pos)) = players.iter().next() else {
            continue;
        };

        let player_pos = player_pos.translation();
        let enemy_pos = transform.translation();
        let dir = player_pos - enemy_pos;

        if dir.length_squared() < behaviour.activation_range.powi(2) {
            debug!("Enemy activated");
            behaviour.tracking = Some(player);
        }
    }
}

pub fn follow(
    mut enemies: Query<(&AttractBehaviour, &GlobalTransform, &mut TargetMovement)>,
    players: Query<&GlobalTransform, With<PlayerShip>>,
) {
    for (behaviour, transform, mut target_movement) in enemies.iter_mut() {
        let Some(target) = behaviour.tracking else {
            continue;
        };

        let Ok(player) = players.get(target) else {
            continue;
        };

        let player_pos = player.translation();
        let enemy_pos = transform.translation();
        let dir = player_pos - enemy_pos;

        target_movement.0 = dir.xy();
    }
}
