<script lang="ts">
	import type { NetworkInfo } from '$lib/types/system';
	import { createEventDispatcher } from 'svelte';
	
	export let networkInfo: NetworkInfo | null = null;
	
	const dispatch = createEventDispatcher();
	
	// Helper functions
	function formatLatency(ms: number): string {
		if (ms < 1) return '<1ms';
		return `${Math.round(ms)}ms`;
	}
	
	function formatBandwidth(bytes: number): string {
		if (!bytes) return '0 B/s';
		
		const sizes = ['B/s', 'KB/s', 'MB/s', 'GB/s'];
		const i = Math.floor(Math.log(bytes) / Math.log(1024));
		return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${sizes[i]}`;
	}
	
	function getStatusColor(status: string): string {
		switch (status) {
			case 'connected': return 'text-success-primary';
			case 'connecting': return 'text-warning-primary';
			case 'disconnected': return 'text-error-primary';
			default: return 'text-gray-400';
		}
	}
	
	function getStatusIcon(status: string): string {
		switch (status) {
			case 'connected': return '●';
			case 'connecting': return '◐';
			case 'disconnected': return '○';
			default: return '?';
		}
	}
</script>

<div class="bpi-card h-full">
	<div class="p-6">
		<!-- Header -->
		<div class="flex items-center justify-between mb-6">
			<h3 class="text-lg font-semibold text-white">Network Topology</h3>
			<div class="flex items-center space-x-2">
				{#if networkInfo}
					<span class="text-2xl {getStatusColor(networkInfo.meshStatus)}">
						{getStatusIcon(networkInfo.meshStatus)}
					</span>
					<span class="text-sm font-medium {getStatusColor(networkInfo.meshStatus)} capitalize">
						{networkInfo.meshStatus}
					</span>
				{/if}
			</div>
		</div>
		
		{#if networkInfo}
			<!-- Network Overview -->
			<div class="grid grid-cols-3 gap-4 mb-6">
				<div class="text-center">
					<div class="text-2xl font-bold text-white">{networkInfo.peers.length}</div>
					<div class="text-xs text-gray-400">Connected Peers</div>
				</div>
				<div class="text-center">
					<div class="text-2xl font-bold text-success-primary">{formatLatency(networkInfo.latency)}</div>
					<div class="text-xs text-gray-400">Avg Latency</div>
				</div>
				<div class="text-center">
					<div class="text-2xl font-bold text-info-primary">
						{networkInfo.peers.filter(p => p.status === 'connected').length}
					</div>
					<div class="text-xs text-gray-400">Active Peers</div>
				</div>
			</div>
			
			<!-- Bandwidth Stats -->
			<div class="grid grid-cols-2 gap-4 mb-6">
				<div class="bg-gray-800 rounded-lg p-3">
					<div class="flex items-center justify-between">
						<span class="text-sm text-gray-400">Upload</span>
						<span class="text-sm font-medium text-success-primary">
							{formatBandwidth(networkInfo.bandwidth.upload)}
						</span>
					</div>
				</div>
				<div class="bg-gray-800 rounded-lg p-3">
					<div class="flex items-center justify-between">
						<span class="text-sm text-gray-400">Download</span>
						<span class="text-sm font-medium text-info-primary">
							{formatBandwidth(networkInfo.bandwidth.download)}
						</span>
					</div>
				</div>
			</div>
			
			<!-- Peer List -->
			{#if networkInfo.peers.length > 0}
				<div class="space-y-3">
					<h4 class="text-sm font-medium text-white">Connected Peers</h4>
					<div class="max-h-48 overflow-y-auto space-y-2">
						{#each networkInfo.peers.slice(0, 5) as peer}
							<div class="bg-gray-800 rounded-lg p-3">
								<div class="flex items-center justify-between mb-2">
									<div class="flex items-center space-x-2">
										<div class="w-2 h-2 rounded-full {
											peer.status === 'connected' ? 'bg-success-primary' :
											peer.status === 'connecting' ? 'bg-warning-primary animate-pulse' :
											'bg-error-primary'
										}"></div>
										<span class="text-sm font-medium text-white">
											{peer.id.substring(0, 8)}...
										</span>
									</div>
									<span class="text-xs text-gray-400">{formatLatency(peer.latency)}</span>
								</div>
								<div class="flex items-center justify-between text-xs text-gray-400">
									<span>{peer.address}</span>
									<span>{peer.version}</span>
								</div>
								{#if peer.capabilities.length > 0}
									<div class="flex flex-wrap gap-1 mt-2">
										{#each peer.capabilities.slice(0, 3) as capability}
											<span class="px-2 py-1 bg-gray-700 text-xs text-gray-300 rounded">
												{capability}
											</span>
										{/each}
									</div>
								{/if}
							</div>
						{/each}
						
						{#if networkInfo.peers.length > 5}
							<div class="text-center py-2">
								<button 
									class="text-sm text-quantum-silver hover:text-white transition-colors"
									on:click={() => dispatch('view-all-peers')}
								>
									View all {networkInfo.peers.length} peers
								</button>
							</div>
						{/if}
					</div>
				</div>
			{:else}
				<div class="text-center py-8">
					<div class="text-gray-400 mb-2">
						<svg class="w-12 h-12 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 1.657-3.657 3.657-3.657a4 4 0 014 4C18 12 19.657 13.657 18.657 17.657z" />
						</svg>
					</div>
					<p class="text-sm text-gray-400">No peers connected</p>
				</div>
			{/if}
			
		{:else}
			<!-- Loading State -->
			<div class="flex items-center justify-center py-8">
				<div class="text-center">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-quantum-silver mx-auto mb-2"></div>
					<p class="text-sm text-gray-400">Loading network topology...</p>
				</div>
			</div>
		{/if}
		
		<!-- Actions -->
		<div class="mt-6 pt-6 border-t border-gray-700">
			<div class="grid grid-cols-2 gap-2">
				<button 
					class="bpi-btn-secondary text-xs px-3 py-2"
					on:click={() => dispatch('topology-view')}
				>
					Topology View
				</button>
				<button 
					class="bpi-btn-secondary text-xs px-3 py-2"
					on:click={() => dispatch('network-settings')}
				>
					Network Settings
				</button>
			</div>
		</div>
	</div>
</div>
