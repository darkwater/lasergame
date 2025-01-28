use bevy::{
    asset::RenderAssetUsages,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef},
        render_resource::{
            AsBindGroup, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef, ShaderType,
            SpecializedMeshPipelineError, VertexFormat,
        },
    },
};

#[derive(Reflect, ShaderType, Debug, Clone)]
pub struct LineMaterialUniform {
    pub color: LinearRgba,
    pub line_width: f32,
    pub depth_bias: f32,
    pub line_scale: f32,
    pub gap_scale: f32,
}

#[derive(Reflect, Asset, AsBindGroup, Debug, Clone)]
pub struct LineMaterial {
    #[uniform(0)]
    pub uniform: LineMaterialUniform,
}

impl LineMaterial {
    pub const ATTRIBUTE_POSITION_A: MeshVertexAttribute =
        MeshVertexAttribute::new("LineMaterial_PositionA", 986361501, VertexFormat::Float32x3);

    pub const ATTRIBUTE_POSITION_B: MeshVertexAttribute =
        MeshVertexAttribute::new("LineMaterial_PositionB", 986361502, VertexFormat::Float32x3);

    pub const ATTRIBUTE_COLOR_A: MeshVertexAttribute =
        MeshVertexAttribute::new("LineMaterial_ColorA", 986361503, VertexFormat::Float32x4);

    pub const ATTRIBUTE_COLOR_B: MeshVertexAttribute =
        MeshVertexAttribute::new("LineMaterial_ColorB", 986361504, VertexFormat::Float32x4);

    pub fn new<C: Into<LinearRgba>>(color: C) -> Self {
        Self {
            uniform: LineMaterialUniform {
                color: color.into(),
                line_width: 100.,
                depth_bias: 0.,
                line_scale: 1.,
                gap_scale: 1.,
            },
        }
    }
}

impl Material for LineMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // if layout.0.attribute_ids().contains(&Mesh::ATTRIBUTE_COLOR.id) {
        descriptor.vertex.shader_defs.push("PERSPECTIVE".into());
        // }

        let vertex_layout = layout.0.get_layout(&[
            LineMaterial::ATTRIBUTE_POSITION_A.at_shader_location(0),
            LineMaterial::ATTRIBUTE_POSITION_B.at_shader_location(1),
            LineMaterial::ATTRIBUTE_COLOR_A.at_shader_location(2),
            LineMaterial::ATTRIBUTE_COLOR_B.at_shader_location(3),
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];

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
    pub lines: Vec<[Vec3; 2]>,
    pub colors: Vec<[LinearRgba; 2]>,
}

impl From<LineList> for Mesh {
    fn from(list: LineList) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

        assert_eq!(list.lines.len(), list.colors.len());

        let (vertices_a, vertices_b): (Vec<_>, Vec<_>) = list
            .lines
            .into_iter()
            .flat_map(|[a, b]| [(a, b); 6])
            .unzip();

        let (colors_a, colors_b): (Vec<_>, Vec<_>) = list
            .colors
            .into_iter()
            .flat_map(|[a, b]| [(a.to_f32_array(), b.to_f32_array()); 6])
            .unzip();

        assert_eq!(vertices_a.len(), colors_a.len());

        mesh.insert_attribute(LineMaterial::ATTRIBUTE_POSITION_A, vertices_a);
        mesh.insert_attribute(LineMaterial::ATTRIBUTE_POSITION_B, vertices_b);
        mesh.insert_attribute(LineMaterial::ATTRIBUTE_COLOR_A, colors_a);
        mesh.insert_attribute(LineMaterial::ATTRIBUTE_COLOR_B, colors_b);
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
        LineList::from(line).into()
    }
}

impl From<LineStrip> for LineList {
    fn from(strip: LineStrip) -> Self {
        let lines = strip.points.array_windows().copied().collect();
        let colors = vec![[LinearRgba::WHITE; 2]; strip.points.len() - 1];

        LineList { lines, colors }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_strip_into_list() {
        let strip = LineStrip {
            points: vec![Vec3::ZERO, Vec3::X, Vec3::Y],
        };

        let list = LineList::from(strip);

        assert_eq!(list.lines, vec![[Vec3::ZERO, Vec3::X], [Vec3::X, Vec3::Y]]);
        assert_eq!(list.colors, vec![[LinearRgba::WHITE; 2]; 2]);
    }
}
