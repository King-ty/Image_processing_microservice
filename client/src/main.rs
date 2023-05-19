use api::api_gateway_client::ApiGatewayClient;
use api::{ProcessImageRequest, ProcessingType};
use config::Config;
use std::fs::File;
use std::io::{Read, Write};
// use std::env;
// use std::path::Path;
// use tonic::transport::Channel;

pub mod api {
    tonic::include_proto!("api");
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
    let image_path = config.get("IMAGE_PATH").unwrap_or("test.jpg".to_string());
    let output_path = config.get("OUTPUT_PATH").unwrap_or("./output".to_string());

    let mut file = File::open(image_path)?;
    let mut image_data = Vec::new();
    file.read_to_end(&mut image_data)?;

    let mut client = ApiGatewayClient::connect(server_addr).await?;

    // test Grayscale
    if true {
        let request = ProcessImageRequest {
            image_data: image_data.clone(),
            processing_type: ProcessingType::Grayscale as i32,
        };

        let response = client.process_image(request).await?.into_inner();

        let processed_image_data = response.image_result;

        // Save the processed image
        let output_path = format!("{}-grayscale.jpg", &output_path);
        let mut output_file = File::create(&output_path)?;
        output_file.write_all(&processed_image_data)?;

        println!("Grayscaled image saved to {:?}", output_path);
    }

    // test Pixelate
    if true {
        let request = ProcessImageRequest {
            image_data: image_data.clone(),
            processing_type: ProcessingType::Pixelate as i32,
        };

        let response = client.process_image(request).await?.into_inner();

        let processed_image_data = response.image_result;

        // Save the processed image
        let output_path = format!("{}-pixelate.jpg", &output_path);
        let mut output_file = File::create(&output_path)?;
        output_file.write_all(&processed_image_data)?;

        println!("Pixelated image saved to {:?}", output_path);
    }

    // test Blur
    if true {
        let request = ProcessImageRequest {
            image_data: image_data.clone(),
            processing_type: ProcessingType::Blur as i32,
        };

        let response = client.process_image(request).await?.into_inner();

        let processed_image_data = response.image_result;

        // Save the processed image
        let output_path = format!("{}-blur.jpg", &output_path);
        let mut output_file = File::create(&output_path)?;
        output_file.write_all(&processed_image_data)?;

        println!("Blurred image saved to {:?}", output_path);
    }

    // test Resize
    if true {
        let request = ProcessImageRequest {
            image_data: image_data.clone(),
            processing_type: ProcessingType::Resize as i32,
        };

        let response = client.process_image(request).await?.into_inner();

        let processed_image_data = response.image_result;

        // Save the processed image
        let output_path = format!("{}-resize.jpg", &output_path);
        let mut output_file = File::create(&output_path)?;
        output_file.write_all(&processed_image_data)?;

        println!("Resized image saved to {:?}", output_path);
    }

    //test Ascii
    if true {
        let request = ProcessImageRequest {
            image_data: image_data.clone(),
            processing_type: ProcessingType::Ascii as i32,
        };

        let response = client.process_image(request).await?.into_inner();

        let processed_string_data = response.string_result;

        // Save the processed image
        let output_path = format!("{}-ascii.txt", &output_path);
        let mut output_file = File::create(&output_path)?;
        output_file.write_all(processed_string_data.as_bytes())?;

        println!("Ascii image saved to {:?}", output_path);
    }

    Ok(())
}
