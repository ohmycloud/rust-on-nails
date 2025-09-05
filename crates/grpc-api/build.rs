fn main() -> std::io::Result<()> {
    tonic_prost_build::configure().compile_protos(&["protos/api.proto"], &["protos"])?;
    Ok(())
}
