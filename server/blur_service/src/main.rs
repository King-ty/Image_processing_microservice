use tonic::transport::Server;

mod blur_service;
use blur_service::BlurServiceImpl;

use blur_service::image_processing::blur_service_server::BlurServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50054".parse()?;
    let blur_service = BlurServiceImpl::default();

    Server::builder()
        .add_service(BlurServiceServer::new(blur_service))
        .serve(addr)
        .await?;

    Ok(())
}
