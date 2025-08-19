import { writable, derived } from 'svelte/store';
import type { Writable } from 'svelte/store';

// Authentication interfaces
export interface User {
	id: string;
	username: string;
	email: string;
	role: 'admin' | 'operator' | 'viewer' | 'bank_user' | 'government_user';
	permissions: string[];
	wallet_address?: string;
	bank_stamp?: BankStamp;
	government_stamp?: GovernmentStamp;
	created_at: string;
	last_login: string;
	mfa_enabled: boolean;
	compliance_status: 'compliant' | 'pending' | 'violation';
}

export interface BankStamp {
	stamp_id: string;
	authority_name: string;
	issued_at: string;
	expires_at: string;
	compliance_level: string;
	transaction_limits: {
		daily_limit: number;
		monthly_limit: number;
		single_transaction_limit: number;
	};
}

export interface GovernmentStamp {
	stamp_id: string;
	jurisdiction: string;
	authority_name: string;
	issued_at: string;
	expires_at: string;
	clearance_level: string;
	geographic_restrictions: string[];
}

export interface AuthSession {
	token: string;
	refresh_token: string;
	expires_at: string;
	user: User;
	wallet_connected: boolean;
	security_level: 'standard' | 'enhanced' | 'military' | 'quantum';
}

interface AuthState {
	user: User | null;
	session: AuthSession | null;
	loading: boolean;
	error: string | null;
	isAuthenticated: boolean;
	loginAttempts: number;
	lastLoginAttempt: Date | null;
	mfaRequired: boolean;
	mfaToken: string | null;
}

