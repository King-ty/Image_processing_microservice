fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/image_processing.proto"], &["proto/"])?;
    Ok(())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tonic_build::compile_protos("proto/image_processing.proto")?;
//     Ok(())
// }
