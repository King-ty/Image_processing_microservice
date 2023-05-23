use tonic::metadata::MetadataValue;
use tonic::{Request, Status};

pub fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let user_token: MetadataValue<_> = "Bearer user-auth-token".parse().unwrap();
    let admin_token: MetadataValue<_> = "Bearer admin-auth-token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if user_token == t || admin_token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
