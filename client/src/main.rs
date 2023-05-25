mod middleware;

use api::api_gateway_client::ApiGatewayClient;
use api::ProcessImageRequest;
use clap::{Parser, ValueEnum};
use config::Config;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
// use std::env;
// use std::path::Path;
// use tonic::transport::Channel;

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Fun with the client", long_about = None)]
struct CmdOptions {
    /// The number of times to run the test
    #[arg(short, long, default_value_t = 20)]
    num_tests: usize,

    /// The interval between tests (ms)
    #[arg(short, long, default_value_t = 1000)]
    interval: u64,

    /// The size of the image to process
    #[arg(short, long, default_value = "512")]
    size: String,

    #[arg(value_enum, short = 't', long, default_value_t = TestProcessingType::Grayscale)]
    processing_type: TestProcessingType,
}

// 注意一定要与 proto 文件中的 ProcessingType 一致
#[derive(ValueEnum, Debug, Clone)]
enum TestProcessingType {
    Grayscale = 0,
    Pixelate = 1,
    Blur = 2,
    Ascii = 3,
    Resize = 4,
}

impl std::string::ToString for TestProcessingType {
    fn to_string(&self) -> String {
        match self {
            TestProcessingType::Grayscale => "Grayscale".to_string(),
            TestProcessingType::Pixelate => "Pixelate".to_string(),
            TestProcessingType::Blur => "Blur".to_string(),
            TestProcessingType::Ascii => "Ascii".to_string(),
            TestProcessingType::Resize => "Resize".to_string(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()?;
    let server_addr = config
        .get("API_GATEWAY_ADDR")
        .unwrap_or("http://[::1]:50051".to_string());
    // let server_addr = env::var("API_GATEWAY_ADDR").unwrap_or("http://[::]:50051".to_string());
    let image_path = config.get("IMAGE_PATH").unwrap_or("./img".to_string());
    let output_path = config.get("OUTPUT_PATH").unwrap_or("./output".to_string());

    let options = CmdOptions::parse();

    let num_tests = options.num_tests;
    let interval = Duration::from_millis(options.interval);
    let size = options.size;

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

    let image_path = format!("{}/Lenna_{}.png", image_path, size);
    let mut file = File::open(image_path)?;
    let mut image_data = Vec::new();
    file.read_to_end(&mut image_data)?;

    let processing_type = options.processing_type.clone() as i32;

    // 创建gRPC客户端
    let channel = Channel::from_shared(server_addr)?
        .tls_config(client_tls)?
        .connect()
        .await?;
    let channel = tower::ServiceBuilder::new()
        .timeout(Duration::from_millis(1000))
        .layer(tonic::service::interceptor(middleware::insert_auth))
        .service(channel);
    let mut client = ApiGatewayClient::new(channel);

    let mut tot_time = 0;

    // Run the test
    for i in 0..num_tests {
        let start_time = Instant::now();
        let request = ProcessImageRequest {
            image_data: image_data.clone(),
            processing_type,
        };
        let response = client.process_image(request).await?.into_inner();
        let processed_image_data = match options.processing_type {
            TestProcessingType::Ascii => response.string_result.into_bytes(),
            _ => response.image_result,
        };
        let duration = Instant::now().duration_since(start_time);
        // println!("{}: {:?}", i, duration);
        println!("{:?}", duration);
        tot_time += duration.as_micros();
        sleep(interval).await;
        if i == num_tests - 1 {
            // Save the processed image
            let extension = match options.processing_type {
                TestProcessingType::Ascii => "txt",
                _ => "png",
            };
            let output_path = format!(
                "{}/Lenna_{}_{}.{}",
                &output_path,
                size,
                options.processing_type.to_string(),
                extension
            );
            let mut output_file = File::create(&output_path)?;
            output_file.write_all(&processed_image_data)?;

            println!("Processed image saved to {:?}", output_path);
        }
    }

    println!(
        "Average time: {} ms",
        tot_time as f64 / num_tests as f64 / 1000.0
    );

    Ok(())
}
