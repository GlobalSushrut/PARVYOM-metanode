<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import type { SecurityStatus } from '$lib/types/system';
	
	const dispatch = createEventDispatcher();
	
	// Mock security status data (will be replaced with real API calls)
	let securityStatus: SecurityStatus | null = null;
	let loading = true;
	
	// Fetch security status from BPI gateway
	async function fetchSecurityStatus() {
		try {
			loading = true;
			const response = await fetch('/api/security/status');
			if (response.ok) {
				securityStatus = await response.json();
			} else {
				console.error('Failed to fetch security status');
			}
		} catch (error) {
			console.error('Security status fetch error:', error);
		} finally {
			loading = false;
		}
	}
	
	// Initialize with mock data for development
	onMount(() => {
		// Mock data for development
		securityStatus = {
			deterministicCage: {
				active: true,
				violations: 0,
				lastViolation: null
			},
			busBios: {
				mode: 'secure',
				policyViolations: 0,
				lastPolicyCheck: new Date().toISOString()
			},
			encryption: {
				enabled: true,
				algorithm: 'AES-256-GCM',
				keyRotation: '2025-01-19T00:00:00Z'
			}
		};
		loading = false;
		
		// Uncomment for real API integration
		// fetchSecurityStatus();
	});
	
	// Helper functions
	function getStatusColor(status: boolean | string): string {
		if (typeof status === 'boolean') {
			return status ? 'text-success-primary' : 'text-error-primary';
		}
		
		switch (status) {
			case 'secure':
			case 'normal': return 'text-success-primary';
			case 'warning': return 'text-warning-primary';
			case 'emergency':
			case 'error': return 'text-error-primary';
			default: return 'text-gray-400';
		}
	}
	
	function getStatusIcon(status: boolean | string): string {
		if (typeof status === 'boolean') {
			return status ? '●' : '○';
		}
		
		switch (status) {
			case 'secure':
			case 'normal': return '●';
			case 'warning': return '▲';
			case 'emergency':
			case 'error': return '✕';
			default: return '?';
		}
	}
	
	function formatLastCheck(timestamp: string): string {
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
			<h3 class="text-lg font-semibold text-white">Security & Compliance</h3>
			<button 
				class="text-quantum-silver hover:text-white transition-colors"
				on:click={fetchSecurityStatus}
				title="Refresh security status"
			>
				<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
				</svg>
			</button>
		</div>
		
		{#if loading}
			<!-- Loading State -->
			<div class="flex items-center justify-center py-8">
				<div class="text-center">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-quantum-silver mx-auto mb-2"></div>
					<p class="text-sm text-gray-400">Loading security status...</p>
				</div>
			</div>
		{:else if securityStatus}
			<div class="space-y-6">
				<!-- DeterminismCage Status -->
				<div class="bg-gray-800 rounded-lg p-4">
					<div class="flex items-center justify-between mb-3">
						<h4 class="text-sm font-medium text-white">DeterminismCage</h4>
						<div class="flex items-center space-x-2">
							<span class="text-lg {getStatusColor(securityStatus.deterministicCage.active)}">
								{getStatusIcon(securityStatus.deterministicCage.active)}
							</span>
							<span class="text-xs font-medium {getStatusColor(securityStatus.deterministicCage.active)}">
								{securityStatus.deterministicCage.active ? 'Active' : 'Inactive'}
							</span>
						</div>
					</div>
					<div class="flex justify-between text-xs text-gray-400">
						<span>Violations</span>
						<span class="{securityStatus.deterministicCage.violations > 0 ? 'text-warning-primary' : 'text-success-primary'}">
							{securityStatus.deterministicCage.violations}
						</span>
					</div>
					{#if securityStatus.deterministicCage.lastViolation}
						<div class="flex justify-between text-xs text-gray-400 mt-1">
							<span>Last Violation</span>
							<span>{formatLastCheck(securityStatus.deterministicCage.lastViolation)}</span>
						</div>
					{/if}
				</div>
				
				<!-- Bus BIOS Status -->
				<div class="bg-gray-800 rounded-lg p-4">
					<div class="flex items-center justify-between mb-3">
						<h4 class="text-sm font-medium text-white">Bus BIOS</h4>
						<div class="flex items-center space-x-2">
							<span class="text-lg {getStatusColor(securityStatus.busBios.mode)}">
								{getStatusIcon(securityStatus.busBios.mode)}
							</span>
							<span class="text-xs font-medium {getStatusColor(securityStatus.busBios.mode)} capitalize">
								{securityStatus.busBios.mode}
							</span>
						</div>
					</div>
					<div class="space-y-1">
						<div class="flex justify-between text-xs text-gray-400">
							<span>Policy Violations</span>
							<span class="{securityStatus.busBios.policyViolations > 0 ? 'text-warning-primary' : 'text-success-primary'}">
								{securityStatus.busBios.policyViolations}
							</span>
						</div>
						<div class="flex justify-between text-xs text-gray-400">
							<span>Last Policy Check</span>
							<span>{formatLastCheck(securityStatus.busBios.lastPolicyCheck)}</span>
						</div>
					</div>
				</div>
				
				<!-- Encryption Status -->
				<div class="bg-gray-800 rounded-lg p-4">
					<div class="flex items-center justify-between mb-3">
						<h4 class="text-sm font-medium text-white">Encryption</h4>
						<div class="flex items-center space-x-2">
							<span class="text-lg {getStatusColor(securityStatus.encryption.enabled)}">
								{getStatusIcon(securityStatus.encryption.enabled)}
							</span>
							<span class="text-xs font-medium {getStatusColor(securityStatus.encryption.enabled)}">
								{securityStatus.encryption.enabled ? 'Enabled' : 'Disabled'}
							</span>
						</div>
					</div>
					<div class="space-y-1">
						<div class="flex justify-between text-xs text-gray-400">
							<span>Algorithm</span>
							<span class="text-white font-mono">{securityStatus.encryption.algorithm}</span>
						</div>
						<div class="flex justify-between text-xs text-gray-400">
							<span>Key Rotation</span>
							<span>{formatLastCheck(securityStatus.encryption.keyRotation)}</span>
						</div>
					</div>
				</div>
				
				<!-- Security Score -->
				<div class="pt-4 border-t border-gray-700">
					<div class="flex items-center justify-between mb-2">
						<span class="text-sm font-medium text-white">Security Score</span>
						<span class="text-lg font-bold text-success-primary">98%</span>
					</div>
					<div class="w-full bg-gray-700 rounded-full h-2">
						<div class="bg-success-primary h-2 rounded-full transition-all duration-300" style="width: 98%"></div>
					</div>
					<div class="text-xs text-gray-400 mt-1 text-center">
						Excellent security posture
					</div>
				</div>
			</div>
		{:else}
			<!-- Error State -->
			<div class="text-center py-8">
				<div class="text-gray-400 mb-2">
					<svg class="w-12 h-12 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
					</svg>
				</div>
				<p class="text-sm text-gray-400">Failed to load security status</p>
				<button 
					class="bpi-btn-secondary text-xs px-3 py-1 mt-2"
					on:click={fetchSecurityStatus}
				>
					Retry
				</button>
			</div>
		{/if}
		
		<!-- Actions -->
		<div class="mt-6 pt-6 border-t border-gray-700">
			<div class="grid grid-cols-2 gap-2">
				<button 
					class="bpi-btn-secondary text-xs px-3 py-2"
					on:click={() => dispatch('security-audit')}
				>
					Run Audit
				</button>
				<button 
					class="bpi-btn-secondary text-xs px-3 py-2"
					on:click={() => dispatch('security-settings')}
				>
					Settings
				</button>
			</div>
		</div>
	</div>
</div>
