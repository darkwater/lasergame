use avian3d::prelude::{Collider, CollisionLayers, LinearVelocity, PhysicsLayer as _, RigidBody};
use bevy::{
    prelude::*,
    utils::{Duration, Instant},
};

use self::damage::{Damage, DamageType, ImpactDamage};
use crate::{
    line_material::{LineMaterial, LineStrip},
    misc::GameLayer,
    team::Team,
};

pub mod damage;

#[derive(Default)]
pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Shoot>()
            .add_event::<ShootActiveWeapon>()
            .add_event::<damage::DamageEvent>()
            .add_event::<damage::FatalDamage>()
            .add_systems(
                Update,
                (
                    damage::contact_damage,
                    damage::apply_damage,
                    damage::despawn_on_fatal_damage,
                    shoot,
                    shoot_active_weapon.before(shoot),
                ),
            );
    }
}

/// Trigger a Weapon entity to shoot.
#[derive(Event)]
pub struct Shoot(pub Entity);

/// Send a [Shoot] event to target entity's ActiveWeapon entity.
#[derive(Event)]
pub struct ShootActiveWeapon(pub Entity);

#[derive(Component)]
pub struct ActiveWeapon(pub Entity);

#[derive(Component)]
#[require(WeaponState, Transform)]
pub struct Weapon {
    pub cooldown: Duration,
}

#[derive(Component, Default)]
pub struct WeaponState {
    pub last_used: Option<Instant>,
}

fn shoot(
    mut events: EventReader<Shoot>,
    mut weapons: Query<(&Weapon, &mut WeaponState, &GlobalTransform, Option<&Team>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut commands: Commands,
) {
    for Shoot(entity) in events.read() {
        if let Ok((weapon, mut state, transform, team)) = weapons.get_mut(*entity) {
            if state
                .last_used
                .is_none_or(|last_used| last_used.elapsed() >= weapon.cooldown)
            {
                state.last_used = Some(Instant::now());

                commands.spawn((
                    Mesh3d(meshes.add(LineStrip { points: vec![Vec3::ZERO, Vec3::X] })),
                    MeshMaterial3d(materials.add(LinearRgba::RED * 50.)),
                    Transform::from_translation(transform.translation() + transform.right() * 1.)
                        .with_scale(Vec3::splat(2.))
                        .with_rotation(transform.rotation()),
                    RigidBody::Kinematic,
                    LinearVelocity(transform.right() * 50.),
                    Collider::segment(Vec3::ZERO, Vec3::X),
                    // Sensor,
                    CollisionLayers::new(
                        GameLayer::Bullet,
                        GameLayer::all_bits() ^ team.unwrap().game_layer().to_bits(),
                    ),
                    ImpactDamage {
                        damage: Damage {
                            value: 10.,
                            ty: DamageType::Energy,
                            credit: Some(*entity),
                        },
                        despawn_on_impact: true,
                    },
                ));
            }
        }
    }
}

pub fn shoot_active_weapon(
    active_weapons: Query<&ActiveWeapon>,
    mut reader: EventReader<ShootActiveWeapon>,
    mut writer: EventWriter<Shoot>,
) {
    for ShootActiveWeapon(entity) in reader.read() {
        if let Ok(active_weapon) = active_weapons.get(*entity) {
            writer.send(Shoot(active_weapon.0));
        }
    }
}
