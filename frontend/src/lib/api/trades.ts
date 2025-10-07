import { authStore } from '$lib/stores/auth';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

interface Trade {
	id: string;
	symbol: string;
	direction: string;
	entry_price: number;
	exit_price?: number;
	quantity: number;
	entry_time: string;
	exit_time?: string;
	pnl?: number;
	pnl_percentage?: number;
	fees: number;
	notes?: string;
	tags: string[];
	setup_type?: string;
	mistakes: string[];
	emotions: string[];
	status: string;
	created_at: string;
	updated_at: string;
}

interface CreateTradeData {
	symbol: string;
	direction: string;
	entry_price: number;
	exit_price?: number;
	quantity: number;
	entry_time: string;
	exit_time?: string;
	fees?: number;
	notes?: string;
	tags?: string[];
	setup_type?: string;
	mistakes?: string[];
	emotions?: string[];
}

interface TradeFilters {
	symbol?: string;
	direction?: string;
	status?: string;
	from_date?: string;
	to_date?: string;
	limit?: number;
	offset?: number;
}

async function request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
	const token = authStore.getToken();
	
	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		...options.headers
	};

	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	const response = await fetch(`${API_URL}${endpoint}`, {
		...options,
		headers
	});

	if (!response.ok) {
		const error = await response.json().catch(() => ({ error: 'Request failed' }));
		throw new Error(error.error || 'Request failed');
	}

	return response.json();
}

export const tradesApi = {
	async create(data: CreateTradeData): Promise<Trade> {
		return request<Trade>('/api/trades', {
			method: 'POST',
			body: JSON.stringify(data)
		});
	},

	async list(filters?: TradeFilters): Promise<Trade[]> {
		const params = new URLSearchParams();
		if (filters) {
			Object.entries(filters).forEach(([key, value]) => {
				if (value !== undefined) {
					params.append(key, String(value));
				}
			});
		}

		const query = params.toString();
		return request<Trade[]>(`/api/trades${query ? `?${query}` : ''}`);
	},

	async get(id: string): Promise<Trade> {
		return request<Trade>(`/api/trades/${id}`);
	},

	async update(id: string, data: Partial<CreateTradeData>): Promise<Trade> {
		return request<Trade>(`/api/trades/${id}`, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	},

	async delete(id: string): Promise<void> {
		await fetch(`${API_URL}/api/trades/${id}`, {
			method: 'DELETE',
			headers: {
				'Authorization': `Bearer ${authStore.getToken()}`
			}
		});
	}
};

export const analyticsApi = {
	async getOverview() {
		return request('/api/analytics/overview');
	},

	async getBySymbol() {
		return request('/api/analytics/symbols');
	},

	async getBySetup() {
		return request('/api/analytics/setups');
	},

	async getMistakes() {
		return request('/api/analytics/mistakes');
	}
};

// Helper functions for backward compatibility
export async function getTrades(filters?: TradeFilters): Promise<Trade[]> {
	return tradesApi.list(filters);
}

export async function createTrade(data: CreateTradeData): Promise<Trade> {
	return tradesApi.create(data);
}

export async function deleteTrade(id: string): Promise<void> {
	return tradesApi.delete(id);
}

export type { Trade, CreateTradeData, TradeFilters };

