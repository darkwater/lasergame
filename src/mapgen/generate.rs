#![allow(unused_labels)] // they help describe loops too

use std::{mem::swap, ops::RangeInclusive};

use bevy::{
    math::{Rect, Vec2},
    prelude::*,
};
use rand::{seq::IndexedRandom as _, Rng as _, SeedableRng as _};
use rand_pcg::Pcg64Mcg;
use rand_seeder::Seeder;

use crate::utils::RoundTo as _;

pub type MapgenRng = Pcg64Mcg;

#[derive(Reflect, Clone)]
pub struct MapgenParams {
    pub map_size: Vec2,
    pub grid_size: f32,
    pub room_size: RangeInclusive<Vec2>,
    pub room_padding: usize,
    pub num_rooms: RangeInclusive<usize>,
    pub corridor_length: RangeInclusive<usize>,
    pub corridor_width: f32,
    pub seed: String,
}

impl MapgenParams {
    pub fn is_valid(&self) -> bool {
        self.room_size.end().x <= self.map_size.x
            && self.room_size.end().y <= self.map_size.y
            && self.room_size.start().x <= self.room_size.end().x
            && self.room_size.start().y <= self.room_size.end().y
            && self.num_rooms.start() <= self.num_rooms.end()
    }
}

impl Default for MapgenParams {
    fn default() -> Self {
        Self {
            map_size: Vec2::new(700., 700.),
            grid_size: 50.,
            room_size: Vec2::new(100., 100.)..=Vec2::new(250., 250.),
            room_padding: 2,
            num_rooms: 10..=20,
            corridor_length: 5..=10,
            corridor_width: 40.0,
            seed: "random".to_string(),
        }
    }
}

pub struct MapgenContext {
    pub params: MapgenParams,
    pub rng: MapgenRng,
}

#[derive(Reflect, Clone)]
pub struct DetailedMapgenOutput {
    pub params: MapgenParams,
    pub rooms: Vec<MapgenRoom>,
    pub corridors: Vec<MapgenCorridor>,
}

#[derive(Reflect, Clone, Debug)]
pub struct MapgenRoom {
    pub id: usize,
    pub attempt: usize,
    pub rect: Rect,
}

#[derive(Reflect, Clone)]
pub struct MapgenCorridor {
    pub from: usize,
    pub to: usize,
    pub entrance: Vec2,
    // pub exit: [Vec2; 2],
    pub midpoints: Vec<Vec2>,
    pub left: Vec<Vec2>,
    pub right: Vec<Vec2>,
}

pub fn generate_rooms(context: &mut MapgenContext) -> Vec<MapgenRoom> {
    let mut rooms = Vec::<MapgenRoom>::new();
    let num_rooms = context.rng.random_range(context.params.num_rooms.clone());

    for id in 0..num_rooms {
        for attempt in 0..50 {
            let room_size = Vec2::new(
                context.rng.random_range(
                    context.params.room_size.start().x..=context.params.room_size.end().x,
                ),
                context.rng.random_range(
                    context.params.room_size.start().y..=context.params.room_size.end().y,
                ),
            )
            .round_to(context.params.grid_size);

            let room_position = Vec2::new(
                context
                    .rng
                    .random_range(0.0..=context.params.map_size.x - room_size.x),
                context
                    .rng
                    .random_range(0.0..=context.params.map_size.y - room_size.y),
            )
            .round_to(context.params.grid_size);

            if rooms.iter().all(|r| {
                r.rect
                    .intersect(
                        Rect::from_corners(room_position, room_position + room_size)
                            .inflate(context.params.room_padding as f32 * context.params.grid_size),
                    )
                    .is_empty()
            }) {
                rooms.push(MapgenRoom {
                    rect: Rect::from_corners(room_position, room_position + room_size),
                    id,
                    attempt,
                });
                break;
            }
        }
    }

    rooms
}

