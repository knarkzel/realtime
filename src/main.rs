use anyhow::Result;
use axum::{routing::get, Router};
use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tracing::info;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

fn on_connect(socket: SocketRef) {
    info!("Socket connected: {:?}", socket.id);

    socket.on("join", |socket: SocketRef, Data::<String>(room)| {
        info!("Received join: {room:?}");
        let _ = socket.leave_all();
        let _ = socket.join(room);
    });
    
    socket.on(
        "message",
        |socket: SocketRef, Data::<String>(data)| {
            info!("Received message: {data}");
            let _ = socket.within("default").emit("response", data);
        },
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    // Logging
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    // SocketIO
    let (socket, io) = SocketIo::new_layer();
    io.ns("/", on_connect);

    // Routes
    let app = Router::new().route("/", get(root)).layer(
        ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(socket),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    Ok(axum::serve(listener, app).await?)
}

async fn root() -> &'static str {
    "Hello, World!"
}
