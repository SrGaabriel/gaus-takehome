mod routing;
mod util;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::app::TakehomeApp;

pub async fn serve(app: TakehomeApp) {
    let listener = TcpListener::bind(&app.config.rest_addr)
        .await
        .expect("Failed to bind rest API address");
    let router = routing::router(app);
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>()
    )
        .await
        .expect("Failed to run server");
}