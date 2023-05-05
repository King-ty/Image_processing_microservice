use api::api_gateway_client::ApiGatewayClient;
use api::{ProcessImageRequest, ProcessingType};

pub mod api {
    tonic::include_proto!("api");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "http://[::1]:50051";
    let mut client = ApiGatewayClient::connect(server_addr).await?;

    let request = tonic::Request::new(ProcessImageRequest {
        image_url: "xxx".into(), // TODO
        processing_type: ProcessingType::Grayscale as i32,
    });

    let response = client.process_image(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
