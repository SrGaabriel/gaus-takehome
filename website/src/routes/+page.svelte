<script lang="ts">
    import {PUBLIC_REST_ADDR} from '$env/static/public';

    interface StockAnalysis {
        executive_summary: string;
        performance_analysis: {
            recent_performance: string;
            key_drivers: string[];
            recent_pullback_reasons: string | null;
        };
        comparative_analysis: string;
        risk_factors: string[];
        opportunities: string[];
        key_takeaway: string;
    }

    let ticker = $state('');
    let analysis = $state<StockAnalysis | null>(null);
    let loading = $state(false);
    let error = $state('');

    async function handleSubmit(e: Event) {
        e.preventDefault();
        if (!ticker.trim()) return;

        loading = true;
        error = '';
        analysis = null;

        try {
            const response = await fetch(`http://${PUBLIC_REST_ADDR}/api/analysis/stock`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ ticker: ticker.toUpperCase() }),
            });

            if (!response.ok) {
                throw new Error('Failed to fetch analysis');
            }

            analysis = await response.json();
        } catch (err) {
            error = err instanceof Error ? err.message : 'An error occurred';
        } finally {
            loading = false;
        }
    }

    function reset() {
        ticker = '';
        analysis = null;
        error = '';
    }
</script>

<div class="container">
    {#if !analysis}
        <div class="input-section">
            <h1 class="title">Stock Analysis</h1>
            <p class="subtitle">Enter a ticker symbol to get comprehensive analysis</p>

            <form onsubmit={handleSubmit}>
                <div class="input-wrapper">
                    <input
                            type="text"
                            bind:value={ticker}
                            placeholder="AAPL"
                            class="ticker-input"
                            disabled={loading}
                    />
                    <button type="submit" class="submit-button" disabled={loading || !ticker.trim()}>
                        {loading ? 'Analyzing...' : 'Analyze'}
                    </button>
                </div>
            </form>

            {#if error}
                <div class="error">{error}</div>
            {/if}
        </div>
    {:else}
        <div class="results">
            <div class="results-header">
                <div>
                    <div class="ticker-badge">{ticker.toUpperCase()}</div>
                    <h2 class="results-title">Stock Analysis Report</h2>
                </div>
                <button onclick={reset} class="reset-button">New Analysis</button>
            </div>

            <div class="card executive-card">
                <div class="card-header">
                    <div class="accent-bar executive"></div>
                    <h3>Executive Summary</h3>
                </div>
                <p class="card-content">{analysis.executive_summary}</p>
            </div>

            <div class="grid">
                <div class="card">
                    <div class="card-header">
                        <div class="accent-bar performance"></div>
                        <h3>Performance Analysis</h3>
                    </div>
                    {#if analysis.performance_analysis}
                        <div class="card-content">
                            <p class="performance-text">{analysis.performance_analysis.recent_performance}</p>

                            {#if analysis.performance_analysis.key_drivers && analysis.performance_analysis.key_drivers.length > 0}
                                <h4 class="subsection-title">Key Drivers</h4>
                                <ul class="list">
                                    {#each analysis.performance_analysis.key_drivers as driver ("driver-" + driver)}
                                        <li>{driver}</li>
                                    {/each}
                                </ul>
                            {/if}

                            {#if analysis.performance_analysis.recent_pullback_reasons}
                                <h4 class="subsection-title">Recent Pullback Reasons</h4>
                                <p class="performance-text">{analysis.performance_analysis.recent_pullback_reasons}</p>
                            {/if}
                        </div>
                    {/if}
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="accent-bar comparative"></div>
                        <h3>Comparative Analysis</h3>
                    </div>
                    <p class="card-content">{analysis.comparative_analysis}</p>
                </div>
            </div>

            <div class="grid">
                <div class="card">
                    <div class="card-header">
                        <div class="accent-bar risk"></div>
                        <h3>Risk Factors</h3>
                    </div>
                    <ul class="list">
                        {#each analysis.risk_factors as risk ("risk-" + risk)}
                            <li>{risk}</li>
                        {/each}
                    </ul>
                </div>

                <div class="card">
                    <div class="card-header">
                        <div class="accent-bar opportunity"></div>
                        <h3>Opportunities</h3>
                    </div>
                    <ul class="list">
                        {#each analysis.opportunities as opportunity ("opportunity-" + opportunity)}
                            <li>{opportunity}</li>
                        {/each}
                    </ul>
                </div>
            </div>

            <div class="card takeaway-card">
                <div class="card-header">
                    <div class="accent-bar takeaway"></div>
                    <h3>Key Takeaway</h3>
                </div>
                <p class="card-content takeaway-text">{analysis.key_takeaway}</p>
            </div>
        </div>
    {/if}
</div>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
        background: linear-gradient(135deg, #faf8f5 0%, #f5f1ea 100%);
        min-height: 100vh;
    }

    .container {
        max-width: 1200px;
        margin: 0 auto;
        padding: 60px 24px;
        min-height: 100vh;
    }

    .input-section {
        max-width: 600px;
        margin: 0 auto;
        text-align: center;
        padding-top: 120px;
    }

    .title {
        font-family: Georgia, 'Times New Roman', serif;
        font-size: 56px;
        font-weight: 400;
        color: #1a1a1a;
        margin: 0 0 16px 0;
        letter-spacing: -0.02em;
    }

    .subtitle {
        font-size: 18px;
        color: #666;
        margin: 0 0 48px 0;
        line-height: 1.6;
    }

    .input-wrapper {
        display: flex;
        gap: 12px;
        margin-bottom: 24px;
    }

    .ticker-input {
        flex: 1;
        padding: 20px 24px;
        font-size: 24px;
        font-weight: 600;
        text-transform: uppercase;
        border: 2px solid #e5e1d8;
        border-radius: 12px;
        background: white;
        color: #1a1a1a;
        transition: all 0.2s;
        letter-spacing: 0.05em;
    }

    .ticker-input:focus {
        outline: none;
        border-color: #3b82f6;
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
    }

    .ticker-input::placeholder {
        color: #ccc;
        font-weight: 400;
    }

    .ticker-input:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .submit-button {
        padding: 20px 40px;
        font-size: 16px;
        font-weight: 600;
        color: white;
        background: #3b82f6;
        border: none;
        border-radius: 12px;
        cursor: pointer;
        transition: all 0.2s;
        white-space: nowrap;
    }

    .submit-button:hover:not(:disabled) {
        background: #2563eb;
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
    }

    .submit-button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
        transform: none;
    }

    .error {
        color: #dc2626;
        padding: 16px;
        background: #fee;
        border-radius: 8px;
        margin-top: 16px;
    }

    .results {
        animation: fadeIn 0.5s ease-in;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    .results-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 32px;
        flex-wrap: wrap;
        gap: 16px;
    }

    .ticker-badge {
        display: inline-block;
        padding: 8px 16px;
        background: #3b82f6;
        color: white;
        border-radius: 8px;
        font-weight: 700;
        font-size: 14px;
        letter-spacing: 0.05em;
        margin-bottom: 8px;
    }

    .results-title {
        font-family: Georgia, 'Times New Roman', serif;
        font-size: 36px;
        font-weight: 400;
        color: #1a1a1a;
        margin: 0;
        letter-spacing: -0.02em;
    }

    .reset-button {
        padding: 12px 24px;
        font-size: 14px;
        font-weight: 600;
        color: #666;
        background: white;
        border: 2px solid #e5e1d8;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s;
    }

    .reset-button:hover {
        border-color: #3b82f6;
        color: #3b82f6;
    }

    .card {
        background: white;
        border-radius: 16px;
        padding: 32px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
        margin-bottom: 24px;
        transition: all 0.3s;
    }

    .card:hover {
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
    }

    .executive-card {
        border-left: 4px solid #3b82f6;
    }

    .takeaway-card {
        border-left: 4px solid #8b5cf6;
        background: linear-gradient(135deg, #faf8ff 0%, #f5f3ff 100%);
    }

    .card-header {
        display: flex;
        align-items: center;
        gap: 12px;
        margin-bottom: 16px;
    }

    .accent-bar {
        width: 4px;
        height: 24px;
        border-radius: 2px;
    }

    .accent-bar.executive {
        background: #3b82f6;
    }

    .accent-bar.performance {
        background: #10b981;
    }

    .accent-bar.comparative {
        background: #f59e0b;
    }

    .accent-bar.risk {
        background: #ef4444;
    }

    .accent-bar.opportunity {
        background: #06b6d4;
    }

    .accent-bar.takeaway {
        background: #8b5cf6;
    }

    .card-header h3 {
        font-family: Georgia, 'Times New Roman', serif;
        font-size: 20px;
        font-weight: 600;
        color: #1a1a1a;
        margin: 0;
    }

    .card-content {
        font-size: 15px;
        line-height: 1.7;
        color: #4a4a4a;
        margin: 0;
    }

    .takeaway-text {
        font-size: 17px;
        font-weight: 500;
        color: #1a1a1a;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 24px;
        margin-bottom: 24px;
    }

    .list {
        margin: 0;
        padding-left: 20px;
        list-style: none;
    }

    .list li {
        font-size: 15px;
        line-height: 1.7;
        color: #4a4a4a;
        margin-bottom: 12px;
        padding-left: 12px;
        position: relative;
    }

    .list li::before {
        content: '•';
        position: absolute;
        left: 0;
        color: #3b82f6;
        font-weight: bold;
    }

    @media (max-width: 768px) {
        .container {
            padding: 40px 16px;
        }

        .input-section {
            padding-top: 60px;
        }

        .title {
            font-size: 40px;
        }

        .input-wrapper {
            flex-direction: column;
        }

        .ticker-input {
            font-size: 20px;
        }

        .results-title {
            font-size: 28px;
        }

        .card {
            padding: 24px;
        }

        .grid {
            grid-template-columns: 1fr;
        }
    }
</style>
