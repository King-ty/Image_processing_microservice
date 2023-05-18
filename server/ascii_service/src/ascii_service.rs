use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

pub mod image_processing {
    tonic::include_proto!("image_processing");
}

use image_processing::ascii_service_server::AsciiService;
use image_processing::grayscale_service_client::GrayscaleServiceClient;
use image_processing::{AsciiRequest, AsciiResponse, ImageRequest};

pub struct AsciiServiceImpl {
    grayscale_client: Arc<Mutex<GrayscaleServiceClient<Channel>>>,
}

impl AsciiServiceImpl {
    pub async fn new(
        grayscale_service_address: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // let channel = tonic::transport::Channel::from_shared(grayscale_service_address)
        //     .map_err(|err| format!("Failed to create gRPC channel: {}", err))?
        //     .connect()
        //     .await
        //     .map_err(|err| format!("Failed to connect to gRPC server: {}", err))?;
        // let grayscale_client = GrayscaleServiceClient::new(channel);

        let grayscale_client = Arc::new(Mutex::new(
            GrayscaleServiceClient::connect(grayscale_service_address).await?,
        ));

        Ok(Self { grayscale_client })
    }
}

#[tonic::async_trait]
impl AsciiService for AsciiServiceImpl {
    async fn convert_to_ascii(
        &self,
        request: Request<AsciiRequest>,
    ) -> Result<Response<AsciiResponse>, Status> {
        // 获取 ASCII 转换所需的参数和请求数据
        let ascii_request = request.into_inner();
        let image_request = ImageRequest {
            image_data: ascii_request.image_data,
        };

        // 调用生成灰度图微服务的方法来获取灰度图像数据
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
