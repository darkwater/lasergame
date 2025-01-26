use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{
    animation::{animated_field, AnimationTarget, AnimationTargetId},
    prelude::*,
};
use leafwing_input_manager::{plugin::InputManagerPlugin, InputManagerBundle};

use crate::{
    line_material::LineMaterial,
    mapgen::cell_tracker::CellTracker,
    misc::{CameraOffset, MovementSpeed, LOCKED_AXES},
    team::Team,
    utils::LookAt2d as _,
};

mod input;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<input::Action>::default())
            .add_systems(Startup, init_player)
            .add_systems(
                Update,
                (
                    input::update_velocity,
                    input::update_zoom,
                    input::update_target_pos,
                    aim_player_ship,
                    camera_follow_player,
                ),
            );
    }
}

#[derive(Component)]
pub struct PlayerShip;

#[derive(Component)]
pub struct PlayerAimTarget;

fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
) {
    let mesh_name = Name::new("Player ship model");
    let anim_target_id = AnimationTargetId::from_name(&mesh_name);
    let mut animation = AnimationClip::default();
    animation.add_curve_to_target(
        anim_target_id,
        AnimatableCurve::new(
            animated_field!(Transform::rotation),
            UnevenSampleAutoCurve::new([0.0, 1.0, 2.0, 3.0, 4.0].into_iter().zip([
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::X, PI * 0.5),
                Quat::from_axis_angle(Vec3::X, PI * 1.0),
                Quat::from_axis_angle(Vec3::X, PI * 1.5),
                Quat::IDENTITY,
            ]))
            .expect("Failed to build rotation curve"),
        ),
    );

    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

    let mut animation_player = AnimationPlayer::default();
    animation_player.play(animation_index).repeat();

    let animation = commands
        .spawn((AnimationGraphHandle(graphs.add(graph)), animation_player))
        .id();

    let mesh = commands
        .spawn((
            mesh_name,
            Mesh3d(asset_server.load("models/ship.mdl.ron")),
            MeshMaterial3d(line_materials.add(Color::WHITE)),
            AnimationTarget {
                id: anim_target_id,
                player: animation,
            },
        ))
        .id();

    commands
        .spawn((
            Name::new("Player ship"),
            PlayerShip,
            Transform::default(),
            Visibility::default(),
            RigidBody::Dynamic,
            LOCKED_AXES,
            Collider::sphere(1.),
            Team::Player,
            InputManagerBundle::with_map(input::input_map()),
            MovementSpeed { max_speed: 30., acceleration: 15. },
            CameraOffset::default(),
            CellTracker::default(),
        ))
        .add_children(&[mesh, animation]);

    commands.spawn((Name::new("Player aim target"), Transform::default(), PlayerAimTarget));
}

fn camera_follow_player(
    query: Query<(&CameraOffset, &Transform), With<PlayerShip>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<PlayerShip>)>,
) {
    for (cam_offset, player_transform) in query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = player_transform.translation + cam_offset.offset;
            camera_transform
                .look_at(player_transform.translation + cam_offset.look_offset, Vec3::Z);
        }
    }
}

pub fn aim_player_ship(
    mut player_ship: Query<&mut Transform, (With<PlayerShip>, Without<PlayerAimTarget>)>,
    player_aim_target: Query<&Transform, (With<PlayerAimTarget>, Without<PlayerShip>)>,
) {
    let mut player_ship = player_ship.single_mut();
    let player_aim_target = player_aim_target.single();

    if player_ship.translation != player_aim_target.translation {
        player_ship.look_at_2d(player_aim_target.translation);
    }
}
