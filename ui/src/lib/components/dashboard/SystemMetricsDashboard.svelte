<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Chart, registerables } from 'chart.js';
	
	Chart.register(...registerables);

	// System metrics data
	let systemMetrics = {
		memory: {
			usage: 73,
			available: 71,
			used: 23,
			reserved: 6,
			total: '32 GB',
			used_gb: '7.4 GB',
			available_gb: '23.2 GB',
			reserved_gb: '1.9 GB'
		},
		cpu: {
			usage: [8, 7, 7, 7], // CPU 0-3
			temperature: 42,
			frequency: [2.4, 2.4, 2.5, 2.5], // GHz
			voltage: 918.75,
			power: 75
		},
		gpu: {
			usage: [6, 6, 5, 5],
			memory: 34,
			temperature: 45,
			power: 100,
			voltage: 0.04720,
			fan: 0
		},
		storage: {
			nvme: { usage: 96, health: 100, temp: 45 },
			ssd: { usage: 100, health: 100, temp: 40 },
			hdd: { usage: 100, health: 100, temp: 35 }
		},
		network: {
			bandwidth_usage: 30,
			upload: 24,
			download: 6
		},
		system: {
			uptime: '1d 21:06:30',
			power: 59.0,
			voltage: 215.1,
			current: 18,
			frequency: 4091
		}
	};

	let charts: { [key: string]: Chart } = {};
	let updateInterval: ReturnType<typeof setInterval>;

	onMount(() => {
		initializeCharts();
		startMetricsUpdate();
	});

	onDestroy(() => {
		Object.values(charts).forEach(chart => chart.destroy());
		if (updateInterval) clearInterval(updateInterval);
	});

	function initializeCharts() {
		// Memory History Chart
		const memoryCtx = document.getElementById('memoryHistoryChart') as HTMLCanvasElement;
		if (memoryCtx) {
			charts.memory = new Chart(memoryCtx, {
				type: 'line',
				data: {
					labels: Array.from({length: 50}, (_, i) => ''),
					datasets: [{
						label: 'Physical Memory',
						data: generateRandomData(50, 20, 80),
						borderColor: '#10b981',
						backgroundColor: 'rgba(16, 185, 129, 0.1)',
						borderWidth: 2,
						fill: true,
						tension: 0.4
					}, {
						label: 'Virtual Memory',
						data: generateRandomData(50, 15, 60),
						borderColor: '#3b82f6',
						backgroundColor: 'rgba(59, 130, 246, 0.1)',
						borderWidth: 2,
						fill: true,
						tension: 0.4
					}]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					plugins: { legend: { display: false } },
					scales: {
						x: { display: false },
						y: { display: false }
					},
					elements: { point: { radius: 0 } }
				}
			});
		}

		// CPU Usage Chart
		const cpuCtx = document.getElementById('cpuUsageChart') as HTMLCanvasElement;
		if (cpuCtx) {
			charts.cpu = new Chart(cpuCtx, {
				type: 'line',
				data: {
					labels: Array.from({length: 100}, (_, i) => ''),
					datasets: [{
						data: generateRandomData(100, 5, 15),
						borderColor: '#f59e0b',
						backgroundColor: 'rgba(245, 158, 11, 0.2)',
						borderWidth: 1,
						fill: true,
						tension: 0.4
					}]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					plugins: { legend: { display: false } },
					scales: {
						x: { display: false },
						y: { display: false }
					},
					elements: { point: { radius: 0 } }
				}
			});
		}

		// Temperature Chart
		const tempCtx = document.getElementById('temperatureChart') as HTMLCanvasElement;
		if (tempCtx) {
			charts.temperature = new Chart(tempCtx, {
				type: 'line',
				data: {
					labels: Array.from({length: 50}, (_, i) => ''),
					datasets: [{
						label: 'CPU Temp',
						data: generateRandomData(50, 40, 50),
						borderColor: '#ef4444',
						borderWidth: 2,
						fill: false,
						tension: 0.4
					}, {
						label: 'GPU Temp',
						data: generateRandomData(50, 35, 45),
						borderColor: '#8b5cf6',
						borderWidth: 2,
						fill: false,
						tension: 0.4
					}]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					plugins: { legend: { display: false } },
					scales: {
						x: { display: false },
						y: { display: false }
					},
					elements: { point: { radius: 0 } }
				}
			});
		}

		// Power Generation Chart
		const powerCtx = document.getElementById('powerChart') as HTMLCanvasElement;
		if (powerCtx) {
			charts.power = new Chart(powerCtx, {
				type: 'line',
				data: {
					labels: Array.from({length: 30}, (_, i) => ''),
					datasets: [{
						label: 'Current',
						data: generateRandomData(30, 2, 4),
						borderColor: '#10b981',
						backgroundColor: 'rgba(16, 185, 129, 0.2)',
						borderWidth: 2,
						fill: true,
						tension: 0.4
					}, {
						label: 'Total',
						data: generateRandomData(30, 40, 50),
						borderColor: '#f59e0b',
						backgroundColor: 'rgba(245, 158, 11, 0.2)',
						borderWidth: 2,
						fill: true,
						tension: 0.4
					}]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					plugins: { legend: { display: false } },
					scales: {
						x: { display: false },
						y: { display: false }
					},
					elements: { point: { radius: 0 } }
				}
			});
		}
	}

	function generateRandomData(length: number, min: number, max: number): number[] {
		return Array.from({length}, () => Math.random() * (max - min) + min);
	}

	function startMetricsUpdate() {
		updateInterval = setInterval(() => {
			// Update charts with new data
			Object.values(charts).forEach(chart => {
				if (chart && chart.data.datasets) {
					chart.data.datasets.forEach(dataset => {
						if (dataset.data) {
							dataset.data.shift();
							dataset.data.push(Math.random() * 100);
						}
					});
					chart.update('none');
				}
			});

			// Update system metrics
			systemMetrics.cpu.usage = systemMetrics.cpu.usage.map(() => Math.floor(Math.random() * 15) + 5);
			systemMetrics.memory.usage = Math.floor(Math.random() * 10) + 70;
			systemMetrics.gpu.usage = systemMetrics.gpu.usage.map(() => Math.floor(Math.random() * 10) + 3);
		}, 2000);
	}

	function formatUptime(uptime: string): string {
		return uptime;
	}

	function getUsageColor(usage: number): string {
		if (usage < 30) return 'text-green-400';
		if (usage < 70) return 'text-yellow-400';
		return 'text-red-400';
	}

	function getHealthColor(health: number): string {
		if (health >= 95) return 'text-green-400';
		if (health >= 80) return 'text-yellow-400';
		return 'text-red-400';
	}
