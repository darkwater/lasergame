use std::ops::Sub;

use avian3d::prelude::*;
use bevy::prelude::*;

use super::behaviour::attract::AttractBehaviour;
use crate::{
    line_material::LineMaterial,
    misc::{GameLayer, MovementSpeed, LOCKED_AXES},
    team::Team,
    weapon::damage::{Damage, DamageType, Health, ImpactDamage},
};

#[derive(Component)]
pub struct DotEnemy;

#[derive(Resource)]
pub struct DotEnemyResources {
    mesh: Handle<Mesh>,
    material: Handle<LineMaterial>,
}

pub fn init_resource(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut commands: Commands,
) {
    let mesh = asset_server.load("models/dot.mdl.json");
    let material = materials.add(LinearRgba::GREEN * 10.);

    commands.insert_resource(DotEnemyResources { mesh, material: dbg!(material) });
}

pub fn populate(
    dots: Query<Entity, Added<DotEnemy>>,
    res: Res<DotEnemyResources>,
    mut commands: Commands,
) {
    for ent in dots.iter() {
        commands.entity(ent).insert_if_new((
            Mesh3d(res.mesh.clone()),
            MeshMaterial3d(res.material.clone()),
            LOCKED_AXES,
            Team::Enemy,
            RigidBody::Dynamic,
            Restitution::new(0.5),
            Collider::sphere(0.5),
            CollisionLayers::new(GameLayer::Enemy, GameLayer::all_bits()),
            Health::max(10.),
            ImpactDamage {
                damage: Damage { value: 10., ty: DamageType::Impact },
                despawn_on_impact: false,
            },
            AttractBehaviour::new(50.),
            MovementSpeed { max_speed: 50., acceleration: 1. },
        ));
    }
}
