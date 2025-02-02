use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

pub mod behaviour {
    pub mod attract;
}
pub mod dot;

#[derive(Default)]
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (dot::init_resource,)).add_systems(
            Update,
            (
                behaviour::attract::activate.run_if(on_timer(Duration::from_millis(200))),
                behaviour::attract::follow.run_if(on_timer(Duration::from_millis(200))),
                dot::populate,
            ),
        );
    }
}
