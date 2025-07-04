use std::ops::Deref;

use glad_gl::gl;
use log::{debug, error};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ShaderUniform {
    Float(f32),
    Int(i32),
    UInt(u32),
    Bool(bool),

    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),

    IVec2([i32; 2]),
    IVec3([i32; 3]),
    IVec4([i32; 4]),

    UVec2([u32; 2]),
    UVec3([u32; 3]),
    UVec4([u32; 4]),

    BVec2([bool; 2]),
    BVec3([bool; 3]),
    BVec4([bool; 4]),

    Mat2([f32; 4]),
    Mat3([f32; 9]),
    Mat4([f32; 16]),

    Texture(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Shader(u32);

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_path = std::path::Path::new(vertex_path);
        let fragment_path = std::path::Path::new(fragment_path);

        if !vertex_path.exists() {
            error!("Vertex shader file does not exist: {vertex_path:?}");
            panic!();
        }

        if !fragment_path.exists() {
            error!("Fragment shader file does not exist: {fragment_path:?}");
            panic!();
        }

        debug!("Loading shaders from files");
        debug!("[VERTEX] {}", vertex_path.display());
        debug!("[FRAGMENT] {}", fragment_path.display());

        let vertex_source = std::fs::read_to_string(vertex_path).unwrap();
        let fragment_source = std::fs::read_to_string(fragment_path).unwrap();

        Self::from_source(&vertex_source, &fragment_source)
    }

    pub fn instance() -> Self {
        Self::new("assets/shader/instance/vert.glsl", "assets/shader/instance/frag.glsl")
    }

    pub fn from_source(vertex_source: &str, fragment_source: &str) -> Self {
        debug!("Compiling vertex shader");
        let vertex_shader = Self::compile_shader(vertex_source, gl::VERTEX_SHADER);
        debug!("Compiling fragment shader");
        let fragment_shader = Self::compile_shader(fragment_source, gl::FRAGMENT_SHADER);
        debug!("Linking program");
        let id = Self::link_program(vertex_shader, fragment_shader);
        Self(id)
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(**self);
        }
    }

    pub fn set_uniform(&self, name: &str, uniform: &ShaderUniform) {
        let name = std::ffi::CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(**self, name.as_ptr()) };
        match uniform {
            ShaderUniform::Float(value) => unsafe { gl::Uniform1f(location, *value) },
            ShaderUniform::Int(value) => unsafe { gl::Uniform1i(location, *value) },
            ShaderUniform::UInt(value) => unsafe { gl::Uniform1ui(location, *value) },
            ShaderUniform::Bool(value) => unsafe { gl::Uniform1i(location, *value as i32) },

            ShaderUniform::Vec2(value) => unsafe { gl::Uniform2fv(location, 1, value.as_ptr()) },
            ShaderUniform::Vec3(value) => unsafe { gl::Uniform3fv(location, 1, value.as_ptr()) },
            ShaderUniform::Vec4(value) => unsafe { gl::Uniform4fv(location, 1, value.as_ptr()) },

            ShaderUniform::IVec2(value) => unsafe { gl::Uniform2iv(location, 1, value.as_ptr()) },
            ShaderUniform::IVec3(value) => unsafe { gl::Uniform3iv(location, 1, value.as_ptr()) },
            ShaderUniform::IVec4(value) => unsafe { gl::Uniform4iv(location, 1, value.as_ptr()) },

            ShaderUniform::UVec2(value) => unsafe { gl::Uniform2uiv(location, 1, value.as_ptr()) },
            ShaderUniform::UVec3(value) => unsafe { gl::Uniform3uiv(location, 1, value.as_ptr()) },
            ShaderUniform::UVec4(value) => unsafe { gl::Uniform4uiv(location, 1, value.as_ptr()) },

            ShaderUniform::BVec2(value) => unsafe { gl::Uniform2iv(location, 1, value.as_ptr() as *const i32) },
            ShaderUniform::BVec3(value) => unsafe { gl::Uniform3iv(location, 1, value.as_ptr() as *const i32) },
            ShaderUniform::BVec4(value) => unsafe { gl::Uniform4iv(location, 1, value.as_ptr() as *const i32) },

            ShaderUniform::Mat2(value) => unsafe { gl::UniformMatrix2fv(location, 1, gl::FALSE, value.as_ptr()) },
            ShaderUniform::Mat3(value) => unsafe { gl::UniformMatrix3fv(location, 1, gl::FALSE, value.as_ptr()) },
            ShaderUniform::Mat4(value) => unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr()) },

            ShaderUniform::Texture(value) => unsafe {
                gl::Uniform1i(location, *value as i32);
            },
        }
    }

    // ----------------< Private >----------------
    fn compile_shader(source: &str, shader_type: u32) -> u32 {
        let source = std::ffi::CString::new(source).unwrap();
        let shader = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut success = 1;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0; len as usize];
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
                error!("Failed to compile shader: {}", std::str::from_utf8(&buffer).unwrap());
            }
        }
        shader
    }

    fn link_program(vertex_shader: u32, fragment_shader: u32) -> u32 {
        let shader_program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = 1;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0; len as usize];
                gl::GetProgramInfoLog(shader_program, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut i8);
                error!("Failed to link shader program: {}", std::str::from_utf8(&buffer).unwrap());
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        shader_program
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(**self);
        }
    }
}

impl Deref for Shader {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Shader {
    fn default() -> Self {
        Self::new("assets/shader/default/vert.glsl", "assets/shader/default/frag.glsl")
    }
}
