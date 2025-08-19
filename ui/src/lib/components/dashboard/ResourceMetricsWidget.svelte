<script lang="ts">
	import type { NodeMetrics } from '$lib/types/system';
	import { createEventDispatcher } from 'svelte';
	
	export let metrics: NodeMetrics | null = null;
	
	const dispatch = createEventDispatcher();
	
	// Helper functions
	function formatBytes(bytes: number): string {
		if (!bytes) return '0 B';
		
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(1024));
		return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`;
	}
	
	function formatPercentage(value: number): string {
		return `${Math.round(value)}%`;
	}
	
	function getUsageColor(percentage: number): string {
		if (percentage >= 90) return 'text-error-primary';
		if (percentage >= 75) return 'text-warning-primary';
		return 'text-success-primary';
	}
	
	function getProgressColor(percentage: number): string {
		if (percentage >= 90) return 'bg-error-primary';
		if (percentage >= 75) return 'bg-warning-primary';
		return 'bg-success-primary';
	}
</script>

<div class="bpi-card h-full">
	<div class="p-6">
		<!-- Header -->
		<div class="flex items-center justify-between mb-6">
			<h3 class="text-lg font-semibold text-white">Resource Metrics</h3>
			<button 
				class="text-quantum-silver hover:text-white transition-colors"
				on:click={() => dispatch('refresh')}
				title="Refresh metrics"
			>
				<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
				</svg>
			</button>
		</div>
		
		{#if metrics}
			<div class="space-y-6">
				<!-- CPU Usage -->
				<div>
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm font-medium text-gray-300">CPU Usage</span>
						<span class="text-sm font-semibold {getUsageColor(metrics.cpu.usage)}">
							{formatPercentage(metrics.cpu.usage)}
						</span>
					</div>
					<div class="w-full bg-gray-700 rounded-full h-2">
						<div 
							class="h-2 rounded-full transition-all duration-300 {getProgressColor(metrics.cpu.usage)}"
							style="width: {Math.min(metrics.cpu.usage, 100)}%"
						></div>
					</div>
					<div class="flex justify-between text-xs text-gray-400 mt-1">
						<span>{metrics.cpu.cores} cores</span>
						<span>Load: {metrics.cpu.loadAverage[0]?.toFixed(2) || 'N/A'}</span>
					</div>
				</div>
				
				<!-- Memory Usage -->
				<div>
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm font-medium text-gray-300">Memory Usage</span>
						<span class="text-sm font-semibold {getUsageColor(metrics.memory.percentage)}">
							{formatPercentage(metrics.memory.percentage)}
						</span>
					</div>
					<div class="w-full bg-gray-700 rounded-full h-2">
						<div 
							class="h-2 rounded-full transition-all duration-300 {getProgressColor(metrics.memory.percentage)}"
							style="width: {Math.min(metrics.memory.percentage, 100)}%"
						></div>
					</div>
					<div class="flex justify-between text-xs text-gray-400 mt-1">
						<span>{formatBytes(metrics.memory.used)} used</span>
						<span>{formatBytes(metrics.memory.total)} total</span>
					</div>
				</div>
				
				<!-- Disk Usage -->
				<div>
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm font-medium text-gray-300">Disk Usage</span>
						<span class="text-sm font-semibold {getUsageColor(metrics.disk.percentage)}">
							{formatPercentage(metrics.disk.percentage)}
						</span>
					</div>
					<div class="w-full bg-gray-700 rounded-full h-2">
						<div 
							class="h-2 rounded-full transition-all duration-300 {getProgressColor(metrics.disk.percentage)}"
							style="width: {Math.min(metrics.disk.percentage, 100)}%"
						></div>
					</div>
					<div class="flex justify-between text-xs text-gray-400 mt-1">
						<span>{formatBytes(metrics.disk.used)} used</span>
						<span>{formatBytes(metrics.disk.total)} total</span>
					</div>
				</div>
				
				<!-- Network Stats -->
				<div class="pt-4 border-t border-gray-700">
					<h4 class="text-sm font-medium text-white mb-3">Network Activity</h4>
					<div class="grid grid-cols-2 gap-4">
						<div class="text-center">
							<div class="text-lg font-semibold text-success-primary">
								{formatBytes(metrics.network.bytesIn)}
							</div>
							<div class="text-xs text-gray-400">Bytes In</div>
						</div>
						<div class="text-center">
							<div class="text-lg font-semibold text-info-primary">
								{formatBytes(metrics.network.bytesOut)}
							</div>
							<div class="text-xs text-gray-400">Bytes Out</div>
						</div>
					</div>
					<div class="flex justify-between text-xs text-gray-400 mt-2">
						<span>{metrics.network.packetsIn.toLocaleString()} packets in</span>
						<span>{metrics.network.connections} connections</span>
					</div>
				</div>
			</div>
			
		{:else}
			<!-- Loading State -->
			<div class="flex items-center justify-center py-8">
				<div class="text-center">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-quantum-silver mx-auto mb-2"></div>
					<p class="text-sm text-gray-400">Loading metrics...</p>
				</div>
			</div>
		{/if}
		
		<!-- Quick Actions -->
		<div class="mt-6 pt-6 border-t border-gray-700">
			<div class="grid grid-cols-2 gap-2">
				<button 
					class="bpi-btn-secondary text-xs px-3 py-2"
					on:click={() => dispatch('details')}
				>
					View Details
				</button>
				<button 
					class="bpi-btn-secondary text-xs px-3 py-2"
					on:click={() => dispatch('history')}
				>
					History
				</button>
			</div>
		</div>
	</div>
</div>
