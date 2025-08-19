import { writable, derived } from 'svelte/store';
import type { Writable } from 'svelte/store';
import { authStore } from './auth';

// BPI Wallet interfaces based on the actual Rust implementation
export interface BPIWallet {
	address: string;
	balance: number;
	currency: string;
	wallet_type: 'bank_stamped' | 'government_stamped' | 'standard';
	bank_stamp?: BankStamp;
	government_stamp?: GovernmentStamp;
	multi_sig_threshold: number;
	transaction_history: BankTransaction[];
	statistics: WalletStatistics;
	created_at: string;
	last_activity: string;
	compliance_status: 'compliant' | 'pending' | 'violation';
	geographic_restrictions: string[];
}

export interface BankStamp {
	stamp_id: string;
	authority_id: string;
	authority_name: string;
	issued_at: string;
	expires_at: string;
	compliance_metadata: {
		kyc_verified: boolean;
		aml_checked: boolean;
		regulatory_status: string;
		risk_score: number;
	};
	transaction_limits: {
		daily_limit: number;
		monthly_limit: number;
		single_transaction_limit: number;
		remaining_daily: number;
		remaining_monthly: number;
	};
	authority_signature: string;
}

export interface GovernmentStamp {
	stamp_id: string;
	jurisdiction: string;
	authority_name: string;
	clearance_level: string;
	issued_at: string;
	expires_at: string;
	geographic_restrictions: string[];
	authority_permissions: string[];
}

export interface BankTransaction {
	transaction_id: string;
	transaction_type: 'transfer' | 'deposit' | 'withdrawal' | 'payment' | 'fee';
	counterparty: string;
	amount: number;
	currency: string;
	status: 'pending' | 'completed' | 'failed' | 'cancelled';
	compliance_flags: string[];
	timestamp: string;
	banking_metadata: Record<string, string>;
	multi_sig_required: boolean;
	signatures_collected: number;
	fee: number;
	block_height?: number;
	confirmation_count?: number;
}

export interface WalletStatistics {
	total_transactions: number;
	total_volume: number;
	multi_sig_transactions: number;
	compliance_violations: number;
	average_transaction_amount: number;
	last_compliance_check: string | null;
	monthly_volume: number;
	daily_volume: number;
	largest_transaction: number;
	risk_score: number;
}

export interface TransactionRequest {
	to_address: string;
	amount: number;
	currency: string;
	transaction_type: string;
	memo?: string;
	metadata?: Record<string, string>;
}

export interface MultiSigRequest {
	transaction_id: string;
	signatures: string[];
	threshold_met: boolean;
	pending_signers: string[];
}

interface WalletState {
	wallet: BPIWallet | null;
	loading: boolean;
	error: string | null;
	connected: boolean;
	pendingTransactions: BankTransaction[];
	multiSigRequests: MultiSigRequest[];
	complianceAlerts: string[];
	lastUpdate: Date | null;
}

