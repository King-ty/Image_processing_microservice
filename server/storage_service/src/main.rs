use tonic::transport::Server;

mod storage_service;
use storage_service::storage_service::storage_service_server::StorageServiceServer;
use storage_service::StorageServiceImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50055".parse()?;
    let sql_url = "mysql://username:password@localhost:3306/database";
    let storage_service = StorageServiceServer::new(StorageServiceImpl::new(sql_url)?);

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(storage_service)
        .serve(addr)
        .await?;

    Ok(())
}
