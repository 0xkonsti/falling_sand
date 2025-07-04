use glad_gl::gl;

pub mod attribute;
pub mod instance;
pub mod vertex;

use attribute::*;
use vertex::*;

use crate::graphics::mesh::instance::Instance;

const F32_SIZE: u32 = size_of::<f32>() as u32;
const U32_SIZE: u32 = size_of::<u32>() as u32;
const BASE_STRIDE: u32 = POSITION_SIZE * F32_SIZE + COLOR_SIZE * F32_SIZE;

pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,

    vertices:   Vec<Vertex>,
    attributes: Vec<VertexAttribute>,
    indices:    Vec<u32>,
    stride:     u32,

    build: bool,
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_attribute(&mut self, attribute_type: AttributeType, data: Vec<f32>) {
        self.stride += attribute_type.size() * F32_SIZE;

        self.attributes.push(VertexAttribute::new(&attribute_type));

        if self.vertices.is_empty() {
            self.vertices = data
                .chunks(attribute_type.size() as usize)
                .map(|chunk| {
                    let mut vertex = Vertex::new();
                    vertex.add_attribute(&attribute_type, chunk.to_vec());
                    vertex
                })
                .collect();
        } else {
            for (vertex, chunks) in self.vertices.iter_mut().zip(data.chunks(attribute_type.size() as usize)) {
                vertex.add_attribute(&attribute_type, chunks.to_vec());
            }
        }
    }

    pub fn with_attribute(mut self, attribute_type: AttributeType, data: Vec<f32>) -> Self {
        self.add_attribute(attribute_type, data);
        self
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.set_indices(indices);
        self
    }

    pub fn create_instance(self, base_size: usize) -> Instance {
        Instance::new(self, base_size).build()
    }

    pub fn build(mut self) -> Mesh {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        let flattened: Vec<f32> = self.vertices.iter().flat_map(|v| v.flatten()).collect();

        unsafe {
            // Create and bind the VAO
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create and bind the VBO
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (flattened.len() * F32_SIZE as usize) as isize,
                flattened.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Set up the vertex attributes layout
            let stride = if self.stride <= BASE_STRIDE {
                // This makes sure unused default attributes are included in the stride
                // (e.g. position, color, etc.)
                BASE_STRIDE
            } else {
                self.stride
            };

            let mut offset = 0;
            for (i, attr) in self.attributes.iter().enumerate() {
                let index = i as u32;
                let size = attr.size as i32;

                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(
                    index,
                    size,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    (offset * F32_SIZE) as *const _,
                );

                offset += size as u32;
            }

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * U32_SIZE as usize) as isize,
                self.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(0); // Unbind the VAO
        };

        self.vao = vao;
        self.vbo = vbo;
        self.ebo = ebo;

        self.build = true;

        self
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            ebo: 0,

            vertices:   Vec::new(),
            indices:    Vec::new(),
            attributes: Vec::new(),
            stride:     0,

            build: false,
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        if !self.build {
            return; // No need to clean up if the mesh wasn't built
        }
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);

            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
