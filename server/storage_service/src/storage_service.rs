use jsonwebtoken::{decode, DecodingKey, Validation};
use mysql::params;
use mysql::prelude::*;
use tonic::{Request, Response, Status};

pub mod storage_service {
    tonic::include_proto!("storage_service");
}

use storage_service::{
    storage_service_server::StorageService, GetRequest, GetResponse, StoreRequest, StoreResponse,
};

pub struct StorageServiceImpl {
    pool: mysql::Pool,
}

impl StorageServiceImpl {
    pub fn new(url: &str) -> Result<Self, mysql::Error> {
        let pool = mysql::Pool::new(url)?;
        Ok(Self { pool })
    }

    fn store_image_in_database(&self, image_data: &[u8]) -> Result<usize, mysql::Error> {
        let mut conn = self.pool.get_conn()?;
        let result = conn.exec_first::<u64, _, _>(
            "INSERT INTO images (image_data) VALUES (:image_data)",
            params! {
                "image_data" => image_data,
            },
        )?;

        Ok(result.unwrap() as usize)
    }

    fn read_image_from_database(&self, image_id: &str) -> Option<Vec<u8>> {
        let mut conn = self.pool.get_conn().ok()?;
        let result = conn
            .exec_first::<Vec<u8>, _, _>(
                "SELECT image_data FROM images WHERE image_id = :image_id",
                params! {
                    "image_id" => image_id,
                },
            )
            .ok();

        result.and_then(|row| row)
    }
}

#[tonic::async_trait]
impl StorageService for StorageServiceImpl {
    async fn store_image(
        &self,
        request: Request<StoreRequest>,
    ) -> Result<Response<StoreResponse>, Status> {
        let req = request.into_inner();
        // 验证Token
        let token = req.token;
        if !validate_token(&token) {
            return Err(Status::unauthenticated("Invalid token"));
        }

        // 处理存储图片的逻辑
        let image_data = req.image_data;
        let image_id = match self.store_image_in_database(&image_data) {
            Ok(image_id) => image_id,
            Err(err) => {
                return Err(Status::internal(format!(
                    "Failed to store image in database: {}",
                    err
                )));
            }
        };

        let response = StoreResponse {
            state: 0,
            image_id: image_id.to_string(),
            msg: "Image stored successfully".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_image(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        // 验证Token
        let token = req.token;
        if !validate_token(&token) {
            return Err(Status::unauthenticated("Invalid token"));
        }

        // 处理读取图片的逻辑
        let image_id = req.image_id;
        let image_data = match self.read_image_from_database(&image_id) {
            Some(image_data) => image_data,
            None => {
                return Err(Status::not_found(format!(
                    "Image not found with image ID: {}",
                    image_id
                )));
            }
        };

        let response = GetResponse {
            state: 0,
            image_data,
            msg: "Image retrieved successfully".to_string(),
        };

        Ok(Response::new(response))
    }
}

fn validate_token(token: &str) -> bool {
    // 假设 JWT 密钥为 "administrator"
    let secret = "administrator";

    // 验证 JWT
    match decode::<serde_json::Value>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(_) => true,   // 验证成功
        Err(_) => false, // 验证失败
    }
}
