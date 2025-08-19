<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Chart, registerables } from 'chart.js';
	import 'chartjs-adapter-date-fns';
	import type { NodeMetrics } from '$lib/types/system';

	Chart.register(...registerables);

	export let metrics: NodeMetrics[];
	export let type: 'cpu' | 'memory' | 'disk' | 'network' = 'cpu';
	export let title: string = 'System Metrics';
	export let height: number = 300;

	let canvas: HTMLCanvasElement;
	let chart: Chart | null = null;

	// BPI color scheme
	const colors = {
		primary: '#8B9DC3', // Quantum Silver
		secondary: '#0A1628', // BPI Deep Blue
		success: '#10B981',
		warning: '#F59E0B',
		error: '#EF4444',
		grid: '#374151'
	};

	function getChartData() {
		if (!metrics || metrics.length === 0) return { labels: [], datasets: [] };

		const labels = metrics.map(m => new Date(m.timestamp));
		
		switch (type) {
			case 'cpu':
				return {
					labels,
					datasets: [{
						label: 'CPU Usage (%)',
						data: metrics.map(m => m.cpu.usage),
						borderColor: colors.primary,
						backgroundColor: colors.primary + '20',
						fill: true,
						tension: 0.4
					}]
				};
			case 'memory':
				return {
					labels,
					datasets: [{
						label: 'Memory Usage (%)',
						data: metrics.map(m => (m.memory.used / m.memory.total) * 100),
						borderColor: colors.success,
						backgroundColor: colors.success + '20',
						fill: true,
						tension: 0.4
					}]
				};
			case 'disk':
				return {
					labels,
					datasets: [{
						label: 'Disk Usage (%)',
						data: metrics.map(m => (m.disk.used / m.disk.total) * 100),
						borderColor: colors.warning,
						backgroundColor: colors.warning + '20',
						fill: true,
						tension: 0.4
					}]
				};
			case 'network':
				return {
					labels,
					datasets: [
						{
							label: 'Bytes In',
							data: metrics.map(m => m.network.bytesIn),
							borderColor: colors.primary,
							backgroundColor: colors.primary + '20',
							fill: false,
							tension: 0.4
						},
						{
							label: 'Bytes Out',
							data: metrics.map(m => m.network.bytesOut),
							borderColor: colors.error,
							backgroundColor: colors.error + '20',
							fill: false,
							tension: 0.4
						}
					]
				};
			default:
				return { labels: [], datasets: [] };
		}
	}

	function createChart() {
		if (!canvas) return;

		const ctx = canvas.getContext('2d');
		if (!ctx) return;

		chart = new Chart(ctx, {
			type: 'line',
			data: getChartData(),
			options: {
				responsive: true,
				maintainAspectRatio: false,
				plugins: {
					title: {
						display: true,
						text: title,
						color: colors.primary,
						font: {
							family: 'Inter Variable',
							size: 14,
							weight: '600'
						}
					},
					legend: {
						labels: {
							color: colors.primary,
							font: {
								family: 'Inter Variable'
							}
						}
					}
				},
				scales: {
					x: {
						type: 'time',
						time: {
							displayFormats: {
								minute: 'HH:mm',
								hour: 'HH:mm'
							}
						},
						grid: {
							color: colors.grid
						},
						ticks: {
							color: colors.primary,
							font: {
								family: 'Inter Variable'
							}
						}
					},
					y: {
						beginAtZero: true,
						grid: {
							color: colors.grid
						},
						ticks: {
							color: colors.primary,
							font: {
								family: 'Inter Variable'
							}
						}
					}
				},
				elements: {
					point: {
						radius: 2,
						hoverRadius: 4
					}
				}
			}
		});
	}

	function updateChart() {
		if (!chart) return;
		
		chart.data = getChartData();
		chart.update('none');
	}

	onMount(() => {
		createChart();
	});

	onDestroy(() => {
		if (chart) {
			chart.destroy();
			chart = null;
		}
	});

	$: if (chart && metrics) {
		updateChart();
	}
</script>

<div class="w-full" style="height: {height}px;">
	<canvas bind:this={canvas} class="w-full h-full"></canvas>
</div>

<style>
	canvas {
		background: transparent;
	}
</style>
