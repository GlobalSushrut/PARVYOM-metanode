import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

// System metrics interfaces
export interface SystemMetrics {
	memory: MemoryMetrics;
	cpu: CPUMetrics;
	gpu: GPUMetrics;
	storage: StorageMetrics;
	network: NetworkMetrics;
	system: SystemInfo;
	bpi: BPIMetrics;
}

export interface MemoryMetrics {
	usage: number;
	available: number;
	used: number;
	reserved: number;
	total: string;
	used_gb: string;
	available_gb: string;
	reserved_gb: string;
	history: number[];
	virtual_history: number[];
}

export interface CPUMetrics {
	usage: number[];
	temperature: number;
	frequency: number[];
	voltage: number;
	power: number;
	history: number[][];
}

export interface GPUMetrics {
	usage: number[];
	memory: number;
	temperature: number;
	power: number;
	voltage: number;
	fan: number;
	clock: number;
	memory_clock: number;
}

export interface StorageMetrics {
	nvme: StorageDevice;
	ssd: StorageDevice;
	hdd: StorageDevice;
	partitions: PartitionInfo[];
}

export interface StorageDevice {
	usage: number;
	health: number;
	temp: number;
	read_speed: number;
	write_speed: number;
}

export interface PartitionInfo {
	name: string;
	usage: number;
	free: string;
	total: string;
}

export interface NetworkMetrics {
	bandwidth_usage: number;
	upload: number;
	download: number;
	latency: number;
	packet_loss: number;
	history: { upload: number[]; download: number[] };
}

export interface SystemInfo {
	uptime: string;
	power: number;
	voltage: number;
	current: number;
	frequency: number;
	temperature: number;
	load_average: number[];
}

export interface BPIMetrics {
	nodes_active: number;
	consensus_health: number;
	transaction_rate: number;
	block_height: number;
	peer_count: number;
	sync_status: 'synced' | 'syncing' | 'offline';
	validator_status: 'active' | 'inactive' | 'slashed';
	staking_rewards: number;
	network_hash_rate: string;
}

