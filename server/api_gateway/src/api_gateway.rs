use tonic::{Request, Response, Status};

pub mod gateway {
    tonic::include_proto!("api");
}

// use gateway::{ProcessImageRequest, ProcessImageResponse, ProcessingType};
use gateway::{ProcessImageRequest, ProcessImageResponse};

pub struct ApiGateway {}

impl Default for ApiGateway {
    fn default() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl gateway::api_gateway_server::ApiGateway for ApiGateway {
    async fn process_image(
        &self,
        request: Request<ProcessImageRequest>,
    ) -> Result<Response<ProcessImageResponse>, Status> {
        let data = request.into_inner();
        let processing_type = data.processing_type;
        let image_url = data.image_url;

        println!("{:?} {:?}", processing_type, image_url);

        // Implement logic to call the appropriate image processing service based on the processing_type value.
        // You can use the tonic client to call the services.

        Ok(Response::new(ProcessImageResponse {
            result: "Image processed".to_string(),
        }))
    }
}
