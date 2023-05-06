use image::ImageFormat;
use image_processing::blur_service_server::BlurService;
use image_processing::{BlurRequest, BlurResponse};
use std::io::Cursor;
use tonic::{Request, Response, Status};

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

#[derive(Default)]
pub struct BlurServiceImpl;

#[tonic::async_trait]
impl BlurService for BlurServiceImpl {
    async fn blur_image(
        &self,
        request: Request<BlurRequest>,
    ) -> Result<Response<BlurResponse>, Status> {
        let req = request.into_inner();

        // Implement the logic to convert the image to grayscale.
        let img = match image::load_from_memory(&req.image_data) {
            Ok(img) => img,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid image data or format"));
            }
        };
        let sigma = req.sigma;

        let blurred_image = image::imageops::blur(&img, sigma);

        let mut buffer = Cursor::new(Vec::new());
        if let Err(_) = blurred_image.write_to(&mut buffer, ImageFormat::Jpeg) {
            return Err(Status::internal("Failed to write blurred image to buffer"));
        }

        let resp = BlurResponse {
            image_data: buffer.into_inner(),
        };

        Ok(Response::new(resp))
    }
}
