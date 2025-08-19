<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import GrafanaPanel from './GrafanaPanel.svelte';
	import { coreComponentsStore, getStatusColor, formatUptime } from '$lib/stores/bpi-live-data';

	let refreshInterval: ReturnType<typeof setInterval>;

	// Component status mapping
	const componentLabels = {
		math: 'Mathematical Core',
		mempool: 'Transaction Mempool',
		gateway: 'API Gateway',
		merkle: 'Merkle Tree Engine',
		vrf: 'Verifiable Random Function',
		receipts: 'Receipt Processing',
		billing: 'Billing System',
		dashboard: 'Dashboard Service',
		config: 'Configuration Manager',
		http: 'HTTP Server',
		shadow_registry: 'Shadow Registry',
		notary: 'Notary Service',
		court: 'Court Node',
		auditing: 'Audit System',
		inclusion: 'Inclusion Lists'
	};

	function getComponentIcon(component: string): string {
		const icons: Record<string, string> = {
			math: '∑',
			mempool: '⧉',
			gateway: '🚪',
			merkle: '🌳',
			vrf: '🎲',
			receipts: '📄',
			billing: '💰',
			dashboard: '📊',
			config: '⚙️',
			http: '🌐',
			shadow_registry: '👥',
			notary: '✓',
			court: '⚖️',
			auditing: '🔍',
			inclusion: '📋'
		};
		return icons[component] || '⚡';
	}

	onMount(() => {
		// Auto-refresh every 5 seconds
		refreshInterval = setInterval(() => {
			// Trigger refresh via store
		}, 5000);
	});

	onDestroy(() => {
		if (refreshInterval) clearInterval(refreshInterval);
	});
</script>

<GrafanaPanel
	title="BPI Core Components Status"
	subtitle="Real-time monitoring of all Metanode Core components"
	height="h-96"
	loading={!$coreComponentsStore}
	lastUpdate={$coreComponentsStore ? new Date() : null}
>
	{#if $coreComponentsStore}
		<div class="grid grid-cols-3 gap-4 h-full overflow-y-auto">
			{#each Object.entries($coreComponentsStore.components) as [componentKey, component]}
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700 hover:border-gray-600 transition-all">
					<!-- Component Header -->
					<div class="flex items-center justify-between mb-3">
						<div class="flex items-center space-x-2">
							<span class="text-2xl">{getComponentIcon(componentKey)}</span>
							<div>
								<h4 class="text-sm font-semibold text-white font-mono">
									{componentLabels[componentKey] || componentKey.toUpperCase()}
								</h4>
								<p class="text-xs text-gray-400 font-mono">{componentKey}</p>
							</div>
						</div>
						
						<!-- Status Indicator -->
						<div class="flex items-center space-x-2">
							<div class="w-3 h-3 rounded-full {component.status === 'Running' ? 'bg-green-400 animate-pulse' : component.status === 'Error' ? 'bg-red-400' : component.status === 'Maintenance' ? 'bg-yellow-400' : 'bg-gray-400'}"></div>
							<span class="text-xs font-mono {getStatusColor(component.status)}">
								{component.status.toUpperCase()}
							</span>
						</div>
					</div>

					<!-- Component Metrics -->
					<div class="space-y-2">
						<div class="flex justify-between text-xs">
							<span class="text-gray-400 font-mono">Uptime:</span>
							<span class="text-green-400 font-mono">{formatUptime(component.uptime)}</span>
						</div>
						
						{#if component.last_error}
							<div class="text-xs">
								<span class="text-red-400 font-mono">Last Error:</span>
								<p class="text-red-300 font-mono text-xs mt-1 truncate" title={component.last_error}>
									{component.last_error}
								</p>
							</div>
						{/if}

						{#if component.metrics}
							<div class="mt-2 pt-2 border-t border-gray-700">
								<p class="text-xs text-gray-400 font-mono mb-1">Metrics:</p>
								{#each Object.entries(component.metrics) as [metric, value]}
									<div class="flex justify-between text-xs">
										<span class="text-gray-400 font-mono">{metric}:</span>
										<span class="text-blue-400 font-mono">
											{typeof value === 'number' ? value.toLocaleString() : value}
										</span>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>
			{/each}
		</div>

		<!-- Summary Stats -->
		<div class="mt-4 pt-4 border-t border-gray-700">
			<div class="grid grid-cols-4 gap-4 text-center">
				<div class="bg-gray-800 rounded p-3">
					<div class="text-2xl font-bold text-green-400 font-mono">
						{Object.values($coreComponentsStore.components).filter(c => c.status === 'Running').length}
					</div>
					<div class="text-xs text-gray-400 font-mono">RUNNING</div>
				</div>
				<div class="bg-gray-800 rounded p-3">
					<div class="text-2xl font-bold text-red-400 font-mono">
						{Object.values($coreComponentsStore.components).filter(c => c.status === 'Error').length}
					</div>
					<div class="text-xs text-gray-400 font-mono">ERRORS</div>
				</div>
				<div class="bg-gray-800 rounded p-3">
					<div class="text-2xl font-bold text-yellow-400 font-mono">
						{Object.values($coreComponentsStore.components).filter(c => c.status === 'Maintenance').length}
					</div>
					<div class="text-xs text-gray-400 font-mono">MAINTENANCE</div>
				</div>
				<div class="bg-gray-800 rounded p-3">
					<div class="text-2xl font-bold text-blue-400 font-mono">
						{$coreComponentsStore.uptime ? formatUptime($coreComponentsStore.uptime) : 'N/A'}
					</div>
					<div class="text-xs text-gray-400 font-mono">SYSTEM UPTIME</div>
				</div>
			</div>
		</div>
	{:else}
		<div class="flex items-center justify-center h-full">
			<div class="text-center">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-400 mx-auto mb-4"></div>
				<p class="text-gray-400 font-mono">Loading BPI Core Components...</p>
			</div>
		</div>
	{/if}
</GrafanaPanel>
