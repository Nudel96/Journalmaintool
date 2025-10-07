<script lang="ts">
	import { authStore } from '$lib/stores/auth';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let user = $authStore.user;

	onMount(() => {
		if (!$authStore.isAuthenticated) {
			goto('/login');
		}
	});

	function handleLogout() {
		authStore.clearAuth();
		goto('/login');
	}
</script>

<div class="min-h-screen p-8">
	<div class="vignette"></div>
	
	<div class="max-w-7xl mx-auto relative z-10">
		<div class="flex justify-between items-center mb-8">
			<div>
				<h1 class="text-4xl font-bold mb-2">Trading Dashboard</h1>
				<p class="text-text-muted">Welcome back, {user?.name || 'Trader'}!</p>
			</div>
			<button on:click={handleLogout} class="btn btn-ghost">
				Logout
			</button>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
			<div class="stat-card p-6">
				<p class="text-text-muted text-sm mb-1">Total Trades</p>
				<p class="text-3xl font-bold">0</p>
			</div>

			<div class="stat-card p-6">
				<p class="text-text-muted text-sm mb-1">Win Rate</p>
				<p class="text-3xl font-bold text-success">0%</p>
			</div>

			<div class="stat-card p-6">
				<p class="text-text-muted text-sm mb-1">Total P&L</p>
				<p class="text-3xl font-bold text-accent">$0.00</p>
			</div>
		</div>

		<div class="card p-8">
			<h2 class="text-2xl font-bold mb-4">Recent Trades</h2>
			<p class="text-text-muted">No trades yet. Start by adding your first trade!</p>
		</div>
	</div>
</div>

