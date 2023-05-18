use tonic::{Request, Response, Status};

pub mod gateway {
    tonic::include_proto!("api");
}

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

// use gateway::{ProcessImageRequest, ProcessImageResponse, ProcessingType};
use gateway::{ProcessImageRequest, ProcessImageResponse, ProcessingType};
use image_processing::ImageRequest;
use image_processing::{
    ascii_service_client::AsciiServiceClient,
    blur_service_client::BlurServiceClient,
    // Add new Client here
    grayscale_service_client::GrayscaleServiceClient,
    pixelate_service_client::PixelateServiceClient,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;

pub struct ApiGatewayImpl {
    grayscale_client: Arc<Mutex<GrayscaleServiceClient<Channel>>>,
    pixelate_client: Arc<Mutex<PixelateServiceClient<Channel>>>,
    blur_client: Arc<Mutex<BlurServiceClient<Channel>>>,
    ascii_client: Arc<Mutex<AsciiServiceClient<Channel>>>,
    // Add new client here
}

impl ApiGatewayImpl {
    pub async fn new(
        grayscale_service_addr: String,
        pixelate_service_addr: String,
        blur_service_addr: String,
        ascii_service_addr: String,
        // Add new addr here
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let grayscale_client = Arc::new(Mutex::new(
            GrayscaleServiceClient::connect(grayscale_service_addr).await?,
        ));

        let pixelate_client = Arc::new(Mutex::new(
            PixelateServiceClient::connect(pixelate_service_addr).await?,
        ));

        let blur_client = Arc::new(Mutex::new(
            BlurServiceClient::connect(blur_service_addr).await?,
        ));

        let ascii_client = Arc::new(Mutex::new(
            AsciiServiceClient::connect(ascii_service_addr).await?,
        ));

        // Add new client here

        Ok(Self {
            grayscale_client,
            pixelate_client,
            blur_client,
            ascii_client,
            // Add new client here
        })
    }
}

#[tonic::async_trait]
impl gateway::api_gateway_server::ApiGateway for ApiGatewayImpl {
    async fn process_image(
        &self,
        request: Request<ProcessImageRequest>,
    ) -> Result<Response<ProcessImageResponse>, Status> {
        let data = request.into_inner();
        let processing_type = data.processing_type;
        let image_data = data.image_data;

        let process_request = ImageRequest { image_data };

        // Implement logic to call the appropriate image processing service based on the processing_type value.
        let (image_result, string_result) = match ProcessingType::from_i32(processing_type).unwrap()
        {
            ProcessingType::Grayscale => {
                let mut grayscale_client = self.grayscale_client.lock().await;
                let grayscale_response = grayscale_client
                    .grayscale_image(process_request)
                    .await
                    .map_err(|e| Status::internal(format!("Grayscale service error: {}", e)))?;
                (grayscale_response.into_inner().image_data, "".to_string())
            }
            ProcessingType::Pixelate => {
                let mut pixelate_client = self.pixelate_client.lock().await;
                let pixelate_response = pixelate_client
                    .pixelate_image(process_request)
                    .await
                    .map_err(|e| Status::internal(format!("Pixelate service error: {}", e)))?;
                (pixelate_response.into_inner().image_data, "".to_string())
            }
            ProcessingType::Blur => {
                let mut blur_client = self.blur_client.lock().await;
                let blur_response = blur_client
                    .blur_image(process_request)
                    .await
                    .map_err(|e| Status::internal(format!("Blur service error: {}", e)))?;
                (blur_response.into_inner().image_data, "".to_string())
            }
            ProcessingType::Ascii => {
                let mut ascii_client = self.ascii_client.lock().await;
                let ascii_response = ascii_client
                    .convert_to_ascii(process_request)
                    .await
                    .map_err(|e| Status::internal(format!("Ascii service error: {}", e)))?;
                (vec![], ascii_response.into_inner().ascii_data)
            } // Add new client here
        };

        Ok(Response::new(ProcessImageResponse {
            image_result,
            string_result,
        }))
    }
}
