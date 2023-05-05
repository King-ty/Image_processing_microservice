use tonic::transport::Server;

mod grayscale_service;
use grayscale_service::GrayscaleService;

use grayscale_service::image_processing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50052".parse().unwrap();
    let grayscale_service = GrayscaleService::default();

    Server::builder()
        .add_service(image_processing::grayscale_service_server::GrayscaleServiceServer::new(
            grayscale_service,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
