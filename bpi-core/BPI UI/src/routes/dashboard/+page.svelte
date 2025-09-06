<script>
	import { onMount } from 'svelte';
	import MetricCard from '$lib/components/MetricCard.svelte';
	import BpiCoreMonitor from '$lib/components/BpiCoreMonitor.svelte';
	import NetworkStatus from '$lib/components/NetworkStatus.svelte';
	import MetricCard from '$lib/components/MetricCard.svelte';
	import NetworkStatus from '$lib/components/NetworkStatus.svelte';
	import { goto } from '$app/navigation';
	import { 
		bpiVmStatus, 
		bpiVmLoading, 
		bpiVmError, 
		walletData, 
		bpciEconomy,
		isConnected,
		securityRating,
		networkStatus,
		bpiActions 
	} from '$lib/stores/bpi';
	
	$: vmStatus = $bpiVmStatus;
	$: wallet = $walletData;
	$: economy = $bpciEconomy;
	$: loading = $bpiVmLoading;
	$: error = $bpiVmError;
	$: connected = $isConnected;
	$: security = $securityRating;
	$: network = $networkStatus;
	
	let showAdvancedMonitoring = true;
	
	onMount(() => {
		// Start real-time updates when component mounts
		return bpiActions.startRealTimeUpdates();
	});
	
	function goToWallet() {
		goto('/wallet');
	}
	
	function goToInstaller() {
		goto('/installer');
	}
	
	function goHome() {
		goto('/');
	}
</script>

<svelte:head>
	<title>BPI Dashboard - System Monitoring</title>
</svelte:head>

<div class="dashboard-container">
	<div class="container">
		<div class="dashboard-header">
			<div class="header-left">
				<Logo size="small" />
				<div class="header-title">
					<h1>BPI Core Dashboard</h1>
					<p>Real-time monitoring and system status</p>
				</div>
			</div>
			<div class="header-actions">
				<button class="btn btn-secondary" on:click={goHome}>
					🏠 Home
				</button>
				<button class="btn btn-primary" on:click={goToWallet}>
					💼 Wallet
				</button>
			</div>
		</div>
		
		<div class="metrics-grid fade-in">
			<MetricCard
				title="VM Server Status"
				value={vmStatus?.vm_server?.status || 'Disconnected'}
				icon="🖥️"
				type="status"
				status={connected ? 'active' : 'inactive'}
				{loading}
			/>
			<MetricCard
				title="Security Rating"
				value="{security}/10"
				icon="🔒"
				type="number"
				status={parseFloat(security) >= 9 ? 'active' : 'warning'}
				{loading}
			/>
			<MetricCard
				title="Network Type"
				value={network}
				icon="🌐"
				type="status"
				status={network === 'Disconnected' ? 'inactive' : 'active'}
				{loading}
			/>
			<MetricCard
				title="HTTP Cage"
				value={vmStatus?.integrations?.http_cage?.enabled ? 'Enabled' : 'Disabled'}
				icon="🔐"
				type="status"
				status={vmStatus?.integrations?.http_cage?.enabled ? 'active' : 'inactive'}
				{loading}
			/>
			<MetricCard
				title="ZKLock"
				value={vmStatus?.integrations?.zklock?.enabled ? 'Enabled' : 'Disabled'}
				icon="🔐"
				type="status"
				status={vmStatus?.integrations?.zklock?.enabled ? 'active' : 'inactive'}
				{loading}
			/>
			<MetricCard
				title="Post-Quantum"
				value={vmStatus?.vm_server?.post_quantum_enabled ? 'Active' : 'Inactive'}
				icon="⚛️"
				type="status"
				status={vmStatus?.vm_server?.post_quantum_enabled ? 'active' : 'inactive'}
				{loading}
			/>
		</div>
		
		<div class="dashboard-content slide-up">
			<div class="content-grid">
				<div class="network-section">
					<NetworkStatus />
				</div>
				
				<div class="quick-actions glass-card">
					<h3>Quick Actions</h3>
					<div class="actions-grid">
						<button class="action-btn" on:click={goToWallet}>
							<div class="action-icon">💼</div>
							<div class="action-text">
								<h4>Open Wallet</h4>
								<p>Manage your BPI assets</p>
							</div>
						</button>
						
						<button class="action-btn" on:click={goToInstaller}>
							<div class="action-icon">🔧</div>
							<div class="action-text">
								<h4>Reinstall</h4>
								<p>Run installer again</p>
							</div>
						</button>
						
						<button class="action-btn">
							<div class="action-icon">📊</div>
							<div class="action-text">
								<h4>Analytics</h4>
								<p>View detailed metrics</p>
							</div>
						</button>
						
						<button class="action-btn">
							<div class="action-icon">⚙️</div>
							<div class="action-text">
								<h4>Settings</h4>
								<p>Configure your node</p>
							</div>
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.dashboard-container {
		min-height: 100vh;
		padding: 20px 0;
	}
	
	.dashboard-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 30px;
		color: white;
		padding: 0 20px;
	}
	
	.header-left {
		display: flex;
		align-items: center;
		gap: 20px;
	}
	
	.header-title h1 {
		font-size: 2rem;
		font-weight: 700;
		margin: 0;
		text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
	}
	
	.header-title p {
		margin: 5px 0 0 0;
		opacity: 0.8;
	}
	
	.header-actions {
		display: flex;
		gap: 15px;
	}
	
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 20px;
		margin-bottom: 30px;
	}
	
	.dashboard-content {
		margin-top: 30px;
	}
	
	.content-grid {
		display: grid;
		grid-template-columns: 2fr 1fr;
		gap: 30px;
	}
	
	.network-section {
		min-height: 400px;
	}
	
	.quick-actions {
		padding: 25px;
		color: white;
	}
	
	.quick-actions h3 {
		margin-bottom: 20px;
		font-size: 1.3rem;
	}
	
	.actions-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 15px;
	}
	
	.action-btn {
		display: flex;
		align-items: center;
		gap: 15px;
		padding: 15px;
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 8px;
		color: white;
		text-align: left;
		cursor: pointer;
		transition: all 0.3s ease;
	}
	
	.action-btn:hover {
		background: rgba(255, 255, 255, 0.2);
		transform: translateY(-2px);
	}
	
	.action-icon {
		font-size: 1.5rem;
		min-width: 40px;
	}
	
	.action-text h4 {
		margin: 0 0 5px 0;
		font-size: 1rem;
	}
	
	.action-text p {
		margin: 0;
		font-size: 0.8rem;
		opacity: 0.8;
	}
	
	@media (max-width: 1024px) {
		.content-grid {
			grid-template-columns: 1fr;
		}
	}
	
	@media (max-width: 768px) {
		.dashboard-header {
			flex-direction: column;
			gap: 20px;
			text-align: center;
		}
		
		.header-left {
			flex-direction: column;
			gap: 10px;
		}
		
		.metrics-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
