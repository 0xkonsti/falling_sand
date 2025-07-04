mod camera;
mod color;
mod material;
mod mesh;
mod shader;
mod transform;
mod window;

pub use camera::Camera2D;
pub use color::Color;
pub use material::{Material, PROJECTION_UNIFORM, TRANSFORM_UNIFORM};
pub use mesh::{
    Mesh,
    attribute::{AttributeType, VertexAttribute, positions_from_vec3s},
    instance::{Instance, InstanceData},
    vertex::Vertex,
};
pub use shader::{Shader, ShaderUniform};
pub use transform::Transform;
pub use window::{Samples, Window, WindowConfig, WindowMode};