// Create the wallet store
function createWalletStore() {
	const { subscribe, set, update }: Writable<WalletState> = writable({
		wallet: null,
		loading: false,
		error: null,
		connected: false,
		pendingTransactions: [],
		multiSigRequests: [],
		complianceAlerts: [],
		lastUpdate: null
	});

	return {
		subscribe,

		// Initialize wallet connection
		async connectWallet(address?: string): Promise<boolean> {
			try {
				update(state => ({ ...state, loading: true, error: null }));

				const auth = get(authStore);
				if (!auth.isAuthenticated) {
					throw new Error('Authentication required');
				}

				const response = await fetch('/api/bpi/wallet/connect', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						'Authorization': `Bearer ${auth.session?.token}`,
						'X-BPI-Security-Level': 'MAXIMUM'
					},
					body: JSON.stringify({ address })
				});

				const result = await response.json();

				if (!response.ok) {
					throw new Error(result.message || 'Wallet connection failed');
				}

				update(state => ({
					...state,
					wallet: result.wallet,
					connected: true,
					loading: false,
					error: null,
					lastUpdate: new Date()
				}));

				// Load additional wallet data
				await get(walletStore).loadTransactionHistory();
				await get(walletStore).loadMultiSigRequests();
				await get(walletStore).checkCompliance();

				return true;

			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Wallet connection failed';
				update(state => ({
					...state,
					loading: false,
					error: errorMessage,
					connected: false
				}));
				return false;
			}
		},

		// Create new bank-stamped wallet
		async createBankWallet(initialBalance: number = 0): Promise<boolean> {
			try {
				update(state => ({ ...state, loading: true, error: null }));

				const auth = get(authStore);
				if (!auth.isAuthenticated) {
					throw new Error('Authentication required');
				}

				const response = await fetch('/api/bpi/wallet/create/bank', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						'Authorization': `Bearer ${auth.session?.token}`,
						'X-BPI-Security-Level': 'MAXIMUM'
					},
					body: JSON.stringify({ 
						initial_balance: initialBalance,
						core_maintainer_id: auth.user?.id 
					})
				});

				const result = await response.json();

				if (!response.ok) {
					throw new Error(result.message || 'Wallet creation failed');
				}

				update(state => ({
					...state,
					wallet: result.wallet,
					connected: true,
					loading: false,
					error: null,
					lastUpdate: new Date()
				}));

				return true;

			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Wallet creation failed';
				update(state => ({
					...state,
					loading: false,
					error: errorMessage
				}));
				return false;
			}
		},

		// Execute transaction
		async executeTransaction(request: TransactionRequest): Promise<string | null> {
			try {
				update(state => ({ ...state, loading: true, error: null }));

				const auth = get(authStore);
				const wallet = get(walletStore).wallet;

				if (!auth.isAuthenticated || !wallet) {
					throw new Error('Wallet not connected');
				}

				const response = await fetch('/api/bpi/wallet/transaction', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						'Authorization': `Bearer ${auth.session?.token}`,
						'X-BPI-Security-Level': 'MAXIMUM'
					},
					body: JSON.stringify({
						wallet_address: wallet.address,
						...request
					})
				});

				const result = await response.json();

				if (!response.ok) {
					throw new Error(result.message || 'Transaction failed');
				}

				// Refresh wallet data
				await get(walletStore).refreshWallet();

				update(state => ({
					...state,
					loading: false,
					error: null
				}));

				return result.transaction_id;

			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Transaction failed';
				update(state => ({
					...state,
					loading: false,
					error: errorMessage
				}));
				return null;
			}
		},

		// Load transaction history
		async loadTransactionHistory(): Promise<void> {
			try {
				const auth = get(authStore);
				const wallet = get(walletStore).wallet;

				if (!auth.isAuthenticated || !wallet) return;

				const response = await fetch(`/api/bpi/wallet/${wallet.address}/transactions`, {
					headers: {
						'Authorization': `Bearer ${auth.session?.token}`,
						'X-BPI-Security-Level': 'MAXIMUM'
					}
				});

				if (response.ok) {
					const result = await response.json();
					update(state => ({
						...state,
						wallet: state.wallet ? {
							...state.wallet,
							transaction_history: result.transactions
						} : null
					}));
				}

			} catch (error) {
				console.error('Failed to load transaction history:', error);
			}
		},

		// Load multi-signature requests
		async loadMultiSigRequests(): Promise<void> {
			try {
				const auth = get(authStore);
				const wallet = get(walletStore).wallet;

				if (!auth.isAuthenticated || !wallet) return;

				const response = await fetch(`/api/bpi/wallet/${wallet.address}/multisig`, {
					headers: {
						'Authorization': `Bearer ${auth.session?.token}`,
						'X-BPI-Security-Level': 'MAXIMUM'
					}
				});

				if (response.ok) {
					const result = await response.json();
					update(state => ({
						...state,
						multiSigRequests: result.requests
					}));
				}

			} catch (error) {
				console.error('Failed to load multi-sig requests:', error);
			}
		},

		// Sign multi-signature transaction
		async signMultiSigTransaction(transactionId: string, signature: string): Promise<boolean> {
			try {
				update(state => ({ ...state, loading: true, error: null }));

				const auth = get(authStore);
				const wallet = get(walletStore).wallet;

				if (!auth.isAuthenticated || !wallet) {
					throw new Error('Wallet not connected');
				}

				const response = await fetch('/api/bpi/wallet/multisig/sign', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						'Authorization': `Bearer ${auth.session?.token}`,
						'X-BPI-Security-Level': 'MAXIMUM'
					},
					body: JSON.stringify({
						wallet_address: wallet.address,
						transaction_id: transactionId,
						signature: signature
					})
				});

				const result = await response.json();

				if (!response.ok) {
					throw new Error(result.message || 'Signature failed');
				}

				// Refresh multi-sig requests
				await get(walletStore).loadMultiSigRequests();

				update(state => ({
					...state,
					loading: false,
					error: null
				}));

				return true;

			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Signature failed';
				update(state => ({
					...state,
					loading: false,
					error: errorMessage
				}));
				return false;
			}
		},

		// Check compliance status
		async checkCompliance(): Promise<void> {
			try {
				const auth = get(authStore);
				const wallet = get(walletStore).wallet;

				if (!auth.isAuthenticated || !wallet) return;

				const response = await fetch(`/api/bpi/wallet/${wallet.address}/compliance`, {
					headers: {
						'Authorization': `Bearer ${auth.session?.token}`,
						'X-BPI-Security-Level': 'MAXIMUM'
					}
				});

				if (response.ok) {
					const result = await response.json();
					update(state => ({
						...state,
						complianceAlerts: result.alerts || [],
						wallet: state.wallet ? {
							...state.wallet,
							compliance_status: result.status
						} : null
					}));
				}

			} catch (error) {
				console.error('Failed to check compliance:', error);
			}
		},

		// Update multi-signature threshold
		async updateMultiSigThreshold(threshold: number): Promise<boolean> {
			try {
				update(state => ({ ...state, loading: true, error: null }));

				const auth = get(authStore);
				const wallet = get(walletStore).wallet;

				if (!auth.isAuthenticated || !wallet) {
					throw new Error('Wallet not connected');
				}

				const response = await fetch('/api/bpi/wallet/multisig/threshold', {
					method: 'PUT',
					headers: {
						'Content-Type': 'application/json',
						'Authorization': `Bearer ${auth.session?.token}`,
						'X-BPI-Security-Level': 'MAXIMUM'
					},
					body: JSON.stringify({
						wallet_address: wallet.address,
						threshold: threshold
					})
				});

				const result = await response.json();

				if (!response.ok) {
					throw new Error(result.message || 'Threshold update failed');
				}

				update(state => ({
					...state,
					wallet: state.wallet ? {
						...state.wallet,
						multi_sig_threshold: threshold
					} : null,
					loading: false,
					error: null
				}));

				return true;

			} catch (error) {
				const errorMessage = error instanceof Error ? error.message : 'Threshold update failed';
				update(state => ({
					...state,
					loading: false,
					error: errorMessage
				}));
				return false;
			}
		},

		// Refresh wallet data
		async refreshWallet(): Promise<void> {
			const wallet = get(walletStore).wallet;
			if (wallet) {
				await get(walletStore).connectWallet(wallet.address);
			}
		},

		// Disconnect wallet
		disconnect(): void {
			set({
				wallet: null,
				loading: false,
				error: null,
				connected: false,
				pendingTransactions: [],
				multiSigRequests: [],
				complianceAlerts: [],
				lastUpdate: null
			});
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

export const walletStore = createWalletStore();

// Derived stores for convenience
export const isWalletConnected = derived(walletStore, $wallet => $wallet.connected);
export const walletBalance = derived(walletStore, $wallet => $wallet.wallet?.balance || 0);
export const walletAddress = derived(walletStore, $wallet => $wallet.wallet?.address);
export const isBankStamped = derived(walletStore, $wallet => $wallet.wallet?.wallet_type === 'bank_stamped');
export const isGovernmentStamped = derived(walletStore, $wallet => $wallet.wallet?.wallet_type === 'government_stamped');
export const complianceStatus = derived(walletStore, $wallet => $wallet.wallet?.compliance_status);
export const hasComplianceAlerts = derived(walletStore, $wallet => $wallet.complianceAlerts.length > 0);
export const pendingMultiSig = derived(walletStore, $wallet => $wallet.multiSigRequests.length);

// Utility functions
export function formatBalance(balance: number, currency: string = 'USD'): string {
	return new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency: currency,
		minimumFractionDigits: 2,
		maximumFractionDigits: 2
	}).format(balance);
}

export function formatTransactionAmount(amount: number, currency: string = 'USD'): string {
	const prefix = amount >= 0 ? '+' : '';
	return prefix + formatBalance(Math.abs(amount), currency);
}

export function getTransactionStatusColor(status: string): string {
	switch (status) {
		case 'completed': return 'text-green-400';
		case 'pending': return 'text-yellow-400';
		case 'failed': return 'text-red-400';
		case 'cancelled': return 'text-gray-400';
		default: return 'text-blue-400';
	}
}

export function getComplianceStatusColor(status: string): string {
	switch (status) {
		case 'compliant': return 'text-green-400';
		case 'pending': return 'text-yellow-400';
		case 'violation': return 'text-red-400';
		default: return 'text-gray-400';
	}
}
