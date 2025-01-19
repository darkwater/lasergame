use bevy::prelude::*;

use self::model::ModelLoader;

pub mod model;

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(ModelLoader);
    }
}
