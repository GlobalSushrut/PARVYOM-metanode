<script lang="ts">
	import { page } from '$app/stores';
	import { systemStore } from '$lib/stores/system';
	
	// Navigation items based on BPI component specifications
	const navigationItems = [
		{
			name: 'Dashboard',
			href: '/',
			icon: 'dashboard',
			description: 'System overview and status'
		},
		{
			name: 'Mesh Network',
			href: '/mesh',
			icon: 'network',
			description: 'BPCI mesh topology and peers'
		},
		{
			name: 'Containers',
			href: '/containers',
			icon: 'container',
			description: 'DockLock container management'
		},
		{
			name: 'Security',
			href: '/security',
			icon: 'shield',
			description: 'DeterminismCage and Bus BIOS'
		},
		{
			name: 'Wallet',
			href: '/wallet',
			icon: 'wallet',
			description: 'Wallet and economics'
		},
		{
			name: 'Analytics',
			href: '/analytics',
			icon: 'chart',
			description: 'Performance metrics and logs'
		},
		{
			name: 'Policies',
			href: '/policies',
			icon: 'policy',
			description: 'YAML policies and agreements'
		},
		{
			name: 'Settings',
			href: '/settings',
			icon: 'settings',
			description: 'System configuration'
		}
	];
	
	// Icon component mapping
	function getIcon(iconName: string): string {
		const icons = {
			dashboard: 'M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586l-2 2V5H5v14h7v2H4a1 1 0 01-1-1V4z',
			network: 'M12 2l3.09 6.26L22 9l-5.09 3.74L19 22l-7-5.27L5 22l2.09-8.26L2 9l6.91-0.74L12 2z',
			container: 'M3 3h18v4H3V3zm0 6h18v8H3V9zm2 2v4h14v-4H5z',
			shield: 'M12 1l9 4v6c0 5.55-3.84 10.74-9 12-5.16-1.26-9-6.45-9-12V5l9-4z',
			wallet: 'M21 18v1a2 2 0 01-2 2H5a2 2 0 01-2-2V5a2 2 0 012-2h14a2 2 0 012 2v1m0 4h.01M21 18h.01',
			chart: 'M16 18v-6a2 2 0 00-2-2h-4a2 2 0 00-2 2v6M8 18v-4a2 2 0 012-2h4a2 2 0 012 2v4',
			policy: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z',
			settings: 'M12 15a3 3 0 100-6 3 3 0 000 6z M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z'
		};
		return icons[iconName] || icons.dashboard;
	}
	
	$: currentPath = $page.url.pathname;
	$: systemData = $systemStore;
</script>

<div class="flex flex-col h-full bg-gray-900">
	<!-- Logo and Brand -->
	<div class="p-6 border-b border-gray-700">
		<div class="flex items-center space-x-3">
			<div class="w-8 h-8 bg-gradient-bpi rounded-lg flex items-center justify-center">
				<span class="text-white font-bold text-sm">BPI</span>
			</div>
			<div>
				<h1 class="text-white font-semibold text-lg">BPI Console</h1>
				<p class="text-xs text-gray-400">Operations Dashboard</p>
			</div>
		</div>
	</div>
	
	<!-- System Status Indicator -->
	<div class="px-6 py-4 border-b border-gray-700">
		<div class="flex items-center justify-between">
			<span class="text-sm text-gray-400">System Status</span>
			<div class="flex items-center space-x-2">
				<div class="w-2 h-2 rounded-full {
					systemData.status?.health === 'healthy' ? 'bg-success-primary' :
					systemData.status?.health === 'warning' ? 'bg-warning-primary' :
					systemData.status?.health === 'error' ? 'bg-error-primary' :
					'bg-gray-500'
				}"></div>
				<span class="text-xs text-white capitalize">
					{systemData.status?.health || 'Unknown'}
				</span>
			</div>
		</div>
	</div>
	
	<!-- Navigation Menu -->
	<nav class="flex-1 px-4 py-6 space-y-2 overflow-y-auto">
		{#each navigationItems as item}
			<a
				href={item.href}
				class="nav-item {currentPath === item.href ? 'nav-item-active' : 'nav-item-inactive'}"
				title={item.description}
			>
				<svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={getIcon(item.icon)} />
				</svg>
				<span class="font-medium">{item.name}</span>
			</a>
		{/each}
	</nav>
	
	<!-- System Information Footer -->
	<div class="p-4 border-t border-gray-700">
		<div class="space-y-2">
			<!-- Node ID -->
			{#if systemData.status?.nodeId}
				<div class="flex items-center justify-between">
					<span class="text-xs text-gray-400">Node ID</span>
					<span class="text-xs font-mono text-white">
						{systemData.status.nodeId.substring(0, 8)}...
					</span>
				</div>
			{/if}
			
			<!-- Version -->
			{#if systemData.status?.version}
				<div class="flex items-center justify-between">
					<span class="text-xs text-gray-400">Version</span>
					<span class="text-xs text-white">{systemData.status.version}</span>
				</div>
			{/if}
			
			<!-- Uptime -->
			{#if systemData.status?.uptime}
				<div class="flex items-center justify-between">
					<span class="text-xs text-gray-400">Uptime</span>
					<span class="text-xs text-white">
						{Math.floor(systemData.status.uptime / 3600)}h {Math.floor((systemData.status.uptime % 3600) / 60)}m
					</span>
				</div>
			{/if}
		</div>
		
		<!-- Quick Actions -->
		<div class="mt-4 pt-4 border-t border-gray-800">
			<div class="grid grid-cols-2 gap-2">
				<button 
					class="bpi-btn-secondary text-xs px-2 py-1"
					on:click={() => systemStore.refresh()}
					disabled={systemData.loading}
				>
					{#if systemData.loading}
						<div class="animate-spin rounded-full h-3 w-3 border-b border-white mx-auto"></div>
					{:else}
						Refresh
					{/if}
				</button>
				<button class="bpi-btn-secondary text-xs px-2 py-1">
					Help
				</button>
			</div>
		</div>
	</div>
</div>
