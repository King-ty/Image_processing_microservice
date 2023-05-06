use api::api_gateway_client::ApiGatewayClient;
// use api::{ProcessImageRequest, ProcessImageResponse, ProcessingType};
use api::{ProcessImageRequest, ProcessingType};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
// use tonic::transport::Channel;

pub mod api {
    tonic::include_proto!("api");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = env::var("API_GATEWAY_ADDR").unwrap_or("http://[::1]:50051".to_string());
    let image_path = env::var("IMAGE_PATH").unwrap_or("test.jpg".to_string());
    let output_path = env::var("OUTPUT_PATH").unwrap_or("output.jpg".to_string());

    let mut file = File::open(image_path)?;
    let mut image_data = Vec::new();
    file.read_to_end(&mut image_data)?;

    let mut client = ApiGatewayClient::connect(server_addr).await?;

    let request = ProcessImageRequest {
        image_data,
        processing_type: ProcessingType::Grayscale as i32,
    };

    let response = client.process_image(request).await?.into_inner();

    let processed_image_data = response.result;

    // Save the processed image
    let output_path = Path::new(&output_path);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&processed_image_data)?;

    println!("Processed image saved to {:?}", output_path);

    Ok(())
}
