use crate::graphics::{ShaderUniform, camera::Camera2D, shader::Shader, transform::Transform};

pub const TRANSFORM_UNIFORM: &str = "uModel";
pub const PROJECTION_UNIFORM: &str = "uProjection";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Material {
    shader: Shader,
}

impl Material {
    pub fn new(shader: Shader) -> Self {
        Self { shader }
    }

    pub fn apply(&self, uniforms: &[(&'static str, ShaderUniform)]) {
        self.shader.use_program();

        for (name, uniform) in uniforms {
            self.shader.set_uniform(name, uniform);
        }
    }
}
