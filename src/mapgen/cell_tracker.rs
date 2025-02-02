use bevy::prelude::*;

use super::Cell;

#[derive(Component, Default, Debug, PartialEq, Eq)]
pub struct CellTracker(pub Cell);

pub fn update(
    mut cells: Query<
        (&mut CellTracker, &GlobalTransform),
        Or<(Added<CellTracker>, Changed<GlobalTransform>)>,
    >,
) {
    for (mut cell, transform) in cells.iter_mut() {
        let translation = transform.translation();

        let new = CellTracker(Cell {
            x: (translation.x / Cell::SIZE).round() as i32,
            y: (translation.y / Cell::SIZE).round() as i32,
        });

        if new != *cell {
            trace!("cell changed");
            *cell = new;
        }
    }
}
