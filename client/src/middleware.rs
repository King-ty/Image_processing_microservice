use tonic::metadata::MetadataValue;
use tonic::{Request, Status};

pub fn insert_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer user-auth-token".parse().unwrap();
    req.metadata_mut().insert("authorization", token);
    Ok(req)
}
