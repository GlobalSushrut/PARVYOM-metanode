// BPI/BPCI Reactive Stores - Real backend integration
import { writable, derived, type Readable } from 'svelte/store';
import { apiService, type BpiVmStatus, type WalletData, type BpciEconomyStatus } from '../services/api';

// Core BPI VM status store
export const bpiVmStatus = writable<BpiVmStatus | null>(null);
export const bpiVmError = writable<string | null>(null);
export const bpiVmLoading = writable<boolean>(false);

// Wallet data store
export const walletData = writable<WalletData | null>(null);
export const walletError = writable<string | null>(null);
export const walletLoading = writable<boolean>(false);

// BPCI Economy status store
export const bpciEconomy = writable<BpciEconomyStatus | null>(null);
export const bpciEconomyError = writable<string | null>(null);
export const bpciEconomyLoading = writable<boolean>(false);

// Installation progress store
export const installationProgress = writable<{
	currentStep: number;
	isInstalling: boolean;
	completed: boolean;
	error: string | null;
}>({
	currentStep: 0,
	isInstalling: false,
	completed: false,
	error: null
});

// Derived stores for computed values
export const isConnected: Readable<boolean> = derived(
	bpiVmStatus,
	($bpiVmStatus) => $bpiVmStatus?.vm_server?.status === 'active'
);

export const securityRating = derived(bpiVmStatus, ($bpiVmStatus) => {
	return $bpiVmStatus?.vm_server?.security_rating?.toString() || 'Unknown';
});

export const totalBalance = derived(walletData, ($walletData) => {
	return ($walletData?.balance?.bpci || 0) + ($walletData?.balance?.eth || 0) + ($walletData?.balance?.btc || 0);
});

export const totalBalanceUSD = derived(
	[walletData, totalBalance],
	([$walletData, $totalBalance]) => {
		const priceUSD = 2.0; // Fixed price for BPCI
		return $totalBalance * priceUSD;
	}
);

export const networkStatus = derived(
	[bpiVmStatus, walletData],
	([$bpiVmStatus, $walletData]) => {
		if (!$bpiVmStatus?.vm_server?.status) return 'Disconnected';
		if ($walletData?.connectionStatus === 'connected') return 'Mainnet (BPCI)';
		return 'Mainnet (Local)';
	}
);

// Actions for updating stores
export const bpiActions = {
	// Fetch BPI VM status from real backend
	async fetchVmStatus() {
		bpiVmLoading.set(true);
		bpiVmError.set(null);
		
		try {
			const status = await apiService.getBpiVmStatus();
			bpiVmStatus.set(status);
			
			// Update wallet data based on VM status
			const wallet = await apiService.generateWalletData(status);
			walletData.set(wallet);
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'Unknown error';
			bpiVmError.set(errorMessage);
			console.error('Failed to fetch BPI VM status:', error);
		} finally {
			bpiVmLoading.set(false);
		}
	},

	// Fetch BPCI economy status
	async fetchEconomyStatus() {
		bpciEconomyLoading.set(true);
		bpciEconomyError.set(null);
		
		try {
			const economy = await apiService.getBpciEconomyStatus();
			bpciEconomy.set(economy);
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'BPCI server not available';
			bpciEconomyError.set(errorMessage);
			console.error('Failed to fetch BPCI economy status:', error);
		} finally {
			bpciEconomyLoading.set(false);
		}
	},

	// Send transaction through real backend
	async sendTransaction(toAddress: string, amount: number, gasFee: number) {
		walletLoading.set(true);
		walletError.set(null);
		
		try {
			const result = await apiService.sendTransaction(toAddress, amount, gasFee);
			
			// Update wallet data after transaction
			await this.fetchVmStatus();
			
			return result;
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'Transaction failed';
			walletError.set(errorMessage);
			throw error;
		} finally {
			walletLoading.set(false);
		}
	},

	// Installation process
	async runInstallation() {
		const steps = [
			'system-check',
			'download-core',
			'configure-network',
			'initialize-wallet',
			'start-node'
		];

		installationProgress.set({
			currentStep: 0,
			isInstalling: true,
			completed: false,
			error: null
		});

		try {
			for (let i = 0; i < steps.length; i++) {
				installationProgress.update(state => ({
					...state,
					currentStep: i
				}));

				await apiService.installBpiCore(steps[i]);
				
				// Simulate realistic installation timing
				await new Promise(resolve => setTimeout(resolve, 2000 + Math.random() * 3000));
			}

			installationProgress.update(state => ({
				...state,
				completed: true,
				isInstalling: false
			}));

			// Refresh status after installation
			await this.fetchVmStatus();
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'Installation failed';
			installationProgress.update(state => ({
				...state,
				error: errorMessage,
				isInstalling: false
			}));
			throw error;
		}
	},

	// Start real-time updates
	startRealTimeUpdates() {
		// Initial fetch
		this.fetchVmStatus();
		this.fetchEconomyStatus();

		// Set up intervals for real-time updates (like the original HTML)
		const vmInterval = setInterval(() => {
			this.fetchVmStatus();
		}, 10000); // Every 10 seconds

		const economyInterval = setInterval(() => {
			this.fetchEconomyStatus();
		}, 5000); // Every 5 seconds

		// Return cleanup function
		return () => {
			clearInterval(vmInterval);
			clearInterval(economyInterval);
		};
	}
};

// Auto-start real-time updates when stores are imported
if (typeof window !== 'undefined') {
	// Only run in browser
	let cleanup: (() => void) | null = null;
	
	// Start updates when first subscriber
	let subscriberCount = 0;
	
	const originalSubscribe = bpiVmStatus.subscribe;
	bpiVmStatus.subscribe = (run) => {
		subscriberCount++;
		if (subscriberCount === 1 && !cleanup) {
			cleanup = bpiActions.startRealTimeUpdates();
		}
		
		const unsubscribe = originalSubscribe(run);
		
		return () => {
			subscriberCount--;
			unsubscribe();
			if (subscriberCount === 0 && cleanup) {
				cleanup();
				cleanup = null;
			}
		};
	};
}
