use api::api_gateway_client::ApiGatewayClient;
use api::ProcessImageRequest;
use clap::{Parser, ValueEnum};
use config::Config;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use tokio::time::sleep;
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
    #[arg(short, long, default_value_t = 10)]
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

    let image_path = format!("{}/Lenna_{}.jpg", image_path, size);
    let mut file = File::open(image_path)?;
    let mut image_data = Vec::new();
    file.read_to_end(&mut image_data)?;

    let mut client = ApiGatewayClient::connect(server_addr).await?;

    let processing_type = options.processing_type.clone() as i32;

    // Run the test
    for i in 0..num_tests {
        let start = Instant::now();
        let request = ProcessImageRequest {
            image_data: image_data.clone(),
            processing_type,
        };
        let response = client.process_image(request).await?.into_inner();
        let processed_image_data = match options.processing_type {
            TestProcessingType::Ascii => response.string_result.into_bytes(),
            _ => response.image_result,
        };
        let duration = Instant::now().duration_since(start);
        println!("{}: {:?}", i, duration);
        sleep(interval).await;
        if i == num_tests - 1 {
            // Save the processed image
            let extension = match options.processing_type {
                TestProcessingType::Ascii => "txt",
                _ => "jpg",
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

    Ok(())
}
