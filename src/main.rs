use avian3d::{prelude::Gravity, PhysicsPlugins};
use bevy::{
    core_pipeline::bloom::Bloom,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use self::{
    assets::AssetsPlugin, line_material::LineMaterial, misc::CAMERA_OFFSET, player::PlayerPlugin,
};

mod assets;
mod line_material;
mod misc;
mod player;
mod team;

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
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity::ZERO)
        .add_plugins(MaterialPlugin::<LineMaterial>::default())
        .register_type::<LineMaterial>()
        .add_plugins(AssetsPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, init_camera)
        .add_systems(Update, close_on_esc)
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
        Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
    ));
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
