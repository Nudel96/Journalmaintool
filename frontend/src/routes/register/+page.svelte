<script lang="ts">
	import { authStore } from '$lib/stores/auth';
	import { authApi, ApiError } from '$lib/api/client';
	import { goto } from '$app/navigation';

	let name = '';
	let email = '';
	let password = '';
	let confirmPassword = '';
	let error = '';
	let isLoading = false;

	$: passwordStrength = getPasswordStrength(password);
	$: passwordsMatch = password === confirmPassword;

	function getPasswordStrength(pwd: string): { score: number; label: string; color: string } {
		if (pwd.length === 0) return { score: 0, label: '', color: '' };
		if (pwd.length < 8) return { score: 1, label: 'Weak', color: 'text-danger' };
		
		let score = 1;
		if (pwd.length >= 12) score++;
		if (/[a-z]/.test(pwd) && /[A-Z]/.test(pwd)) score++;
		if (/\d/.test(pwd)) score++;
		if (/[^a-zA-Z0-9]/.test(pwd)) score++;

		if (score <= 2) return { score, label: 'Weak', color: 'text-danger' };
		if (score === 3) return { score, label: 'Medium', color: 'text-yellow-500' };
		return { score, label: 'Strong', color: 'text-success' };
	}

	async function handleRegister() {
		error = '';

		if (password !== confirmPassword) {
			error = 'Passwords do not match';
			return;
		}

		if (password.length < 8) {
			error = 'Password must be at least 8 characters';
			return;
		}

		isLoading = true;

		try {
			const response = await authApi.register(name, email, password);
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
		<h1 class="text-3xl font-bold mb-2">Create Account</h1>
		<p class="text-text-muted mb-8">Start your trading journal today</p>

		{#if error}
			<div class="bg-danger/10 border border-danger/50 rounded-button p-4 mb-6">
				<p class="text-danger text-sm">{error}</p>
			</div>
		{/if}

		<form on:submit|preventDefault={handleRegister} class="space-y-6">
			<div>
				<label for="name" class="block text-sm font-medium mb-2">Name</label>
				<div class="input-field">
					<input
						id="name"
						type="text"
						bind:value={name}
						placeholder="John Doe"
						required
						disabled={isLoading}
					/>
				</div>
			</div>

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
				{#if password.length > 0}
					<p class="text-sm mt-2 {passwordStrength.color}">
						Strength: {passwordStrength.label}
					</p>
				{/if}
			</div>

			<div>
				<label for="confirmPassword" class="block text-sm font-medium mb-2">
					Confirm Password
				</label>
				<div class="input-field">
					<input
						id="confirmPassword"
						type="password"
						bind:value={confirmPassword}
						placeholder="••••••••"
						required
						disabled={isLoading}
					/>
				</div>
				{#if confirmPassword.length > 0 && !passwordsMatch}
					<p class="text-sm mt-2 text-danger">Passwords do not match</p>
				{/if}
			</div>

			<button
				type="submit"
				class="btn btn-primary w-full"
				disabled={isLoading || !passwordsMatch}
			>
				{isLoading ? 'Creating account...' : 'Create Account'}
			</button>
		</form>

		<p class="text-center text-text-muted mt-6">
			Already have an account?
			<a href="/login" class="text-brand hover:text-accent transition-colors">
				Sign in
			</a>
		</p>
	</div>
</div>

