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

    pub fn can_damage(&self, other: &Team) -> bool {
        *self == Team::Unassigned || self != other
    }
}

pub fn propagate_team(query: Query<(&Team, &Children), Changed<Team>>, mut commands: Commands) {
    for (team, children) in query.iter() {
        for child in children.iter() {
            commands.entity(*child).insert(*team);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn team_damage() {
        assert!(Team::Player.can_damage(&Team::Enemy));
        assert!(Team::Enemy.can_damage(&Team::Player));
        assert!(!Team::Player.can_damage(&Team::Player));
        assert!(!Team::Enemy.can_damage(&Team::Enemy));
        assert!(Team::Unassigned.can_damage(&Team::Unassigned));
    }
}
