<script lang="ts">
  import { goto } from '$app/navigation';
  import { isAuthenticated } from '$lib/stores/auth';
  import { onMount } from 'svelte';

  onMount(() => {
    // Redirect to dashboard if already logged in
    if ($isAuthenticated) {
      goto('/dashboard');
    }
  });
</script>

<div class="min-h-screen flex flex-col">
  <!-- Hero Section -->
  <div class="flex-1 flex items-center justify-center px-4 py-16">
    <div class="max-w-4xl mx-auto text-center">
      <!-- Logo/Title -->
      <div class="mb-8">
        <h1 class="text-5xl md:text-7xl font-bold mb-4 bg-gradient-to-r from-brand via-accent to-brand bg-clip-text text-transparent">
          Trading Journal
        </h1>
        <p class="text-xl md:text-2xl text-muted">
          Track, Analyze, and Improve Your Trading Performance
        </p>
      </div>

      <!-- Features Grid -->
      <div class="grid md:grid-cols-3 gap-6 mb-12">
        <div class="card p-6">
          <div class="text-4xl mb-3">📊</div>
          <h3 class="text-lg font-bold mb-2">Advanced Analytics</h3>
          <p class="text-sm text-muted">
            Comprehensive statistics and performance metrics
          </p>
        </div>

        <div class="card p-6">
          <div class="text-4xl mb-3">📈</div>
          <h3 class="text-lg font-bold mb-2">Visual Charts</h3>
          <p class="text-sm text-muted">
            Interactive charts to visualize your trading journey
          </p>
        </div>

        <div class="card p-6">
          <div class="text-4xl mb-3">🎯</div>
          <h3 class="text-lg font-bold mb-2">Trade Management</h3>
          <p class="text-sm text-muted">
            Organize and categorize all your trades efficiently
          </p>
        </div>
      </div>

      <!-- CTA Buttons -->
      <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
        <a href="/register" class="btn btn-primary w-full sm:w-auto min-w-[200px]">
          Get Started Free
        </a>
        <a href="/login" class="btn btn-ghost w-full sm:w-auto min-w-[200px]">
          Sign In
        </a>
      </div>

      <!-- Stats -->
      <div class="mt-16 grid grid-cols-3 gap-8 max-w-2xl mx-auto">
        <div>
          <div class="text-3xl font-bold text-accent">1000+</div>
          <div class="text-sm text-muted">Active Traders</div>
        </div>
        <div>
          <div class="text-3xl font-bold text-accent">50K+</div>
          <div class="text-sm text-muted">Trades Logged</div>
        </div>
        <div>
          <div class="text-3xl font-bold text-accent">99.9%</div>
          <div class="text-sm text-muted">Uptime</div>
        </div>
      </div>
    </div>
  </div>

  <!-- Footer -->
  <footer class="py-8 border-t border-white/10">
    <div class="max-w-4xl mx-auto px-4 text-center text-sm text-muted">
      <p>&copy; 2025 Trading Journal. All rights reserved.</p>
    </div>
  </footer>
</div>
