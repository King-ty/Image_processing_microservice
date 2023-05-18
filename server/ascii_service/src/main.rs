use tonic::transport::Server;

mod ascii_service;
use ascii_service::image_processing::ascii_service_server::AsciiServiceServer;
use ascii_service::AsciiServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ascii_address = "[::1]:50056".parse()?;
    let grayscale_address = "[::1]:50052".to_string();
    let ascii_service = AsciiServiceImpl::new(grayscale_address).await?;
    let ascii_server = AsciiServiceServer::new(ascii_service);

    Server::builder()
        .add_service(ascii_server)
        .serve(ascii_address)
        .await?;

    Ok(())
}
