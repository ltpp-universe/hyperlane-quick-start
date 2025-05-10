use super::*;
use hyperlane::{
    dashmap::DashSet,
    tokio::sync::broadcast::{Receiver, Sender, channel},
};
use std::{net::SocketAddr, sync::OnceLock};

type Message = Vec<u8>;
static BROADCAST_CHANNEL: OnceLock<Sender<Message>> = OnceLock::new();
static REGISTERED_CLIENTS: OnceLock<DashSet<SocketAddr>> = OnceLock::new();

fn ensure_broadcast_channel() -> Sender<Message> {
    BROADCAST_CHANNEL
        .get_or_init(|| {
            let (sender, _) = channel(1);
            sender
        })
        .clone()
}

fn ensure_client_registry() -> &'static DashSet<SocketAddr> {
    REGISTERED_CLIENTS.get_or_init(DashSet::new)
}

fn spawn_pipe_listener(ctx: Context, addr: SocketAddr) {
    let receiver: Receiver<Vec<u8>> = ensure_broadcast_channel().subscribe();
    tokio::spawn(async move {
        listen_loop(ctx, receiver, addr).await;
    });
}

async fn listen_loop(ctx: Context, mut receiver: Receiver<Message>, addr: SocketAddr) {
    while let Ok(msg) = receiver.recv().await {
        if ctx.send_response_body(msg).await.is_err() || ctx.flush().await.is_err() {
            break;
        }
    }
    ensure_client_registry().remove(&addr);
}

pub async fn handle(ctx: Context) {
    let addr: SocketAddr = ctx.get_socket_addr().await.unwrap();
    let sender: Sender<Vec<u8>> = ensure_broadcast_channel();
    let registry: &DashSet<SocketAddr> = ensure_client_registry();
    if registry.insert(addr.clone()) {
        spawn_pipe_listener(ctx.clone(), addr.clone());
    }
    let request_body: Vec<u8> = ctx.get_request_body().await;
    let _ = sender.send(request_body.clone());
}
