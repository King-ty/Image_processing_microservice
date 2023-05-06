use tonic::transport::Server;

mod pixelate_service;
use pixelate_service::PixelateServiceImpl;

use pixelate_service::image_processing::pixelate_service_server::PixelateServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50053".parse()?;
    let pixelate_service = PixelateServiceImpl::default();

    Server::builder()
        .add_service(PixelateServiceServer::new(pixelate_service))
        .serve(addr)
        .await?;

    Ok(())
}
