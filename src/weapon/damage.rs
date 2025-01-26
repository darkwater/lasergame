use avian3d::prelude::CollidingEntities;
use bevy::prelude::*;

use crate::team::Team;

#[derive(Component)]
#[require(Team)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn max(max: f32) -> Self {
        Self { max, current: max }
    }
}

#[derive(Component)]
#[require(Team, CollidingEntities)]
pub struct ImpactDamage {
    pub damage: Damage,
    pub despawn_on_impact: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Damage {
    pub value: f32,
    pub ty: DamageType,
    pub credit: Option<Entity>,
}

/// Transfer credit to the specified entity. Useful for weapons, where the credit should be
/// transferred to the entity that fired the weapon.
#[derive(Component)]
pub struct TransferCredit(pub Entity);

#[derive(Debug, Clone, Copy)]
pub enum DamageType {
    Energy,
    Explosion,
}

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage: Damage,
}

/// Emitted after a [DamageEvent] caused an entity's health to drop to 0.
#[derive(Event)]
pub struct FatalDamage {
    /// The entity that died
    pub target: Entity,
    /// The damage that caused the entity to die
    pub damage: Damage,
}

pub fn contact_damage(
    bullets: Query<(Entity, &ImpactDamage, &CollidingEntities)>,
    targets: Query<Entity, With<Health>>,
    mut writer: EventWriter<DamageEvent>,
    mut commands: Commands,
) {
    for (bullet, impact_damage, colliding_entities) in bullets.iter() {
        for target in colliding_entities.0.iter() {
            if let Ok(target) = targets.get(*target) {
                writer.send(DamageEvent { target, damage: impact_damage.damage });

                if impact_damage.despawn_on_impact {
                    trace!("despawning bullet {entity:?} after impact", entity = bullet);
                    commands.entity(bullet).despawn();
                }
            }
        }
    }
}

pub fn apply_damage(
    mut reader: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
    mut writer: EventWriter<FatalDamage>,
) {
    for event in reader.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            let dead = health.current <= 0.;
            health.current -= event.damage.value;

            if !dead && health.current <= 0. {
                trace!("entity {entity:?} died", entity = event.target);
                writer.send(FatalDamage {
                    target: event.target,
                    damage: event.damage,
                });
            }
        }
    }
}

pub fn despawn_on_fatal_damage(mut commands: Commands, mut reader: EventReader<FatalDamage>) {
    for FatalDamage { target, damage } in reader.read() {
        if let Some(mut e) = commands.get_entity(*target) {
            trace!("despawning entity {target:?} after fatal damage from {damage:?}");
            e.despawn()
        } else {
            warn!("despawn_on_fatal_damage: entity {target:?} already despawned");
        }
    }
}
