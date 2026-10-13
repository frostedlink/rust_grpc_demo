fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().build_server(false)
    .out_dir("proto")
    .compile_protos(&["proto/chat.proto"],&["proto"])?;
    Ok(())
}