use glam::Vec3;

pub const POSITION_SIZE: u32 = 3;
pub const COLOR_SIZE: u32 = 4;

pub const DEFAULT_POSITION: [f32; POSITION_SIZE as usize] = [0.0; POSITION_SIZE as usize];
pub const DEFAULT_COLOR: [f32; COLOR_SIZE as usize] = [0.0; COLOR_SIZE as usize];

pub fn positions_from_vec3s<T: IntoIterator<Item = Vec3>>(positions: T) -> Vec<f32> {
    positions.into_iter().flat_map(|pos| pos.to_array().to_vec()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttributeType {
    Position,
    Color,
}

impl AttributeType {
    pub fn name(&self) -> &str {
        match self {
            AttributeType::Position => "position",
            AttributeType::Color => "color",
        }
    }

    pub const fn size(&self) -> u32 {
        match self {
            AttributeType::Position => POSITION_SIZE,
            AttributeType::Color => COLOR_SIZE,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VertexAttribute {
    pub name: String,
    pub size: u32,

    __type: AttributeType,
}

impl VertexAttribute {
    pub fn new(attribute_type: &AttributeType) -> Self {
        Self {
            name:   attribute_type.name().to_string(),
            size:   attribute_type.size(),
            __type: attribute_type.clone(),
        }
    }
}
