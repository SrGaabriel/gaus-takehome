# Stock Analyzer

An intuitive AI-powered tool that analyzes stock performance that carefully tracks context, trends, and volatility to deliver actionable insights.

Considered factors:
* Current price
* Change in price over time (1 month, 3 months and 1 year)
* Product sector
* Market cap
* S&P 500 (for comparison to market)
* Recent news
* LLM's own knowledge

What makes it unique:
1. Explains why the stock is performing the way it is
2. Compares performance against benchmarks with context
3. Identifies specific risks and opportunities
4. Provides actionable key takeaways

# Stack

**Backend:** Rust + Axum
**Frontend:** Svelte + TypeScript
**AI:** Claude
**Data:** Alpha Vantage

# How to run

1. Install Rust
2. Install Python
3. Install Deno
4. Copy the .env.example file to .env and fill in the values
> Note: You can get the free Alpha Vantage API key from https://www.alphavantage.co/support/#api-key
5. Run `py run_dev.py`

# What I would work on next

1. Add caching
2. Add more types of analysis (architecture supports that)
3. Make frontend prettier with better animations and design
4. Comparison tool
5. Live chat with AI (maybe for premium users only?) to discuss analysis
6. Make it paid (money doesn't grow on trees kkk)

# Challenges

Not many but these were the most annoying:

1. Synchronizing the .env for both the website and server, that's why the python script exists to avoid other uglier workarounds
2. Getting around Alpha Vantage's rate limit (had to change networks all the time)
3. Making a half-decent frontend (had to rely on Claude 😢)

# Thanks

Daniel for the opportunity!