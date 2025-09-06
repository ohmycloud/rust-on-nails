use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    std::fs::create_dir_all("generated")?;

    tonic_prost_build::configure()
        .file_descriptor_set_path(out_dir.join("api_descriptor_set.bin"))
        .file_descriptor_set_path("generated/api_descriptor_set.bin")
        .compile_protos(&["protos/api.proto"], &["protos"])?;

    // 将 protobuf 代码输出到 OUT_DIR 目录
    println!("cargo:rerun-if-changed=protos/api.proto");
    println!("cargo:include={out_dir:?}");

    Ok(())
}
