use config::Config;
use tonic::transport::Server;

mod pixelate_service;
use pixelate_service::PixelateServiceImpl;

use pixelate_service::image_processing::pixelate_service_server::PixelateServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?;
    let addr = config.get("addr").unwrap_or("[::1]:50053".parse()?);
    let pixelate_service = PixelateServiceImpl::default();

    println!("Listening on: {}", addr); // Debug

    Server::builder()
        .add_service(PixelateServiceServer::new(pixelate_service))
        .serve(addr)
        .await?;

    Ok(())
}
