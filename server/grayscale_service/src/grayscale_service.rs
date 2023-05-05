use tonic::{Request, Response, Status};

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

use image_processing::{ImageUrl, ProcessingResult};

pub struct GrayscaleService {}

impl Default for GrayscaleService {
    fn default() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl image_processing::grayscale_service_server::GrayscaleService for GrayscaleService {
    async fn process_image(
        &self,
        request: Request<ImageUrl>,
    ) -> Result<Response<ProcessingResult>, Status> {
        let image_url = request.into_inner().image_url;

        // Implement the logic to convert the image to grayscale.
        // You can use the 'image' crate to perform the conversion.

        Ok(Response::new(ProcessingResult {
            result: "Image converted to grayscale".to_string(),
        }))
    }
}
