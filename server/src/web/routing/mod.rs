mod stock;

use axum::Router;
use axum::routing::post;
use crate::app::TakehomeApp;
use crate::web::routing::stock::stock_analysis_handler;

pub fn router(state: TakehomeApp) -> Router {
    Router::new()
        .nest("/api/analysis", Router::new()
            .route("/stock", post(stock_analysis_handler))
        )
        .with_state(state)
}