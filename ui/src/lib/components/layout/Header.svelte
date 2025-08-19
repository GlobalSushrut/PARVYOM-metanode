<script lang="ts">
	import { page } from '$app/stores';
	import { systemStore } from '$lib/stores/system';
	import { websocketStore } from '$lib/stores/websocket';
	
	// Get current page title based on route
	function getPageTitle(pathname: string): string {
		const titles = {
			'/': 'Dashboard',
			'/mesh': 'Mesh Network',
			'/containers': 'Container Management',
			'/security': 'Security & Compliance',
			'/wallet': 'Wallet & Economics',
			'/analytics': 'Analytics & Monitoring',
			'/policies': 'Policies & Agreements',
			'/settings': 'System Settings'
		};
		return titles[pathname] || 'BPI Console';
	}
	
	$: currentPath = $page.url.pathname;
	$: pageTitle = getPageTitle(currentPath);
	$: systemData = $systemStore;
	$: wsData = $websocketStore;
</script>

<header class="bg-gray-800 border-b border-gray-700 px-6 py-4">
	<div class="flex items-center justify-between">
		<!-- Page Title and Breadcrumb -->
		<div class="flex items-center space-x-4">
			<h1 class="text-xl font-semibold text-white">{pageTitle}</h1>
			
			<!-- Breadcrumb for nested pages -->
			{#if currentPath !== '/'}
				<nav class="flex items-center space-x-2 text-sm text-gray-400">
					<a href="/" class="hover:text-white transition-colors">Dashboard</a>
					<span>›</span>
					<span class="text-white">{pageTitle}</span>
				</nav>
			{/if}
		</div>
		
		<!-- Header Actions and Status -->
		<div class="flex items-center space-x-6">
			<!-- Connection Status -->
			<div class="flex items-center space-x-2">
				<div class="flex items-center space-x-1">
					<div class="w-2 h-2 rounded-full bg-green-400 animate-pulse"></div>
					<span class="text-sm text-green-300">
						Dashboard Online
					</span>
				</div>
				
				{#if wsData.connecting}
					<div class="animate-spin rounded-full h-4 w-4 border-b border-quantum-silver"></div>
				{/if}
			</div>
			
			<!-- System Health Indicator -->
			{#if systemData.status}
				<div class="flex items-center space-x-2">
					<div class="w-2 h-2 rounded-full {
						systemData.status.health === 'healthy' ? 'bg-success-primary' :
						systemData.status.health === 'warning' ? 'bg-warning-primary' :
						systemData.status.health === 'error' ? 'bg-error-primary' :
						'bg-gray-500'
					}"></div>
					<span class="text-sm text-gray-400 capitalize">
						{systemData.status.health}
					</span>
				</div>
			{/if}
			
			<!-- Quick Actions -->
			<div class="flex items-center space-x-2">
				<!-- Notifications -->
				<button 
					class="p-2 text-gray-400 hover:text-white transition-colors rounded-md hover:bg-gray-700"
					title="Notifications"
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
					</svg>
				</button>
				
				<!-- Search -->
				<button 
					class="p-2 text-gray-400 hover:text-white transition-colors rounded-md hover:bg-gray-700"
					title="Search"
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
					</svg>
				</button>
				
				<!-- Settings -->
				<button 
					class="p-2 text-gray-400 hover:text-white transition-colors rounded-md hover:bg-gray-700"
					title="Settings"
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
					</svg>
				</button>
			</div>
			
			<!-- User Menu -->
			<div class="flex items-center space-x-3">
				<div class="text-right">
					<div class="text-sm font-medium text-white">BPI Operator</div>
					<div class="text-xs text-gray-400">Local Node</div>
				</div>
				<div class="w-8 h-8 bg-bpi-deep-blue rounded-full flex items-center justify-center">
					<span class="text-white text-sm font-medium">OP</span>
				</div>
			</div>
		</div>
	</div>
	
	<!-- Secondary Header for Additional Context -->
	{#if currentPath !== '/'}
		<div class="mt-3 pt-3 border-t border-gray-700">
			<div class="flex items-center justify-between">
				<!-- Page-specific info -->
				<div class="flex items-center space-x-4 text-sm text-gray-400">
					{#if currentPath === '/mesh'}
						<span>BPCI Mesh Network Status</span>
					{:else if currentPath === '/containers'}
						<span>DockLock Container Orchestration</span>
					{:else if currentPath === '/security'}
						<span>DeterminismCage & Bus BIOS Security</span>
					{:else if currentPath === '/wallet'}
						<span>Multi-Token Wallet Management</span>
					{:else if currentPath === '/analytics'}
						<span>Real-time Performance Analytics</span>
					{:else if currentPath === '/policies'}
						<span>YAML Policy Engine</span>
					{:else if currentPath === '/settings'}
						<span>System Configuration</span>
					{/if}
				</div>
				
				<!-- Page-specific actions -->
				<div class="flex items-center space-x-2">
					{#if systemData.lastUpdate}
						<span class="text-xs text-gray-500">
							Updated {new Date(systemData.lastUpdate).toLocaleTimeString()}
						</span>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</header>
