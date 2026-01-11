//! Helpers for parsing Xbox 360 shader containers.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedShader {
    pub physical_offset: u32,
    pub size: u32,
    pub field_8: u32,
    pub field_c: u32,
    pub field_10: u32,
    pub interpolator_info: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedShaderContainer {
    pub flags: u32,
    pub virtual_size: u32,
    pub physical_size: u32,
    pub field_c: u32,
    pub constant_table_offset: u32,
    pub definition_table_offset: u32,
    pub shader_offset: u32,
    pub field_1c: u32,
    pub field_20: u32,
    pub shader: ParsedShader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    BufferTooSmall,
    ShaderOutOfBounds,
}

fn read_be_u32(data: &[u8], offset: usize) -> Result<u32, ParseError> {
    let end = offset + 4;
    if end > data.len() {
        return Err(ParseError::BufferTooSmall);
    }
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[offset..end]);
    Ok(u32::from_be_bytes(bytes))
}

pub fn parse_shader_container(data: &[u8]) -> Result<ParsedShaderContainer, ParseError> {
    if data.len() < 36 {
        return Err(ParseError::BufferTooSmall);
    }

    let flags = read_be_u32(data, 0)?;
    let virtual_size = read_be_u32(data, 4)?;
    let physical_size = read_be_u32(data, 8)?;
    let field_c = read_be_u32(data, 12)?;
    let constant_table_offset = read_be_u32(data, 16)?;
    let definition_table_offset = read_be_u32(data, 20)?;
    let shader_offset = read_be_u32(data, 24)?;
    let field_1c = read_be_u32(data, 28)?;
    let field_20 = read_be_u32(data, 32)?;

    let shader_offset_usize = shader_offset as usize;
    let shader_end = shader_offset_usize + 24;
    if shader_end > data.len() {
        return Err(ParseError::ShaderOutOfBounds);
    }

    let shader = ParsedShader {
        physical_offset: read_be_u32(data, shader_offset_usize)?,
        size: read_be_u32(data, shader_offset_usize + 4)?,
        field_8: read_be_u32(data, shader_offset_usize + 8)?,
        field_c: read_be_u32(data, shader_offset_usize + 12)?,
        field_10: read_be_u32(data, shader_offset_usize + 16)?,
        interpolator_info: read_be_u32(data, shader_offset_usize + 20)?,
    };

    Ok(ParsedShaderContainer {
        flags,
        virtual_size,
        physical_size,
        field_c,
        constant_table_offset,
        definition_table_offset,
        shader_offset,
        field_1c,
        field_20,
        shader,
    })
}

pub fn sample_shader_container() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(&0x102A1100u32.to_be_bytes());
    data.extend_from_slice(&60u32.to_be_bytes());
    data.extend_from_slice(&4u32.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(&36u32.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());

    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(&1u32.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());

    data.extend_from_slice(&0u32.to_be_bytes());

    data
}

#[cfg(test)]
mod tests {
    use super::{parse_shader_container, sample_shader_container};

    #[test]
    fn parses_sample_shader_container() {
        let data = sample_shader_container();
        let parsed = parse_shader_container(&data).expect("sample shader should parse");

        assert_eq!(parsed.flags, 0x102A1100);
        assert_eq!(parsed.virtual_size, 60);
        assert_eq!(parsed.physical_size, 4);
        assert_eq!(parsed.shader_offset, 36);
        assert_eq!(parsed.shader.size, 1);
    }
}
