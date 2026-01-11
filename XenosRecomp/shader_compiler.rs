//! Rust translation of the shader recompiler interface from `shader_recompiler.*`.

use std::collections::HashMap;
use std::fmt::Write;

use crate::shader::VertexElement;

pub struct StringBuffer {
    pub out: String,
}

impl StringBuffer {
    pub fn new() -> Self {
        Self { out: String::new() }
    }

    pub fn print(&mut self, args: std::fmt::Arguments<'_>) {
        let _ = self.out.write_fmt(args);
    }

    pub fn println(&mut self, args: std::fmt::Arguments<'_>) {
        let _ = self.out.write_fmt(args);
        self.out.push('\n');
    }
}

#[derive(Default)]
pub struct ShaderCompiler {
    pub indentation: u32,
    pub is_pixel_shader: bool,
    pub constant_table_data: Option<Vec<u8>>,
    pub vertex_elements: HashMap<u32, VertexElement>,
    pub interpolators: HashMap<u32, String>,
    pub float4_constants: HashMap<u32, String>,
    pub bool_constants: HashMap<u32, String>,
    pub samplers: HashMap<u32, String>,
    pub if_end_labels: HashMap<u32, u32>,
    pub spec_constants_mask: u32,
    pub buffer: StringBuffer,
}

impl ShaderCompiler {
    pub fn new() -> Self {
        Self {
            buffer: StringBuffer::new(),
            ..Default::default()
        }
    }

    pub fn indent(&mut self) {
        for _ in 0..self.indentation {
            self.buffer.out.push('\t');
        }
    }

    pub fn print_dst_swizzle(&mut self, _dst_swizzle: u32, _operand: bool) {
        // TODO: Port swizzle decoding from shader_recompiler.cpp.
    }

    pub fn print_dst_swizzle01(&mut self, _dst_register: u32, _dst_swizzle: u32) {
        // TODO: Port swizzle handling for zero/one lanes.
    }

    pub fn recompile_vertex_fetch(&mut self, _instr: &VertexFetchInstruction, _address: u32) {
        // TODO: Port vertex fetch instruction handling.
    }

    pub fn recompile_texture_fetch(&mut self, _instr: &TextureFetchInstruction, _bicubic: bool) {
        // TODO: Port texture fetch instruction handling.
    }

    pub fn recompile_alu(&mut self, _instr: &AluInstruction) {
        // TODO: Port ALU instruction handling.
    }

    pub fn recompile_shader(&mut self, _shader_data: &[u8], _include: &str) {
        // TODO: Port shader decoding and HLSL generation.
    }
}

// Placeholder instruction types until shader_code.rs is ported.
#[derive(Debug, Clone, Copy)]
pub struct VertexFetchInstruction;

#[derive(Debug, Clone, Copy)]
pub struct TextureFetchInstruction;

#[derive(Debug, Clone, Copy)]
pub struct AluInstruction;
