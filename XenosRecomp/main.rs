//! Rust translation of `main.cpp` entrypoint.

mod dxc_compiler;
mod shader;
mod shader_compiler;
mod shader_common;
mod shader_parsing;

use std::collections::BTreeMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use dxc_compiler::DxcCompiler;
use shader_compiler::ShaderCompiler;

#[derive(Debug, Default)]
struct RecompiledShader {
    data: Vec<u8>,
    dxil: Option<Vec<u8>>,
    spirv: Vec<u8>,
    spec_constants_mask: u32,
}

fn read_all_bytes(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path)
}

fn write_all_bytes(path: &Path, data: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(data)
}

fn collect_shader_containers(_data: &[u8]) -> Vec<Vec<u8>> {
    // TODO: Port shader container scanning from main.cpp.
    Vec::new()
}

fn hash_shader(data: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

fn visit_dir(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            visit_dir(&entry_path, files)?;
        } else {
            files.push(entry_path);
        }
    }
    Ok(())
}

fn handle_directory(input: &Path, output: &Path, include: &str) -> io::Result<()> {
    let mut shaders: BTreeMap<u64, RecompiledShader> = BTreeMap::new();
    let mut files = Vec::new();
    visit_dir(input, &mut files)?;

    for path in files {
        let file_data = read_all_bytes(&path)?;
        for container in collect_shader_containers(&file_data) {
            let hash = hash_shader(&container);
            shaders.entry(hash).or_insert_with(|| RecompiledShader {
                data: container,
                ..Default::default()
            });
        }
    }

    let dxc = DxcCompiler::new();

    for shader in shaders.values_mut() {
        let mut recompiler = ShaderCompiler::new();
        recompiler.recompile_shader(&shader.data, include);
        shader.spec_constants_mask = recompiler.spec_constants_mask;

        shader.dxil = dxc
            .compile(
                &recompiler.buffer.out,
                recompiler.is_pixel_shader,
                shader.spec_constants_mask != 0,
                false,
            )
            .ok();

        shader.spirv = dxc
            .compile(&recompiler.buffer.out, recompiler.is_pixel_shader, false, true)
            .unwrap_or_default();
    }

    // TODO: Port shader cache serialization and compression.
    write_all_bytes(output, b"// TODO: shader cache output")
}

fn handle_single_file(input: &Path, output: &Path, include: &str) -> io::Result<()> {
    let data = read_all_bytes(input)?;
    let mut recompiler = ShaderCompiler::new();
    recompiler.recompile_shader(&data, include);
    write_all_bytes(output, recompiler.buffer.out.as_bytes())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: XenosRecomp [input path] [output path] [shader common header file path]");
        return Ok(());
    }

    let input = PathBuf::from(&args[1]);
    let output = PathBuf::from(&args[2]);
    let include_path = PathBuf::from(&args[3]);
    let include_bytes = read_all_bytes(&include_path)?;
    let include = String::from_utf8_lossy(&include_bytes);

    if input.is_dir() {
        handle_directory(&input, &output, &include)
    } else {
        handle_single_file(&input, &output, &include)
    }
}
