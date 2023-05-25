use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::{Request, Response, Status};
use tower::timeout::Timeout;

use crate::middleware;

pub mod gateway {
    tonic::include_proto!("api");
}

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

// use gateway::{ProcessImageRequest, ProcessImageResponse, ProcessingType};
use gateway::{ProcessImageRequest, ProcessImageResponse, ProcessingType};
use image_processing::{
    ascii_service_client::AsciiServiceClient,
    blur_service_client::BlurServiceClient,
    // Add new Client here
    grayscale_service_client::GrayscaleServiceClient,
    pixelate_service_client::PixelateServiceClient,
    resize_service_client::ResizeServiceClient,
};
use image_processing::{ImageRequest, ResizeRequest};

pub struct ApiGatewayImpl {
    grayscale_client: Arc<
        Mutex<
            GrayscaleServiceClient<
                Timeout<
                    InterceptedService<Channel, fn(Request<()>) -> Result<Request<()>, Status>>,
                >,
            >,
        >,
    >,
    pixelate_client: Arc<
        Mutex<
            PixelateServiceClient<
                Timeout<
                    InterceptedService<Channel, fn(Request<()>) -> Result<Request<()>, Status>>,
                >,
            >,
        >,
    >,
    blur_client: Arc<
        Mutex<
            BlurServiceClient<
                Timeout<
                    InterceptedService<Channel, fn(Request<()>) -> Result<Request<()>, Status>>,
                >,
            >,
        >,
    >,
    ascii_client: Arc<
        Mutex<
            AsciiServiceClient<
                Timeout<
                    InterceptedService<Channel, fn(Request<()>) -> Result<Request<()>, Status>>,
                >,
            >,
        >,
    >,
    resize_client: Arc<
        Mutex<
            ResizeServiceClient<
                Timeout<
                    InterceptedService<Channel, fn(Request<()>) -> Result<Request<()>, Status>>,
                >,
            >,
        >,
    >,
    // Add new client here
}

impl ApiGatewayImpl {
    pub async fn new(
        grayscale_service_addr: String,
        pixelate_service_addr: String,
        blur_service_addr: String,
        ascii_service_addr: String,
        resize_service_addr: String,
        // Add new addr here
        client_tls_config: ClientTlsConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let service_builder = tower::ServiceBuilder::new()
            .timeout(Duration::from_millis(1000))
            .layer(tonic::service::interceptor(
                middleware::insert_auth as fn(Request<()>) -> Result<Request<()>, Status>,
            ));

        // grayscale
        let grayscale_channel = Channel::from_shared(grayscale_service_addr)?
            .tls_config(client_tls_config.clone())?
            .connect()
            .await?;
        let grayscale_channel = service_builder.service(grayscale_channel);
        let grayscale_client = Arc::new(Mutex::new(GrayscaleServiceClient::new(grayscale_channel)));

        // pixelate
        let pixelate_channel = Channel::from_shared(pixelate_service_addr)?
            .tls_config(client_tls_config.clone())?
            .connect()
            .await?;
        let pixelate_channel = service_builder.service(pixelate_channel);
        let pixelate_client = Arc::new(Mutex::new(PixelateServiceClient::new(pixelate_channel)));

        // blur
        let blur_channel = Channel::from_shared(blur_service_addr)?
            .tls_config(client_tls_config.clone())?
            .connect()
            .await?;
        let blur_channel = service_builder.service(blur_channel);
        let blur_client = Arc::new(Mutex::new(BlurServiceClient::new(blur_channel)));

        // ascii
        let ascii_channel = Channel::from_shared(ascii_service_addr)?
            .tls_config(client_tls_config.clone())?
            .connect()
            .await?;
        let ascii_channel = service_builder.service(ascii_channel);
        let ascii_client = Arc::new(Mutex::new(AsciiServiceClient::new(ascii_channel)));

        // resize
        let resize_channel = Channel::from_shared(resize_service_addr)?
            .tls_config(client_tls_config.clone())?
            .connect()
            .await?;
        let resize_channel = service_builder.service(resize_channel);
        let resize_client = Arc::new(Mutex::new(ResizeServiceClient::new(resize_channel)));

        // Add new client here

        Ok(Self {
            grayscale_client,
            pixelate_client,
            blur_client,
            ascii_client,
            resize_client,
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
        let start_time = Instant::now();

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
            }
            ProcessingType::Resize => {
                let mut resize_client = self.resize_client.lock().await;
                let resize_response = resize_client
                    .resize_image(ResizeRequest {
                        image_data: process_request.image_data,
                        max_width: 256,
                    })
                    .await
                    .map_err(|e| Status::internal(format!("Resize service error: {}", e)))?;
                (resize_response.into_inner().image_data, "".to_string())
            } // Add new client here
        };


        let duration = Instant::now().duration_since(start_time);
        println!("{:?}", duration);

        Ok(Response::new(ProcessImageResponse {
            image_result,
            string_result,
        }))
    }
}
