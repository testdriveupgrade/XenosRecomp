//! Rust equivalents of shader parsing structures from the C++ codebase.
//!
//! This module focuses on decoding big-endian binary structures from Xbox 360
//! shader blobs. It is intentionally dependency-free for easy integration.

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

impl ShaderContainer {
    pub const SIZE: usize = 9 * 4;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            flags: read_u32_be(data, 0)?,
            virtual_size: read_u32_be(data, 4)?,
            physical_size: read_u32_be(data, 8)?,
            field_c: read_u32_be(data, 12)?,
            constant_table_offset: read_u32_be(data, 16)?,
            definition_table_offset: read_u32_be(data, 20)?,
            shader_offset: read_u32_be(data, 24)?,
            field_1c: read_u32_be(data, 28)?,
            field_20: read_u32_be(data, 32)?,
        })
    }
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

impl Shader {
    pub const SIZE: usize = 6 * 4;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            physical_offset: read_u32_be(data, 0)?,
            size: read_u32_be(data, 4)?,
            field_8: read_u32_be(data, 8)?,
            field_c: read_u32_be(data, 12)?,
            field_10: read_u32_be(data, 16)?,
            interpolator_info: read_u32_be(data, 20)?,
        })
    }

    pub fn interpolator_count(&self) -> u32 {
        (self.interpolator_info >> 5) & 0x1f
    }

    pub fn sv_position_register(&self) -> u32 {
        (self.field_c >> 8) & 0xff
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VertexShaderHeader {
    pub base: Shader,
    pub field_18: u32,
    pub vertex_element_count: u32,
    pub field_20: u32,
}

impl VertexShaderHeader {
    pub const SIZE: usize = Shader::SIZE + 3 * 4;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        let base = Shader::parse(data)?;
        Some(Self {
            base,
            field_18: read_u32_be(data, 24)?,
            vertex_element_count: read_u32_be(data, 28)?,
            field_20: read_u32_be(data, 32)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelShaderHeader {
    pub base: Shader,
    pub field_18: u32,
    pub outputs: u32,
}

impl PixelShaderHeader {
    pub const SIZE: usize = Shader::SIZE + 2 * 4;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        let base = Shader::parse(data)?;
        Some(Self {
            base,
            field_18: read_u32_be(data, 24)?,
            outputs: read_u32_be(data, 28)?,
        })
    }
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
    Unknown = 0xffff,
}

impl DeclUsage {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::BlendWeight,
            2 => Self::BlendIndices,
            3 => Self::Normal,
            4 => Self::PointSize,
            5 => Self::TexCoord,
            6 => Self::Tangent,
            7 => Self::Binormal,
            8 => Self::TessFactor,
            9 => Self::PositionT,
            10 => Self::Color,
            11 => Self::Fog,
            12 => Self::Depth,
            13 => Self::Sample,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VertexElement {
    pub address: u16,
    pub usage: DeclUsage,
    pub usage_index: u8,
}

impl VertexElement {
    pub fn from_raw(value: u32) -> Self {
        let address = (value & 0x0fff) as u16;
        let usage = DeclUsage::from_u32((value >> 12) & 0x0f);
        let usage_index = ((value >> 16) & 0x0f) as u8;
        Self {
            address,
            usage,
            usage_index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interpolator {
    pub usage_index: u8,
    pub usage: DeclUsage,
    pub reg: u8,
}

impl Interpolator {
    pub fn from_raw(value: u32) -> Self {
        let usage_index = (value & 0x0f) as u8;
        let usage = DeclUsage::from_u32((value >> 4) & 0x0f);
        let reg = ((value >> 8) & 0x0f) as u8;
        Self {
            usage_index,
            usage,
            reg,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantTableContainer {
    pub size: u32,
    pub table: ConstantTable,
}

impl ConstantTableContainer {
    pub const SIZE: usize = 4 + ConstantTable::SIZE;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            size: read_u32_be(data, 0)?,
            table: ConstantTable::parse(&data[4..])?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantTable {
    pub size: u32,
    pub creator: u32,
    pub version: u32,
    pub constants: u32,
    pub constant_info: u32,
    pub flags: u32,
    pub target: u32,
}

impl ConstantTable {
    pub const SIZE: usize = 7 * 4;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            size: read_u32_be(data, 0)?,
            creator: read_u32_be(data, 4)?,
            version: read_u32_be(data, 8)?,
            constants: read_u32_be(data, 12)?,
            constant_info: read_u32_be(data, 16)?,
            flags: read_u32_be(data, 20)?,
            target: read_u32_be(data, 24)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantInfo {
    pub name: u32,
    pub register_set: RegisterSet,
    pub register_index: u16,
    pub register_count: u16,
    pub reserved: u16,
    pub type_info: u32,
    pub default_value: u32,
}

impl ConstantInfo {
    pub const SIZE: usize = 4 + 2 + 2 + 2 + 2 + 4 + 4;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            name: read_u32_be(data, 0)?,
            register_set: RegisterSet::from_u16(read_u16_be(data, 4)?),
            register_index: read_u16_be(data, 6)?,
            register_count: read_u16_be(data, 8)?,
            reserved: read_u16_be(data, 10)?,
            type_info: read_u32_be(data, 12)?,
            default_value: read_u32_be(data, 16)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeInfo {
    pub parameter_class: ParameterClass,
    pub parameter_type: ParameterType,
    pub rows: u16,
    pub columns: u16,
    pub elements: u16,
    pub struct_members: u16,
    pub struct_member_info: u32,
}

impl TypeInfo {
    pub const SIZE: usize = 2 + 2 + 2 + 2 + 2 + 2 + 4;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            parameter_class: ParameterClass::from_u16(read_u16_be(data, 0)?),
            parameter_type: ParameterType::from_u16(read_u16_be(data, 2)?),
            rows: read_u16_be(data, 4)?,
            columns: read_u16_be(data, 6)?,
            elements: read_u16_be(data, 8)?,
            struct_members: read_u16_be(data, 10)?,
            struct_member_info: read_u32_be(data, 12)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructMemberInfo {
    pub name: u32,
    pub type_info: u32,
}

impl StructMemberInfo {
    pub const SIZE: usize = 8;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            name: read_u32_be(data, 0)?,
            type_info: read_u32_be(data, 4)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterClass {
    Scalar = 0,
    Vector = 1,
    MatrixRows = 2,
    MatrixColumns = 3,
    Object = 4,
    Struct = 5,
    Unknown = 0xffff,
}

impl ParameterClass {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => Self::Scalar,
            1 => Self::Vector,
            2 => Self::MatrixRows,
            3 => Self::MatrixColumns,
            4 => Self::Object,
            5 => Self::Struct,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
    Void = 0,
    Bool = 1,
    Int = 2,
    Float = 3,
    String = 4,
    Texture = 5,
    Texture1D = 6,
    Texture2D = 7,
    Texture3D = 8,
    TextureCube = 9,
    Sampler = 10,
    Sampler1D = 11,
    Sampler2D = 12,
    Sampler3D = 13,
    SamplerCube = 14,
    PixelShader = 15,
    VertexShader = 16,
    PixelFragment = 17,
    VertexFragment = 18,
    Unsupported = 19,
    Unknown = 0xffff,
}

impl ParameterType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => Self::Void,
            1 => Self::Bool,
            2 => Self::Int,
            3 => Self::Float,
            4 => Self::String,
            5 => Self::Texture,
            6 => Self::Texture1D,
            7 => Self::Texture2D,
            8 => Self::Texture3D,
            9 => Self::TextureCube,
            10 => Self::Sampler,
            11 => Self::Sampler1D,
            12 => Self::Sampler2D,
            13 => Self::Sampler3D,
            14 => Self::SamplerCube,
            15 => Self::PixelShader,
            16 => Self::VertexShader,
            17 => Self::PixelFragment,
            18 => Self::VertexFragment,
            19 => Self::Unsupported,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterSet {
    Bool = 0,
    Int4 = 1,
    Float4 = 2,
    Sampler = 3,
    Unknown = 0xffff,
}

impl RegisterSet {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => Self::Bool,
            1 => Self::Int4,
            2 => Self::Float4,
            3 => Self::Sampler,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionTable {
    pub field_0: u32,
    pub field_4: u32,
    pub field_8: u32,
    pub field_c: u32,
    pub size: u32,
}

impl DefinitionTable {
    pub const SIZE: usize = 5 * 4;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            field_0: read_u32_be(data, 0)?,
            field_4: read_u32_be(data, 4)?,
            field_8: read_u32_be(data, 8)?,
            field_c: read_u32_be(data, 12)?,
            size: read_u32_be(data, 16)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Float4Definition {
    pub register_index: u16,
    pub count: u16,
    pub physical_offset: u32,
}

impl Float4Definition {
    pub const SIZE: usize = 8;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            register_index: read_u16_be(data, 0)?,
            count: read_u16_be(data, 2)?,
            physical_offset: read_u32_be(data, 4)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Int4DefinitionHeader {
    pub register_index: u16,
    pub count: u16,
}

impl Int4DefinitionHeader {
    pub const SIZE: usize = 4;

    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < Self::SIZE {
            return None;
        }
        Some(Self {
            register_index: read_u16_be(data, 0)?,
            count: read_u16_be(data, 2)?,
        })
    }
}

fn read_u16_be(data: &[u8], offset: usize) -> Option<u16> {
    let bytes = data.get(offset..offset + 2)?;
    Some(u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn read_u32_be(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset + 4)?;
    Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}
