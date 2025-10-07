<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { isAuthenticated, currentUser } from '$lib/stores/auth';
  import { getTrades, type Trade } from '$lib/api/trades';

  let trades: Trade[] = [];
  let loading = true;
  let error = '';

  // Stats
  let totalTrades = 0;
  let winningTrades = 0;
  let losingTrades = 0;
  let winRate = 0;
  let totalPnL = 0;
  let avgWin = 0;
  let avgLoss = 0;
  let profitFactor = 0;

  onMount(async () => {
    if (!$isAuthenticated) {
      goto('/login');
      return;
    }

    try {
      trades = await getTrades();
      calculateStats();
    } catch (err: any) {
      error = err.message || 'Failed to load trades';
    } finally {
      loading = false;
    }
  });

  function calculateStats() {
    totalTrades = trades.length;
    
    if (totalTrades === 0) {
      return;
    }

    let totalWinAmount = 0;
    let totalLossAmount = 0;

    trades.forEach(trade => {
      const pnl = trade.pnl || 0;
      totalPnL += pnl;

      if (pnl > 0) {
        winningTrades++;
        totalWinAmount += pnl;
      } else if (pnl < 0) {
        losingTrades++;
        totalLossAmount += Math.abs(pnl);
      }
    });

    winRate = totalTrades > 0 ? (winningTrades / totalTrades) * 100 : 0;
    avgWin = winningTrades > 0 ? totalWinAmount / winningTrades : 0;
    avgLoss = losingTrades > 0 ? totalLossAmount / losingTrades : 0;
    profitFactor = totalLossAmount > 0 ? totalWinAmount / totalLossAmount : 0;
  }

  function formatCurrency(value: number): string {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 2
    }).format(value);
  }

  function formatPercent(value: number): string {
    return `${value.toFixed(1)}%`;
  }
</script>

