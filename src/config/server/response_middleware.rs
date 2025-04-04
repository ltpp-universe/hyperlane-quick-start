use crate::*;
use app::middleware::response::*;

pub async fn register(server: &Server) {
    server.response_middleware(send::func::send).await;
    server.response_middleware(log::func::log).await;
    println_success!("Server response middleware initialization completed");
}
