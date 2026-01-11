//! Rust counterparts to the shared shader constants and data layouts.
//!
//! This module mirrors the HLSL-facing definitions in `shader_common.h` so that
//! Rust tooling can reason about the same layouts and bit flags.

/// Specialization constant bit for R11G11B10 normal decoding.
pub const SPEC_CONSTANT_R11G11B10_NORMAL: u32 = 1 << 0;
/// Specialization constant bit for alpha testing.
pub const SPEC_CONSTANT_ALPHA_TEST: u32 = 1 << 1;

/// Specialization constant bit for bicubic GI filtering.
pub const SPEC_CONSTANT_BICUBIC_GI_FILTER: u32 = 1 << 2;
/// Specialization constant bit for alpha-to-coverage.
pub const SPEC_CONSTANT_ALPHA_TO_COVERAGE: u32 = 1 << 3;
/// Specialization constant bit for reverse-Z depth.
pub const SPEC_CONSTANT_REVERSE_Z: u32 = 1 << 4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PushConstants {
    pub vertex_shader_constants: u64,
    pub pixel_shader_constants: u64,
    pub shared_constants: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CubeMapData {
    pub cube_map_directions: [[f32; 3]; 2],
    pub cube_map_index: u32,
}
