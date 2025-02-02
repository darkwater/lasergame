#![feature(array_windows)]
#![warn(clippy::unused_trait_names)]

use avian3d::{prelude::Gravity, PhysicsPlugins};
use bevy::{
    asset::AssetMetaCheck,
    core_pipeline::bloom::Bloom,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{bevy_egui, bevy_inspector, DefaultInspectorConfigPlugin};
use enemy::EnemyPlugin;
use team::Team;

use self::{
    assets::AssetsPlugin, line_material::LineMaterial, mapgen::MapgenPlugin, misc::CameraOffset,
    player::PlayerPlugin, weapon::WeaponPlugin,
};

mod assets;
mod enemy;
mod line_material;
mod mapgen;
mod misc;
mod player;
mod shapes;
mod team;
mod utils;
mod weapon;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "lasergame=trace,wgpu=warn,big_brain=debug".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "lasergame".into(),
                        name: Some("red.dark.lasergame".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
            #[cfg(not(target_arch = "wasm32"))]
            FrameTimeDiagnosticsPlugin,
            PhysicsPlugins::default(),
            EguiPlugin,
            DefaultInspectorConfigPlugin,
        ))
        .add_plugins((AssetsPlugin, EnemyPlugin, MapgenPlugin, PlayerPlugin, WeaponPlugin))
        .insert_resource(Gravity::ZERO)
        .add_plugins(MaterialPlugin::<LineMaterial>::default())
        .register_type::<CameraOffset>()
        .register_type::<LineMaterial>()
        .register_type::<Team>()
        .register_asset_reflect::<LineMaterial>()
        .add_systems(Startup, (init_camera, init_misc))
        .add_systems(
            Update,
            (
                team::propagate_team,
                misc::target_movement,
                misc::expire,
                debug_overlay,
                close_on_esc,
            ),
        )
        .run();
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Bloom::NATURAL,
    ));
}

fn init_misc(// mut materials: ResMut<Assets<LineMaterial>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut commands: Commands,
) {
}

fn debug_overlay(world: &mut World) {
    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
        .cloned()
    else {
        return;
    };

    let ctx = egui_context.get_mut();

    egui::Window::new("Diagnostics")
        .default_open(true)
        .default_pos((10., 10.))
        .show(ctx, |ui| {
            let diagnostics = world.get_resource::<DiagnosticsStore>().unwrap();

            for diag in diagnostics.iter() {
                if let Some(value) = diag.smoothed() {
                    ui.label(format!("{}: {:.2?}", diag.path(), value));
                }
            }
        });

    egui::Window::new("Inspector")
        .default_open(false)
        .default_pos((10., 110.))
        .show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_world(world, ui);
            });
        });
}

fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
