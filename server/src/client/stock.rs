use claude_client::claude::ClaudeClient;
use serde::{Deserialize, Serialize};
use crate::finance::MarketData;

#[derive(Debug, Serialize, Deserialize)]
pub struct StockAnalysis {
    pub executive_summary: String,
    pub performance_analysis: PerformanceAnalysis,
    pub comparative_analysis: String,
    pub risk_factors: Vec<String>,
    pub opportunities: Vec<String>,
    pub key_takeaway: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub recent_performance: String,
    pub key_drivers: Vec<String>,
    pub recent_pullback_reasons: Option<Vec<String>>,
}

pub async fn analyze_stock(
    client: &ClaudeClient,
    model: &str,
    market_data: &MarketData,
) -> Result<StockAnalysis, Box<dyn std::error::Error>> {
    let context = build_context(market_data);
    let prompt = build_analysis_prompt(&market_data.ticker, &context);

    let analysis = super::send_json_prompt::<StockAnalysis>(
        client,
        Some(model),
        None,
        &prompt,
    )
        .await
        .expect("Failed to build Claude request");

    Ok(analysis)
}

fn build_context(data: &MarketData) -> String {
    format!(
        r#"Stock: {}
Current Price: ${}
1-month change: {}%
3-month change: {}%
1-year change: {}%
Sector: {}
Market Cap: {}

S&P 500 performance (same period):
1-month: {}%
3-month: {}%

Recent news headlines:
{}
"#,
        data.ticker,
        data.current_price,
        data.change_1m,
        data.change_3m,
        data.change_1y,
        data.sector,
        data.market_cap,
        data.sp500_1m,
        data.sp500_3m,
        data.recent_news.iter()
            .enumerate()
            .map(|(i, news)| format!("{}. {}", i + 1, news))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn build_analysis_prompt(ticker: &str, context: &str) -> String {
    format!(
        r#"You are an experienced financial analyst. A client asked you about {ticker}.

Here's the data you have:
{context}

Provide a professional analysis in JSON format with this EXACT structure:

{{
  "executive_summary": "2-3 sentence big picture summary",
  "performance_analysis": {{
    "recent_performance": "Overview of recent performance",
    "key_drivers": ["Driver 1", "Driver 2", "Driver 3"],
    "recent_pullback_reasons": ["Reason 1", "Reason 2"] // optional, only if there was a pullback
  }},
  "comparative_analysis": "Comparison to S&P 500 and sector, explaining WHY the divergence exists",
  "risk_factors": [
    "Specific risk 1 with concrete impact",
    "Specific risk 2 with concrete impact",
    "Specific risk 3 with concrete impact"
  ],
  "opportunities": [
    "Specific opportunity 1 with catalyst",
    "Specific opportunity 2 with catalyst",
    "Specific opportunity 3 with catalyst"
  ],
  "key_takeaway": "One actionable insight an investor should know - be specific about what to watch for"
}}

CRITICAL RULES:
- Be insightful and specific, not generic
- Connect price movements to actual events (earnings, news, market trends)
- Explain WHY things happened, not just WHAT happened
- Avoid boilerplate disclaimers like "diversify your portfolio"
- Risk factors should be specific with concrete impacts, not vague warnings
- Opportunities should identify specific catalysts
- Key takeaway should be actionable (e.g., "Watch for X in next earnings")

Return ONLY the JSON, no additional text."#,
        ticker = ticker,
        context = context
    )
}