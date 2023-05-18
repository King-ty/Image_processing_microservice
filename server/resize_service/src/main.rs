use tonic::transport::Server;

mod resize_service;
use resize_service::ResizeServiceImpl;

use resize_service::image_processing::resize_service_server::ResizeServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50057".parse()?;
    let grayscale_service = ResizeServiceImpl::default();

    Server::builder()
        .add_service(ResizeServiceServer::new(grayscale_service))
        .serve(addr)
        .await?;

    Ok(())
}
