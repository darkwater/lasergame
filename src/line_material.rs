use bevy::{
    asset::RenderAssetUsages,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, PolygonMode, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

#[derive(Reflect, Asset, AsBindGroup, Debug, Clone)]
pub struct LineMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl LineMaterial {
    pub fn new<C: Into<LinearRgba>>(color: C) -> Self {
        Self {
            color: color.into(),
        }
    }
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;

        if dbg!(layout.0.attribute_ids().contains(&Mesh::ATTRIBUTE_COLOR.id)) {
            descriptor
                .fragment
                .as_mut()
                .unwrap()
                .shader_defs
                .push("MESH_COLOR".into());
        }

        Ok(())
    }
}

impl From<LinearRgba> for LineMaterial {
    fn from(color: LinearRgba) -> Self {
        Self::new(color)
    }
}

impl From<Color> for LineMaterial {
    fn from(color: Color) -> Self {
        Self::new(color)
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
    pub color: LinearRgba,
}

impl From<LineList> for Mesh {
    fn from(list: LineList) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD);

        let colors: Vec<_> = list
            .lines
            .iter()
            .flat_map(|(_, _)| [list.color.to_f32_array(), list.color.to_f32_array()])
            .collect();

        let vertices: Vec<_> = list.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let mut mesh = Mesh::new(
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
        mesh
    }
}