// Create the authentication store
function createAuthStore() {
	const { subscribe, set, update }: Writable<AuthState> = writable({
		user: null,
		session: null,
		loading: false,
		error: null,
		isAuthenticated: false,
		loginAttempts: 0,
		lastLoginAttempt: null,
		mfaRequired: false,
		mfaToken: null
	});

	return {
		subscribe,
		
		// Login with username/password
		async login(username: string, password: string): Promise<boolean> {
			try {
				update(state => ({ ...state, loading: true, error: null }));

				const response = await fetch('/api/auth/login', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify({ username, password })
				});

				const result = await response.json();

				if (!response.ok) {
					if (result.requires_mfa) {
						update(state => ({
							...state,
							loading: false,
							mfaRequired: true,
							mfaToken: result.mfa_token,
							error: null
						}));
						return false;
					}
					
					throw new Error(result.message || 'Login failed');
				}

				// Store session data
				localStorage.setItem('bpi_session', JSON.stringify(result.session));
				localStorage.setItem('bpi_token', result.session.token);

				update(state => ({
					...state,
					user: result.session.user,
					session: result.session,
					isAuthenticated: true,
					loading: false,
					error: null,
					loginAttempts: 0,
					mfaRequired: false,
					mfaToken: null
				}));

				return true;

			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Login failed';
				update(state => ({
					...state,
					loading: false,
					error: errorMessage,
					loginAttempts: state.loginAttempts + 1,
					lastLoginAttempt: new Date()
				}));
				return false;
			}
		},

		// Complete MFA authentication
		async completeMFA(mfaCode: string): Promise<boolean> {
			try {
				update(state => ({ ...state, loading: true, error: null }));

				const response = await fetch('/api/auth/mfa/verify', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify({ 
						mfa_token: get(authStore).mfaToken,
						mfa_code: mfaCode 
					})
				});

				const result = await response.json();

				if (!response.ok) {
					throw new Error(result.message || 'MFA verification failed');
				}

				// Store session data
				localStorage.setItem('bpi_session', JSON.stringify(result.session));
				localStorage.setItem('bpi_token', result.session.token);

				update(state => ({
					...state,
					user: result.session.user,
					session: result.session,
					isAuthenticated: true,
					loading: false,
					error: null,
					mfaRequired: false,
					mfaToken: null
				}));

				return true;

			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'MFA verification failed';
				update(state => ({
					...state,
					loading: false,
					error: errorMessage
				}));
				return false;
			}
		},

		// Wallet authentication (for bank-stamped wallets)
		async connectWallet(walletAddress: string, signature: string): Promise<boolean> {
			try {
				update(state => ({ ...state, loading: true, error: null }));

				const response = await fetch('/api/auth/wallet/connect', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						'Authorization': `Bearer ${get(authStore).session?.token}`
					},
					body: JSON.stringify({ 
						wallet_address: walletAddress,
						signature: signature 
					})
				});

				const result = await response.json();

				if (!response.ok) {
					throw new Error(result.message || 'Wallet connection failed');
				}

				update(state => ({
					...state,
					session: result.session,
					user: result.session.user,
					loading: false,
					error: null
				}));

				return true;

			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Wallet connection failed';
				update(state => ({
					...state,
					loading: false,
					error: errorMessage
				}));
				return false;
			}
		},

		// Logout
		async logout(): Promise<void> {
			try {
				const session = get(authStore).session;
				if (session) {
					await fetch('/api/auth/logout', {
						method: 'POST',
						headers: {
							'Authorization': `Bearer ${session.token}`
						}
					});
				}
			} catch (error) {
				console.error('Logout API call failed:', error);
			}

			// Clear local storage
			localStorage.removeItem('bpi_session');
			localStorage.removeItem('bpi_token');

			// Reset state
			set({
				user: null,
				session: null,
				loading: false,
				error: null,
				isAuthenticated: false,
				loginAttempts: 0,
				lastLoginAttempt: null,
				mfaRequired: false,
				mfaToken: null
			});
		},

		// Initialize auth from stored session
		async initializeAuth(): Promise<void> {
			try {
				const storedSession = localStorage.getItem('bpi_session');
				const storedToken = localStorage.getItem('bpi_token');

				if (!storedSession || !storedToken) {
					return;
				}

				const session: AuthSession = JSON.parse(storedSession);

				// Verify token is still valid
				const response = await fetch('/api/auth/verify', {
					headers: {
						'Authorization': `Bearer ${storedToken}`
					}
				});

				if (!response.ok) {
					// Token expired, clear storage
					localStorage.removeItem('bpi_session');
					localStorage.removeItem('bpi_token');
					return;
				}

				const result = await response.json();

				update(state => ({
					...state,
					user: result.user,
					session: result.session,
					isAuthenticated: true
				}));

			} catch (error) {
				console.error('Auth initialization failed:', error);
				localStorage.removeItem('bpi_session');
				localStorage.removeItem('bpi_token');
			}
		},

		// Refresh token
		async refreshToken(): Promise<boolean> {
			try {
				const session = get(authStore).session;
				if (!session?.refresh_token) {
					return false;
				}

				const response = await fetch('/api/auth/refresh', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
					},
					body: JSON.stringify({ 
						refresh_token: session.refresh_token 
					})
				});

				const result = await response.json();

				if (!response.ok) {
					throw new Error(result.message || 'Token refresh failed');
				}

				// Update session
				localStorage.setItem('bpi_session', JSON.stringify(result.session));
				localStorage.setItem('bpi_token', result.session.token);

				update(state => ({
					...state,
					session: result.session
				}));

				return true;

			} catch (error) {
				console.error('Token refresh failed:', error);
				// Force logout on refresh failure
				authStore.logout();
				return false;
			}
		},

		// Clear error
		clearError(): void {
			update(state => ({ ...state, error: null }));
		}
	};
}

// Helper function to get current store value
function get<T>(store: { subscribe: (fn: (value: T) => void) => () => void }): T {
	let value: T;
	const unsubscribe = store.subscribe((v: T) => { value = v; });
	unsubscribe();
	return value!;
}

export const authStore = createAuthStore();

// Derived stores for convenience
export const isAuthenticated = derived(authStore, $auth => $auth.isAuthenticated);
export const currentUser = derived(authStore, $auth => $auth.user);
export const userRole = derived(authStore, $auth => $auth.user?.role);
export const hasWallet = derived(authStore, $auth => !!$auth.user?.wallet_address);
export const isBankUser = derived(authStore, $auth => $auth.user?.role === 'bank_user');
export const isGovernmentUser = derived(authStore, $auth => $auth.user?.role === 'government_user');
export const complianceStatus = derived(authStore, $auth => $auth.user?.compliance_status);

// Permission checking helper
export function hasPermission(permission: string): boolean {
	const auth = get(authStore);
	return auth.user?.permissions.includes(permission) || false;
}

// Role checking helpers
export function isAdmin(): boolean {
	return get(authStore).user?.role === 'admin';
}

export function canAccessWallet(): boolean {
	const auth = get(authStore);
	return auth.user?.role === 'bank_user' || auth.user?.role === 'admin';
}
