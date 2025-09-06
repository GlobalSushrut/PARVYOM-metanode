<script lang="ts">
	import { onMount } from 'svelte';
	import { bpiVmStatus, bpiVmLoading, bpiVmError, walletData } from '$lib/stores/bpi';
	
	export let metrics = {};
	
	let realTimeUpdates = true;
	let lastUpdated = new Date();
	
	$: vmStatus = $bpiVmStatus;
	$: wallet = $walletData;
	$: loading = $bpiVmLoading;
	$: error = $bpiVmError;
	
	// Update last updated time when data changes
	$: if (vmStatus) {
		lastUpdated = new Date();
	}
	
	onMount(() => {
		const interval = setInterval(() => {
			if (realTimeUpdates) {
				lastUpdated = new Date();
			}
		}, 1000);
		
		return () => clearInterval(interval);
	});
	
	function toggleRealTimeUpdates() {
		realTimeUpdates = !realTimeUpdates;
	}
	
	function getStatusColor(status) {
		switch (status?.toLowerCase()) {
			case 'active':
			case 'enabled':
			case 'connected':
				return '#4CAF50';
			case 'inactive':
			case 'disabled':
			case 'disconnected':
				return '#dc3545';
			case 'warning':
				return '#ffc107';
			default:
				return '#6c757d';
		}
	}
</script>

