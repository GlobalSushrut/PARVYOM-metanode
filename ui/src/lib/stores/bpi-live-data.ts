import { writable, derived } from 'svelte/store';
import type { Writable } from 'svelte/store';

// Real BPI Core Component Interfaces based on actual codebase
export interface BPILiveData {
	// Metanode Core Components
	metanode_core: {
		version: string;
		uptime: number;
		status: 'Running' | 'Stopped' | 'Error' | 'Maintenance';
		components: {
			math: ComponentStatus;
			mempool: ComponentStatus;
			gateway: ComponentStatus;
			merkle: ComponentStatus;
			vrf: ComponentStatus;
			receipts: ComponentStatus;
			billing: ComponentStatus;
			dashboard: ComponentStatus;
			config: ComponentStatus;
			http: ComponentStatus;
			shadow_registry: ComponentStatus;
			notary: ComponentStatus;
			court: ComponentStatus;
			auditing: ComponentStatus;
			inclusion: ComponentStatus;
		};
	};

	// Security Components
	metanode_security: {
		encryption: {
			bpi_enc_active: boolean;
			encryption_enabled: boolean;
			keys_rotated: number;
			last_key_rotation: string;
			encryption_strength: string;
		};
		auditing: {
			split_origin_active: boolean;
			audit_entries: number;
			last_audit: string;
			compliance_score: number;
		};
		court_node: {
			active: boolean;
			disputes_processed: number;
			pending_disputes: number;
			resolution_rate: number;
		};
		notary_registry: {
			registered_notaries: number;
			active_notarizations: number;
			verification_rate: number;
		};
		shadow_registry: {
			entries: number;
			cross_chain_verifications: number;
			shadow_sync_status: string;
		};
	};

	// Consensus Components
	metanode_consensus: {
		status: 'Active' | 'Syncing' | 'Offline';
		block_height: number;
		finalized_height: number;
		validators: {
			total: number;
			active: number;
			slashed: number;
		};
		performance: {
			tps: number;
			block_time: number;
			finality_time: number;
		};
	};

	// Economics Components
	metanode_economics: {
		billing: {
			total_revenue: number;
			active_subscriptions: number;
			resource_usage: {
				compute: number;
				storage: number;
				bandwidth: number;
			};
		};
		fees: {
			base_fee: number;
			priority_fee: number;
			total_fees_collected: number;
		};
	};

	// Stamped Wallets
	stamped_wallets: {
		total_wallets: number;
		active_wallets: number;
		transactions_processed: number;
		wallet_security_score: number;
	};

	// BLS Aggregation
	blsagg: {
		signatures_aggregated: number;
		verification_rate: number;
		aggregation_efficiency: number;
	};

	// Light Client
	bpi_light_client: {
		connected_clients: number;
		sync_status: string;
		data_served: number;
	};

	// Validator Components
	validator: {
		status: 'Active' | 'Inactive' | 'Jailed';
		stake: number;
		rewards: number;
		uptime: number;
		slashing_events: number;
	};

	// Mempool
	mempool: {
		pending_transactions: number;
		pool_size_mb: number;
		throughput: number;
		fee_estimation: {
			low: number;
			medium: number;
			high: number;
		};
	};

	// Inclusion Lists
	inclusion_lists: {
		active_lists: number;
		inclusion_rate: number;
		pending_inclusions: number;
	};

	// DockLock (Container Management)
	docklock: {
		containers: {
			total: number;
			running: number;
			stopped: number;
			failed: number;
		};
		clusters: {
			active_clusters: number;
			total_nodes: number;
			resource_utilization: {
				cpu: number;
				memory: number;
				storage: number;
			};
		};
		encryption_clusters: {
			active: number;
			key_rotations: number;
			encryption_load: number;
		};
	};

	// Network Metrics
	network: {
		peers: number;
		bandwidth_in: number;
		bandwidth_out: number;
		latency: number;
		packet_loss: number;
	};

	// System Resources
	system: {
		cpu_usage: number;
		memory_usage: number;
		disk_usage: number;
		network_io: number;
		temperature: number;
	};
}

interface ComponentStatus {
	status: 'Running' | 'Stopped' | 'Error' | 'Maintenance';
	uptime: number;
	last_error?: string;
	metrics?: Record<string, number>;
}

interface BPIDataStore {
	data: BPILiveData | null;
	loading: boolean;
	error: string | null;
	lastUpdate: Date | null;
	connected: boolean;
}


