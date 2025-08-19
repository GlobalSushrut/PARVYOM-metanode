<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import type { SystemEvent } from '$lib/types/system';
	
	const dispatch = createEventDispatcher();
	
	// Recent activity data
	let recentEvents: SystemEvent[] = [];
	let loading = true;
	
	// Fetch recent activity from BPI gateway
	async function fetchRecentActivity() {
		try {
			loading = true;
			const response = await fetch('/api/events/recent?limit=20');
			if (response.ok) {
				recentEvents = await response.json();
			} else {
				console.error('Failed to fetch recent activity');
			}
		} catch (error) {
			console.error('Recent activity fetch error:', error);
		} finally {
			loading = false;
		}
	}
	
	// Initialize with mock data for development
	onMount(() => {
		// Mock data for development
		recentEvents = [
			{
				id: '1',
				timestamp: new Date(Date.now() - 5 * 60000).toISOString(),
				type: 'success',
				category: 'system',
				title: 'Node Started',
				message: 'BPI node successfully started and connected to mesh network'
			},
			{
				id: '2',
				timestamp: new Date(Date.now() - 15 * 60000).toISOString(),
				type: 'info',
				category: 'network',
				title: 'Peer Connected',
				message: 'New peer connected from 192.168.1.100'
			},
			{
				id: '3',
				timestamp: new Date(Date.now() - 30 * 60000).toISOString(),
				type: 'warning',
				category: 'container',
				title: 'Container Restart',
				message: 'Container bpi-wallet restarted due to memory limit'
			},
			{
				id: '4',
				timestamp: new Date(Date.now() - 45 * 60000).toISOString(),
				type: 'info',
				category: 'security',
				title: 'Policy Check',
				message: 'Bus BIOS policy check completed successfully'
			},
			{
				id: '5',
				timestamp: new Date(Date.now() - 60 * 60000).toISOString(),
				type: 'success',
				category: 'wallet',
				title: 'Transaction Confirmed',
				message: 'Transaction 0x1234...abcd confirmed on block 12345'
			},
			{
				id: '6',
				timestamp: new Date(Date.now() - 90 * 60000).toISOString(),
				type: 'error',
				category: 'system',
				title: 'Service Error',
				message: 'Temporary connection error to external service'
			}
		];
		loading = false;
		
		// Uncomment for real API integration
		// fetchRecentActivity();
	});
	
	// Helper functions
	function getEventIcon(type: string, category: string): string {
		const icons = {
			success: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z',
			info: 'M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
			warning: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z',
			error: 'M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z'
		};
		return icons[type] || icons.info;
	}
	
	function getEventColor(type: string): string {
		switch (type) {
			case 'success': return 'text-success-primary';
			case 'warning': return 'text-warning-primary';
			case 'error': return 'text-error-primary';
			case 'info': return 'text-info-primary';
			default: return 'text-gray-400';
		}
	}
	
	function getCategoryColor(category: string): string {
		switch (category) {
			case 'system': return 'bg-bpi-deep-blue';
			case 'network': return 'bg-success-secondary';
			case 'container': return 'bg-info-secondary';
			case 'security': return 'bg-warning-secondary';
			case 'wallet': return 'bg-quantum-silver';
			default: return 'bg-gray-600';
		}
	}
	
	function formatTimestamp(timestamp: string): string {
		const date = new Date(timestamp);
		const now = new Date();
		const diff = now.getTime() - date.getTime();
		const minutes = Math.floor(diff / 60000);
		
		if (minutes < 1) return 'Just now';
		if (minutes < 60) return `${minutes}m ago`;
		const hours = Math.floor(minutes / 60);
		if (hours < 24) return `${hours}h ago`;
		const days = Math.floor(hours / 24);
		return `${days}d ago`;
	}
</script>

<div class="bpi-card h-full">
	<div class="p-6">
		<!-- Header -->
		<div class="flex items-center justify-between mb-6">
			<h3 class="text-lg font-semibold text-white">Recent Activity</h3>
			<div class="flex items-center space-x-2">
				<button 
					class="text-quantum-silver hover:text-white transition-colors"
					on:click={fetchRecentActivity}
					title="Refresh activity"
				>
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
					</svg>
				</button>
				<button 
					class="text-quantum-silver hover:text-white transition-colors"
					on:click={() => dispatch('view-all-events')}
					title="View all events"
				>
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
					</svg>
				</button>
			</div>
		</div>
		
		{#if loading}
			<!-- Loading State -->
			<div class="flex items-center justify-center py-8">
				<div class="text-center">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-quantum-silver mx-auto mb-2"></div>
					<p class="text-sm text-gray-400">Loading recent activity...</p>
				</div>
			</div>
		{:else if recentEvents.length > 0}
			<!-- Activity List -->
			<div class="space-y-3 max-h-96 overflow-y-auto">
				{#each recentEvents as event}
					<div class="flex items-start space-x-3 p-3 bg-gray-800 rounded-lg hover:bg-gray-750 transition-colors">
						<!-- Event Icon -->
						<div class="flex-shrink-0 mt-0.5">
							<svg class="w-5 h-5 {getEventColor(event.type)}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={getEventIcon(event.type, event.category)} />
							</svg>
						</div>
						
						<!-- Event Content -->
						<div class="flex-1 min-w-0">
							<div class="flex items-center justify-between mb-1">
								<h4 class="text-sm font-medium text-white truncate">{event.title}</h4>
								<div class="flex items-center space-x-2 flex-shrink-0">
									<span class="px-2 py-1 {getCategoryColor(event.category)} text-white text-xs rounded-full capitalize">
										{event.category}
									</span>
									<span class="text-xs text-gray-400">
										{formatTimestamp(event.timestamp)}
									</span>
								</div>
							</div>
							<p class="text-sm text-gray-400 line-clamp-2">{event.message}</p>
						</div>
					</div>
				{/each}
			</div>
			
			<!-- View More -->
			{#if recentEvents.length >= 20}
				<div class="mt-4 text-center">
					<button 
						class="text-sm text-quantum-silver hover:text-white transition-colors"
						on:click={() => dispatch('view-all-events')}
					>
						View all events →
					</button>
				</div>
			{/if}
		{:else}
			<!-- Empty State -->
			<div class="text-center py-8">
				<div class="text-gray-400 mb-2">
					<svg class="w-12 h-12 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
					</svg>
				</div>
				<p class="text-sm text-gray-400">No recent activity</p>
			</div>
		{/if}
		
		<!-- Activity Summary -->
		<div class="mt-6 pt-6 border-t border-gray-700">
			<div class="grid grid-cols-4 gap-4 text-center">
				<div>
					<div class="text-lg font-semibold text-success-primary">
						{recentEvents.filter(e => e.type === 'success').length}
					</div>
					<div class="text-xs text-gray-400">Success</div>
				</div>
				<div>
					<div class="text-lg font-semibold text-info-primary">
						{recentEvents.filter(e => e.type === 'info').length}
					</div>
					<div class="text-xs text-gray-400">Info</div>
				</div>
				<div>
					<div class="text-lg font-semibold text-warning-primary">
						{recentEvents.filter(e => e.type === 'warning').length}
					</div>
					<div class="text-xs text-gray-400">Warning</div>
				</div>
				<div>
					<div class="text-lg font-semibold text-error-primary">
						{recentEvents.filter(e => e.type === 'error').length}
					</div>
					<div class="text-xs text-gray-400">Error</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>
