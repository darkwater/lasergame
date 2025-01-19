use bevy::{
    asset::{AssetLoader, RenderAssetUsages},
    prelude::*,
    render::render_resource::PrimitiveTopology,
    utils::ConditionalSendFuture,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, TypePath)]
pub struct Model {
    pub lines: Vec<(Vec3, Vec3)>,
    pub colors: ColorSpec,
}

#[derive(Serialize, Deserialize)]
pub enum ColorSpec {
    Uniform(LinearRgba),
    PerLine(Vec<LinearRgba>),
    PerVertex(Vec<(LinearRgba, LinearRgba)>),
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
            let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);

            let colors: Vec<_> = match model.colors {
                ColorSpec::PerVertex(colors) => colors
                    .iter()
                    .flat_map(|(a, b)| [a.to_f32_array(), b.to_f32_array()])
                    .collect(),
                ColorSpec::PerLine(colors) => colors
                    .into_iter()
                    .flat_map(|c| [c.to_f32_array(), c.to_f32_array()])
                    .collect(),
                ColorSpec::Uniform(color) => vec![color.to_f32_array(); model.lines.len() * 2],
            };

            let vertices: Vec<_> = model.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

            Ok(mesh)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["mdl.ron"]
    }
}
