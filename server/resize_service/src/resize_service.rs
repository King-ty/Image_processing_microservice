use image::{imageops, ImageFormat};
use std::io::Cursor;
use std::time::Instant;
use tonic::{Request, Response, Status};

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

use image_processing::resize_service_server::ResizeService;
use image_processing::{ResizeRequest, ResizeResponse};

#[derive(Default)]
pub struct ResizeServiceImpl;

#[tonic::async_trait]
impl ResizeService for ResizeServiceImpl {
    async fn resize_image(
        &self,
        request: Request<ResizeRequest>,
    ) -> Result<Response<ResizeResponse>, Status> {
        let start_time = Instant::now();

        let req = request.into_inner();

        let image_data = req.image_data;
        let img_format = image::guess_format(&image_data).unwrap_or(ImageFormat::Png);
        let img = match image::load_from_memory(&image_data) {
            Ok(img) => img,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid image data or format"));
            }
        };

        // Calculate resized dimensions while preserving aspect ratio
        let (width, height) =
            calculate_resized_dimensions(img.width(), img.height(), req.max_width);

        // Resize the image
        let resized_img = imageops::resize(&img, width, height, imageops::FilterType::Triangle);

        // Encode resized image to bytes
        let mut buffer = Cursor::new(Vec::new());
        if let Err(_) = resized_img.write_to(&mut buffer, img_format) {
            return Err(Status::internal("Failed to write resized image to buffer"));
        }

        let resp = ResizeResponse {
            image_data: buffer.into_inner(),
        };

        let duration = Instant::now().duration_since(start_time);
        println!("{:?}", duration);

        Ok(Response::new(resp))
    }
}

fn calculate_resized_dimensions(
    original_width: u32,
    original_height: u32,
    max_width: u32,
) -> (u32, u32) {
    let ratio = max_width as f64 / original_width as f64;
    let width = max_width;
    let height = (original_height as f64 * ratio) as u32;
    (width, height)
}
