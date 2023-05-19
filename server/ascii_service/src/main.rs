use config::Config;
use tonic::transport::Server;

mod ascii_service;
use ascii_service::image_processing::ascii_service_server::AsciiServiceServer;
use ascii_service::AsciiServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?;
    let ascii_addr = config
        .get("ascii_addr")
        .unwrap_or("[::1]:50056".parse()?);
    let grayscale_service_addr = config
        .get("grayscale_service_addr")
        .unwrap_or("http://[::1]:50052".to_string());
    let resize_service_addr = config
        .get("resize_service_addr")
        .unwrap_or("http://[::1]:50057".to_string());

    let ascii_service = AsciiServiceImpl::new(grayscale_service_addr, resize_service_addr).await?;
    let ascii_server = AsciiServiceServer::new(ascii_service);

    println!("Listening on: {}", ascii_addr); // Debug

    Server::builder()
        .add_service(ascii_server)
        .serve(ascii_addr)
        .await?;

    Ok(())
}
