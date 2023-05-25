use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tonic::codegen::InterceptedService;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::{Request, Response, Status};
use tower::timeout::Timeout;

use crate::middleware;

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

use image_processing::ascii_service_server::AsciiService;
use image_processing::{
    grayscale_service_client::GrayscaleServiceClient, resize_service_client::ResizeServiceClient,
};
use image_processing::{AsciiResponse, ImageRequest, ResizeRequest};

pub struct AsciiServiceImpl {
    grayscale_client: Arc<
        Mutex<
            GrayscaleServiceClient<
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
}

impl AsciiServiceImpl {
    pub async fn new(
        grayscale_service_address: String,
        resize_service_address: String,
        client_tls_config: ClientTlsConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let service_builder = tower::ServiceBuilder::new()
            .timeout(Duration::from_micros(1000))
            .layer(tonic::service::interceptor(
                middleware::insert_auth as fn(Request<()>) -> Result<Request<()>, Status>,
            ));

        // grayscale
        let grayscale_channel = Channel::from_shared(grayscale_service_address)?
            .tls_config(client_tls_config.clone())?
            .connect()
            .await?;
        let grayscale_channel = service_builder.service(grayscale_channel);
        let grayscale_client = Arc::new(Mutex::new(GrayscaleServiceClient::new(grayscale_channel)));

        // resize
        let resize_channel = Channel::from_shared(resize_service_address)?
            .tls_config(client_tls_config)?
            .connect()
            .await?;
        let resize_channel = service_builder.service(resize_channel);
        let resize_client = Arc::new(Mutex::new(ResizeServiceClient::new(resize_channel)));

        // let resize_client = Arc::new(Mutex::new(
        //     ResizeServiceClient::connect(resize_service_address).await?,
        // ));

        Ok(Self {
            grayscale_client,
            resize_client,
        })
    }
}

#[tonic::async_trait]
impl AsciiService for AsciiServiceImpl {
    async fn convert_to_ascii(
        &self,
        request: Request<ImageRequest>,
    ) -> Result<Response<AsciiResponse>, Status> {
        // 记录开始时间
        let start_time = Instant::now();

        // 获取 ASCII 转换所需的参数和请求数据
        let ascii_request = request.into_inner();

        let mut image_data = ascii_request.image_data;

        let origin_image = match image::load_from_memory(&image_data) {
            Ok(img) => img,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid image data or format"));
            }
        };

        // 如果太大先裁剪
        let max_width = 256; // 图片最大宽度
        if origin_image.width() > max_width {
            let resize_request = ResizeRequest {
                image_data,
                max_width,
            };

            let mut resize_client = self.resize_client.lock().await;
            let resize_response = resize_client
                .resize_image(Request::new(resize_request))
                .await
                .map_err(|err| {
                    Status::internal(format!("Failed to call grayscale service: {}", err))
                })?
                .into_inner();

            image_data = resize_response.image_data;
        }

        // 调用生成灰度图微服务的方法来获取灰度图像数据
        let image_request = ImageRequest { image_data };
        let mut grayscale_client = self.grayscale_client.lock().await;
        let grayscale_response = grayscale_client
            .grayscale_image(Request::new(image_request))
            .await
            .map_err(|err| Status::internal(format!("Failed to call grayscale service: {}", err)))?
            .into_inner();

        let grayscale_image = match image::load_from_memory(&grayscale_response.image_data) {
            Ok(img) => img,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid image data or format"));
            }
        };

        // 执行 ASCII 转换逻辑，将灰度图像数据转换为 ASCII 字符串
        let ascii_image = convert_to_ascii(grayscale_image.as_bytes(), grayscale_image.width()); // , grayscale_image.height()

        // 构造 ASCII 转换微服务的响应
        let ascii_response = AsciiResponse {
            ascii_data: ascii_image,
        };

        let duration = Instant::now().duration_since(start_time);
        println!("{:?}", duration);

        Ok(Response::new(ascii_response))
    }
}

fn convert_to_ascii(gray_image_data: &[u8], width: u32) -> String {
    // 定义字符集，将像素值映射到不同的字符上
    let char_set = "@%#*+=-:. ";

    // 将图像数据划分为像素值，并将每个像素值映射到字符集中的对应字符
    let ascii_chars: Vec<char> = gray_image_data
        .iter()
        .map(|&pixel| {
            let index = (pixel as usize * (char_set.len() - 1)) / 255;
            char_set.chars().nth(index).unwrap_or(' ')
        })
        .collect();

    // 将字符数组重新排列为图像的行列形式
    let ascii_image: String = ascii_chars
        .chunks(width as usize)
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    ascii_image
}
