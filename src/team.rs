use bevy::prelude::*;

use crate::misc::GameLayer;

#[derive(Component, Reflect, Default, PartialEq, Eq, Clone, Copy, Debug)]
#[reflect(Component)]
pub enum Team {
    #[default]
    Unassigned,
    Player,
    Enemy,
}

impl Team {
    pub fn game_layer(&self) -> GameLayer {
        match self {
            Team::Player => GameLayer::Player,
            Team::Enemy => GameLayer::Enemy,
            _ => GameLayer::Default,
        }
    }
}

pub fn propagate_team(query: Query<(&Team, &Children), Changed<Team>>, mut commands: Commands) {
    for (team, children) in query.iter() {
        for child in children.iter() {
            commands.entity(*child).insert(*team);
        }
    }
}
