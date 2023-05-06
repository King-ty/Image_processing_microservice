use image::ImageFormat;
use image_processing::pixelate_service_server::PixelateService;
use image_processing::{ImageRequest, ImageResponse};
use std::io::Cursor;
use tonic::{Request, Response, Status};

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

#[derive(Default)]
pub struct PixelateServiceImpl;

#[tonic::async_trait]
impl PixelateService for PixelateServiceImpl {
    async fn pixelate_image(
        &self,
        request: Request<ImageRequest>,
    ) -> Result<Response<ImageResponse>, Status> {
        let req = request.into_inner();

        // Implement the logic to convert the image to grayscale.
        let img = match image::load_from_memory(&req.image_data) {
            Ok(img) => img,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid image data or format"));
            }
        };

        let (width, height) = (img.width(), img.height());
        let pixelated_image = img
            .resize(
                width / 10,
                height / 10,
                image::imageops::FilterType::Nearest,
            )
            .resize(width, height, image::imageops::FilterType::Nearest);

        let mut buffer = Cursor::new(Vec::new());
        if let Err(_) = pixelated_image.write_to(&mut buffer, ImageFormat::Jpeg) {
            return Err(Status::internal(
                "Failed to write grayscale image to buffer",
            ));
        }

        let resp = ImageResponse {
            image_data: buffer.into_inner(),
        };

        Ok(Response::new(resp))
    }
}
