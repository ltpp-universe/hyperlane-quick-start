use crate::*;

pub async fn log_dir(server: &mut Server) {
    server.log_dir("./logs").await;
}

pub async fn log_size(server: &mut Server) {
    server.log_size(100_024_000).await;
}

pub async fn log_interval_millis(server: &mut Server) {
    server.log_interval_millis(1000).await;
}
