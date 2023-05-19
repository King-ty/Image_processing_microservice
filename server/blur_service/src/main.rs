use config::Config;
use tonic::transport::Server;

mod blur_service;
use blur_service::BlurServiceImpl;

use blur_service::image_processing::blur_service_server::BlurServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?;
    let addr = config.get("addr").unwrap_or("[::1]:50054".parse()?);
    let blur_service = BlurServiceImpl::default();

    println!("Listening on: {}", addr); // Debug

    Server::builder()
        .add_service(BlurServiceServer::new(blur_service))
        .serve(addr)
        .await?;

    Ok(())
}
