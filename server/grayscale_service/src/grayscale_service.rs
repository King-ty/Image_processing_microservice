// use image::{DynamicImage, GenericImageView, ImageError, ImageFormat};
use image::ImageFormat;
use image_processing::grayscale_service_server::GrayscaleService;
use image_processing::{ImageRequest, ImageResponse};
use std::io::Cursor;
use std::time::Instant;
use tonic::{Request, Response, Status};

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

#[derive(Default)]
pub struct GrayscaleServiceImpl;

#[tonic::async_trait]
impl GrayscaleService for GrayscaleServiceImpl {
    async fn grayscale_image(
        &self,
        request: Request<ImageRequest>,
    ) -> Result<Response<ImageResponse>, Status> {
        let start_time = Instant::now();

        let req = request.into_inner();

        // Implement the logic to convert the image to grayscale.
        let img = match image::load_from_memory(&req.image_data) {
            Ok(img) => img,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid image data or format"));
            }
        };

        let gray_image = img.into_luma8();

        let mut buffer = Cursor::new(Vec::new());
        if let Err(_) = gray_image.write_to(&mut buffer, ImageFormat::Jpeg) {
            return Err(Status::internal(
                "Failed to write grayscale image to buffer",
            ));
        }

        let resp = ImageResponse {
            image_data: buffer.into_inner(),
        };

        let duration = Instant::now().duration_since(start_time);
        println!("{:?}", duration);

        Ok(Response::new(resp))
    }
}
