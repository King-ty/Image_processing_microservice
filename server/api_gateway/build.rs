fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/image_processing.proto"], &["proto/"])?;
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/api.proto"], &["proto/"])?;
    Ok(())
}
