mod stock;

use axum::Router;
use axum::routing::post;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use crate::app::TakehomeApp;
use crate::web::routing::stock::stock_analysis_handler;

pub fn router(state: TakehomeApp) -> Router {
    Router::new()
        .nest("/api/analysis", Router::new()
            .route("/stock", post(stock_analysis_handler))
        )
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true))
                )
        )
        .with_state(state)
}