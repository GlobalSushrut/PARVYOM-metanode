<script lang="ts">
	import { onMount } from 'svelte';
	import GrafanaPanel from './GrafanaPanel.svelte';
	import { securityStore } from '$lib/stores/bpi-live-data';

	function getSecurityLevelColor(score: number): string {
		if (score >= 90) return 'text-green-400';
		if (score >= 70) return 'text-yellow-400';
		return 'text-red-400';
	}

	function getSecurityLevelBg(score: number): string {
		if (score >= 90) return 'bg-green-400';
		if (score >= 70) return 'bg-yellow-400';
		return 'bg-red-400';
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleString();
	}
</script>

<GrafanaPanel
	title="Security & Compliance Monitoring"
	subtitle="Real-time security status, auditing, and compliance metrics"
	height="h-[600px]"
	loading={!$securityStore}
	lastUpdate={$securityStore ? new Date() : null}
>
	{#if $securityStore}
		<div class="grid grid-cols-2 gap-6 h-full overflow-y-auto">
			<!-- Encryption Status -->
			<div class="space-y-4">
				<h4 class="text-lg font-semibold text-white font-mono border-b border-gray-700 pb-2">
					🔐 Encryption Systems
				</h4>
				
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<div class="flex items-center justify-between mb-3">
						<span class="text-sm font-mono text-gray-300">BPI ENCRYPTION</span>
						<div class="flex items-center space-x-2">
							<div class="w-3 h-3 rounded-full {$securityStore.encryption.bpi_enc_active ? 'bg-green-400 animate-pulse' : 'bg-red-400'}"></div>
							<span class="text-xs font-mono {$securityStore.encryption.bpi_enc_active ? 'text-green-400' : 'text-red-400'}">
								{$securityStore.encryption.bpi_enc_active ? 'ACTIVE' : 'INACTIVE'}
							</span>
						</div>
					</div>
					
					<div class="space-y-2 text-xs">
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Encryption Enabled:</span>
							<span class="font-mono {$securityStore.encryption.encryption_enabled ? 'text-green-400' : 'text-red-400'}">
								{$securityStore.encryption.encryption_enabled ? 'YES' : 'NO'}
							</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Keys Rotated:</span>
							<span class="text-blue-400 font-mono">{$securityStore.encryption.keys_rotated}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Last Rotation:</span>
							<span class="text-gray-300 font-mono">{formatDate($securityStore.encryption.last_key_rotation)}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Strength:</span>
							<span class="text-purple-400 font-mono">{$securityStore.encryption.encryption_strength}</span>
						</div>
					</div>
				</div>

				<!-- Court Node Status -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<div class="flex items-center justify-between mb-3">
						<span class="text-sm font-mono text-gray-300">⚖️ COURT NODE</span>
						<div class="flex items-center space-x-2">
							<div class="w-3 h-3 rounded-full {$securityStore.court_node.active ? 'bg-green-400 animate-pulse' : 'bg-gray-400'}"></div>
							<span class="text-xs font-mono {$securityStore.court_node.active ? 'text-green-400' : 'text-gray-400'}">
								{$securityStore.court_node.active ? 'ACTIVE' : 'INACTIVE'}
							</span>
						</div>
					</div>
					
					<div class="space-y-2 text-xs">
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Disputes Processed:</span>
							<span class="text-green-400 font-mono">{$securityStore.court_node.disputes_processed}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Pending Disputes:</span>
							<span class="text-yellow-400 font-mono">{$securityStore.court_node.pending_disputes}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Resolution Rate:</span>
							<span class="text-blue-400 font-mono">{$securityStore.court_node.resolution_rate}%</span>
						</div>
					</div>
				</div>

				<!-- Shadow Registry -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<h5 class="text-sm font-mono text-gray-300 mb-3">👥 SHADOW REGISTRY</h5>
					<div class="space-y-2 text-xs">
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Registry Entries:</span>
							<span class="text-blue-400 font-mono">{$securityStore.shadow_registry.entries}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Cross-Chain Verifications:</span>
							<span class="text-green-400 font-mono">{$securityStore.shadow_registry.cross_chain_verifications}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Sync Status:</span>
							<span class="text-purple-400 font-mono">{$securityStore.shadow_registry.shadow_sync_status}</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Auditing & Compliance -->
			<div class="space-y-4">
				<h4 class="text-lg font-semibold text-white font-mono border-b border-gray-700 pb-2">
					🔍 Auditing & Compliance
				</h4>
				
				<!-- Compliance Score -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<div class="flex items-center justify-between mb-3">
						<span class="text-sm font-mono text-gray-300">COMPLIANCE SCORE</span>
						<span class="text-2xl font-bold font-mono {getSecurityLevelColor($securityStore.auditing.compliance_score)}">
							{$securityStore.auditing.compliance_score}%
						</span>
					</div>
					
					<div class="w-full bg-gray-700 rounded-full h-3 mb-3">
						<div 
							class="h-3 rounded-full {getSecurityLevelBg($securityStore.auditing.compliance_score)}"
							style="width: {$securityStore.auditing.compliance_score}%"
						></div>
					</div>
					
					<div class="text-xs text-center font-mono {getSecurityLevelColor($securityStore.auditing.compliance_score)}">
						{$securityStore.auditing.compliance_score >= 90 ? 'EXCELLENT' : 
						 $securityStore.auditing.compliance_score >= 70 ? 'GOOD' : 'NEEDS ATTENTION'}
					</div>
				</div>

				<!-- Split-Origin Auditing -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<div class="flex items-center justify-between mb-3">
						<span class="text-sm font-mono text-gray-300">SPLIT-ORIGIN AUDIT</span>
						<div class="flex items-center space-x-2">
							<div class="w-3 h-3 rounded-full {$securityStore.auditing.split_origin_active ? 'bg-green-400 animate-pulse' : 'bg-red-400'}"></div>
							<span class="text-xs font-mono {$securityStore.auditing.split_origin_active ? 'text-green-400' : 'text-red-400'}">
								{$securityStore.auditing.split_origin_active ? 'ACTIVE' : 'INACTIVE'}
							</span>
						</div>
					</div>
					
					<div class="space-y-2 text-xs">
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Audit Entries:</span>
							<span class="text-blue-400 font-mono">{$securityStore.auditing.audit_entries.toLocaleString()}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Last Audit:</span>
							<span class="text-gray-300 font-mono">{formatDate($securityStore.auditing.last_audit)}</span>
						</div>
					</div>
				</div>

				<!-- Notary Registry -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<h5 class="text-sm font-mono text-gray-300 mb-3">✓ NOTARY REGISTRY</h5>
					<div class="space-y-2 text-xs">
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Registered Notaries:</span>
							<span class="text-green-400 font-mono">{$securityStore.notary_registry.registered_notaries}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Active Notarizations:</span>
							<span class="text-blue-400 font-mono">{$securityStore.notary_registry.active_notarizations}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-gray-400 font-mono">Verification Rate:</span>
							<span class="text-purple-400 font-mono">{$securityStore.notary_registry.verification_rate}%</span>
						</div>
					</div>
				</div>

				<!-- Security Actions -->
				<div class="bg-gray-800 rounded-lg p-4 border border-gray-700">
					<h5 class="text-sm font-mono text-gray-300 mb-3">SECURITY ACTIONS</h5>
					<div class="grid grid-cols-2 gap-2">
						<button class="px-3 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded font-mono text-xs transition-colors">
							🔄 Rotate Keys
						</button>
						<button class="px-3 py-2 bg-green-600 hover:bg-green-700 text-white rounded font-mono text-xs transition-colors">
							🔍 Run Audit
						</button>
						<button class="px-3 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded font-mono text-xs transition-colors">
							✓ Verify Notary
						</button>
						<button class="px-3 py-2 bg-red-600 hover:bg-red-700 text-white rounded font-mono text-xs transition-colors">
							🚨 Security Alert
						</button>
					</div>
				</div>
			</div>
		</div>
	{:else}
		<div class="flex items-center justify-center h-full">
			<div class="text-center">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-green-400 mx-auto mb-4"></div>
				<p class="text-gray-400 font-mono">Loading Security Status...</p>
			</div>
		</div>
	{/if}
</GrafanaPanel>
