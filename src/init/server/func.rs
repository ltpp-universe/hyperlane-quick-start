use crate::*;
use config::server::*;

pub async fn create_server() {
    let server: Server = Server::new();
    host::host(&server).await;
    port::port(&server).await;
    print::print(&server).await;
    log::log_dir(&server).await;
    log::log_size(&server).await;
    log::inner_log(&server).await;
    buffer_size::http_line_buffer_size(&server).await;
    buffer_size::websocket_buffer_size(&server).await;
    route::register(&server).await;
    request_middleware::register(&server).await;
    response_middleware::register(&server).await;
    let cfg: ServerConfig<'_> = server.get_cfg().read().await.clone();
    let host_port: String = format!("{}:{}", cfg.get_host(), cfg.get_port());
    println_success!("Server initialization successful");
    let server_result: ServerResult = server.listen().await;
    match server_result {
        Ok(_) => println_success!("Server listen in: ", host_port),
        Err(server_error) => println_error!("Server run error: ", server_error),
    }
}

pub async fn run() {
    plugin::server_manager::func::create_server_manage(create_server).await;
}
