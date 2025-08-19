<script lang="ts">
	import { systemStore } from '$lib/stores/system';
	import { createEventDispatcher } from 'svelte';
	import type { SystemStatus } from '$lib/types/system';
	
	export let status: SystemStatus | null = null;
	
	const dispatch = createEventDispatcher();
	
	// Helper functions
	function formatUptime(seconds: number): string {
		if (!seconds) return 'Unknown';
		
		const days = Math.floor(seconds / 86400);
		const hours = Math.floor((seconds % 86400) / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		
		if (days > 0) return `${days}d ${hours}h ${minutes}m`;
		if (hours > 0) return `${hours}h ${minutes}m`;
		return `${minutes}m`;
	}
	
	function getHealthColor(health: string): string {
		switch (health) {
			case 'healthy': return 'text-success-primary';
			case 'warning': return 'text-warning-primary';
			case 'error': return 'text-error-primary';
			case 'offline': return 'text-gray-500';
			default: return 'text-gray-400';
		}
	}
	
	function getHealthIcon(health: string): string {
		switch (health) {
			case 'healthy': return '●';
			case 'warning': return '▲';
			case 'error': return '✕';
			case 'offline': return '○';
			default: return '?';
		}
	}
	
	$: healthColor = status ? getHealthColor(status.health) : 'text-gray-400';
	$: healthIcon = status ? getHealthIcon(status.health) : '?';
</script>

<div class="bpi-card h-full">
	<div class="p-6">
		<!-- Header -->
		<div class="flex items-center justify-between mb-6">
			<h3 class="text-lg font-semibold text-white">System Status</h3>
			<div class="flex items-center space-x-2">
				<span class="text-2xl {healthColor}">{healthIcon}</span>
				<span class="text-sm font-medium {healthColor} capitalize">
					{status?.health || 'Unknown'}
				</span>
			</div>
		</div>
		
		{#if status}
			<!-- System Information -->
			<div class="space-y-4">
				<!-- Node ID -->
				<div class="flex justify-between items-center">
					<span class="text-sm text-gray-400">Node ID</span>
					<span class="text-sm font-mono text-white truncate ml-2" title={status.nodeId}>
						{status.nodeId.substring(0, 12)}...
					</span>
				</div>
				
				<!-- Version -->
				<div class="flex justify-between items-center">
					<span class="text-sm text-gray-400">Version</span>
					<span class="text-sm font-medium text-white">{status.version}</span>
				</div>
				
				<!-- Uptime -->
				<div class="flex justify-between items-center">
					<span class="text-sm text-gray-400">Uptime</span>
					<span class="text-sm font-medium text-white">{formatUptime(status.uptime)}</span>
				</div>
				
				<!-- Last Update -->
				<div class="flex justify-between items-center">
					<span class="text-sm text-gray-400">Last Update</span>
					<span class="text-sm font-medium text-white">
						{new Date(status.timestamp).toLocaleTimeString()}
					</span>
				</div>
			</div>
			
			<!-- Services Status -->
			{#if status.services && status.services.length > 0}
				<div class="mt-6 pt-6 border-t border-gray-700">
					<h4 class="text-sm font-medium text-white mb-3">Services</h4>
					<div class="space-y-2">
						{#each status.services as service}
							<div class="flex items-center justify-between">
								<div class="flex items-center space-x-2">
									<div class="w-2 h-2 rounded-full {
										service.status === 'running' ? 'bg-success-primary' :
										service.status === 'starting' ? 'bg-warning-primary animate-pulse' :
										service.status === 'error' ? 'bg-error-primary' :
										'bg-gray-500'
									}"></div>
									<span class="text-sm text-white">{service.name}</span>
								</div>
								<div class="flex items-center space-x-2">
									{#if service.port}
										<span class="text-xs text-gray-400">:{service.port}</span>
									{/if}
									<span class="text-xs text-gray-400 capitalize">{service.status}</span>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
			
		{:else}
			<!-- Loading State -->
			<div class="flex items-center justify-center py-8">
				<div class="text-center">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-quantum-silver mx-auto mb-2"></div>
					<p class="text-sm text-gray-400">Loading system status...</p>
				</div>
			</div>
		{/if}
		
		<!-- Actions -->
		<div class="mt-6 pt-6 border-t border-gray-700">
			<div class="flex space-x-2">
				<button 
					class="bpi-btn-secondary text-xs px-3 py-1 flex-1"
					on:click={() => dispatch('refresh')}
				>
					Refresh
				</button>
				<button 
					class="bpi-btn-secondary text-xs px-3 py-1 flex-1"
					on:click={() => dispatch('details')}
				>
					Details
				</button>
			</div>
		</div>
	</div>
</div>
