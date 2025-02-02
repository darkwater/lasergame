use std::f32::consts::PI;

use avian3d::prelude::{Collider, CollisionLayers, PhysicsLayer as _, Restitution, RigidBody};
use bevy::{prelude::*, utils::HashMap};
use rand::Rng as _;

use self::{cell::Cell, cell_tracker::CellTracker};
use crate::{
    enemy::dot::DotEnemy,
    line_material::LineMaterial,
    misc::{DebugVisibility, GameLayer},
    player::PlayerShip,
    shapes::Square,
    weapon::damage::Health,
};

pub mod cell;
pub mod cell_tracker;

/// Square radius, square length = 2 * radius + 1
pub const GEN_RADIUS: i32 = 2;

#[derive(Default)]
pub struct MapgenPlugin;

impl Plugin for MapgenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapgenState>()
            .add_event::<GenerateCell>()
            .add_systems(Update, (cell_tracker::update, generate_around_player, generate));
    }
}

#[derive(Resource, Default)]
struct MapgenState {
    cells: HashMap<Cell, CellState>,
}

enum CellState {
    Generated,
}

#[derive(Event)]
pub struct GenerateCell(pub Cell);

fn generate(
    mut events: EventReader<GenerateCell>,
    mut state: ResMut<MapgenState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for GenerateCell(cell) in events.read() {
        state.cells.insert(*cell, CellState::Generated);

        let mut rng = rand::thread_rng();

        commands.spawn((
            Mesh3d(meshes.add(Square::new(Cell::SIZE * 0.99))),
            MeshMaterial3d(materials.add((LinearRgba::RED * 0.01).with_alpha(1.0))),
            Transform::from_translation(Vec3::new(
                cell.x as f32 * Cell::SIZE,
                cell.y as f32 * Cell::SIZE,
                0.,
            )),
            DebugVisibility,
        ));

        for _ in 0..rng.gen_range(1..=10) {
            let size = rng.gen_range(2.0..10.0);
            commands.spawn((
                Mesh3d(asset_server.load("models/testcube.mdl.json")),
                MeshMaterial3d(materials.add(Color::hsv(
                    rng.gen_range(0.0..360.0),
                    0.1,
                    rng.gen_range(0.8..5.0),
                ))),
                RigidBody::Static,
                Collider::cuboid(1., 1., 1.),
                CollisionLayers::new(GameLayer::MapGeometry, GameLayer::all_bits()),
                Transform {
                    translation: Vec3::new(
                        cell.center().x + rng.gen_range(-Cell::SIZE..Cell::SIZE) / 2.,
                        cell.center().y + rng.gen_range(-Cell::SIZE..Cell::SIZE) / 2.,
                        0.,
                    ),
                    rotation: Quat::from_rotation_z(rng.gen_range(-PI..PI)),
                    scale: Vec3::splat(size),
                },
                Health::max(size * 100.),
                Restitution::new(1.5),
            ));
        }

        for _ in 0..rng.gen_range(1..=10) {
            commands.spawn((DotEnemy, Transform {
                translation: Vec3::new(
                    cell.center().x + rng.gen_range(-Cell::SIZE..Cell::SIZE) / 2.,
                    cell.center().y + rng.gen_range(-Cell::SIZE..Cell::SIZE) / 2.,
                    0.,
                ),
                rotation: rng.gen(),
                scale: Vec3::splat(1.),
            }));
        }

        for x in -2..=2 {
            for y in -2..=2 {
                commands.spawn((
                    Mesh3d(meshes.add(Square::new(4.))),
                    MeshMaterial3d(materials.add(Color::hsv(
                        200. + x as f32 / 20. * 100.,
                        1.,
                        0.2,
                    ))),
                    Transform::from_xyz(
                        (cell.x as f32 * Cell::SIZE) + x as f32 * 10.,
                        (cell.y as f32 * Cell::SIZE) + y as f32 * 10.,
                        rand::thread_rng().gen_range(-5..-1) as f32 * 10.,
                    ),
                ));
            }
        }
    }
}

fn generate_around_player(
    player_ship: Query<&CellTracker, (With<PlayerShip>, Changed<CellTracker>)>,
    state: Res<MapgenState>,
    mut events: EventWriter<GenerateCell>,
) {
    let Ok(tracker) = player_ship.get_single() else {
        return;
    };

    for x in -GEN_RADIUS..=GEN_RADIUS {
        for y in -GEN_RADIUS..=GEN_RADIUS {
            let cell = tracker.0 + IVec2::new(x, y);

            if let Some(CellState::Generated) = state.cells.get(&cell) {
                continue;
            }

            trace!("generate_around_player: {cell:?}");

            events.send(GenerateCell(cell));
        }
    }
}
