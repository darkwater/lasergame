use std::ops::RangeInclusive;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{egui, EguiContext},
    egui::{emath, DragValue, Ui, WidgetText},
};

use crate::{misc::CameraOffset, player::PlayerShip, utils::RoundTo as _};

pub mod generate;

#[derive(Default)]
pub struct MapgenPlugin;

impl Plugin for MapgenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugMapgen>()
            .register_type::<DebugMapgen>()
            .register_type::<generate::MapgenParams>()
            .register_type::<generate::DetailedMapgenOutput>()
            .add_systems(Update, debug_mapgen);
    }
}

#[derive(Resource, Reflect, Default, Clone)]
struct DebugMapgen {
    params: generate::MapgenParams,
    output: Option<generate::DetailedMapgenOutput>,
    player_cam_outline: bool,
}

fn debug_mapgen(
    mut egui_ctx: Single<&mut EguiContext, With<PrimaryWindow>>,
    mut debug_mapgen: ResMut<DebugMapgen>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    mut time: ResMut<Time<Virtual>>,
    player: Query<&Transform, With<PlayerShip>>,
) {
    let ctx = egui_ctx.get_mut();

    egui::Window::new("Mapgen")
        .default_open(true)
        .show(ctx, |ui| {
            vec2_group(ui, "Map size", &mut debug_mapgen.params.map_size, Vec2::splat(100.)..=Vec2::splat(5000.));

            num_group(ui, "Grid size", &mut debug_mapgen.params.grid_size, 10.0..=100.0);

            let map_size = debug_mapgen.params.map_size;
            vec2_range_group(ui, "Room size", &mut debug_mapgen.params.room_size, 1.0..=map_size.x, 1.0..=map_size.y);
            num_group(ui, "Room padding (cells)", &mut debug_mapgen.params.room_padding, 0..=10);
            num_range_group(ui, "Num rooms", &mut debug_mapgen.params.num_rooms, 1..=50);

            num_range_group(ui, "Corridor length", &mut debug_mapgen.params.corridor_length, 1..=20);
            num_group(ui, "Corridor width", &mut debug_mapgen.params.corridor_width, 2.0..=100.0);

            ui.text_edit_singleline(&mut debug_mapgen.params.seed);

            ui.separator();

            ui.horizontal(|ui| {
                if ui
                    .add_enabled_ui(debug_mapgen.params.is_valid(), |ui| ui.button("Generate"))
                    .inner
                    .clicked()
                {
                    debug_mapgen.output = Some(generate::generate(debug_mapgen.params.clone()));
                }

                if ui.button("Reset player").clicked() {
                    commands.queue(|world: &mut World| {
                        let mut query = world.query_filtered::<(&mut Transform, &mut CameraOffset), With<PlayerShip>>();
                        let (mut pos, mut cam_offset) = query.single_mut(world);
                        *pos = default();
                        *cam_offset = default();
                    });
                }

                ui.checkbox(&mut debug_mapgen.player_cam_outline, "Camera outline");

                let mut time_scale = time.relative_speed();
                if ui.add(DragValue::new(&mut time_scale).speed(0.1).prefix("Time scale: ").range(1.0..=10.0)).changed() {
                    time.set_relative_speed(time_scale);
                }
            });
        });

    let Ok(player) = player.get_single() else {
        return;
    };

    if debug_mapgen.player_cam_outline {
        rect(
            &mut gizmos,
            Rect::from_center_size(player.translation.truncate(), Vec2::new(160., 90.)),
            LinearRgba::RED,
        );
    }

    if let Some(ref output) = debug_mapgen.output {
        gizmos.grid(
            player.translation.round_to(debug_mapgen.params.grid_size) + Vec3::NEG_Z,
            UVec2::splat(20),
            Vec2::splat(output.params.grid_size),
            LinearRgba::new(0.01, 0.0, 0.01, 1.0),
        );

        rect(
            &mut gizmos,
            Rect::from_corners(Vec2::ZERO, output.params.map_size),
            LinearRgba::GREEN,
        );

        for room in &output.rooms {
            rect(&mut gizmos, room.rect, LinearRgba::BLUE);
        }

        for corridor in &output.corridors {
            gizmos.linestrip(
                corridor
                    .midpoints
                    .iter()
                    .enumerate()
                    .map(|(idx, &p)| p.extend(idx as f32)),
                LinearRgba::new(0., 0.02, 0.1, 1.0),
            );

            gizmos.linestrip(
                corridor
                    .left
                    .iter()
                    .enumerate()
                    .map(|(idx, &p)| p.extend(idx as f32)),
                LinearRgba::new(0., 0.5, 1.0, 1.0),
            );

            gizmos.linestrip(
                corridor
                    .right
                    .iter()
                    .enumerate()
                    .map(|(idx, &p)| p.extend(idx as f32)),
                LinearRgba::new(0., 0.5, 1.0, 1.0),
            );
        }
    }
}

fn rect(gizmos: &mut Gizmos, rect: Rect, color: LinearRgba) {
    gizmos.linestrip(
        [
            rect.min.extend(0.),
            rect.min.extend(0.) + Vec3::new(rect.width(), 0., 0.),
            rect.max.extend(0.),
            rect.min.extend(0.) + Vec3::new(0., rect.height(), 0.),
            rect.min.extend(0.),
        ],
        color,
    );
}

fn num_group<N: emath::Numeric>(
    ui: &mut Ui,
    label: impl Into<WidgetText>,
    value: &mut N,
    limits: RangeInclusive<N>,
) {
    ui.group(|ui| {
        ui.label(label);
        ui.add(egui::Slider::new(value, (*limits.start())..=(*limits.end())));
    });
}

fn vec2_group(
    ui: &mut Ui,
    label: impl Into<WidgetText>,
    range: &mut Vec2,
    limits: RangeInclusive<Vec2>,
) {
    ui.group(|ui| {
        ui.label(label);
        ui.add(egui::Slider::new(&mut range.x, limits.start().x..=limits.end().x).text("X"));
        ui.add(egui::Slider::new(&mut range.y, limits.start().y..=limits.end().y).text("Y"));
    });
}

fn vec2_range_group(
    ui: &mut Ui,
    label: impl Into<WidgetText>,
    range: &mut RangeInclusive<Vec2>,
    limits_x: RangeInclusive<f32>,
    limits_y: RangeInclusive<f32>,
) {
    let mut start = *range.start();
    let mut end = *range.end();

    ui.group(|ui| {
        ui.label(label);
        ui.add(egui::Slider::new(&mut start.x, limits_x.clone()).text("Min X"));
        ui.add(egui::Slider::new(&mut end.x, limits_x).text("Max X"));
        ui.add(egui::Slider::new(&mut start.y, limits_y.clone()).text("Min Y"));
        ui.add(egui::Slider::new(&mut end.y, limits_y).text("Max Y"));
    });

    if start != *range.start() || end != *range.end() {
        *range = start..=end;
    }
}

fn num_range_group<N: emath::Numeric>(
    ui: &mut Ui,
    label: impl Into<WidgetText>,
    range: &mut RangeInclusive<N>,
    limits: RangeInclusive<N>,
) {
    let mut start = *range.start();
    let mut end = *range.end();

    ui.group(|ui| {
        ui.label(label);
        ui.add(egui::Slider::new(&mut start, limits.clone()).text("Min"));
        ui.add(egui::Slider::new(&mut end, limits).text("Max"));
    });

    if start != *range.start() || end != *range.end() {
        *range = start..=end;
    }
}