<div class="network-status glass-card">
	<div class="status-header">
		<h3>🌐 Network Status</h3>
		<div class="header-controls">
			<button 
				class="toggle-btn" 
				class:active={realTimeUpdates}
				on:click={toggleRealTimeUpdates}
			>
				{realTimeUpdates ? '⏸️' : '▶️'} Real-time
			</button>
		</div>
	</div>
	
	{#if loading}
		<div class="loading-container">
			<div class="loading-spinner"></div>
			<p>Connecting to BPI network...</p>
		</div>
	{:else if error}
		<div class="error-container">
			<div class="error-icon">⚠️</div>
			<h4>Connection Error</h4>
			<p>{error}</p>
			<p class="error-hint">Make sure BPI VM server is running on port 7777</p>
		</div>
	{:else if vmStatus}
		<div class="status-grid">
			<!-- VM Server Status -->
			<div class="status-item">
				<div class="status-label">VM Server</div>
				<div 
					class="status-value"
					style="color: {getStatusColor(vmStatus.vm_server.status)}"
				>
					{vmStatus.vm_server.status}
				</div>
				<div class="status-detail">
					Security: {vmStatus.vm_server.security_rating}/10
				</div>
			</div>
			
			<!-- HTTP Cage -->
			<div class="status-item">
				<div class="status-label">HTTP Cage</div>
				<div 
					class="status-value"
					style="color: {getStatusColor(vmStatus.integrations.http_cage.enabled ? 'enabled' : 'disabled')}"
				>
					{vmStatus.integrations.http_cage.enabled ? 'Enabled' : 'Disabled'}
				</div>
				<div class="status-detail">
					Port: {vmStatus.integrations.http_cage.port} | Requests: {vmStatus.integrations.http_cage.requests}
				</div>
			</div>
			
			<!-- Shadow Registry -->
			<div class="status-item">
				<div class="status-label">Shadow Registry</div>
				<div 
					class="status-value"
					style="color: {getStatusColor(vmStatus.integrations.shadow_registry.enabled ? 'enabled' : 'disabled')}"
				>
					{vmStatus.integrations.shadow_registry.enabled ? 'Enabled' : 'Disabled'}
				</div>
				<div class="status-detail">
					Lookups: {vmStatus.integrations.shadow_registry.lookups}
				</div>
			</div>
			
			<!-- ZKLock -->
			<div class="status-item">
				<div class="status-label">ZKLock</div>
				<div 
					class="status-value"
					style="color: {getStatusColor(vmStatus.integrations.zklock.enabled ? 'enabled' : 'disabled')}"
				>
					{vmStatus.integrations.zklock.enabled ? 'Enabled' : 'Disabled'}
				</div>
				<div class="status-detail">
					Connections: {vmStatus.integrations.zklock.connections}
				</div>
			</div>
			
			<!-- Post-Quantum Security -->
			<div class="status-item">
				<div class="status-label">Post-Quantum</div>
				<div 
					class="status-value"
					style="color: {getStatusColor(vmStatus.vm_server.post_quantum_enabled ? 'active' : 'inactive')}"
				>
					{vmStatus.vm_server.post_quantum_enabled ? 'Active' : 'Inactive'}
				</div>
				<div class="status-detail">
					Operations: {vmStatus.statistics.post_quantum_operations}
				</div>
			</div>
			
			<!-- Network Type -->
			{#if wallet}
				<div class="status-item">
					<div class="status-label">Network</div>
					<div 
						class="status-value"
						style="color: {wallet.networkType === 'testnet' ? '#ffc107' : '#4CAF50'}"
					>
						{wallet.networkType.toUpperCase()}
					</div>
					<div class="status-detail">
						{wallet.walletType}
					</div>
				</div>
			{/if}
		</div>
		
		<div class="statistics-section">
			<h4>📊 System Statistics</h4>
			<div class="stats-grid">
				<div class="stat-item">
					<span class="stat-label">Total Requests:</span>
					<span class="stat-value">{vmStatus.statistics.total_requests.toLocaleString()}</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">Running Instances:</span>
					<span class="stat-value">{vmStatus.statistics.running_instances}</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">Avg Response Time:</span>
					<span class="stat-value">{vmStatus.statistics.avg_response_time_ms}ms</span>
				</div>
				<div class="stat-item">
					<span class="stat-label">Security Incidents:</span>
					<span class="stat-value" style="color: {vmStatus.statistics.security_incidents > 0 ? '#dc3545' : '#4CAF50'}">
						{vmStatus.statistics.security_incidents}
					</span>
				</div>
			</div>
		</div>
	{/if}
	
	<div class="status-footer">
		<div class="last-updated">
			Last updated: {lastUpdated.toLocaleTimeString()}
		</div>
		{#if realTimeUpdates}
			<div class="live-indicator">
				<div class="live-dot"></div>
				LIVE
			</div>
		{/if}
	</div>
</div>

<style>
	.network-status {
		padding: 25px;
		color: white;
		height: 100%;
		display: flex;
		flex-direction: column;
	}
	
	.status-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 25px;
	}
	
	.status-header h3 {
		margin: 0;
		font-size: 1.5rem;
	}
	
	.header-controls {
		display: flex;
		gap: 10px;
	}
	
	.toggle-btn {
		padding: 8px 12px;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 6px;
		color: white;
		cursor: pointer;
		transition: all 0.3s ease;
		font-size: 0.9rem;
	}
	
	.toggle-btn:hover {
		background: rgba(255, 255, 255, 0.2);
	}
	
	.toggle-btn.active {
		background: rgba(76, 175, 80, 0.3);
		border-color: rgba(76, 175, 80, 0.5);
	}
	
	.loading-container,
	.error-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		flex: 1;
		text-align: center;
		gap: 15px;
	}
	
	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(255, 255, 255, 0.3);
		border-top: 3px solid white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	
	.error-icon {
		font-size: 3rem;
	}
	
	.error-container h4 {
		margin: 0;
		color: #dc3545;
	}
	
	.error-hint {
		font-size: 0.9rem;
		opacity: 0.8;
		font-style: italic;
	}
	
	.status-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 20px;
		margin-bottom: 25px;
	}
	
	.status-item {
		background: rgba(255, 255, 255, 0.05);
		padding: 15px;
		border-radius: 8px;
		border: 1px solid rgba(255, 255, 255, 0.1);
	}
	
	.status-label {
		font-size: 0.9rem;
		opacity: 0.8;
		margin-bottom: 5px;
	}
	
	.status-value {
		font-size: 1.2rem;
		font-weight: 600;
		margin-bottom: 5px;
		text-transform: uppercase;
	}
	
	.status-detail {
		font-size: 0.8rem;
		opacity: 0.7;
	}
	
	.statistics-section {
		margin-top: auto;
		padding-top: 20px;
		border-top: 1px solid rgba(255, 255, 255, 0.2);
	}
	
	.statistics-section h4 {
		margin: 0 0 15px 0;
		font-size: 1.1rem;
	}
	
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 10px;
	}
	
	.stat-item {
		display: flex;
		justify-content: space-between;
		padding: 8px 0;
	}
	
	.stat-label {
		font-size: 0.9rem;
		opacity: 0.8;
	}
	
	.stat-value {
		font-weight: 600;
	}
	
	.status-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 20px;
		padding-top: 15px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		font-size: 0.8rem;
		opacity: 0.8;
	}
	
	.live-indicator {
		display: flex;
		align-items: center;
		gap: 5px;
		color: #4CAF50;
	}
	
	.live-dot {
		width: 8px;
		height: 8px;
		background: #4CAF50;
		border-radius: 50%;
		animation: pulse 2s infinite;
	}
	
	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}
	
	@keyframes pulse {
		0% { opacity: 1; }
		50% { opacity: 0.5; }
		100% { opacity: 1; }
	}
	
	@media (max-width: 768px) {
		.status-grid {
			grid-template-columns: 1fr;
		}
		
		.stats-grid {
			grid-template-columns: 1fr;
		}
		
		.status-footer {
			flex-direction: column;
			gap: 10px;
		}
	}
</style>