// Create the main metrics store
function createSystemMetricsStore() {
	const initialMetrics: SystemMetrics = {
		memory: {
			usage: 73,
			available: 71,
			used: 23,
			reserved: 6,
			total: '32 GB',
			used_gb: '7.4 GB',
			available_gb: '23.2 GB',
			reserved_gb: '1.9 GB',
			history: Array(50).fill(0).map(() => Math.random() * 30 + 50),
			virtual_history: Array(50).fill(0).map(() => Math.random() * 20 + 30)
		},
		cpu: {
			usage: [8, 7, 7, 7],
			temperature: 42,
			frequency: [2.4, 2.4, 2.5, 2.5],
			voltage: 918.75,
			power: 75,
			history: Array(4).fill(0).map(() => Array(100).fill(0).map(() => Math.random() * 15 + 5))
		},
		gpu: {
			usage: [6, 6, 5, 5],
			memory: 34,
			temperature: 45,
			power: 100,
			voltage: 0.04720,
			fan: 0,
			clock: 1350,
			memory_clock: 6000
		},
		storage: {
			nvme: { usage: 96, health: 100, temp: 45, read_speed: 3500, write_speed: 3200 },
			ssd: { usage: 100, health: 100, temp: 40, read_speed: 550, write_speed: 520 },
			hdd: { usage: 100, health: 100, temp: 35, read_speed: 150, write_speed: 140 },
			partitions: [
				{ name: 'NVME', usage: 43.49, free: '512 GB', total: '1 TB' },
				{ name: 'SSD (D:)', usage: 133, free: '128 GB', total: '512 GB' },
				{ name: 'SSD2 (E:)', usage: 113, free: '64 GB', total: '256 GB' },
				{ name: 'HDD (F:)', usage: 3.09, free: '1.8 TB', total: '2 TB' },
				{ name: 'HDD2 (G:)', usage: 5.39, free: '3.6 TB', total: '4 TB' },
				{ name: 'USB', usage: 59.17, free: '32 GB', total: '64 GB' }
			]
		},
		network: {
			bandwidth_usage: 30,
			upload: 24,
			download: 6,
			latency: 12,
			packet_loss: 0.1,
			history: {
				upload: Array(50).fill(0).map(() => Math.random() * 50 + 10),
				download: Array(50).fill(0).map(() => Math.random() * 100 + 20)
			}
		},
		system: {
			uptime: '1d 21:06:30',
			power: 59.0,
			voltage: 215.1,
			current: 18,
			frequency: 4091,
			temperature: 38,
			load_average: [0.85, 1.2, 1.4]
		},
		bpi: {
			nodes_active: 247,
			consensus_health: 98.5,
			transaction_rate: 1247,
			block_height: 2847392,
			peer_count: 156,
			sync_status: 'synced',
			validator_status: 'active',
			staking_rewards: 24.7,
			network_hash_rate: '847.2 TH/s'
		}
	};

	const { subscribe, set, update } = writable<SystemMetrics>(initialMetrics);

	let updateInterval: ReturnType<typeof setInterval> | null = null;
	let wsConnection: WebSocket | null = null;

	// Simulate real-time data updates
	function startRealTimeUpdates() {
		if (!browser) return;

		// Try WebSocket connection first
		try {
			wsConnection = new WebSocket('ws://127.0.0.1:8617/ws/metrics');
			
			wsConnection.onmessage = (event) => {
				try {
					const data = JSON.parse(event.data);
					if (data.type === 'system_metrics') {
						update(metrics => ({
							...metrics,
							...data.payload
						}));
					}
				} catch (e) {
					console.warn('Failed to parse WebSocket metrics:', e);
				}
			};

			wsConnection.onerror = () => {
				console.warn('WebSocket connection failed, falling back to simulation');
				startSimulatedUpdates();
			};

			wsConnection.onclose = () => {
				console.warn('WebSocket connection closed, falling back to simulation');
				startSimulatedUpdates();
			};

		} catch (e) {
			console.warn('WebSocket not available, using simulated data');
			startSimulatedUpdates();
		}
	}

	function startSimulatedUpdates() {
		if (updateInterval) return;

		updateInterval = setInterval(() => {
			update(metrics => {
				// Update CPU usage
				const newCpuUsage = metrics.cpu.usage.map(usage => 
					Math.max(1, Math.min(100, usage + (Math.random() - 0.5) * 4))
				);

				// Update memory usage
				const newMemoryUsage = Math.max(60, Math.min(90, 
					metrics.memory.usage + (Math.random() - 0.5) * 3
				));

				// Update GPU usage
				const newGpuUsage = metrics.gpu.usage.map(usage => 
					Math.max(1, Math.min(100, usage + (Math.random() - 0.5) * 2))
				);

				// Update network
				const newUpload = Math.max(0, Math.min(100, 
					metrics.network.upload + (Math.random() - 0.5) * 10
				));
				const newDownload = Math.max(0, Math.min(200, 
					metrics.network.download + (Math.random() - 0.5) * 15
				));

				// Update BPI metrics
				const newTransactionRate = Math.max(800, Math.min(2000,
					metrics.bpi.transaction_rate + (Math.random() - 0.5) * 100
				));

				// Update histories
				const newMemoryHistory = [...metrics.memory.history.slice(1), newMemoryUsage];
				const newVirtualHistory = [...metrics.memory.virtual_history.slice(1), 
					Math.max(20, Math.min(80, metrics.memory.virtual_history[metrics.memory.virtual_history.length - 1] + (Math.random() - 0.5) * 5))
				];

				const newUploadHistory = [...metrics.network.history.upload.slice(1), newUpload];
				const newDownloadHistory = [...metrics.network.history.download.slice(1), newDownload];

				// Update CPU histories
				const newCpuHistory = metrics.cpu.history.map((history, i) => [
					...history.slice(1),
					newCpuUsage[i]
				]);

				return {
					...metrics,
					memory: {
						...metrics.memory,
						usage: Math.round(newMemoryUsage),
						used_gb: `${(newMemoryUsage * 0.32).toFixed(1)} GB`,
						available_gb: `${((100 - newMemoryUsage) * 0.32).toFixed(1)} GB`,
						history: newMemoryHistory,
						virtual_history: newVirtualHistory
					},
					cpu: {
						...metrics.cpu,
						usage: newCpuUsage.map(u => Math.round(u)),
						temperature: Math.round(Math.max(35, Math.min(65, 
							metrics.cpu.temperature + (Math.random() - 0.5) * 2
						))),
						history: newCpuHistory
					},
					gpu: {
						...metrics.gpu,
						usage: newGpuUsage.map(u => Math.round(u)),
						temperature: Math.round(Math.max(40, Math.min(80, 
							metrics.gpu.temperature + (Math.random() - 0.5) * 3
						))),
						memory: Math.round(Math.max(20, Math.min(90, 
							metrics.gpu.memory + (Math.random() - 0.5) * 2
						)))
					},
					network: {
						...metrics.network,
						upload: Math.round(newUpload),
						download: Math.round(newDownload),
						latency: Math.round(Math.max(5, Math.min(50, 
							metrics.network.latency + (Math.random() - 0.5) * 3
						))),
						history: {
							upload: newUploadHistory,
							download: newDownloadHistory
						}
					},
					system: {
						...metrics.system,
						power: Math.round((Math.max(45, Math.min(85, 
							metrics.system.power + (Math.random() - 0.5) * 2
						)) * 10)) / 10,
						temperature: Math.round(Math.max(30, Math.min(50, 
							metrics.system.temperature + (Math.random() - 0.5) * 1
						)))
					},
					bpi: {
						...metrics.bpi,
						transaction_rate: Math.round(newTransactionRate),
						consensus_health: Math.round((Math.max(95, Math.min(100, 
							metrics.bpi.consensus_health + (Math.random() - 0.5) * 0.5
						)) * 10)) / 10,
						block_height: metrics.bpi.block_height + (Math.random() > 0.7 ? 1 : 0),
						peer_count: Math.round(Math.max(100, Math.min(200, 
							metrics.bpi.peer_count + (Math.random() - 0.5) * 5
						)))
					}
				};
			});
		}, 2000);
	}

	function stopUpdates() {
		if (updateInterval) {
			clearInterval(updateInterval);
			updateInterval = null;
		}
		if (wsConnection) {
			wsConnection.close();
			wsConnection = null;
		}
	}

	// Auto-start updates in browser
	if (browser) {
		startRealTimeUpdates();
	}

	return {
		subscribe,
		set,
		update,
		startRealTimeUpdates,
		stopUpdates
	};
}

