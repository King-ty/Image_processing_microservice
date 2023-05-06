use tonic::{Request, Response, Status};

pub mod gateway {
    tonic::include_proto!("api");
}

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

// use gateway::{ProcessImageRequest, ProcessImageResponse, ProcessingType};
use gateway::{ProcessImageRequest, ProcessImageResponse, ProcessingType};
use image_processing::grayscale_service_client::GrayscaleServiceClient;
use image_processing::ImageRequest;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;

pub struct ApiGatewayImpl {
    grayscale_client: Arc<Mutex<GrayscaleServiceClient<Channel>>>,
}

impl ApiGatewayImpl {
    pub async fn new(grayscale_service_addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let grayscale_client = GrayscaleServiceClient::connect(grayscale_service_addr).await?;
        let grayscale_client = Arc::new(Mutex::new(grayscale_client));

        Ok(Self { grayscale_client })
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

        // Implement logic to call the appropriate image processing service based on the processing_type value.
        let result = match ProcessingType::from_i32(processing_type).unwrap() {
            ProcessingType::Grayscale => {
                let grayscale_request = ImageRequest { image_data };
                let mut grayscale_client = self.grayscale_client.lock().await;
                let grayscale_response = grayscale_client
                    .grayscale_image(grayscale_request)
                    .await
                    .map_err(|e| Status::internal(format!("Grayscale service error: {}", e)))?;
                grayscale_response.into_inner().image_data
            }
            _ => return Err(Status::unimplemented("Processing type not implemented")),
        };

        Ok(Response::new(ProcessImageResponse { result }))
    }
}