pub fn generate_corridors(
    context: &mut MapgenContext,
    rooms: &[MapgenRoom],
) -> Vec<MapgenCorridor> {
    let mut corridors = Vec::<MapgenCorridor>::new();

    'paths: for _ in 0..1000 {
        // the room to start from
        let room = rooms.choose(&mut context.rng).unwrap();

        // the direction to start in
        let mut dir = *[Vec2::X, Vec2::Y, Vec2::NEG_X, Vec2::NEG_Y]
            .choose(&mut context.rng)
            .unwrap();

        // the start of the corridor
        let entrance = Vec2 {
            x: context.rng.random_range(0.0..=room.rect.width()) + room.rect.min.x,
            y: context.rng.random_range(0.0..=room.rect.height()) + room.rect.min.y,
        };

        // lock start to the grid (onto the center of a cell)
        let mut entrance =
            entrance.round_to_offset(context.params.grid_size, context.params.grid_size / 2.);

        // move the entrance to the edge of the room in the right direction
        match dir {
            Vec2::X => entrance.x = room.rect.min.x + room.rect.width(),
            Vec2::NEG_X => entrance.x = room.rect.min.x,
            Vec2::Y => entrance.y = room.rect.min.y + room.rect.height(),
            Vec2::NEG_Y => entrance.y = room.rect.min.y,
            _ => {}
        }

        // the other direction this corridor can go in
        // we'll randomly alternate between dir and other_dir, so there will be no U-turns
        let mut other_dir = match context.rng.random() {
            true => dir.rotate(Vec2::Y),      // turn left
            false => dir.rotate(Vec2::NEG_Y), // turn right
        };

        // the current working/crawling position
        let mut pos = entrance + dir * (context.params.grid_size / 2.);

        // list of midpoints we'll end up storing
        let mut midpoints = vec![entrance, pos];

        // offsets to get the wall poisitions when using dir
        let mut left_offset = dir.rotate(Vec2::Y) * context.params.corridor_width / 2.;
        let mut right_offset = dir.rotate(Vec2::NEG_Y) * context.params.corridor_width / 2.;

        // offsets to get the wall poisitions when using other_dir
        let mut other_left_offset = other_dir.rotate(Vec2::Y) * context.params.corridor_width / 2.;
        let mut other_right_offset =
            other_dir.rotate(Vec2::NEG_Y) * context.params.corridor_width / 2.;

        // list of wall positions we'll end up storing
        let mut left = vec![entrance + left_offset, pos + left_offset];
        let mut right = vec![entrance + right_offset, pos + right_offset];

        // let's crawl
        'steps: for step in 0..*context.params.corridor_length.end() {
            // randomly decide to turn
            let turning = context.rng.random_ratio(2, 10);

            // swap the dirs and wall offsets
            if turning {
                swap(&mut dir, &mut other_dir);
                swap(&mut left_offset, &mut other_left_offset);
                swap(&mut right_offset, &mut other_right_offset);

                // correct the walls for the corner
                *left.last_mut().unwrap() += left_offset;
                *right.last_mut().unwrap() += right_offset;
            }

            // drop this path if it starts to intersect
            if corridors.iter().any(|c| {
                c.midpoints
                    .iter()
                    .any(|p| p.distance_squared(pos) <= (context.params.grid_size * 1.1).powi(2))
            }) {
                continue 'paths;
            }

            // move along dir
            pos += dir * context.params.grid_size;

            // check if we've hit a room, then end
            if rooms.iter().any(|r| r.rect.contains(pos)) {
                if step < *context.params.corridor_length.start() {
                    // too short
                    continue 'paths;
                }

                midpoints.push((pos + midpoints.last().unwrap()) / 2.);
                left.push(midpoints.last().unwrap() + left_offset);
                right.push(midpoints.last().unwrap() + right_offset);

                corridors.push(MapgenCorridor {
                    from: room.id,
                    to: rooms.iter().find(|r| r.rect.contains(pos)).unwrap().id,
                    entrance,
                    midpoints,
                    left,
                    right,
                });

                continue 'paths;
            } else {
                // store this position
                midpoints.push(pos);
                left.push(pos + left_offset);
                right.push(pos + right_offset);
            }
        }

        // we've stepped too much and didn't hit a room, drop this path
    }

    // deduplicate corridors; we don't want two corridors between the same rooms
    corridors.sort_by_key(|c| {
        let mut key = [c.from, c.to];
        key.sort_unstable();
        key
    });
    corridors.dedup_by(|a, b| {
        let mut a = [a.from, a.to];
        let mut b = [b.from, b.to];
        a.sort_unstable();
        b.sort_unstable();
        a[0] == b[0] && a[1] == b[1]
    });

    corridors
}

pub fn generate(params: MapgenParams) -> DetailedMapgenOutput {
    let rng = if params.seed == "random" {
        MapgenRng::from_os_rng()
    } else {
        Seeder::from(&params.seed).into_rng()
    };

    let mut context = MapgenContext { rng, params };

    for _ in 0..100 {
        let rooms = generate_rooms(&mut context);

        // if rooms.len() < *context.params.num_rooms.start() {
        //     continue;
        // }

        let corridors = generate_corridors(&mut context, &rooms);

        if !all_rooms_connected(&rooms, &corridors) {
            continue;
        }

        return DetailedMapgenOutput {
            params: context.params,
            rooms,
            corridors,
        };
    }

    panic!("failed to generate a map");
}

fn all_rooms_connected(rooms: &[MapgenRoom], corridors: &[MapgenCorridor]) -> bool {
    let mut rooms = rooms.to_vec();
    let mut corridors = corridors.to_vec();

    let mut search_rooms = vec![rooms.pop().unwrap()];

    loop {
        let connected_corridors = corridors.extract_if(.., |c| {
            search_rooms
                .iter()
                .any(|sr| c.from == sr.id || c.to == sr.id)
        });

        let connected_room_ids = connected_corridors
            .filter_map(|c| {
                search_rooms
                    .iter()
                    .filter_map(|sr| {
                        if c.from == sr.id {
                            Some(c.to)
                        } else if c.to == sr.id {
                            Some(c.from)
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .collect::<Vec<_>>();

        search_rooms = rooms
            .extract_if(.., |r| connected_room_ids.contains(&r.id))
            .collect();

        if search_rooms.is_empty() {
            break;
        }
    }

    dbg!(rooms).is_empty()
}
