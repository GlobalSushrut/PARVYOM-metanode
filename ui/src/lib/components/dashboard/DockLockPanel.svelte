<script lang="ts">
	import { onMount } from 'svelte';
	import GrafanaPanel from './GrafanaPanel.svelte';
	import { dockLockStore, formatBytes } from '$lib/stores/bpi-live-data';

	function getContainerStatusColor(status: string): string {
		switch (status.toLowerCase()) {
			case 'running': return 'text-green-400';
			case 'stopped': return 'text-gray-400';
			case 'failed': return 'text-red-400';
			default: return 'text-yellow-400';
		}
	}

	function calculateHealthScore(containers: any): number {
		if (!containers) return 0;
		const total = containers.total || 0;
		const running = containers.running || 0;
		const failed = containers.failed || 0;
		
		if (total === 0) return 100;
		return Math.round(((running / total) * 100) - ((failed / total) * 10));
	}
</script>

<GrafanaPanel
	title="DockLock Container Management"
	subtitle="Real-time container orchestration and encryption cluster monitoring"
	height="h-[500px]"
	loading={!$dockLockStore}
	lastUpdate={$dockLockStore ? new Date() : null}
>
	{#if $dockLockStore}
		<div class="grid grid-cols-2 gap-6 h-full">
			<!-- Container Status Overview -->
			<div class="space-y-4">
				<h4 class="text-lg font-semibold text-white font-mono border-b border-gray-700 pb-2">
					Container Status
				</h4>
				
				<!-- Container Stats Grid -->
				<div class="grid grid-cols-2 gap-3">
					<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
						<div class="flex items-center justify-between">
							<div>
								<div class="text-2xl font-bold text-blue-400 font-mono">
									{$dockLockStore.containers.total}
								</div>
								<div class="text-xs text-gray-400 font-mono">TOTAL CONTAINERS</div>
							</div>
							<div class="text-3xl">📦</div>
						</div>
					</div>
					
					<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
						<div class="flex items-center justify-between">
							<div>
								<div class="text-2xl font-bold text-green-400 font-mono">
									{$dockLockStore.containers.running}
								</div>
								<div class="text-xs text-gray-400 font-mono">RUNNING</div>
							</div>
							<div class="text-3xl">🟢</div>
						</div>
					</div>
					
					<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
						<div class="flex items-center justify-between">
							<div>
								<div class="text-2xl font-bold text-gray-400 font-mono">
									{$dockLockStore.containers.stopped}
								</div>
								<div class="text-xs text-gray-400 font-mono">STOPPED</div>
							</div>
							<div class="text-3xl">⏸️</div>
						</div>
					</div>
					
					<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
						<div class="flex items-center justify-between">
							<div>
								<div class="text-2xl font-bold text-red-400 font-mono">
									{$dockLockStore.containers.failed}
								</div>
								<div class="text-xs text-gray-400 font-mono">FAILED</div>
							</div>
							<div class="text-3xl">❌</div>
						</div>
					</div>
				</div>

				<!-- Health Score -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm font-mono text-gray-400">HEALTH SCORE</span>
						<span class="text-lg font-bold font-mono {calculateHealthScore($dockLockStore.containers) > 80 ? 'text-green-400' : calculateHealthScore($dockLockStore.containers) > 60 ? 'text-yellow-400' : 'text-red-400'}">
							{calculateHealthScore($dockLockStore.containers)}%
						</span>
					</div>
					<div class="w-full bg-gray-700 rounded-full h-2">
						<div 
							class="h-2 rounded-full {calculateHealthScore($dockLockStore.containers) > 80 ? 'bg-green-400' : calculateHealthScore($dockLockStore.containers) > 60 ? 'bg-yellow-400' : 'bg-red-400'}"
							style="width: {calculateHealthScore($dockLockStore.containers)}%"
						></div>
					</div>
				</div>
			</div>

			<!-- Cluster & Encryption Status -->
			<div class="space-y-4">
				<h4 class="text-lg font-semibold text-white font-mono border-b border-gray-700 pb-2">
					Cluster & Encryption
				</h4>
				
				<!-- Cluster Stats -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<h5 class="text-sm font-mono text-gray-300 mb-3">CLUSTER STATUS</h5>
					<div class="space-y-3">
						<div class="flex justify-between">
							<span class="text-xs text-gray-400 font-mono">Active Clusters:</span>
							<span class="text-blue-400 font-mono">{$dockLockStore.clusters.active_clusters}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-xs text-gray-400 font-mono">Total Nodes:</span>
							<span class="text-green-400 font-mono">{$dockLockStore.clusters.total_nodes}</span>
						</div>
					</div>
				</div>

				<!-- Resource Utilization -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<h5 class="text-sm font-mono text-gray-300 mb-3">RESOURCE UTILIZATION</h5>
					<div class="space-y-3">
						<div>
							<div class="flex justify-between text-xs mb-1">
								<span class="text-gray-400 font-mono">CPU</span>
								<span class="text-blue-400 font-mono">{$dockLockStore.clusters.resource_utilization.cpu}%</span>
							</div>
							<div class="w-full bg-gray-700 rounded-full h-2">
								<div 
									class="bg-blue-400 h-2 rounded-full"
									style="width: {$dockLockStore.clusters.resource_utilization.cpu}%"
								></div>
							</div>
						</div>
						
						<div>
							<div class="flex justify-between text-xs mb-1">
								<span class="text-gray-400 font-mono">Memory</span>
								<span class="text-green-400 font-mono">{$dockLockStore.clusters.resource_utilization.memory}%</span>
							</div>
							<div class="w-full bg-gray-700 rounded-full h-2">
								<div 
									class="bg-green-400 h-2 rounded-full"
									style="width: {$dockLockStore.clusters.resource_utilization.memory}%"
								></div>
							</div>
						</div>
						
						<div>
							<div class="flex justify-between text-xs mb-1">
								<span class="text-gray-400 font-mono">Storage</span>
								<span class="text-yellow-400 font-mono">{$dockLockStore.clusters.resource_utilization.storage}%</span>
							</div>
							<div class="w-full bg-gray-700 rounded-full h-2">
								<div 
									class="bg-yellow-400 h-2 rounded-full"
									style="width: {$dockLockStore.clusters.resource_utilization.storage}%"
								></div>
							</div>
						</div>
					</div>
				</div>

				<!-- Encryption Clusters -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<h5 class="text-sm font-mono text-gray-300 mb-3">ENCRYPTION CLUSTERS</h5>
					<div class="space-y-3">
						<div class="flex justify-between">
							<span class="text-xs text-gray-400 font-mono">Active Clusters:</span>
							<span class="text-green-400 font-mono">{$dockLockStore.encryption_clusters.active}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-xs text-gray-400 font-mono">Key Rotations:</span>
							<span class="text-blue-400 font-mono">{$dockLockStore.encryption_clusters.key_rotations}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-xs text-gray-400 font-mono">Encryption Load:</span>
							<span class="text-purple-400 font-mono">{$dockLockStore.encryption_clusters.encryption_load}%</span>
						</div>
						
						<!-- Encryption Load Bar -->
						<div class="w-full bg-gray-700 rounded-full h-2">
							<div 
								class="bg-purple-400 h-2 rounded-full"
								style="width: {$dockLockStore.encryption_clusters.encryption_load}%"
							></div>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Container Actions -->
		<div class="mt-6 pt-4 border-t border-gray-700">
			<div class="flex space-x-3">
				<button class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded font-mono text-sm transition-colors">
					🚀 Deploy Container
				</button>
				<button class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded font-mono text-sm transition-colors">
					🔄 Scale Cluster
				</button>
				<button class="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded font-mono text-sm transition-colors">
					🔐 Rotate Keys
				</button>
				<button class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded font-mono text-sm transition-colors">
					🛑 Emergency Stop
				</button>
			</div>
		</div>
	{:else}
		<div class="flex items-center justify-center h-full">
			<div class="text-center">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-400 mx-auto mb-4"></div>
				<p class="text-gray-400 font-mono">Loading DockLock Status...</p>
			</div>
		</div>
	{/if}
</GrafanaPanel>
