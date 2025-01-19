use bevy::prelude::*;

#[derive(Component, Reflect, Default, PartialEq, Eq, Clone, Copy, Debug)]
#[reflect(Component)]
pub enum Team {
    #[default]
    None,
    Player,
    Enemy,
}
