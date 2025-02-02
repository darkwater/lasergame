use avian3d::prelude::*;
use bevy::{prelude::*, utils::Duration};

use super::damage::{Damage, DamageType, ImpactDamage};
use crate::{line_material::LineMaterial, misc::Expire};

#[derive(Component)]
pub struct Bullet;

#[derive(Resource)]
pub struct BulletResources {
    mesh: Handle<Mesh>,
    material: Handle<LineMaterial>,
}

pub fn init_resource(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut commands: Commands,
) {
    let mesh = asset_server.load("models/laser.mdl.json");
    let material = materials.add(LinearRgba::RED * 50.);

    commands.insert_resource(BulletResources { mesh, material: dbg!(material) });
}

pub fn populate(
    dots: Query<Entity, Added<Bullet>>,
    res: Res<BulletResources>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for ent in dots.iter() {
        commands.entity(ent).insert_if_new((
            Mesh3d(res.mesh.clone()),
            MeshMaterial3d(res.material.clone()),
            RigidBody::Kinematic,
            Collider::segment(Vec3::ZERO, Vec3::X),
            ImpactDamage {
                damage: Damage { value: 10., ty: DamageType::Energy },
                despawn_on_impact: true,
            },
            Expire {
                deadline: time.elapsed() + Duration::from_secs(5),
            },
        ));
    }
}
