import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export interface User {
	id: string;
	name: string;
	email: string;
	email_verified: boolean;
	subscription_status: string;
	subscription_tier: string;
	subscription_interval: string | null;
	created_at: string;
}

interface AuthState {
	user: User | null;
	token: string | null;
	isAuthenticated: boolean;
	isLoading: boolean;
}

const STORAGE_KEY = 'auth_token';

// Initialize state from localStorage
function createAuthStore() {
	const initialToken = browser ? localStorage.getItem(STORAGE_KEY) : null;

	const { subscribe, set, update } = writable<AuthState>({
		user: null,
		token: initialToken,
		isAuthenticated: false,
		isLoading: !!initialToken // If we have a token, we'll verify it
	});

	return {
		subscribe,
		
		// Set authenticated user
		setAuth: (user: User, token: string) => {
			if (browser) {
				localStorage.setItem(STORAGE_KEY, token);
			}
			set({
				user,
				token,
				isAuthenticated: true,
				isLoading: false
			});
		},

		// Clear authentication
		clearAuth: () => {
			if (browser) {
				localStorage.removeItem(STORAGE_KEY);
			}
			set({
				user: null,
				token: null,
				isAuthenticated: false,
				isLoading: false
			});
		},

		// Set loading state
		setLoading: (isLoading: boolean) => {
			update(state => ({ ...state, isLoading }));
		},

		// Get current token
		getToken: (): string | null => {
			if (browser) {
				return localStorage.getItem(STORAGE_KEY);
			}
			return null;
		}
	};
}

export const authStore = createAuthStore();

// Derived stores for convenience
export const isAuthenticated = derived(authStore, $auth => $auth.isAuthenticated);
export const currentUser = derived(authStore, $auth => $auth.user);
export const isLoading = derived(authStore, $auth => $auth.isLoading);

