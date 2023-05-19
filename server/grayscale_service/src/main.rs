use config::Config;
use tonic::transport::Server;

mod grayscale_service;
use grayscale_service::GrayscaleServiceImpl;

use grayscale_service::image_processing::grayscale_service_server::GrayscaleServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?;
    let addr = config.get("addr").unwrap_or("[::1]:50052".parse()?);
    let grayscale_service = GrayscaleServiceImpl::default();

    println!("Listening on: {}", addr); // Debug

    Server::builder()
        .add_service(GrayscaleServiceServer::new(grayscale_service))
        .serve(addr)
        .await?;

    Ok(())
}
