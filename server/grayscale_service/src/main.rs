use tonic::transport::Server;

mod grayscale_service;
use grayscale_service::GrayscaleServiceImpl;

use grayscale_service::image_processing::grayscale_service_server::GrayscaleServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let grayscale_service = GrayscaleServiceImpl::default();

    Server::builder()
        .add_service(GrayscaleServiceServer::new(grayscale_service))
        .serve(addr)
        .await?;

    Ok(())
}
