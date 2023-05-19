use config::Config;
use tonic::transport::Server;

mod resize_service;
use resize_service::ResizeServiceImpl;

use resize_service::image_processing::resize_service_server::ResizeServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?;
    let addr = config.get("addr").unwrap_or("[::1]:50057".parse()?);
    let grayscale_service = ResizeServiceImpl::default();

    println!("Listening on: {}", addr); // Debug

    Server::builder()
        .add_service(ResizeServiceServer::new(grayscale_service))
        .serve(addr)
        .await?;

    Ok(())
}
