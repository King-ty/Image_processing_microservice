use tonic::transport::Server;

mod api_gateway;
use api_gateway::gateway;
use api_gateway::ApiGateway;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let api_gateway = ApiGateway::default();

    Server::builder()
        .add_service(gateway::api_gateway_server::ApiGatewayServer::new(
            api_gateway,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
