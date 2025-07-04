use glad_gl::gl;

use crate::graphics::{
    Color, Mesh, Transform, Vertex, VertexAttribute,
    mesh::{BASE_STRIDE, F32_SIZE, U32_SIZE},
};

const INSTANCE_DATA_SIZE: usize = std::mem::size_of::<InstanceData>();
const INSTANCE_ATTR_COUNT: u32 = 5; // 4 for transform (vec4) + 1 for color (vec4)

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct InstanceData {
    pub transform: [f32; 16],
    pub color:     [f32; 4],
}

impl InstanceData {
    pub fn new(transform: Transform, color: &Color) -> Self {
        Self { transform: transform.flatten(), color: color.as_array() }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform.flatten();
    }

    pub fn as_transform(&self) -> Transform {
        Transform::from_flattened(self.transform)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    vao:           u32,
    vbo_mesh:      u32,
    vbo_instances: u32,
    ebo:           u32,

    vertices:   Vec<Vertex>,
    attributes: Vec<VertexAttribute>,
    indices:    Vec<u32>,
    stride:     u32,

    init_size:     usize,
    instance_size: usize,
    instances:     Vec<InstanceData>,

    build:  bool,
    update: bool,
}

impl Instance {
    pub fn new(mut mesh: Mesh, base_size: usize) -> Self {
        Self {
            vao:           0,
            vbo_mesh:      0,
            vbo_instances: 0,
            ebo:           0,

            vertices:   std::mem::take(&mut mesh.vertices),
            attributes: std::mem::take(&mut mesh.attributes),
            indices:    std::mem::take(&mut mesh.indices),
            stride:     mesh.stride,

            init_size:     base_size,
            instance_size: base_size,
            instances:     Vec::with_capacity(base_size),

            build:  false,
            update: false,
        }
    }

    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }

    pub fn add_instance(&mut self, instance: InstanceData) -> usize {
        if self.instances.len() >= self.instance_size {
            let new_capacity = self.instance_size * 2;
            self.resize_instance_buffer(new_capacity);
        }

        self.instances.push(instance);
        self.update = true;

        self.instances.len() - 1
    }

    pub fn remove_instance(&mut self, index: usize) {
        assert!(index < self.instances.len(), "Index out of bounds for instance data.");

        self.instances.remove(index);
        self.update = true;
    }

    pub fn get_instance(&self, index: usize) -> Option<&InstanceData> {
        if index < self.instances.len() { Some(&self.instances[index]) } else { None }
    }

    pub fn update_instance(&mut self, index: usize, instance: InstanceData) {
        assert!(index < self.instances.len(), "Index out of bounds for instance data.");

        self.instances[index].set_transform(instance.as_transform());
        self.update = true;
    }

    pub fn update_instance_transform(&mut self, index: usize, transform: Transform) {
        assert!(index < self.instances.len(), "Index out of bounds for instance data.");

        self.instances[index].set_transform(transform);
        self.update = true;
    }

    pub fn update_instance_color(&mut self, index: usize, color: &Color) {
        assert!(index < self.instances.len(), "Index out of bounds for instance data.");

        self.instances[index].color = color.as_array();
        self.update = true;
    }

    pub fn build(mut self) -> Self {
        let mut vao = 0;
        let mut vbo_mesh = 0;
        let mut vbo_instances = 0;
        let mut ebo = 0;

        let flattened: Vec<f32> = self.vertices.iter().flat_map(|v| v.flatten()).collect();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo_mesh);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_mesh);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (flattened.len() * F32_SIZE as usize) as isize,
                flattened.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = if self.stride <= BASE_STRIDE { BASE_STRIDE } else { self.stride };

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

            gl::GenBuffers(1, &mut vbo_instances);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_instances);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.instance_size * INSTANCE_DATA_SIZE) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            // Set up instance attributes
            let instance_stride = INSTANCE_DATA_SIZE as i32;

            for i in 0..INSTANCE_ATTR_COUNT {
                let attr_idx = self.attributes.len() as u32 + i;
                gl::EnableVertexAttribArray(attr_idx);
                gl::VertexAttribPointer(
                    attr_idx,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    instance_stride,
                    (i * 4 * F32_SIZE) as *const _,
                );
                gl::VertexAttribDivisor(attr_idx, 1); // Enable instancing for this attribute
            }

            gl::BindVertexArray(0); // Unbind the VAO
        }

        self.vao = vao;
        self.vbo_mesh = vbo_mesh;
        self.ebo = ebo;
        self.vbo_instances = vbo_instances;
        self.build = true;

        self
    }

    pub fn draw(&mut self) {
        if !self.build {
            panic!("Instance not built yet. Call `build()` before drawing.");
        }

        self.update_instances();

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElementsInstanced(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
                self.instances.len() as i32,
            );
            gl::BindVertexArray(0);
        }
    }

    // ----------------< Private >----------------
    fn update_instances(&mut self) {
        if !self.update {
            return;
        }

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_instances);

            let instance_count = self.instances.len();
            if instance_count == 0 {
                gl::BindVertexArray(0);
                return;
            }
            let byte_size = instance_count * INSTANCE_DATA_SIZE;

            let ptr = gl::MapBufferRange(
                gl::ARRAY_BUFFER,
                0,
                byte_size as isize,
                gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_BUFFER_BIT,
            );
            if ptr.is_null() {
                panic!("Failed to map buffer for instance data.");
            }
            std::ptr::copy_nonoverlapping(self.instances.as_ptr(), ptr as *mut InstanceData, self.instances.len());
            gl::UnmapBuffer(gl::ARRAY_BUFFER);
        }

        self.update = false;
    }

    fn resize_instance_buffer(&mut self, new_capacity: usize) {
        unsafe {
            // Neuen VBO anlegen
            let mut new_vbo = 0;
            gl::GenBuffers(1, &mut new_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, new_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (new_capacity * INSTANCE_DATA_SIZE) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            // VAO binden, um neue Attribute zu setzen
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, new_vbo);

            let instance_stride = INSTANCE_DATA_SIZE as i32;
            for i in 0..INSTANCE_ATTR_COUNT {
                let attr_idx = self.attributes.len() as u32 + i;
                gl::EnableVertexAttribArray(attr_idx);
                gl::VertexAttribPointer(
                    attr_idx,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    instance_stride,
                    (i * 4 * F32_SIZE) as *const _,
                );
                gl::VertexAttribDivisor(attr_idx, 1);
            }

            gl::BindVertexArray(0);

            // Altes VBO löschen
            gl::DeleteBuffers(1, &self.vbo_instances);

            // Ersetzen
            self.vbo_instances = new_vbo;
            self.instance_size = new_capacity;
        }

        self.update = true; // Markiere zum Neuladen
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if self.build {
            unsafe {
                gl::DeleteVertexArrays(1, &self.vao);
                gl::DeleteBuffers(1, &self.vbo_mesh);
                gl::DeleteBuffers(1, &self.vbo_instances);
                gl::DeleteBuffers(1, &self.ebo);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RingBufferInstance {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StorageBufferInstance {}
