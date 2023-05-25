mod middleware;
use middleware::check_auth;

use config::Config;
use std::time::Duration;
use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};

mod grayscale_service;
use grayscale_service::GrayscaleServiceImpl;

use grayscale_service::image_processing::grayscale_service_server::GrayscaleServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?;
    let addr = config.get("addr").unwrap_or("[::1]:50052".parse()?);

    // 配置tls
    let cert_dir = config
        .get("cert_dir")
        .unwrap_or("./tls/local.pem".to_string());
    let key_dir = config
        .get("key_dir")
        .unwrap_or("./tls/local.key".to_string());
    let client_ca_dir = config
        .get("client_ca_dir")
        .unwrap_or("./tls/client_ca.pem".to_string());

    let cert = std::fs::read_to_string(cert_dir)?;
    let key = std::fs::read_to_string(key_dir)?;
    let server_identity = Identity::from_pem(cert, key);

    let client_ca_cert = std::fs::read_to_string(client_ca_dir)?;
    let client_ca_cert = Certificate::from_pem(client_ca_cert);
    let tls = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca_cert);

    // 配置中间件
    let layer = tower::ServiceBuilder::new()
        .timeout(Duration::from_millis(1000))
        .layer(tonic::service::interceptor(check_auth))
        .into_inner();

    let grayscale_service = GrayscaleServiceImpl::default();

    println!("Listening on: {}", addr); // Debug

    Server::builder()
        .tls_config(tls)?
        .layer(layer)
        .add_service(GrayscaleServiceServer::new(grayscale_service))
        .serve(addr)
        .await?;

    Ok(())
}