</script>

<div class="min-h-screen bg-gray-900 text-white p-4">
	<!-- Header -->
	<div class="mb-6">
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-2xl font-bold text-white">BPI System Monitoring</h1>
				<p class="text-gray-400">Real-time infrastructure metrics and performance monitoring</p>
			</div>
			<div class="flex items-center space-x-4">
				<div class="bg-green-500/20 border border-green-500/30 rounded-lg px-3 py-1">
					<div class="flex items-center space-x-2">
						<div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
						<span class="text-sm text-green-300">All Systems Operational</span>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Main Grid -->
	<div class="grid grid-cols-12 gap-4">
		<!-- Memory Usage (Large Donut) -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">BPI Memory Usage (%)</h3>
			<div class="relative flex items-center justify-center h-32">
				<div class="relative w-28 h-28">
					<svg class="w-28 h-28 transform -rotate-90" viewBox="0 0 100 100">
						<circle cx="50" cy="50" r="40" stroke="currentColor" stroke-width="8" fill="none" class="text-gray-700"/>
						<circle cx="50" cy="50" r="40" stroke="currentColor" stroke-width="8" fill="none" 
							stroke-dasharray="{2 * Math.PI * 40}" 
							stroke-dashoffset="{2 * Math.PI * 40 * (1 - systemMetrics.memory.usage / 100)}"
							class="text-blue-500 transition-all duration-1000"/>
					</svg>
					<div class="absolute inset-0 flex items-center justify-center">
						<div class="text-center">
							<div class="text-2xl font-bold text-white">{systemMetrics.memory.usage}%</div>
							<div class="text-xs text-gray-400">{systemMetrics.memory.used_gb}</div>
						</div>
					</div>
				</div>
			</div>
			<div class="mt-4 space-y-2">
				<div class="flex justify-between text-sm">
					<span class="text-green-400">Available</span>
					<span class="text-white">{systemMetrics.memory.available}%</span>
				</div>
				<div class="flex justify-between text-sm">
					<span class="text-blue-400">Used</span>
					<span class="text-white">{systemMetrics.memory.used}%</span>
				</div>
				<div class="flex justify-between text-sm">
					<span class="text-gray-400">Reserved</span>
					<span class="text-white">{systemMetrics.memory.reserved}%</span>
				</div>
			</div>
		</div>

		<!-- Memory History Chart -->
		<div class="col-span-6 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">Memory History</h3>
			<div class="h-32">
				<canvas id="memoryHistoryChart"></canvas>
			</div>
			<div class="mt-2 grid grid-cols-4 gap-4 text-xs">
				<div>
					<div class="text-gray-400">40 GB</div>
					<div class="text-gray-400">32 GB</div>
					<div class="text-gray-400">24 GB</div>
					<div class="text-gray-400">16 GB</div>
					<div class="text-gray-400">8 GB</div>
				</div>
			</div>
		</div>

		<!-- Memory Clock -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">Memory Clock</h3>
			<div class="text-center">
				<div class="text-4xl font-bold text-blue-400 mb-2">{systemMetrics.system.frequency}<span class="text-lg">MHz</span></div>
				<div class="text-sm text-gray-400 mb-4">Memory Latency</div>
				<div class="grid grid-cols-4 gap-2 text-xs">
					<div class="text-center">
						<div class="text-white font-mono">18</div>
						<div class="text-gray-400">tCAS</div>
					</div>
					<div class="text-center">
						<div class="text-white font-mono">22</div>
						<div class="text-gray-400">tRCD</div>
					</div>
					<div class="text-center">
						<div class="text-white font-mono">22</div>
						<div class="text-gray-400">tRP</div>
					</div>
					<div class="text-center">
						<div class="text-white font-mono">39</div>
						<div class="text-gray-400">tRAS</div>
					</div>
				</div>
			</div>
		</div>

		<!-- System Power Metrics -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">System Power Metrics</h3>
			<div class="space-y-3">
				<div>
					<div class="text-xs text-gray-400">Power</div>
					<div class="text-2xl font-bold text-green-400">{systemMetrics.system.power}<span class="text-sm">W</span></div>
				</div>
				<div>
					<div class="text-xs text-gray-400">Current</div>
					<div class="text-lg font-bold text-white">{systemMetrics.system.current} <span class="text-sm">mA</span></div>
				</div>
				<div>
					<div class="text-xs text-gray-400">Voltage</div>
					<div class="text-lg font-bold text-white">{systemMetrics.system.voltage}<span class="text-sm">V</span></div>
				</div>
			</div>
			<div class="mt-4 h-16">
				<canvas id="powerChart"></canvas>
			</div>
		</div>

		<!-- BPI Uptime -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">BPI Uptime</h3>
			<div class="text-center">
				<div class="text-3xl font-bold text-red-400 font-mono">{formatUptime(systemMetrics.system.uptime)}</div>
			</div>
		</div>

		<!-- Server Uptime -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">Server Uptime</h3>
			<div class="text-center">
				<div class="text-3xl font-bold text-purple-400 font-mono">1d 14:45:38</div>
			</div>
		</div>

		<!-- CPU Usage Chart -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">CPU Usage (%)</h3>
			<div class="h-32">
				<canvas id="cpuUsageChart"></canvas>
			</div>
			<div class="mt-2 grid grid-cols-4 gap-2">
				{#each systemMetrics.cpu.usage as usage, i}
					<div class="text-center">
						<div class="text-lg font-bold {getUsageColor(usage)}">{usage}%</div>
						<div class="text-xs text-gray-400">CPU {i}</div>
					</div>
				{/each}
			</div>
		</div>

		<!-- CPU Frequency -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">CPU Frequency</h3>
			<div class="space-y-2">
				{#each systemMetrics.cpu.frequency as freq, i}
					<div class="flex justify-between items-center">
						<span class="text-sm text-gray-400">CPU {i}</span>
						<div class="flex items-center space-x-2">
							<div class="w-16 bg-gray-700 rounded-full h-2">
								<div class="bg-blue-500 h-2 rounded-full" style="width: {(freq / 3) * 100}%"></div>
							</div>
							<span class="text-sm text-white font-mono">{freq} GHz</span>
						</div>
					</div>
				{/each}
			</div>
		</div>

		<!-- PIKVIM Memory Usage -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">PIKVIM Memory Usage (%)</h3>
			<div class="relative flex items-center justify-center h-32">
				<div class="relative w-28 h-28">
					<svg class="w-28 h-28 transform -rotate-90" viewBox="0 0 100 100">
						<circle cx="50" cy="50" r="40" stroke="currentColor" stroke-width="8" fill="none" class="text-gray-700"/>
						<circle cx="50" cy="50" r="40" stroke="currentColor" stroke-width="8" fill="none" 
							stroke-dasharray="{2 * Math.PI * 40}" 
							stroke-dashoffset="{2 * Math.PI * 40 * (1 - 73 / 100)}"
							class="text-green-500 transition-all duration-1000"/>
					</svg>
					<div class="absolute inset-0 flex items-center justify-center">
						<div class="text-center">
							<div class="text-2xl font-bold text-white">73%</div>
							<div class="text-xs text-gray-400">5.8 GB</div>
						</div>
					</div>
				</div>
			</div>
			<div class="mt-4 space-y-2">
				<div class="flex justify-between text-sm">
					<span class="text-green-400">Available</span>
					<span class="text-white">73%</span>
				</div>
				<div class="flex justify-between text-sm">
					<span class="text-blue-400">Reserved</span>
					<span class="text-white">3%</span>
				</div>
			</div>
		</div>

		<!-- System Temperature -->
		<div class="col-span-6 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">System Temperature (°C)</h3>
			<div class="h-32">
				<canvas id="temperatureChart"></canvas>
			</div>
			<div class="mt-2 grid grid-cols-4 gap-4 text-xs">
				<div class="text-center">
					<div class="text-red-400">48°C</div>
					<div class="text-gray-400">Raspberry Pi 4</div>
				</div>
				<div class="text-center">
					<div class="text-yellow-400">45°C</div>
					<div class="text-gray-400">PIKVIM</div>
				</div>
				<div class="text-center">
					<div class="text-green-400">40°C</div>
					<div class="text-gray-400">SSD</div>
				</div>
				<div class="text-center">
					<div class="text-blue-400">35°C</div>
					<div class="text-gray-400">HDD</div>
				</div>
			</div>
		</div>

		<!-- GPU Usage -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">GPU / VRAM Usage (%)</h3>
			<div class="space-y-3">
				{#each systemMetrics.gpu.usage as usage, i}
					<div class="flex justify-between items-center">
						<span class="text-sm text-gray-400">GPU {i}</span>
						<div class="flex items-center space-x-2">
							<div class="w-16 bg-gray-700 rounded-full h-2">
								<div class="bg-purple-500 h-2 rounded-full" style="width: {usage * 10}%"></div>
							</div>
							<span class="text-sm {getUsageColor(usage)} font-mono">{usage}%</span>
						</div>
					</div>
				{/each}
			</div>
			<div class="mt-4">
				<div class="text-xs text-gray-400">VRAM Usage</div>
				<div class="text-lg font-bold text-purple-400">{systemMetrics.gpu.memory}%</div>
			</div>
		</div>

		<!-- GPU Power & Voltage -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">GPU Power</h3>
			<div class="space-y-4">
				<div>
					<div class="text-xs text-gray-400">Power</div>
					<div class="text-2xl font-bold text-red-400">{systemMetrics.gpu.power}W</div>
				</div>
				<div>
					<div class="text-xs text-gray-400">Voltage</div>
					<div class="text-4xl font-bold text-red-400">{systemMetrics.gpu.voltage}v</div>
				</div>
				<div>
					<div class="text-xs text-gray-400">Fan</div>
					<div class="text-2xl font-bold text-red-400">{systemMetrics.gpu.fan}rpm</div>
					<div class="text-lg {getUsageColor(systemMetrics.gpu.fan)}">{systemMetrics.gpu.fan}%</div>
				</div>
			</div>
		</div>

		<!-- Storage Health -->
		<div class="col-span-6 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">Drive Health Status (%)</h3>
			<div class="space-y-3">
				<div class="flex justify-between items-center">
					<span class="text-sm text-gray-300">SSD: Patriot Burst</span>
					<div class="flex items-center space-x-4">
						<span class="text-2xl font-bold {getHealthColor(systemMetrics.storage.nvme.health)}">{systemMetrics.storage.nvme.usage}%</span>
					</div>
				</div>
				<div class="flex justify-between items-center">
					<span class="text-sm text-gray-300">SSD: SSV5</span>
					<div class="flex items-center space-x-4">
						<span class="text-2xl font-bold {getHealthColor(systemMetrics.storage.ssd.health)}">{systemMetrics.storage.ssd.usage}%</span>
					</div>
				</div>
				<div class="flex justify-between items-center">
					<span class="text-sm text-gray-300">NVME: SN750</span>
					<div class="flex items-center space-x-4">
						<span class="text-2xl font-bold {getHealthColor(systemMetrics.storage.hdd.health)}">{systemMetrics.storage.hdd.usage}%</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Partition Usage -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">Partition Usage (%)</h3>
			<div class="space-y-2">
				<div class="flex justify-between">
					<span class="text-xs text-gray-400">NVME</span>
					<span class="text-sm text-purple-400">43.49%</span>
				</div>
				<div class="flex justify-between">
					<span class="text-xs text-gray-400">SSD</span>
					<span class="text-sm text-blue-400">133%</span>
				</div>
				<div class="flex justify-between">
					<span class="text-xs text-gray-400">SSD2</span>
					<span class="text-sm text-green-400">113%</span>
				</div>
				<div class="flex justify-between">
					<span class="text-xs text-gray-400">HDD</span>
					<span class="text-sm text-yellow-400">3.09%</span>
				</div>
				<div class="flex justify-between">
					<span class="text-xs text-gray-400">HDD2</span>
					<span class="text-sm text-red-400">5.39%</span>
				</div>
				<div class="flex justify-between">
					<span class="text-xs text-gray-400">USB</span>
					<span class="text-sm text-gray-400">59.17%</span>
				</div>
			</div>
		</div>

		<!-- Network Bandwidth -->
		<div class="col-span-3 bg-gray-800 rounded-lg p-4">
			<h3 class="text-sm font-medium text-gray-300 mb-4">Bandwidth Usage</h3>
			<div class="h-32 flex items-end justify-center space-x-1">
				{#each Array(20) as _, i}
					<div class="bg-gradient-to-t from-purple-600 to-blue-500 w-3 rounded-t" 
						 style="height: {Math.random() * 80 + 20}%"></div>
				{/each}
			</div>
			<div class="mt-2 flex justify-between text-xs">
				<span class="text-green-400">↑ {systemMetrics.network.upload} Mb/s</span>
				<span class="text-blue-400">↓ {systemMetrics.network.download} Mb/s</span>
			</div>
		</div>
	</div>
</div>
