use image::ImageFormat;
use image_processing::blur_service_server::BlurService;
use image_processing::{ImageRequest, ImageResponse};
use std::io::Cursor;
use std::time::Instant;
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
        request: Request<ImageRequest>,
    ) -> Result<Response<ImageResponse>, Status> {
        let start_time = Instant::now();

        let req = request.into_inner();

        let image_data = req.image_data;
        let img_format = image::guess_format(&image_data).unwrap_or(ImageFormat::Png);
        // let img = match image::load_from_memory_with_format(&image_data, img_format) {
        let img = match image::load_from_memory(&image_data) {
            Ok(img) => img,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid image data or format"));
            }
        };

        // let time1 = Instant::now(); // debug

        // TODO: Add sigma logic to gRPC request
        let sigma = 5.0;

        let blurred_image = image::imageops::blur(&img, sigma);

        // let time2 = Instant::now(); // debug

        let mut buffer = Cursor::new(Vec::new());
        if let Err(_) = blurred_image.write_to(&mut buffer, img_format) {
            return Err(Status::internal("Failed to write blurred image to buffer"));
        }

        let resp = ImageResponse {
            image_data: buffer.into_inner(),
        };

        // let time3 = Instant::now(); // debug

        // println!(
        //     "t1={:?}, t2={:?}, t3={:?}",
        //     time1.duration_since(start_time),
        //     time2.duration_since(time1),
        //     time3.duration_since(time2)
        // );

        let duration = Instant::now().duration_since(start_time);
        println!("{:?}", duration);

        Ok(Response::new(resp))
    }
}
