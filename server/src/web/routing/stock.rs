use crate::app::TakehomeApp;
use crate::client::stock::{analyze_stock, StockAnalysis};
use crate::finance::fetch_market_data;
use crate::web::util::valid_json::ValidJson;
use crate::web::util::{api_error, ok, ApiResponse};
use axum::debug_handler;
use axum::extract::State;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, garde::Validate)]
pub struct StockInquiryRequest {
    #[garde(pattern(r"^[A-Z0-9]{1,7}(\.[A-Z]{1,3})?$"))]
    pub ticker: String
}

#[debug_handler]
pub async fn stock_analysis_handler(
    State(app): State<TakehomeApp>,
    ValidJson(payload): ValidJson<StockInquiryRequest>
) -> ApiResponse<StockAnalysis> {
    let market_data_result = fetch_market_data(
        &payload.ticker,
        app.config.alpha_vantage_api_key.as_str()
    ).await;

    match market_data_result {
        Ok(market_data) => {
            let stock_analysis = analyze_stock(
                app.claude.as_ref(),
                app.config.claude_model.as_str(),
                &market_data,
            )
                .await
                .expect("Failed to analyze stock");
            ok(stock_analysis)
        }
        Err(err) => {
            // todo: remove the err from the string since it's just for debugging purposes
            api_error(StatusCode::NOT_FOUND, &format!("Failed to fetch market data: {}", err))
        }
    }
}