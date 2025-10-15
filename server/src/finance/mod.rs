use serde::Deserialize;
use std::collections::HashMap;

const ALPHA_VANTAGE_BASE: &str = "https://www.alphavantage.co/query";
const SP500_TICKER: &str = "SPY";

#[derive(Debug, Clone)]
pub struct MarketData {
    pub ticker: String,
    pub current_price: f64,
    pub change_1m: f64,
    pub change_3m: f64,
    pub change_1y: f64,
    pub sector: String,
    pub market_cap: String,
    pub sp500_1m: f64,
    pub sp500_3m: f64,
    pub recent_news: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    #[serde(rename = "Global Quote")]
    global_quote: GlobalQuote,
}

#[derive(Debug, Deserialize)]
struct GlobalQuote {
    #[serde(rename = "05. price")]
    price: String,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

#[derive(Debug, Deserialize)]
struct OverviewResponse {
    #[serde(rename = "Sector")]
    sector: Option<String>,
    #[serde(rename = "MarketCapitalization")]
    market_cap: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TimeSeriesResponse {
    #[serde(rename = "Time Series (Daily)")]
    time_series: HashMap<String, DailyData>,
}

#[derive(Debug, Deserialize)]
struct DailyData {
    #[serde(rename = "4. close")]
    close: String,
}

#[derive(Debug, Deserialize)]
struct NewsResponse {
    feed: Vec<NewsItem>,
}

#[derive(Debug, Deserialize)]
struct NewsItem {
    title: String,
}

pub async fn fetch_market_data(
    ticker: &str,
    api_key: &str,
) -> Result<MarketData, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let (quote, overview, time_series, sp500_series, news) = tokio::join!(
        fetch_quote(&client, ticker, api_key),
        fetch_overview(&client, ticker, api_key),
        fetch_time_series(&client, ticker, api_key),
        fetch_time_series(&client, SP500_TICKER, api_key),
        fetch_news(&client, ticker, api_key),
    );

    let quote = quote?;
    let overview = overview?;
    let time_series = time_series?;
    let sp500_series = sp500_series?;
    let news = news.unwrap_or_default();

    let prices = extract_prices(&time_series);
    let sp500_prices = extract_prices(&sp500_series);

    let current_price = quote.price.parse::<f64>()?;
    let change_1m = calculate_change(&prices, 20)?; // ~1 month = 20 trading days
    let change_3m = calculate_change(&prices, 60)?; // ~3 months = 60 trading days
    let change_1y = calculate_change(&prices, 252)?; // ~1 year = 252 trading days

    let sp500_1m = calculate_change(&sp500_prices, 20)?;
    let sp500_3m = calculate_change(&sp500_prices, 60)?;

    Ok(MarketData {
        ticker: ticker.to_string(),
        current_price,
        change_1m,
        change_3m,
        change_1y,
        sector: overview.sector.unwrap_or_else(|| "Unknown".to_string()),
        market_cap: format_market_cap(&overview.market_cap.unwrap_or_default()),
        sp500_1m,
        sp500_3m,
        recent_news: news.into_iter().take(5).collect(),
    })
}

async fn fetch_quote(
    client: &reqwest::Client,
    ticker: &str,
    api_key: &str,
) -> Result<GlobalQuote, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "{}?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        ALPHA_VANTAGE_BASE, ticker, api_key
    );

    let resp: QuoteResponse = client.get(&url).send().await?.json().await?;
    Ok(resp.global_quote)
}

async fn fetch_overview(
    client: &reqwest::Client,
    ticker: &str,
    api_key: &str,
) -> Result<OverviewResponse, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "{}?function=OVERVIEW&symbol={}&apikey={}",
        ALPHA_VANTAGE_BASE, ticker, api_key
    );

    let resp: OverviewResponse = client.get(&url).send().await?.json().await?;
    Ok(resp)
}

async fn fetch_time_series(
    client: &reqwest::Client,
    ticker: &str,
    api_key: &str,
) -> Result<TimeSeriesResponse, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "{}?function=TIME_SERIES_DAILY&symbol={}&outputsize=full&apikey={}",
        ALPHA_VANTAGE_BASE, ticker, api_key
    );

    let resp: TimeSeriesResponse = client.get(&url).send().await?.json().await?;
    Ok(resp)
}

async fn fetch_news(
    client: &reqwest::Client,
    ticker: &str,
    api_key: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "{}?function=NEWS_SENTIMENT&tickers={}&apikey={}",
        ALPHA_VANTAGE_BASE, ticker, api_key
    );

    let resp: NewsResponse = client.get(&url).send().await?.json().await?;
    Ok(resp.feed.into_iter().map(|item| item.title).collect())
}

fn extract_prices(time_series: &TimeSeriesResponse) -> Vec<(String, f64)> {
    let mut prices: Vec<(String, f64)> = time_series
        .time_series
        .iter()
        .filter_map(|(date, data)| {
            data.close
                .parse::<f64>()
                .ok()
                .map(|price| (date.clone(), price))
        })
        .collect();

    prices.sort_by(|a, b| b.0.cmp(&a.0));
    prices
}

fn calculate_change(prices: &[(String, f64)], days_ago: usize) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    if prices.len() <= days_ago {
        return Err("Not enough historical data".into());
    }

    let current = prices[0].1;
    let past = prices[days_ago].1;

    Ok(((current - past) / past) * 100.0)
}

fn format_market_cap(cap: &str) -> String {
    if let Ok(cap_num) = cap.parse::<f64>() {
        if cap_num >= 1_000_000_000_000.0 {
            format!("{:.2}T", cap_num / 1_000_000_000_000.0)
        } else if cap_num >= 1_000_000_000.0 {
            format!("{:.2}B", cap_num / 1_000_000_000.0)
        } else if cap_num >= 1_000_000.0 {
            format!("{:.2}M", cap_num / 1_000_000.0)
        } else {
            cap.to_string()
        }
    } else {
        cap.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_change() {
        let prices = vec![
            ("2024-01-15".to_string(), 150.0),
            ("2024-01-14".to_string(), 148.0),
            ("2024-01-13".to_string(), 145.0),
            ("2024-01-12".to_string(), 140.0),
            ("2024-01-11".to_string(), 135.0),
            ("2024-01-10".to_string(), 130.0),
            ("2024-01-09".to_string(), 125.0),
            ("2024-01-08".to_string(), 120.0),
            ("2024-01-07".to_string(), 115.0),
            ("2024-01-06".to_string(), 110.0),
            ("2024-01-05".to_string(), 105.0),
            ("2024-01-04".to_string(), 100.0),
        ];

        let change = calculate_change(&prices, 11).unwrap();
        assert!((change - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_format_market_cap() {
        assert_eq!(format_market_cap("1500000000000"), "1.50T");
        assert_eq!(format_market_cap("2800000000"), "2.80B");
        assert_eq!(format_market_cap("500000000"), "500.00M");
    }
}