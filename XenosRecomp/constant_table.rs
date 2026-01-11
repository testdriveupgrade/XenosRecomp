#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Be<T: BeType>(pub T::Repr);

pub trait BeType: Copy {
    type Repr: Copy;

    fn from_be(value: Self::Repr) -> Self;
    fn to_be(self) -> Self::Repr;
}

impl BeType for u16 {
    type Repr = u16;

    fn from_be(value: Self::Repr) -> Self {
        u16::from_be(value)
    }

    fn to_be(self) -> Self::Repr {
        self.to_be()
    }
}

impl BeType for u32 {
    type Repr = u32;

    fn from_be(value: Self::Repr) -> Self {
        u32::from_be(value)
    }

    fn to_be(self) -> Self::Repr {
        self.to_be()
    }
}

impl<T: BeType> Be<T> {
    pub fn get(self) -> T {
        T::from_be(self.0)
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterClass {
    Scalar,
    Vector,
    MatrixRows,
    MatrixColumns,
    Object,
    Struct,
}

impl BeType for ParameterClass {
    type Repr = u16;

    fn from_be(value: Self::Repr) -> Self {
        unsafe { std::mem::transmute(u16::from_be(value)) }
    }

    fn to_be(self) -> Self::Repr {
        (self as u16).to_be()
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParameterType {
    Void,
    Bool,
    Int,
    Float,
    String,
    Texture,
    Texture1D,
    Texture2D,
    Texture3D,
    TextureCube,
    Sampler,
    Sampler1D,
    Sampler2D,
    Sampler3D,
    SamplerCube,
    PixelShader,
    VertexShader,
    PixelFragment,
    VertexFragment,
    Unsupported,
}

impl BeType for ParameterType {
    type Repr = u16;

    fn from_be(value: Self::Repr) -> Self {
        unsafe { std::mem::transmute(u16::from_be(value)) }
    }

    fn to_be(self) -> Self::Repr {
        (self as u16).to_be()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructMemberInfo {
    pub name: Be<u32>,
    pub type_info: Be<u32>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeInfo {
    pub parameter_class: Be<ParameterClass>,
    pub parameter_type: Be<ParameterType>,
    pub rows: Be<u16>,
    pub columns: Be<u16>,
    pub elements: Be<u16>,
    pub struct_members: Be<u16>,
    pub struct_member_info: Be<u32>,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegisterSet {
    Bool,
    Int4,
    Float4,
    Sampler,
}

impl BeType for RegisterSet {
    type Repr = u16;

    fn from_be(value: Self::Repr) -> Self {
        unsafe { std::mem::transmute(u16::from_be(value)) }
    }

    fn to_be(self) -> Self::Repr {
        (self as u16).to_be()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstantInfo {
    pub name: Be<u32>,
    pub register_set: Be<RegisterSet>,
    pub register_index: Be<u16>,
    pub register_count: Be<u16>,
    pub reserved: Be<u16>,
    pub type_info: Be<u32>,
    pub default_value: Be<u32>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstantTable {
    pub size: Be<u32>,
    pub creator: Be<u32>,
    pub version: Be<u32>,
    pub constants: Be<u32>,
    pub constant_info: Be<u32>,
    pub flags: Be<u32>,
    pub target: Be<u32>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstantTableContainer {
    pub size: Be<u32>,
    pub constant_table: ConstantTable,
}
