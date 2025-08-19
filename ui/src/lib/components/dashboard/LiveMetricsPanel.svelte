<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import GrafanaPanel from './GrafanaPanel.svelte';
	import { consensusStore, economicsStore, networkStore } from '$lib/stores/bpi-live-data';
	import { Chart, registerables } from 'chart.js';

	Chart.register(...registerables);

	let chartCanvas: HTMLCanvasElement;
	let chart: Chart | null = null;
	let metricsHistory: Array<{
		timestamp: Date;
		tps: number;
		blockTime: number;
		validators: number;
		networkLatency: number;
	}> = [];

	function updateChart() {
		if (!chart || !$consensusStore || !$networkStore) return;

		// Add new data point
		metricsHistory.push({
			timestamp: new Date(),
			tps: $consensusStore.performance.tps,
			blockTime: $consensusStore.performance.block_time,
			validators: $consensusStore.validators.active,
			networkLatency: $networkStore.latency
		});

		// Keep only last 50 data points
		if (metricsHistory.length > 50) {
			metricsHistory = metricsHistory.slice(-50);
		}

		// Update chart data
		chart.data.labels = metricsHistory.map(m => m.timestamp.toLocaleTimeString());
		chart.data.datasets[0].data = metricsHistory.map(m => m.tps);
		chart.data.datasets[1].data = metricsHistory.map(m => m.blockTime);
		chart.data.datasets[2].data = metricsHistory.map(m => m.validators);
		chart.data.datasets[3].data = metricsHistory.map(m => m.networkLatency);
		
		chart.update('none');
	}

	onMount(() => {
		if (chartCanvas) {
			chart = new Chart(chartCanvas, {
				type: 'line',
				data: {
					labels: [],
					datasets: [
						{
							label: 'TPS',
							data: [],
							borderColor: '#60a5fa',
							backgroundColor: 'rgba(96, 165, 250, 0.1)',
							tension: 0.4,
							yAxisID: 'y'
						},
						{
							label: 'Block Time (ms)',
							data: [],
							borderColor: '#34d399',
							backgroundColor: 'rgba(52, 211, 153, 0.1)',
							tension: 0.4,
							yAxisID: 'y1'
						},
						{
							label: 'Active Validators',
							data: [],
							borderColor: '#f59e0b',
							backgroundColor: 'rgba(245, 158, 11, 0.1)',
							tension: 0.4,
							yAxisID: 'y2'
						},
						{
							label: 'Network Latency (ms)',
							data: [],
							borderColor: '#ef4444',
							backgroundColor: 'rgba(239, 68, 68, 0.1)',
							tension: 0.4,
							yAxisID: 'y3'
						}
					]
				},
				options: {
					responsive: true,
					maintainAspectRatio: false,
					plugins: {
						legend: {
							labels: {
								color: '#d1d5db',
								font: { family: 'monospace' }
							}
						}
					},
					scales: {
						x: {
							ticks: { color: '#9ca3af', font: { family: 'monospace' } },
							grid: { color: '#374151' }
						},
						y: {
							type: 'linear',
							display: true,
							position: 'left',
							ticks: { color: '#60a5fa', font: { family: 'monospace' } },
							grid: { color: '#374151' }
						},
						y1: {
							type: 'linear',
							display: false,
							position: 'right'
						},
						y2: {
							type: 'linear',
							display: false,
							position: 'right'
						},
						y3: {
							type: 'linear',
							display: false,
							position: 'right'
						}
					},
					animation: { duration: 0 }
				}
			});
		}

		// Update chart every 5 seconds
		const interval = setInterval(updateChart, 5000);
		return () => clearInterval(interval);
	});

	onDestroy(() => {
		if (chart) {
			chart.destroy();
		}
	});

	$: if ($consensusStore && $networkStore) {
		updateChart();
	}
</script>

<GrafanaPanel
	title="Live Performance Metrics"
	subtitle="Real-time consensus, network, and economic performance monitoring"
	height="h-96"
	loading={!$consensusStore || !$networkStore || !$economicsStore}
	lastUpdate={$consensusStore ? new Date() : null}
>
	{#if $consensusStore && $networkStore && $economicsStore}
		<div class="grid grid-cols-3 gap-4 mb-4">
			<!-- Consensus Metrics -->
			<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
				<h5 class="text-sm font-mono text-gray-300 mb-3">⛓️ CONSENSUS</h5>
				<div class="space-y-2 text-xs">
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Status:</span>
						<span class="font-mono {$consensusStore.status === 'Active' ? 'text-green-400' : 'text-yellow-400'}">
							{$consensusStore.status.toUpperCase()}
						</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Block Height:</span>
						<span class="text-blue-400 font-mono">{$consensusStore.block_height.toLocaleString()}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Finalized:</span>
						<span class="text-green-400 font-mono">{$consensusStore.finalized_height.toLocaleString()}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">TPS:</span>
						<span class="text-purple-400 font-mono font-bold">{$consensusStore.performance.tps}</span>
					</div>
				</div>
			</div>

			<!-- Network Metrics -->
			<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
				<h5 class="text-sm font-mono text-gray-300 mb-3">🌐 NETWORK</h5>
				<div class="space-y-2 text-xs">
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Peers:</span>
						<span class="text-blue-400 font-mono">{$networkStore.peers}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Latency:</span>
						<span class="text-green-400 font-mono">{$networkStore.latency}ms</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Bandwidth In:</span>
						<span class="text-yellow-400 font-mono">{($networkStore.bandwidth_in / 1024 / 1024).toFixed(1)} MB/s</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Packet Loss:</span>
						<span class="text-red-400 font-mono">{$networkStore.packet_loss}%</span>
					</div>
				</div>
			</div>

			<!-- Economic Metrics -->
			<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
				<h5 class="text-sm font-mono text-gray-300 mb-3">💰 ECONOMICS</h5>
				<div class="space-y-2 text-xs">
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Revenue:</span>
						<span class="text-green-400 font-mono">${$economicsStore.billing.total_revenue.toLocaleString()}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Subscriptions:</span>
						<span class="text-blue-400 font-mono">{$economicsStore.billing.active_subscriptions}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Base Fee:</span>
						<span class="text-yellow-400 font-mono">{$economicsStore.fees.base_fee} gwei</span>
					</div>
					<div class="flex justify-between">
						<span class="text-gray-400 font-mono">Total Fees:</span>
						<span class="text-purple-400 font-mono">{$economicsStore.fees.total_fees_collected.toLocaleString()}</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Live Chart -->
		<div class="bg-gray-800 rounded-lg p-4 border border-gray-700 h-64">
			<canvas bind:this={chartCanvas} class="w-full h-full"></canvas>
		</div>
	{:else}
		<div class="flex items-center justify-center h-full">
			<div class="text-center">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-400 mx-auto mb-4"></div>
				<p class="text-gray-400 font-mono">Loading Live Metrics...</p>
			</div>
		</div>
	{/if}
</GrafanaPanel>
