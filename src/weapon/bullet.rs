use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    utils::Duration,
};

use super::damage::{Damage, DamageType, ImpactDamage};
use crate::{line_material::LineMaterial, misc::Expire};

#[derive(Component)]
#[component(on_add = populate)]
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

pub fn populate(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    let res = world.resource::<BulletResources>();
    let mesh = res.mesh.clone();
    let material = res.material.clone();

    let deadline = world.resource::<Time>().elapsed() + Duration::from_secs(5);

    world.commands().entity(entity).insert_if_new((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        RigidBody::Kinematic,
        Collider::segment(Vec3::ZERO, Vec3::X),
        ImpactDamage {
            damage: Damage { value: 10., ty: DamageType::Energy },
            despawn_on_impact: true,
        },
        Expire { deadline },
    ));
}
