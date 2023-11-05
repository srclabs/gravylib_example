use std::error::Error;

use spirv_builder::{SpirvBuilder, MetadataPrintout};

fn main() -> Result<(), Box<dyn Error>> {

    if std::env::var("TARGET").unwrap().contains("spirv") {
        return Ok(());
    }
    
    // This should only run when the shader crate is included in a runnern project (i.e. not when compiling the shader itself, which causes an infinite loop)
    let result = SpirvBuilder::new(env!("CARGO_MANIFEST_DIR"), "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::Full)
        .build()?; // Compile the shader to SPIR-V binary

    let shader_name = env!("CARGO_PKG_NAME").replace("-", "_") + ".spv";

    let shader_path = result.module.unwrap_single();

    let _ = std::fs::copy(shader_path, std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(shader_name));

    Ok(())
}