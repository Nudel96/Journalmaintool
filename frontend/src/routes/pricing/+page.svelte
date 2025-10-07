<script lang="ts">
	import { subscriptionApi, ApiError } from '$lib/api/client';
	import { authStore } from '$lib/stores/auth';
	import { goto } from '$app/navigation';

	let isLoading = false;
	let error = '';

	const tiers = [
		{
			name: '1 Month',
			interval: 'month',
			price: '$7',
			perMonth: '$7/month',
			total: '$7',
			savings: null,
			features: [
				'Unlimited trades',
				'Advanced analytics',
				'Export to CSV',
				'Mobile access'
			]
		},
		{
			name: '6 Months',
			interval: 'month_6',
			price: '$5',
			perMonth: '$5/month',
			total: '$30 every 6 months',
			savings: '29% savings',
			popular: true,
			features: [
				'Everything in 1 Month',
				'Priority support',
				'Advanced charts',
				'Custom tags'
			]
		},
		{
			name: '12 Months',
			interval: 'year',
			price: '$4',
			perMonth: '$4/month',
			total: '$48 per year',
			savings: '43% savings',
			features: [
				'Everything in 6 Months',
				'Dedicated support',
				'API access',
				'White-label reports'
			]
		}
	];

	async function handleSubscribe(interval: string) {
		if (!$authStore.isAuthenticated) {
			goto('/login?redirect=/pricing');
			return;
		}

		error = '';
		isLoading = true;

		try {
			const response = await subscriptionApi.createCheckoutSession(interval);
			// Redirect to Stripe Checkout
			window.location.href = response.url;
		} catch (err) {
			if (err instanceof ApiError) {
				error = err.details || err.message;
			} else {
				error = 'Failed to create checkout session';
			}
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="min-h-screen p-8">
	<div class="vignette"></div>
	
	<div class="max-w-7xl mx-auto relative z-10">
		<div class="text-center mb-12">
			<h1 class="text-5xl font-bold mb-4">Choose Your Plan</h1>
			<p class="text-text-muted text-xl">
				Start tracking your trades and improve your performance
			</p>
		</div>

		{#if error}
			<div class="bg-danger/10 border border-danger/50 rounded-button p-4 mb-8 max-w-2xl mx-auto">
				<p class="text-danger text-sm text-center">{error}</p>
			</div>
		{/if}

		<div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
			{#each tiers as tier}
				<div class="card p-8 relative {tier.popular ? 'neon-border' : ''}">
					{#if tier.popular}
						<div class="absolute -top-4 left-1/2 -translate-x-1/2 bg-accent text-bg px-4 py-1 rounded-pill text-sm font-bold">
							MOST POPULAR
						</div>
					{/if}

					<div class="text-center mb-6">
						<h3 class="text-2xl font-bold mb-2">{tier.name}</h3>
						<div class="mb-2">
							<span class="text-5xl font-bold">{tier.price}</span>
							<span class="text-text-muted">/month</span>
						</div>
						<p class="text-text-muted text-sm">{tier.total}</p>
						{#if tier.savings}
							<p class="text-success text-sm font-bold mt-2">{tier.savings}</p>
						{/if}
					</div>

					<ul class="space-y-3 mb-8">
						{#each tier.features as feature}
							<li class="flex items-center gap-2">
								<span class="text-accent">✓</span>
								<span class="text-sm">{feature}</span>
							</li>
						{/each}
					</ul>

					<button
						on:click={() => handleSubscribe(tier.interval)}
						class="btn w-full {tier.popular ? 'btn-primary' : 'btn-ghost'}"
						disabled={isLoading}
					>
						{isLoading ? 'Loading...' : 'Subscribe'}
					</button>
				</div>
			{/each}
		</div>

		<div class="text-center text-text-muted">
			<p class="mb-2">All plans include a 7-day free trial</p>
			<p class="text-sm">Cancel anytime. No questions asked.</p>
		</div>
	</div>
</div>

