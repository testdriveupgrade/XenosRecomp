//! Rust translation of the `DxcCompiler` interface from `dxc_compiler.*`.

#[derive(Debug, Default)]
pub struct DxcCompiler;

#[derive(Debug, Clone)]
pub struct DxcError {
    pub message: String,
}

impl DxcCompiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(
        &self,
        _shader_source: &str,
        _compile_pixel_shader: bool,
        _compile_library: bool,
        _compile_spirv: bool,
    ) -> Result<Vec<u8>, DxcError> {
        Err(DxcError {
            message: "DXC compilation is not yet implemented in Rust".to_string(),
        })
    }
}
