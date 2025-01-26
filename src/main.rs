#![warn(clippy::unused_trait_names)]

use avian3d::{prelude::Gravity, PhysicsPlugins};
use bevy::{
    core_pipeline::bloom::Bloom,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{bevy_egui, bevy_inspector, DefaultInspectorConfigPlugin};
use rand::Rng as _;

use self::{
    assets::AssetsPlugin, line_material::LineMaterial, misc::CameraOffset, player::PlayerPlugin,
};

mod assets;
mod line_material;
mod misc;
mod player;
mod team;
mod utils;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "emitter=trace,wgpu=warn,big_brain=debug".to_string(),
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
            FrameTimeDiagnosticsPlugin,
            PhysicsPlugins::default(),
            AssetsPlugin,
            PlayerPlugin,
            EguiPlugin,
            DefaultInspectorConfigPlugin,
        ))
        .insert_resource(Gravity::ZERO)
        .add_plugins(MaterialPlugin::<LineMaterial>::default())
        .register_type::<LineMaterial>()
        .register_type::<CameraOffset>()
        .add_systems(Startup, (init_camera, init_misc))
        .add_systems(Update, (debug_overlay, close_on_esc))
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

fn init_misc(
    mut materials: ResMut<Assets<LineMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for x in -25..=20 {
        for y in -15..=15 {
            commands.spawn((
                Mesh3d(meshes.add(Mesh::from(Cuboid::from_length(4.)))),
                MeshMaterial3d(materials.add(Color::hsv(200. + x as f32 / 20. * 100., 1., 0.4))),
                Transform::from_translation(
                    Vec3::X * x as f32 * 4.
                        + Vec3::Y * y as f32 * 4.
                        + Vec3::Z * rand::thread_rng().gen_range(-5..-1) as f32 * 2.,
                ),
            ));
        }
    }
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
        .default_open(false)
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
