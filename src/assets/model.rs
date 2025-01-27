use bevy::{asset::AssetLoader, prelude::*, utils::ConditionalSendFuture};
use serde::{Deserialize, Serialize};

use crate::line_material::LineList;

#[derive(Serialize, Deserialize, TypePath)]
pub struct Model {
    pub lines: Vec<[Vec3; 2]>,
    pub colors: ColorSpec,
}

#[derive(Serialize, Deserialize)]
pub enum ColorSpec {
    Uniform(LinearRgba),
    PerLine(Vec<LinearRgba>),
    PerVertex(Vec<[LinearRgba; 2]>),
}

impl From<Model> for LineList {
    fn from(value: Model) -> Self {
        LineList {
            colors: match value.colors {
                ColorSpec::PerVertex(colors) => colors,
                ColorSpec::PerLine(colors) => colors.into_iter().map(|c| [c, c]).collect(),
                ColorSpec::Uniform(color) => vec![[color; 2]; value.lines.len()],
            },
            lines: value.lines,
        }
    }
}

pub struct ModelLoader;
impl AssetLoader for ModelLoader {
    type Asset = Mesh;
    type Settings = ();
    type Error = ron::Error;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = vec![];
            reader.read_to_end(&mut bytes).await?;

            let model: Model = ron::de::from_bytes(&bytes)?;

            Ok(LineList::from(model).into())
        }
    }

    fn extensions(&self) -> &[&str] {
        &["mdl.ron"]
    }
}