// Create the main BPI live data store
function createBPILiveDataStore() {
	const { subscribe, set, update }: Writable<BPIDataStore> = writable({
		data: null,
		loading: false,
		error: null,
		lastUpdate: null,
		connected: false
	});

	let ws: WebSocket | null = null;
	let reconnectTimer: ReturnType<typeof setTimeout> | null = null;
	let fetchInterval: ReturnType<typeof setInterval> | null = null;

	// Fetch live data from BPI Core APIs
	async function fetchLiveData(): Promise<void> {
		try {
			update(state => ({ ...state, loading: true, error: null }));

			// Fetch from multiple BPI Core endpoints
			const [
				coreStatus,
				securityMetrics,
				consensusData,
				economicsData,
				dockerMetrics,
				networkStats
			] = await Promise.all([
				fetch('/api/bpi/metanode/core/status').then(r => r.json()),
				fetch('/api/bpi/metanode/security/metrics').then(r => r.json()),
				fetch('/api/bpi/metanode/consensus/status').then(r => r.json()),
				fetch('/api/bpi/metanode/economics/metrics').then(r => r.json()),
				fetch('/api/bpi/docklock/status').then(r => r.json()),
				fetch('/api/bpi/network/stats').then(r => r.json())
			]);

			const liveData: BPILiveData = {
				metanode_core: coreStatus,
				metanode_security: securityMetrics,
				metanode_consensus: consensusData,
				metanode_economics: economicsData,
				stamped_wallets: coreStatus.stamped_wallets || {},
				blsagg: coreStatus.blsagg || {},
				bpi_light_client: coreStatus.light_client || {},
				validator: consensusData.validator || {},
				mempool: coreStatus.mempool || {},
				inclusion_lists: coreStatus.inclusion_lists || {},
				docklock: dockerMetrics,
				network: networkStats,
				system: coreStatus.system || {}
			};

			set({
				data: liveData,
				loading: false,
				error: null,
				lastUpdate: new Date(),
				connected: true
			});

		} catch (err) {
			const errorMessage = err instanceof Error ? err.message : 'Unknown error';
			update(state => ({
				...state,
				loading: false,
				error: errorMessage,
				connected: false
			}));
		}
	}

	// Initialize WebSocket for real-time updates
	function initWebSocket(): void {
		if (ws?.readyState === WebSocket.OPEN) return;

		try {
			ws = new WebSocket('ws://localhost:8617/ws/bpi/live');
			
			ws.onopen = () => {
				console.log('BPI Live Data WebSocket connected');
				update(state => ({ ...state, connected: true, error: null }));
			};

			ws.onmessage = (event) => {
				try {
					const liveData: BPILiveData = JSON.parse(event.data);
					update(state => ({
						...state,
						data: liveData,
						lastUpdate: new Date(),
						connected: true
					}));
				} catch (err) {
					console.error('Failed to parse WebSocket data:', err);
				}
			};

			ws.onclose = () => {
				console.log('BPI Live Data WebSocket disconnected');
				update(state => ({ ...state, connected: false }));
				scheduleReconnect();
			};

			ws.onerror = (error) => {
				console.error('BPI Live Data WebSocket error:', error);
				update(state => ({ 
					...state, 
					connected: false, 
					error: 'WebSocket connection failed' 
				}));
			};

		} catch (err) {
			console.error('Failed to initialize WebSocket:', err);
			scheduleReconnect();
		}
	}

	function scheduleReconnect(): void {
		if (reconnectTimer) return;
		
		reconnectTimer = setTimeout(() => {
			reconnectTimer = null;
			initWebSocket();
		}, 5000);
	}

	// Start periodic data fetching
	function startDataFetching(): void {
		fetchLiveData(); // Initial fetch
		fetchInterval = setInterval(fetchLiveData, 10000); // Every 10 seconds
	}

	// Stop all connections and intervals
	function cleanup(): void {
		if (ws) {
			ws.close();
			ws = null;
		}
		if (reconnectTimer) {
			clearTimeout(reconnectTimer);
			reconnectTimer = null;
		}
		if (fetchInterval) {
			clearInterval(fetchInterval);
			fetchInterval = null;
		}
	}

	return {
		subscribe,
		fetchLiveData,
		initWebSocket,
		startDataFetching,
		cleanup,
		// Manual refresh
		refresh: fetchLiveData
	};
}

export const bpiLiveDataStore = createBPILiveDataStore();

// Derived stores for specific components
export const coreComponentsStore = derived(
	bpiLiveDataStore,
	($store) => $store.data?.metanode_core
);

export const securityStore = derived(
	bpiLiveDataStore,
	($store) => $store.data?.metanode_security
);

export const consensusStore = derived(
	bpiLiveDataStore,
	($store) => $store.data?.metanode_consensus
);

export const dockLockStore = derived(
	bpiLiveDataStore,
	($store) => $store.data?.docklock
);

export const networkStore = derived(
	bpiLiveDataStore,
	($store) => $store.data?.network
);

export const economicsStore = derived(
	bpiLiveDataStore,
	($store) => $store.data?.metanode_economics
);

// Utility functions for data formatting
export function formatUptime(seconds: number): string {
	const days = Math.floor(seconds / 86400);
	const hours = Math.floor((seconds % 86400) / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	
	if (days > 0) return `${days}d ${hours}h ${minutes}m`;
	if (hours > 0) return `${hours}h ${minutes}m`;
	return `${minutes}m`;
}

export function formatBytes(bytes: number): string {
	const units = ['B', 'KB', 'MB', 'GB', 'TB'];
	let size = bytes;
	let unitIndex = 0;
	
	while (size >= 1024 && unitIndex < units.length - 1) {
		size /= 1024;
		unitIndex++;
	}
	
	return `${size.toFixed(1)} ${units[unitIndex]}`;
}

export function getStatusColor(status: string): string {
	switch (status.toLowerCase()) {
		case 'running': case 'active': return 'text-green-400';
		case 'stopped': case 'inactive': return 'text-gray-400';
		case 'error': case 'failed': return 'text-red-400';
		case 'maintenance': case 'syncing': return 'text-yellow-400';
		default: return 'text-gray-400';
	}
}
