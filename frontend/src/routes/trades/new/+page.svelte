<script lang="ts">
	import { tradesApi } from '$lib/api/trades';
	import { goto } from '$app/navigation';

	let symbol = '';
	let direction = 'long';
	let entry_price = '';
	let exit_price = '';
	let quantity = '';
	let entry_time = '';
	let exit_time = '';
	let fees = '';
	let notes = '';
	let setup_type = '';
	let tags: string[] = [];
	let mistakes: string[] = [];
	let emotions: string[] = [];

	let error = '';
	let isLoading = false;

	const setupTypes = [
		'Support/Resistance',
		'Trendline Break',
		'Moving Average Cross',
		'Chart Pattern',
		'Fibonacci',
		'Supply/Demand',
		'Other'
	];

	const availableTags = ['Breakout', 'Reversal', 'Trend-Following', 'Scalp', 'Swing', 'Day-Trade'];
	const availableMistakes = ['FOMO', 'No Stop-Loss', 'Overtrading', 'Revenge Trading', 'Too Early', 'Too Late'];
	const availableEmotions = ['Confident', 'Fearful', 'Greedy', 'Calm', 'Anxious', 'Excited'];

	function toggleArray(arr: string[], item: string) {
		if (arr.includes(item)) {
			return arr.filter(i => i !== item);
		} else {
			return [...arr, item];
		}
	}

	async function handleSubmit() {
		error = '';
		isLoading = true;

		try {
			await tradesApi.create({
				symbol: symbol.toUpperCase(),
				direction,
				entry_price: parseFloat(entry_price),
				exit_price: exit_price ? parseFloat(exit_price) : undefined,
				quantity: parseFloat(quantity),
				entry_time: new Date(entry_time).toISOString(),
				exit_time: exit_time ? new Date(exit_time).toISOString() : undefined,
				fees: fees ? parseFloat(fees) : 0,
				notes: notes || undefined,
				tags: tags.length > 0 ? tags : undefined,
				setup_type: setup_type || undefined,
				mistakes: mistakes.length > 0 ? mistakes : undefined,
				emotions: emotions.length > 0 ? emotions : undefined
			});

			goto('/trades');
		} catch (err: any) {
			error = err.message || 'Failed to create trade';
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="min-h-screen p-8">
	<div class="vignette"></div>
	
	<div class="max-w-4xl mx-auto relative z-10">
		<div class="mb-8">
			<h1 class="text-4xl font-bold mb-2">New Trade</h1>
			<p class="text-text-muted">Record your trading activity</p>
		</div>

		{#if error}
			<div class="bg-danger/10 border border-danger/50 rounded-button p-4 mb-6">
				<p class="text-danger text-sm">{error}</p>
			</div>
		{/if}

		<form on:submit|preventDefault={handleSubmit} class="card p-8 space-y-6">
			<!-- Symbol & Direction -->
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div>
					<label for="symbol" class="block text-sm font-medium mb-2">Symbol *</label>
					<div class="input-field">
						<input id="symbol" type="text" bind:value={symbol} placeholder="EURUSD" required />
					</div>
				</div>

				<div>
					<label class="block text-sm font-medium mb-2">Direction *</label>
					<div class="flex gap-4">
						<label class="flex items-center gap-2 cursor-pointer">
							<input type="radio" bind:group={direction} value="long" class="text-accent" />
							<span>Long</span>
						</label>
						<label class="flex items-center gap-2 cursor-pointer">
							<input type="radio" bind:group={direction} value="short" class="text-accent" />
							<span>Short</span>
						</label>
					</div>
				</div>
			</div>

			<!-- Prices & Quantity -->
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<div>
					<label for="entry_price" class="block text-sm font-medium mb-2">Entry Price *</label>
					<div class="input-field">
						<input id="entry_price" type="number" step="0.00001" bind:value={entry_price} required />
					</div>
				</div>

				<div>
					<label for="exit_price" class="block text-sm font-medium mb-2">Exit Price</label>
					<div class="input-field">
						<input id="exit_price" type="number" step="0.00001" bind:value={exit_price} />
					</div>
				</div>

				<div>
					<label for="quantity" class="block text-sm font-medium mb-2">Quantity *</label>
					<div class="input-field">
						<input id="quantity" type="number" step="0.01" bind:value={quantity} required />
					</div>
				</div>
			</div>

			<!-- Times -->
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div>
					<label for="entry_time" class="block text-sm font-medium mb-2">Entry Time *</label>
					<div class="input-field">
						<input id="entry_time" type="datetime-local" bind:value={entry_time} required />
					</div>
				</div>

				<div>
					<label for="exit_time" class="block text-sm font-medium mb-2">Exit Time</label>
					<div class="input-field">
						<input id="exit_time" type="datetime-local" bind:value={exit_time} />
					</div>
				</div>
			</div>

			<!-- Fees & Setup -->
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div>
					<label for="fees" class="block text-sm font-medium mb-2">Fees</label>
					<div class="input-field">
						<input id="fees" type="number" step="0.01" bind:value={fees} placeholder="0.00" />
					</div>
				</div>

				<div>
					<label for="setup_type" class="block text-sm font-medium mb-2">Setup Type</label>
					<div class="input-field">
						<select id="setup_type" bind:value={setup_type} class="flex-1 bg-transparent border-0 outline-none">
							<option value="">Select setup...</option>
							{#each setupTypes as type}
								<option value={type}>{type}</option>
							{/each}
						</select>
					</div>
				</div>
			</div>

			<!-- Tags -->
			<div>
				<label class="block text-sm font-medium mb-2">Tags</label>
				<div class="flex flex-wrap gap-2">
					{#each availableTags as tag}
						<button
							type="button"
							on:click={() => tags = toggleArray(tags, tag)}
							class="px-3 py-1 rounded-pill text-sm transition-colors {tags.includes(tag) ? 'bg-accent text-bg' : 'bg-white/5 hover:bg-white/10'}"
						>
							{tag}
						</button>
					{/each}
				</div>
			</div>

			<!-- Mistakes -->
			<div>
				<label class="block text-sm font-medium mb-2">Mistakes</label>
				<div class="flex flex-wrap gap-2">
					{#each availableMistakes as mistake}
						<button
							type="button"
							on:click={() => mistakes = toggleArray(mistakes, mistake)}
							class="px-3 py-1 rounded-pill text-sm transition-colors {mistakes.includes(mistake) ? 'bg-danger text-white' : 'bg-white/5 hover:bg-white/10'}"
						>
							{mistake}
						</button>
					{/each}
				</div>
			</div>

			<!-- Emotions -->
			<div>
				<label class="block text-sm font-medium mb-2">Emotions</label>
				<div class="flex flex-wrap gap-2">
					{#each availableEmotions as emotion}
						<button
							type="button"
							on:click={() => emotions = toggleArray(emotions, emotion)}
							class="px-3 py-1 rounded-pill text-sm transition-colors {emotions.includes(emotion) ? 'bg-brand text-white' : 'bg-white/5 hover:bg-white/10'}"
						>
							{emotion}
						</button>
					{/each}
				</div>
			</div>

			<!-- Notes -->
			<div>
				<label for="notes" class="block text-sm font-medium mb-2">Notes</label>
				<textarea
					id="notes"
					bind:value={notes}
					rows="4"
					class="w-full bg-white/5 border border-white/10 rounded-input px-3 py-2 outline-none focus:border-brand/50 focus:shadow-ring"
					placeholder="Trade notes, observations, lessons learned..."
				></textarea>
			</div>

			<!-- Actions -->
			<div class="flex gap-4">
				<button type="submit" class="btn btn-primary flex-1" disabled={isLoading}>
					{isLoading ? 'Creating...' : 'Create Trade'}
				</button>
				<a href="/trades" class="btn btn-ghost">Cancel</a>
			</div>
		</form>
	</div>
</div>

