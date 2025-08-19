import { writable, derived } from 'svelte/store';
import type { SystemStatus, NodeMetrics, NetworkInfo } from '$lib/types/system';

// System state store
interface SystemState {
	loading: boolean;
	error: string | null;
	status: SystemStatus | null;
	metrics: NodeMetrics | null;
	network: NetworkInfo | null;
	lastUpdate: Date | null;
}

const initialState: SystemState = {
	loading: false,
	error: null,
	status: null,
	metrics: null,
	network: null,
	lastUpdate: null
};

function createSystemStore() {
	const { subscribe, set, update } = writable<SystemState>(initialState);

	return {
		subscribe,
		
		// Initialize system monitoring
		async initialize() {
			update(state => ({ ...state, loading: true }));
			
			try {
				await Promise.all([
					this.fetchStatus(),
					this.fetchMetrics(),
					this.fetchNetworkInfo()
				]);
				
				update(state => ({ 
					...state, 
					loading: false, 
					lastUpdate: new Date() 
				}));
			} catch (error) {
				update(state => ({ 
					...state, 
					loading: false, 
					error: `Failed to initialize system: ${error}` 
				}));
			}
		},

		// Fetch system status from BPI gateway
		async fetchStatus(): Promise<SystemStatus> {
			const response = await fetch('/api/system/status');
			if (!response.ok) {
				throw new Error(`HTTP ${response.status}: ${response.statusText}`);
			}
			
			const status = await response.json();
			update(state => ({ ...state, status }));
			return status;
		},

		// Fetch node metrics
		async fetchMetrics(): Promise<NodeMetrics> {
			const response = await fetch('/api/system/metrics');
			if (!response.ok) {
				throw new Error(`HTTP ${response.status}: ${response.statusText}`);
			}
			
			const metrics = await response.json();
			update(state => ({ ...state, metrics }));
			return metrics;
		},

		// Fetch network information
		async fetchNetworkInfo(): Promise<NetworkInfo> {
			const response = await fetch('/api/network/info');
			if (!response.ok) {
				throw new Error(`HTTP ${response.status}: ${response.statusText}`);
			}
			
			const network = await response.json();
			update(state => ({ ...state, network }));
			return network;
		},

		// Update system status
		setStatus(status: SystemStatus) {
			update(state => ({ ...state, status, lastUpdate: new Date() }));
		},

		// Update metrics
		setMetrics(metrics: NodeMetrics) {
			update(state => ({ ...state, metrics, lastUpdate: new Date() }));
		},

		// Set loading state
		setLoading(loading: boolean) {
			update(state => ({ ...state, loading }));
		},

		// Set error
		setError(error: string) {
			update(state => ({ ...state, error }));
		},

		// Clear error
		clearError() {
			update(state => ({ ...state, error: null }));
		},

		// Refresh all data
		async refresh() {
			await this.initialize();
		}
	};
}

export const systemStore = createSystemStore();

// Derived stores for specific data
export const systemStatus = derived(systemStore, $store => $store.status);
export const systemMetrics = derived(systemStore, $store => $store.metrics);
export const networkInfo = derived(systemStore, $store => $store.network);
export const isSystemHealthy = derived(systemStore, $store => 
	$store.status?.health === 'healthy' && !$store.error
);
