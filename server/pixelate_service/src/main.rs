use tonic::transport::Server;

mod pixelate_service;
use pixelate_service::PixelateService;

use pixelate_service::image_processing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50053".parse().unwrap();
    let pixelate_service = PixelateService::default();

    Server::builder()
        .add_service(
            image_processing::pixelate_service_server::PixelateServiceServer::new(pixelate_service),
        )
        .serve(addr)
        .await?;

    Ok(())
}
