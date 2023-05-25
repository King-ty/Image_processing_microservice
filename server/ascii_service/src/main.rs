mod middleware;
use middleware::check_auth;

use config::Config;
use std::time::Duration;
use tonic::transport::{Certificate, ClientTlsConfig, Identity, Server, ServerTlsConfig};

mod ascii_service;
use ascii_service::image_processing::ascii_service_server::AsciiServiceServer;
use ascii_service::AsciiServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?;
    let ascii_addr = config.get("ascii_addr").unwrap_or("[::1]:50056".parse()?);
    let grayscale_service_addr = config
        .get("grayscale_service_addr")
        .unwrap_or("http://[::1]:50052".to_string());
    let resize_service_addr = config
        .get("resize_service_addr")
        .unwrap_or("http://[::1]:50057".to_string());

    // 配置服务端tls
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
    let server_tls = ServerTlsConfig::new()
        .identity(server_identity)
        .client_ca_root(client_ca_cert);

    // 配置客户端tls
    let server_ca_dir = config
        .get("server_ca_dir")
        .unwrap_or("./tls/server_ca.pem".to_string());
    let client_cert_dir = config
        .get("client_cert_dir")
        .unwrap_or("./tls/client.pem".to_string());
    let client_key_dir = config
        .get("client_key_dir")
        .unwrap_or("./tls/client.key".to_string());

    let server_ca_cert = std::fs::read_to_string(server_ca_dir)?;
    let server_ca_cert = Certificate::from_pem(server_ca_cert);
    let client_cert = std::fs::read_to_string(client_cert_dir)?;
    let client_key = std::fs::read_to_string(client_key_dir)?;
    let client_identity = Identity::from_pem(client_cert, client_key);

    let client_tls = ClientTlsConfig::new()
        .domain_name("localhost")
        .ca_certificate(server_ca_cert)
        .identity(client_identity);

    // 配置中间件
    let layer = tower::ServiceBuilder::new()
        .timeout(Duration::from_millis(1000))
        .layer(tonic::service::interceptor(check_auth))
        .into_inner();

    let ascii_service =
        AsciiServiceImpl::new(grayscale_service_addr, resize_service_addr, client_tls).await?;
    let ascii_server = AsciiServiceServer::new(ascii_service);

    println!("Listening on: {}", ascii_addr); // Debug

    Server::builder()
        .tls_config(server_tls)?
        .layer(layer)
        .add_service(ascii_server)
        .serve(ascii_addr)
        .await?;

    Ok(())
}
