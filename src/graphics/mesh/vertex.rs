use log::error;

use crate::graphics::mesh::attribute::{AttributeType, COLOR_SIZE, DEFAULT_COLOR, DEFAULT_POSITION, POSITION_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vertex {
    position: [f32; POSITION_SIZE as usize],
    color:    [f32; COLOR_SIZE as usize],
}

impl Vertex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_attribute(&mut self, attribute_type: &AttributeType, data: Vec<f32>) {
        match attribute_type {
            AttributeType::Position => {
                self.position = data.as_slice().try_into().unwrap_or_else(|_| {
                    error!("Invalid position data length: expected {} but got {}", POSITION_SIZE, data.len());
                    std::process::exit(1);
                });
            }
            AttributeType::Color => {
                self.color = data.as_slice().try_into().unwrap_or_else(|_| {
                    error!("Invalid color data length: expected {} but got {}", COLOR_SIZE, data.len());
                    std::process::exit(1);
                });
            }
        }
    }

    pub fn flatten(&self) -> Vec<f32> {
        let mut flat = Vec::new();
        flat.extend_from_slice(&self.position);
        flat.extend_from_slice(&self.color);

        flat
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self { position: DEFAULT_POSITION, color: DEFAULT_COLOR }
    }
}
