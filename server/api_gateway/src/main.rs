use config::Config;
use tonic::transport::Server;

mod api_gateway;
use api_gateway::gateway;
use api_gateway::ApiGatewayImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?;
    let addr = config.get("addr").unwrap_or("[::]:50051".parse()?);

    // let addr = "[::1]:50051".parse()?;
    // let grayscale_service_addr = "http://[::1]:50052".to_string();
    // let pixelate_service_addr = "http://[::1]:50053".to_string();
    // let blur_service_addr = "http://[::1]:50054".to_string();
    // let ascii_service_addr = "http://[::1]:50056".to_string();
    // let resize_service_addr = "http://[::1]:50057".to_string();

    let grayscale_service_addr = config
        .get("grayscale_service_addr")
        .unwrap_or("http://[::1]:50052".to_string());
    let pixelate_service_addr = config
        .get("pixelate_service_addr")
        .unwrap_or("http://[::1]:50053".to_string());
    let blur_service_addr = config
        .get("blur_service_addr")
        .unwrap_or("http://[::1]:50054".to_string());
    let ascii_service_addr = config
        .get("ascii_service_addr")
        .unwrap_or("http://[::1]:50056".to_string());
    let resize_service_addr = config
        .get("resize_service_addr")
        .unwrap_or("http://[::1]:50057".to_string());

    // Add new addr here

    let api_gateway = ApiGatewayImpl::new(
        grayscale_service_addr,
        pixelate_service_addr,
        blur_service_addr,
        ascii_service_addr,
        resize_service_addr,
    )
    .await?;

    println!("Listening on: {}", addr); // Debug

    Server::builder()
        .add_service(gateway::api_gateway_server::ApiGatewayServer::new(
            api_gateway,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
