<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { systemStore } from '$lib/stores/system';
	
	const dispatch = createEventDispatcher();
	
	// Quick action items based on BPI specifications
	const quickActions = [
		{
			id: 'start-services',
			title: 'Start Services',
			description: 'Start all BPI services',
			icon: 'play',
			color: 'success',
			action: 'start-services'
		},
		{
			id: 'stop-services',
			title: 'Stop Services',
			description: 'Stop all BPI services',
			icon: 'stop',
			color: 'error',
			action: 'stop-services'
		},
		{
			id: 'restart-node',
			title: 'Restart Node',
			description: 'Restart BPI node',
			icon: 'refresh',
			color: 'warning',
			action: 'restart-node'
		},
		{
			id: 'backup-data',
			title: 'Backup Data',
			description: 'Create system backup',
			icon: 'backup',
			color: 'info',
			action: 'backup-data'
		},
		{
			id: 'run-diagnostics',
			title: 'Diagnostics',
			description: 'Run system diagnostics',
			icon: 'diagnostics',
			color: 'info',
			action: 'run-diagnostics'
		},
		{
			id: 'emergency-stop',
			title: 'Emergency Stop',
			description: 'Emergency shutdown',
			icon: 'emergency',
			color: 'error',
			action: 'emergency-stop'
		}
	];
	
	// Icon mapping
	function getIcon(iconName: string): string {
		const icons = {
			play: 'M8 5v14l11-7z',
			stop: 'M6 6h12v12H6z',
			refresh: 'M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15',
			backup: 'M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10',
			diagnostics: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z',
			emergency: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z'
		};
		return icons[iconName] || icons.play;
	}
	
	// Color mapping
	function getButtonClass(color: string): string {
		switch (color) {
			case 'success': return 'bpi-btn-success';
			case 'warning': return 'bpi-btn-warning';
			case 'error': return 'bpi-btn-error';
			case 'info': return 'bpi-btn-secondary';
			default: return 'bpi-btn-secondary';
		}
	}
	
	// Handle action execution
	async function executeAction(actionId: string) {
		try {
			systemStore.setLoading(true);
			
			// Simulate API call to BPI gateway
			const response = await fetch(`/api/actions/${actionId}`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				}
			});
			
			if (!response.ok) {
				throw new Error(`Action failed: ${response.statusText}`);
			}
			
			const result = await response.json();
			console.log(`Action ${actionId} completed:`, result);
			
			// Refresh system data after action
			await systemStore.refresh();
			
			dispatch('action-completed', { actionId, result });
			
		} catch (error) {
			console.error(`Action ${actionId} failed:`, error);
			systemStore.setError(`Action failed: ${error.message}`);
		} finally {
			systemStore.setLoading(false);
		}
	}
	
	$: systemData = $systemStore;
</script>

<div class="bpi-card h-full">
	<div class="p-6">
		<!-- Header -->
		<div class="flex items-center justify-between mb-6">
			<h3 class="text-lg font-semibold text-white">Quick Actions</h3>
			<div class="text-xs text-gray-400">
				System Control
			</div>
		</div>
		
		<!-- Action Grid -->
		<div class="grid grid-cols-2 gap-3">
			{#each quickActions as action}
				<button
					class="{getButtonClass(action.color)} text-xs px-3 py-3 flex flex-col items-center space-y-2 h-20 relative group"
					on:click={() => executeAction(action.action)}
					disabled={systemData.loading}
					title={action.description}
				>
					<!-- Icon -->
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={getIcon(action.icon)} />
					</svg>
					
					<!-- Title -->
					<span class="font-medium text-center leading-tight">
						{action.title}
					</span>
					
					<!-- Loading overlay -->
					{#if systemData.loading}
						<div class="absolute inset-0 bg-black bg-opacity-50 flex items-center justify-center rounded-md">
							<div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
						</div>
					{/if}
					
					<!-- Hover tooltip -->
					<div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-2 py-1 bg-gray-900 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-10">
						{action.description}
					</div>
				</button>
			{/each}
		</div>
		
		<!-- System Status Summary -->
		<div class="mt-6 pt-6 border-t border-gray-700">
			<div class="grid grid-cols-2 gap-4 text-center">
				<div>
					<div class="text-lg font-semibold {
						systemData.status?.health === 'healthy' ? 'text-success-primary' :
						systemData.status?.health === 'warning' ? 'text-warning-primary' :
						'text-error-primary'
					}">
						{systemData.status?.services?.filter(s => s.status === 'running').length || 0}
					</div>
					<div class="text-xs text-gray-400">Services Running</div>
				</div>
				<div>
					<div class="text-lg font-semibold text-info-primary">
						{systemData.status?.uptime ? Math.floor(systemData.status.uptime / 3600) : 0}h
					</div>
					<div class="text-xs text-gray-400">Uptime</div>
				</div>
			</div>
		</div>
		
		<!-- Emergency Actions -->
		<div class="mt-4 pt-4 border-t border-gray-800">
			<div class="text-xs text-gray-400 mb-2 text-center">Emergency Controls</div>
			<div class="grid grid-cols-2 gap-2">
				<button 
					class="bpi-btn-error text-xs px-2 py-1"
					on:click={() => executeAction('emergency-stop')}
					disabled={systemData.loading}
				>
					Emergency Stop
				</button>
				<button 
					class="bpi-btn-warning text-xs px-2 py-1"
					on:click={() => executeAction('safe-shutdown')}
					disabled={systemData.loading}
				>
					Safe Shutdown
				</button>
			</div>
		</div>
	</div>
</div>
