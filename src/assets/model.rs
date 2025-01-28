use bevy::{asset::AssetLoader, prelude::*, utils::ConditionalSendFuture};
use serde::{Deserialize, Serialize};

use crate::line_material::LineList;

#[derive(Serialize, Deserialize, TypePath)]
pub struct Model {
    pub lines: Vec<Line>,
    pub default_color: Option<Color>,
}

#[derive(Serialize, Deserialize, TypePath)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
    /// If end_color is not set, color will be used for the whole line. Else it's a gradient
    pub color: Option<Color>,
    pub end_color: Option<Color>,
}

impl From<Model> for LineList {
    fn from(value: Model) -> Self {
        let (lines, colors) = value
            .lines
            .into_iter()
            .map(|line| {
                ([line.start, line.end], [
                    line.color
                        .or(value.default_color)
                        .unwrap_or_default()
                        .to_linear(),
                    line.end_color
                        .or(line.color)
                        .or(value.default_color)
                        .unwrap_or_default()
                        .to_linear(),
                ])
            })
            .unzip();

        Self { lines, colors }
    }
}

pub struct ModelLoader;
impl AssetLoader for ModelLoader {
    type Asset = Mesh;
    type Settings = ();
    type Error = std::io::Error;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = vec![];
            reader.read_to_end(&mut bytes).await?;

            let model: Model = serde_json::from_slice(&bytes)?;

            Ok(LineList::from(model).into())
        }
    }

    fn extensions(&self) -> &[&str] {
        &["mdl.json"]
    }
}
