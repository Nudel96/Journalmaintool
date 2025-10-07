import { authStore } from '$lib/stores/auth';
import { get } from 'svelte/store';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

interface RequestOptions extends RequestInit {
	requiresAuth?: boolean;
}

class ApiError extends Error {
	constructor(
		public status: number,
		public message: string,
		public details?: string
	) {
		super(message);
		this.name = 'ApiError';
	}
}

async function request<T>(
	endpoint: string,
	options: RequestOptions = {}
): Promise<T> {
	const { requiresAuth = false, ...fetchOptions } = options;

	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		...fetchOptions.headers
	};

	// Add Authorization header if required
	if (requiresAuth) {
		const token = authStore.getToken();
		if (!token) {
			throw new ApiError(401, 'No authentication token found');
		}
		headers['Authorization'] = `Bearer ${token}`;
	}

	const response = await fetch(`${API_URL}${endpoint}`, {
		...fetchOptions,
		headers
	});

	// Handle non-JSON responses
	const contentType = response.headers.get('content-type');
	if (!contentType?.includes('application/json')) {
		if (!response.ok) {
			throw new ApiError(response.status, 'Request failed');
		}
		return (await response.text()) as T;
	}

	const data = await response.json();

	if (!response.ok) {
		throw new ApiError(
			response.status,
			data.error || 'Request failed',
			data.details
		);
	}

	return data;
}

// Auth API
export const authApi = {
	async register(name: string, email: string, password: string) {
		return request<{ token: string; user: any }>('/api/auth/register', {
			method: 'POST',
			body: JSON.stringify({ name, email, password })
		});
	},

	async login(email: string, password: string) {
		return request<{ token: string; user: any }>('/api/auth/login', {
			method: 'POST',
			body: JSON.stringify({ email, password })
		});
	},

	async me() {
		return request<any>('/api/auth/me', {
			requiresAuth: true
		});
	}
};

// Subscription API
export const subscriptionApi = {
	async createCheckoutSession(interval: string) {
		return request<{ session_id: string; url: string }>('/api/subscriptions/checkout', {
			method: 'POST',
			body: JSON.stringify({ interval }),
			requiresAuth: true
		});
	}
};

export { ApiError };

