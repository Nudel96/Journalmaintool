<script lang="ts">
	import { tradesApi, type Trade } from '$lib/api/trades';
	import { onMount } from 'svelte';

	let trades: Trade[] = [];
	let isLoading = true;
	let error = '';

	// Filters
	let filterSymbol = '';
	let filterDirection = '';
	let filterStatus = '';

	onMount(async () => {
		await loadTrades();
	});

	async function loadTrades() {
		isLoading = true;
		error = '';

		try {
			const filters: any = {};
			if (filterSymbol) filters.symbol = filterSymbol;
			if (filterDirection) filters.direction = filterDirection;
			if (filterStatus) filters.status = filterStatus;

			trades = await tradesApi.list(filters);
		} catch (err: any) {
			error = err.message || 'Failed to load trades';
		} finally {
			isLoading = false;
		}
	}

	async function deleteTrade(id: string) {
		if (!confirm('Are you sure you want to delete this trade?')) return;

		try {
			await tradesApi.delete(id);
			await loadTrades();
		} catch (err: any) {
			error = err.message || 'Failed to delete trade';
		}
	}

	function formatDate(dateString: string) {
		return new Date(dateString).toLocaleString();
	}

	function formatPrice(price: number) {
		return price.toFixed(5);
	}

	function formatPnL(pnl: number | undefined) {
		if (pnl === undefined) return 'N/A';
		return pnl >= 0 ? `+$${pnl.toFixed(2)}` : `-$${Math.abs(pnl).toFixed(2)}`;
	}

	function getPnLColor(pnl: number | undefined) {
		if (pnl === undefined) return 'text-text-muted';
		return pnl >= 0 ? 'text-success' : 'text-danger';
	}
</script>

<div class="min-h-screen p-8">
	<div class="vignette"></div>
	
	<div class="max-w-7xl mx-auto relative z-10">
		<div class="flex justify-between items-center mb-8">
			<div>
				<h1 class="text-4xl font-bold mb-2">Trades</h1>
				<p class="text-text-muted">Manage your trading history</p>
			</div>
			<a href="/trades/new" class="btn btn-primary">+ New Trade</a>
		</div>

		<!-- Filters -->
		<div class="card p-6 mb-6">
			<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
				<div>
					<label class="block text-sm font-medium mb-2">Symbol</label>
					<div class="input-field">
						<input
							type="text"
							bind:value={filterSymbol}
							on:change={loadTrades}
							placeholder="EURUSD"
						/>
					</div>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">Direction</label>
					<div class="input-field">
						<select bind:value={filterDirection} on:change={loadTrades} class="flex-1 bg-transparent border-0 outline-none">
							<option value="">All</option>
							<option value="long">Long</option>
							<option value="short">Short</option>
						</select>
					</div>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">Status</label>
					<div class="input-field">
						<select bind:value={filterStatus} on:change={loadTrades} class="flex-1 bg-transparent border-0 outline-none">
							<option value="">All</option>
							<option value="open">Open</option>
							<option value="closed">Closed</option>
						</select>
					</div>
				</div>

				<div class="flex items-end">
					<button on:click={() => { filterSymbol = ''; filterDirection = ''; filterStatus = ''; loadTrades(); }} class="btn btn-ghost w-full">
						Clear Filters
					</button>
				</div>
			</div>
		</div>

		{#if error}
			<div class="bg-danger/10 border border-danger/50 rounded-button p-4 mb-6">
				<p class="text-danger text-sm">{error}</p>
			</div>
		{/if}

		<!-- Trades Table -->
		<div class="card overflow-hidden">
			{#if isLoading}
				<div class="p-12 text-center">
					<p class="text-text-muted">Loading trades...</p>
				</div>
			{:else if trades.length === 0}
				<div class="p-12 text-center">
					<p class="text-text-muted mb-4">No trades found</p>
					<a href="/trades/new" class="btn btn-primary">Create your first trade</a>
				</div>
			{:else}
				<div class="overflow-x-auto">
					<table class="w-full">
						<thead class="bg-white/5 border-b border-white/10">
							<tr>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider">Date</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider">Symbol</th>
								<th class="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider">Direction</th>
								<th class="px-6 py-3 text-right text-xs font-medium uppercase tracking-wider">Entry</th>
								<th class="px-6 py-3 text-right text-xs font-medium uppercase tracking-wider">Exit</th>
								<th class="px-6 py-3 text-right text-xs font-medium uppercase tracking-wider">P&L</th>
								<th class="px-6 py-3 text-right text-xs font-medium uppercase tracking-wider">P&L %</th>
								<th class="px-6 py-3 text-center text-xs font-medium uppercase tracking-wider">Status</th>
								<th class="px-6 py-3 text-right text-xs font-medium uppercase tracking-wider">Actions</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-white/10">
							{#each trades as trade}
								<tr class="hover:bg-white/5 transition-colors">
									<td class="px-6 py-4 whitespace-nowrap text-sm">
										{formatDate(trade.entry_time)}
									</td>
									<td class="px-6 py-4 whitespace-nowrap font-medium">
										{trade.symbol}
									</td>
									<td class="px-6 py-4 whitespace-nowrap">
										<span class="px-2 py-1 rounded text-xs {trade.direction === 'long' ? 'bg-success/20 text-success' : 'bg-danger/20 text-danger'}">
											{trade.direction.toUpperCase()}
										</span>
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-right text-sm">
										{formatPrice(trade.entry_price)}
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-right text-sm">
										{trade.exit_price ? formatPrice(trade.exit_price) : '-'}
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-right font-medium {getPnLColor(trade.pnl)}">
										{formatPnL(trade.pnl)}
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-right {getPnLColor(trade.pnl)}">
										{trade.pnl_percentage ? `${trade.pnl_percentage.toFixed(2)}%` : '-'}
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-center">
										<span class="px-2 py-1 rounded text-xs {trade.status === 'closed' ? 'bg-white/10' : 'bg-brand/20 text-brand'}">
											{trade.status}
										</span>
									</td>
									<td class="px-6 py-4 whitespace-nowrap text-right text-sm">
										<a href="/trades/{trade.id}" class="text-brand hover:text-accent mr-3">View</a>
										<button on:click={() => deleteTrade(trade.id)} class="text-danger hover:text-red-400">Delete</button>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>

				<div class="p-4 border-t border-white/10 text-center text-sm text-text-muted">
					Showing {trades.length} trade{trades.length !== 1 ? 's' : ''}
				</div>
			{/if}
		</div>
	</div>
</div>

