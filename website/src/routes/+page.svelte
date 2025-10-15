<script lang="ts">
    interface PerformanceAnalysis {
        recent_performance: string;
        key_drivers: string[];
        recent_pullback_reasons: string | null;
    }

    interface StockAnalysis {
        executive_summary: string;
        performance_analysis: PerformanceAnalysis;
        comparative_analysis: string;
        risk_factors: string[];
        opportunities: string[];
        key_takeaway: string;
    }

    let ticker = '';
    let loading = false;
    let analysis: StockAnalysis | null = null;
    let error = '';

    async function analyzeStock() {
        if (!ticker.trim()) return;

        loading = true;
        error = '';
        analysis = null;

        try {
            const response = await fetch('http://localhost:3000/api/analysis/stock', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ ticker: ticker.toUpperCase() })
            });

            if (!response.ok) throw new Error('Failed to fetch analysis');

            analysis = await response.json();
        } catch (e) {
            error = e instanceof Error ? e.message : 'An error occurred';
        } finally {
            loading = false;
        }
    }

    function handleKeyPress(e: KeyboardEvent) {
        if (e.key === 'Enter') analyzeStock();
    }
</script>

<main>
    <div class="container">
        <div class="input-section">
            <div class="input-wrapper">
                <input
                        type="text"
                        bind:value={ticker}
                        on:keypress={handleKeyPress}
                        placeholder="Enter ticker symbol"
                        class="ticker-input"
                        disabled={loading}
                />
                <button
                        on:click={analyzeStock}
                        class="analyze-btn"
                        disabled={loading || !ticker.trim()}
                >
                    {loading ? 'Analyzing...' : 'Analyze'}
                </button>
            </div>
            {#if error}
                <div class="error">{error}</div>
            {/if}
        </div>

        {#if analysis}
            <div class="results">
                <div class="summary-card glow">
                    <div class="card-label">Executive Summary</div>
                    <p class="summary-text">{analysis.executive_summary}</p>
                </div>

                <div class="grid">
                    <div class="card">
                        <div class="card-header">
                            <span class="icon">📊</span>
                            <h3>Recent Performance</h3>
                        </div>
                        <p class="card-content">{analysis.performance_analysis.recent_performance}</p>

                        <div class="subsection">
                            <div class="subsection-title">Key Drivers</div>
                            <ul class="bullet-list">
                                {#each analysis.performance_analysis.key_drivers as driver}
                                    <li>{driver}</li>
                                {/each}
                            </ul>
                        </div>

                        {#if analysis.performance_analysis.recent_pullback_reasons}
                            <div class="subsection">
                                <div class="subsection-title">Pullback Reasons</div>
                                <p class="card-content">{analysis.performance_analysis.recent_pullback_reasons}</p>
                            </div>
                        {/if}
                    </div>

                    <div class="card">
                        <div class="card-header">
                            <span class="icon">🔍</span>
                            <h3>Comparative Analysis</h3>
                        </div>
                        <p class="card-content">{analysis.comparative_analysis}</p>
                    </div>

                    <div class="card risk">
                        <div class="card-header">
                            <span class="icon">⚠️</span>
                            <h3>Risk Factors</h3>
                        </div>
                        <ul class="bullet-list">
                            {#each analysis.risk_factors as risk}
                                <li>{risk}</li>
                            {/each}
                        </ul>
                    </div>

                    <div class="card opportunity">
                        <div class="card-header">
                            <span class="icon">✨</span>
                            <h3>Opportunities</h3>
                        </div>
                        <ul class="bullet-list">
                            {#each analysis.opportunities as opportunity}
                                <li>{opportunity}</li>
                            {/each}
                        </ul>
                    </div>
                </div>

                <div class="takeaway-card">
                    <div class="takeaway-icon">💡</div>
                    <div>
                        <div class="takeaway-label">Key Takeaway</div>
                        <p class="takeaway-text">{analysis.key_takeaway}</p>
                    </div>
                </div>
            </div>
        {/if}
    </div>
</main>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        min-height: 100vh;
    }

    main {
        min-height: 100vh;
        padding: 3rem 1.5rem;
    }

    .container {
        max-width: 1200px;
        margin: 0 auto;
    }

    .input-section {
        margin-bottom: 3rem;
    }

    .input-wrapper {
        display: flex;
        gap: 1rem;
        max-width: 600px;
        margin: 0 auto;
    }

    .ticker-input {
        flex: 1;
        padding: 1.25rem 1.75rem;
        font-size: 1.125rem;
        border: none;
        border-radius: 16px;
        background: rgba(255, 255, 255, 0.95);
        backdrop-filter: blur(10px);
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
        transition: all 0.3s ease;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        font-weight: 600;
    }

    .ticker-input:focus {
        outline: none;
        box-shadow: 0 12px 48px rgba(0, 0, 0, 0.15);
        transform: translateY(-2px);
        background: rgba(255, 255, 255, 1);
    }

    .ticker-input::placeholder {
        color: #a0aec0;
        font-weight: 500;
    }

    .analyze-btn {
        padding: 1.25rem 2.5rem;
        font-size: 1.125rem;
        font-weight: 600;
        border: none;
        border-radius: 16px;
        background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
        color: white;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 8px 32px rgba(245, 87, 108, 0.3);
    }

    .analyze-btn:hover:not(:disabled) {
        transform: translateY(-2px);
        box-shadow: 0 12px 48px rgba(245, 87, 108, 0.4);
    }

    .analyze-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .error {
        text-align: center;
        color: #fff;
        margin-top: 1rem;
        padding: 1rem;
        background: rgba(239, 68, 68, 0.2);
        border-radius: 12px;
        backdrop-filter: blur(10px);
    }

    .results {
        animation: fadeIn 0.6s ease;
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

    .summary-card {
        background: rgba(255, 255, 255, 0.95);
        backdrop-filter: blur(10px);
        border-radius: 24px;
        padding: 2.5rem;
        margin-bottom: 2rem;
        box-shadow: 0 20px 60px rgba(0, 0, 0, 0.15);
    }

    .glow {
        position: relative;
        overflow: hidden;
    }

    .glow::before {
        content: '';
        position: absolute;
        top: -2px;
        left: -2px;
        right: -2px;
        bottom: -2px;
        background: linear-gradient(45deg, #f093fb, #f5576c, #4facfe, #00f2fe);
        border-radius: 24px;
        z-index: -1;
        opacity: 0.6;
        filter: blur(20px);
    }

    .card-label {
        font-size: 0.875rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.1em;
        color: #667eea;
        margin-bottom: 1rem;
    }

    .summary-text {
        font-size: 1.25rem;
        line-height: 1.8;
        color: #2d3748;
        margin: 0;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .card {
        background: rgba(255, 255, 255, 0.95);
        backdrop-filter: blur(10px);
        border-radius: 20px;
        padding: 2rem;
        box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
        transition: all 0.3s ease;
    }

    .card:hover {
        transform: translateY(-4px);
        box-shadow: 0 16px 56px rgba(0, 0, 0, 0.15);
    }

    .card-header {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        margin-bottom: 1.25rem;
    }

    .icon {
        font-size: 1.75rem;
    }

    .card-header h3 {
        margin: 0;
        font-size: 1.25rem;
        color: #2d3748;
        font-weight: 700;
    }

    .card-content {
        color: #4a5568;
        line-height: 1.7;
        margin: 0;
        font-size: 1rem;
    }

    .subsection {
        margin-top: 1.5rem;
    }

    .subsection-title {
        font-weight: 700;
        color: #4a5568;
        margin-bottom: 0.75rem;
        font-size: 0.95rem;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .bullet-list {
        margin: 0;
        padding-left: 1.25rem;
        color: #4a5568;
        line-height: 1.8;
    }

    .bullet-list li {
        margin-bottom: 0.75rem;
    }

    .bullet-list li:last-child {
        margin-bottom: 0;
    }

    .risk {
        border-left: 4px solid #f56565;
    }

    .opportunity {
        border-left: 4px solid #48bb78;
    }

    .takeaway-card {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        border-radius: 24px;
        padding: 2.5rem;
        display: flex;
        gap: 1.5rem;
        align-items: flex-start;
        box-shadow: 0 20px 60px rgba(102, 126, 234, 0.4);
    }

    .takeaway-icon {
        font-size: 3rem;
        flex-shrink: 0;
    }

    .takeaway-label {
        font-size: 0.875rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.1em;
        color: rgba(255, 255, 255, 0.9);
        margin-bottom: 0.75rem;
    }

    .takeaway-text {
        font-size: 1.125rem;
        line-height: 1.8;
        color: white;
        margin: 0;
    }

    @media (max-width: 768px) {
        main {
            padding: 2rem 1rem;
        }

        .input-wrapper {
            flex-direction: column;
        }

        .grid {
            grid-template-columns: 1fr;
        }

        .summary-card,
        .card,
        .takeaway-card {
            padding: 1.5rem;
        }

        .summary-text {
            font-size: 1.125rem;
        }
    }
</style>