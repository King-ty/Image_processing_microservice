use tonic::transport::Server;

mod api_gateway;
use api_gateway::gateway;
use api_gateway::ApiGatewayImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let grayscale_service_addr = "http://[::1]:50052";
    let api_gateway = ApiGatewayImpl::new(grayscale_service_addr.to_string()).await?;

    Server::builder()
        .add_service(gateway::api_gateway_server::ApiGatewayServer::new(
            api_gateway,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
