<script lang="ts">
	import { authStore } from '$lib/stores/auth';
	import { authApi, ApiError } from '$lib/api/client';
	import { goto } from '$app/navigation';

	let email = '';
	let password = '';
	let error = '';
	let isLoading = false;

	async function handleLogin() {
		error = '';
		isLoading = true;

		try {
			const response = await authApi.login(email, password);
			authStore.setAuth(response.user, response.token);
			goto('/dashboard');
		} catch (err) {
			if (err instanceof ApiError) {
				error = err.details || err.message;
			} else {
				error = 'An unexpected error occurred';
			}
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="min-h-screen flex items-center justify-center p-4">
	<div class="vignette"></div>
	
	<div class="card w-full max-w-md p-8 relative z-10">
		<h1 class="text-3xl font-bold mb-2">Welcome Back</h1>
		<p class="text-text-muted mb-8">Sign in to your trading journal</p>

		{#if error}
			<div class="bg-danger/10 border border-danger/50 rounded-button p-4 mb-6">
				<p class="text-danger text-sm">{error}</p>
			</div>
		{/if}

		<form on:submit|preventDefault={handleLogin} class="space-y-6">
			<div>
				<label for="email" class="block text-sm font-medium mb-2">Email</label>
				<div class="input-field">
					<input
						id="email"
						type="email"
						bind:value={email}
						placeholder="you@example.com"
						required
						disabled={isLoading}
					/>
				</div>
			</div>

			<div>
				<label for="password" class="block text-sm font-medium mb-2">Password</label>
				<div class="input-field">
					<input
						id="password"
						type="password"
						bind:value={password}
						placeholder="••••••••"
						required
						disabled={isLoading}
					/>
				</div>
			</div>

			<button
				type="submit"
				class="btn btn-primary w-full"
				disabled={isLoading}
			>
				{isLoading ? 'Signing in...' : 'Sign In'}
			</button>
		</form>

		<p class="text-center text-text-muted mt-6">
			Don't have an account?
			<a href="/register" class="text-brand hover:text-accent transition-colors">
				Sign up
			</a>
		</p>
	</div>
</div>

