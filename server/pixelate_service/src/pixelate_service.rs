use tonic::{Request, Response, Status};

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

use image_processing::{ImageUrl, ProcessingResult};

pub struct PixelateService {}

impl Default for PixelateService {
    fn default() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl image_processing::pixelate_service_server::PixelateService for PixelateService {
    async fn process_image(
        &self,
        request: Request<ImageUrl>,
    ) -> Result<Response<ProcessingResult>, Status> {
        let image_url = request.into_inner().image_url;

        // Implement the logic to pixelate the image.
        // You can use the 'image' crate to perform the pixelation.

        Ok(Response::new(ProcessingResult {
            result: "Image pixelated".to_string(),
        }))
    }
}