<div class="max-w-7xl mx-auto px-4 py-8">
  <!-- Header -->
  <div class="mb-8">
    <h1 class="text-3xl font-bold mb-2">Dashboard</h1>
    <p class="text-muted">Welcome back, {$currentUser?.name || 'Trader'}!</p>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-16">
      <div class="text-center">
        <div class="inline-block animate-spin rounded-full h-12 w-12 border-4 border-brand border-t-transparent mb-4"></div>
        <p class="text-muted">Loading your dashboard...</p>
      </div>
    </div>
  {:else if error}
    <div class="card p-6 border-danger/50 bg-danger/10">
      <p class="text-danger">{error}</p>
    </div>
  {:else}
    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <!-- Total Trades -->
      <div class="stat-card p-6">
        <div class="text-sm text-muted mb-1">Total Trades</div>
        <div class="text-3xl font-bold">{totalTrades}</div>
      </div>

      <!-- Win Rate -->
      <div class="stat-card p-6">
        <div class="text-sm text-muted mb-1">Win Rate</div>
        <div class="text-3xl font-bold" class:text-accent={winRate >= 50} class:text-danger={winRate < 50}>
          {formatPercent(winRate)}
        </div>
        <div class="text-xs text-muted mt-1">
          {winningTrades}W / {losingTrades}L
        </div>
      </div>

      <!-- Total P&L -->
      <div class="stat-card p-6">
        <div class="text-sm text-muted mb-1">Total P&L</div>
        <div class="text-3xl font-bold" class:text-accent={totalPnL >= 0} class:text-danger={totalPnL < 0}>
          {formatCurrency(totalPnL)}
        </div>
      </div>

      <!-- Profit Factor -->
      <div class="stat-card p-6">
        <div class="text-sm text-muted mb-1">Profit Factor</div>
        <div class="text-3xl font-bold" class:text-accent={profitFactor >= 1} class:text-danger={profitFactor < 1}>
          {profitFactor.toFixed(2)}
        </div>
      </div>
    </div>

    <!-- Additional Stats -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
      <!-- Average Win/Loss -->
      <div class="card p-6">
        <h3 class="text-lg font-bold mb-4">Average Win/Loss</h3>
        <div class="space-y-3">
          <div class="flex justify-between items-center">
            <span class="text-muted">Avg Win:</span>
            <span class="font-bold text-accent">{formatCurrency(avgWin)}</span>
          </div>
          <div class="flex justify-between items-center">
            <span class="text-muted">Avg Loss:</span>
            <span class="font-bold text-danger">{formatCurrency(avgLoss)}</span>
          </div>
          <div class="flex justify-between items-center pt-3 border-t border-white/10">
            <span class="text-muted">Risk/Reward:</span>
            <span class="font-bold">
              {avgLoss > 0 ? (avgWin / avgLoss).toFixed(2) : 'N/A'}
            </span>
          </div>
        </div>
      </div>

      <!-- Quick Actions -->
      <div class="card p-6">
        <h3 class="text-lg font-bold mb-4">Quick Actions</h3>
        <div class="space-y-3">
          <a href="/trades/new" class="btn btn-primary w-full">
            + New Trade
          </a>
          <a href="/trades" class="btn btn-ghost w-full">
            View All Trades
          </a>
        </div>
      </div>
    </div>

    <!-- Recent Trades -->
    {#if trades.length > 0}
      <div class="card p-6">
        <div class="flex justify-between items-center mb-4">
          <h3 class="text-lg font-bold">Recent Trades</h3>
          <a href="/trades" class="text-sm text-brand hover:text-accent transition-colors">
            View All →
          </a>
        </div>

        <div class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr class="border-b border-white/10">
                <th class="text-left py-3 px-2 text-sm font-semibold text-muted">Symbol</th>
                <th class="text-left py-3 px-2 text-sm font-semibold text-muted">Direction</th>
                <th class="text-right py-3 px-2 text-sm font-semibold text-muted">Entry</th>
                <th class="text-right py-3 px-2 text-sm font-semibold text-muted">Exit</th>
                <th class="text-right py-3 px-2 text-sm font-semibold text-muted">P&L</th>
                <th class="text-left py-3 px-2 text-sm font-semibold text-muted">Date</th>
              </tr>
            </thead>
            <tbody>
              {#each trades.slice(0, 5) as trade}
                <tr class="border-b border-white/5 hover:bg-white/5 transition-colors">
                  <td class="py-3 px-2 font-medium">{trade.symbol}</td>
                  <td class="py-3 px-2">
                    <span class="inline-flex items-center px-2 py-1 rounded text-xs font-medium"
                          class:bg-accent/20={trade.direction === 'long'}
                          class:text-accent={trade.direction === 'long'}
                          class:bg-danger/20={trade.direction === 'short'}
                          class:text-danger={trade.direction === 'short'}>
                      {trade.direction.toUpperCase()}
                    </span>
                  </td>
                  <td class="py-3 px-2 text-right">{trade.entry_price.toFixed(5)}</td>
                  <td class="py-3 px-2 text-right">{trade.exit_price?.toFixed(5) || 'N/A'}</td>
                  <td class="py-3 px-2 text-right font-bold"
                      class:text-accent={trade.pnl && trade.pnl > 0}
                      class:text-danger={trade.pnl && trade.pnl < 0}>
                    {trade.pnl ? formatCurrency(trade.pnl) : 'N/A'}
                  </td>
                  <td class="py-3 px-2 text-sm text-muted">
                    {new Date(trade.entry_time).toLocaleDateString()}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {:else}
      <div class="card p-12 text-center">
        <div class="text-6xl mb-4">📊</div>
        <h3 class="text-xl font-bold mb-2">No Trades Yet</h3>
        <p class="text-muted mb-6">Start tracking your trading performance by adding your first trade.</p>
        <a href="/trades/new" class="btn btn-primary inline-flex">
          + Add Your First Trade
        </a>
      </div>
    {/if}
  {/if}
</div>