// Create the store instance
export const systemMetrics = createSystemMetricsStore();

// Derived stores for specific metrics
export const memoryMetrics = derived(systemMetrics, $metrics => $metrics.memory);
export const cpuMetrics = derived(systemMetrics, $metrics => $metrics.cpu);
export const gpuMetrics = derived(systemMetrics, $metrics => $metrics.gpu);
export const networkMetrics = derived(systemMetrics, $metrics => $metrics.network);
export const bpiMetrics = derived(systemMetrics, $metrics => $metrics.bpi);
export const systemInfo = derived(systemMetrics, $metrics => $metrics.system);

// Utility functions
export function formatBytes(bytes: number): string {
	const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
	if (bytes === 0) return '0 B';
	const i = Math.floor(Math.log(bytes) / Math.log(1024));
	return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
}

export function formatUptime(seconds: number): string {
	const days = Math.floor(seconds / 86400);
	const hours = Math.floor((seconds % 86400) / 3600);
	const minutes = Math.floor((seconds % 3600) / 60);
	const secs = seconds % 60;
	
	if (days > 0) {
		return `${days}d ${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
	}
	return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

export function getHealthColor(value: number, thresholds = { good: 80, warning: 60 }): string {
	if (value >= thresholds.good) return 'text-green-400';
	if (value >= thresholds.warning) return 'text-yellow-400';
	return 'text-red-400';
}

export function getUsageColor(value: number, thresholds = { low: 30, high: 70 }): string {
	if (value < thresholds.low) return 'text-green-400';
	if (value < thresholds.high) return 'text-yellow-400';
	return 'text-red-400';
}
