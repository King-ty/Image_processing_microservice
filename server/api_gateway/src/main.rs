use tonic::transport::Server;

mod api_gateway;
use api_gateway::gateway;
use api_gateway::ApiGatewayImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let grayscale_service_addr = "http://[::1]:50052".to_string();
    let pixelate_service_addr = "http://[::1]:50053".to_string();
    let blur_service_addr = "http://[::1]:50054".to_string();
    let ascii_service_addr = "http://[::1]:50056".to_string();
    let resize_service_addr = "http://[::1]:50057".to_string();
    // Add new addr here

    let api_gateway = ApiGatewayImpl::new(
        grayscale_service_addr,
        pixelate_service_addr,
        blur_service_addr,
        ascii_service_addr,
        resize_service_addr,
    )
    .await?;

    Server::builder()
        .add_service(gateway::api_gateway_server::ApiGatewayServer::new(
            api_gateway,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
