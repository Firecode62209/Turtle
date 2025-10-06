use std::{env, path::PathBuf};

use glsl_to_spirv::ShaderType;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let workspace_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
    .unwrap();

    let in_dir = workspace_root.join("assets/source/shaders");
    let out_dir = workspace_root.join("assets/generated/shaders");

    // Tell the build script to only run again if we change our source shaders
    println!("cargo:rerun-if-changed={}", in_dir.to_string_lossy());

    // Create destination path if necessary
    std::fs::create_dir_all(out_dir.clone())?;

    for entry in std::fs::read_dir(in_dir)? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            let shader_type = in_path.extension().and_then(|ext| {
                match ext.to_string_lossy().as_ref() {
                    "vert" => Some(ShaderType::Vertex),
                    "frag" => Some(ShaderType::Fragment),
                    _ => None,
                }
            });

            if let Some(shader_type) = shader_type {
                use std::io::Read;

                let source = std::fs::read_to_string(&in_path)?;
                let mut compiled_file = glsl_to_spirv::compile(&source, shader_type)?;

                let mut compiled_bytes = Vec::new();
                compiled_file.read_to_end(&mut compiled_bytes)?;

                let out_path = format!(
                    "{}/{}.spv",
                    out_dir.to_string_lossy(),
                    in_path.file_name().unwrap().to_string_lossy()
                );

                std::fs::write(&out_path.clone(), &compiled_bytes)?;
            }
        }
    }

    Ok(())
}