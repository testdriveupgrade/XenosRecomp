//! Rust equivalents of the shader container data structures from `shader.h`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Float4Definition {
    pub register_index: u16,
    pub count: u16,
    pub physical_offset: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Int4DefinitionHeader {
    pub register_index: u16,
    pub count: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionTable {
    pub field_0: u32,
    pub field_4: u32,
    pub field_8: u32,
    pub field_c: u32,
    pub size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shader {
    pub physical_offset: u32,
    pub size: u32,
    pub field_8: u32,
    pub field_c: u32,
    pub field_10: u32,
    pub interpolator_info: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclUsage {
    Position = 0,
    BlendWeight = 1,
    BlendIndices = 2,
    Normal = 3,
    PointSize = 4,
    TexCoord = 5,
    Tangent = 6,
    Binormal = 7,
    TessFactor = 8,
    PositionT = 9,
    Color = 10,
    Fog = 11,
    Depth = 12,
    Sample = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VertexElement {
    pub address: u16,
    pub usage: DeclUsage,
    pub usage_index: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interpolator {
    pub usage_index: u8,
    pub usage: DeclUsage,
    pub reg: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VertexShader {
    pub base: Shader,
    pub field_18: u32,
    pub vertex_element_count: u32,
    pub field_20: u32,
    pub vertex_elements_and_interpolators: u32,
}

pub const PIXEL_SHADER_OUTPUT_COLOR0: u32 = 0x1;
pub const PIXEL_SHADER_OUTPUT_COLOR1: u32 = 0x2;
pub const PIXEL_SHADER_OUTPUT_COLOR2: u32 = 0x4;
pub const PIXEL_SHADER_OUTPUT_COLOR3: u32 = 0x8;
pub const PIXEL_SHADER_OUTPUT_DEPTH: u32 = 0x10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelShader {
    pub base: Shader,
    pub field_18: u32,
    pub outputs: u32,
    pub interpolators: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShaderContainer {
    pub flags: u32,
    pub virtual_size: u32,
    pub physical_size: u32,
    pub field_c: u32,
    pub constant_table_offset: u32,
    pub definition_table_offset: u32,
    pub shader_offset: u32,
    pub field_1c: u32,
    pub field_20: u32,
}
