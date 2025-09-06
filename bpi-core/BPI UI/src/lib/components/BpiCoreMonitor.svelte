<script lang="ts">
	import { onMount } from 'svelte';
	import { apiService } from '$lib/services/api';
	import MetricCard from './MetricCard.svelte';
	import { writable } from 'svelte/store';

	// Enterprise-grade monitoring stores
	const systemStatus = writable(null);
	const encClusterMetrics = writable(null);
	const zklockStats = writable(null);
	const mempoolStatus = writable(null);
	const oracleMetrics = writable(null);
	const securityMetrics = writable(null);
	const consensusMetrics = writable(null);
	const loading = writable(true);
	const error = writable(null);

	let refreshInterval;
	let realTimeEnabled = true;

	// Advanced monitoring data fetching
	async function fetchAllMetrics() {
		loading.set(true);
		error.set(null);
		
		try {
			// Parallel fetching for enterprise performance
			const [
				systemData,
				encData,
				zklockData,
				mempoolData,
				oracleData,
				securityData,
				consensusData
			] = await Promise.allSettled([
				apiService.getBpiCoreSystemStatus(),
				apiService.getEncClusterMetrics(),
				apiService.getZKLockDeviceStats(),
				apiService.getMempoolStatus(),
				apiService.getOracleNodeMetrics(),
				apiService.getSecurityMetrics(),
				apiService.getConsensusMetrics()
			]);

			// Process results with enterprise error handling
			if (systemData.status === 'fulfilled') systemStatus.set(systemData.value);
			if (encData.status === 'fulfilled') encClusterMetrics.set(encData.value);
			if (zklockData.status === 'fulfilled') zklockStats.set(zklockData.value);
			if (mempoolData.status === 'fulfilled') mempoolStatus.set(mempoolData.value);
			if (oracleData.status === 'fulfilled') oracleMetrics.set(oracleData.value);
			if (securityData.status === 'fulfilled') securityMetrics.set(securityData.value);
			if (consensusData.status === 'fulfilled') consensusMetrics.set(consensusData.value);

		} catch (err) {
			error.set(`Failed to fetch monitoring data: ${err.message}`);
		} finally {
			loading.set(false);
		}
	}

	function startRealTimeMonitoring() {
		if (refreshInterval) clearInterval(refreshInterval);
		refreshInterval = setInterval(fetchAllMetrics, 3000); // 3-second enterprise refresh
	}

	function stopRealTimeMonitoring() {
		if (refreshInterval) clearInterval(refreshInterval);
	}

	function toggleRealTime() {
		realTimeEnabled = !realTimeEnabled;
		if (realTimeEnabled) {
			startRealTimeMonitoring();
		} else {
			stopRealTimeMonitoring();
		}
	}

	onMount(() => {
		fetchAllMetrics();
		if (realTimeEnabled) startRealTimeMonitoring();
		
		return () => {
			if (refreshInterval) clearInterval(refreshInterval);
		};
	});

	// Reactive data
	$: system = $systemStatus;
	$: encCluster = $encClusterMetrics;
	$: zklock = $zklockStats;
	$: mempool = $mempoolStatus;
	$: oracle = $oracleMetrics;
	$: security = $securityMetrics;
	$: consensus = $consensusMetrics;
	$: isLoading = $loading;
	$: errorMessage = $error;
</script>

<div class="bpi-core-monitor">
	<div class="monitor-header">
		<h2 class="monitor-title">
			<span class="title-icon">🚀</span>
			PARVYOM Metanode - BPI Core System Monitor
		</h2>
		<div class="monitor-controls">
			<button 
				class="real-time-toggle {realTimeEnabled ? 'active' : ''}"
				on:click={toggleRealTime}
			>
				<span class="toggle-icon">{realTimeEnabled ? '⏸️' : '▶️'}</span>
				{realTimeEnabled ? 'Pause' : 'Start'} Real-Time
			</button>
			<button class="refresh-btn" on:click={fetchAllMetrics} disabled={isLoading}>
				<span class="refresh-icon {isLoading ? 'spinning' : ''}">🔄</span>
				Refresh
			</button>
		</div>
	</div>

	{#if errorMessage}
		<div class="error-banner">
			<span class="error-icon">⚠️</span>
			{errorMessage}
		</div>
	{/if}

	<!-- BPI Core System Status -->
	<section class="monitor-section">
		<h3 class="section-title">
			<span class="section-icon">🏗️</span>
			Core System Status
		</h3>
		<div class="metrics-grid">
			<MetricCard
				title="System Health"
				value={system?.health || 'Unknown'}
				icon="💚"
				type="status"
				status={system?.health === 'Healthy' ? 'active' : 'warning'}
				loading={isLoading}
			/>
			<MetricCard
				title="Uptime"
				value={system?.uptime || '0d 0h 0m'}
				icon="⏱️"
				type="time"
				loading={isLoading}
			/>
			<MetricCard
				title="CPU Usage"
				value="{system?.cpu_usage || 0}%"
				icon="🖥️"
				type="percentage"
				status={system?.cpu_usage > 80 ? 'warning' : 'active'}
				loading={isLoading}
			/>
			<MetricCard
				title="Memory Usage"
				value="{system?.memory_usage || 0}%"
				icon="💾"
				type="percentage"
				status={system?.memory_usage > 85 ? 'warning' : 'active'}
				loading={isLoading}
			/>
		</div>
	</section>

	<!-- ENC Cluster Monitoring -->
	<section class="monitor-section">
		<h3 class="section-title">
			<span class="section-icon">🔗</span>
			ENC Cluster
		</h3>
		<div class="metrics-grid">
			<MetricCard
				title="Active Nodes"
				value={encCluster?.active_nodes || 0}
				icon="🌐"
				type="number"
				status={encCluster?.active_nodes > 0 ? 'active' : 'inactive'}
				loading={isLoading}
			/>
			<MetricCard
				title="Consensus Status"
				value={encCluster?.consensus_status || 'Unknown'}
				icon="🤝"
				type="status"
				status={encCluster?.consensus_status === 'Active' ? 'active' : 'warning'}
				loading={isLoading}
			/>
			<MetricCard
				title="Block Height"
				value={(encCluster?.block_height || 0).toLocaleString()}
				icon="📦"
				type="number"
				loading={isLoading}
			/>
			<MetricCard
				title="Network Hash Rate"
				value={encCluster?.hash_rate || '0 H/s'}
				icon="⚡"
				type="performance"
				loading={isLoading}
			/>
		</div>
	</section>

	<!-- ZKLock Mobile/IoT Monitoring -->
	<section class="monitor-section">
		<h3 class="section-title">
			<span class="section-icon">🔐</span>
			ZKLock Mobile/IoT
		</h3>
		<div class="metrics-grid">
			<MetricCard
				title="Total Devices"
				value={zklock?.total_devices || 0}
				icon="📱"
				type="number"
				loading={isLoading}
			/>
			<MetricCard
				title="Active Devices"
				value={zklock?.active_devices || 0}
				icon="🟢"
				type="number"
				status={zklock?.active_devices > 0 ? 'active' : 'inactive'}
				loading={isLoading}
			/>
			<MetricCard
				title="Proofs Generated"
				value={(zklock?.total_proofs || 0).toLocaleString()}
				icon="🔏"
				type="number"
				loading={isLoading}
			/>
			<MetricCard
				title="IoT Participation"
				value="{zklock?.iot_participation_rate || 0}%"
				icon="🌐"
				type="percentage"
				loading={isLoading}
			/>
		</div>
	</section>

	<!-- Mempool Monitoring -->
	<section class="monitor-section">
		<h3 class="section-title">
			<span class="section-icon">🔄</span>
			Transaction Mempool
		</h3>
		<div class="metrics-grid">
			<MetricCard
				title="Pending Transactions"
				value={mempool?.pending_transactions || 0}
				icon="⏳"
				type="number"
				status={mempool?.pending_transactions > 1000 ? 'warning' : 'active'}
				loading={isLoading}
			/>
			<MetricCard
				title="Throughput"
				value="{mempool?.throughput || 0} TPS"
				icon="🚀"
				type="performance"
				loading={isLoading}
			/>
			<MetricCard
				title="Queue Size"
				value="{(mempool?.queue_size || 0).toLocaleString()} MB"
				icon="📊"
				type="number"
				loading={isLoading}
			/>
			<MetricCard
				title="Processing Rate"
				value="{mempool?.processing_rate || 0}%"
				icon="⚙️"
				type="percentage"
				loading={isLoading}
			/>
		</div>
	</section>

	<!-- Oracle Node Monitoring -->
	<section class="monitor-section">
		<h3 class="section-title">
			<span class="section-icon">🔮</span>
			Oracle Network
		</h3>
		<div class="metrics-grid">
			<MetricCard
				title="Active Oracles"
				value={oracle?.active_oracles || 0}
				icon="🌟"
				type="number"
				status={oracle?.active_oracles > 0 ? 'active' : 'inactive'}
				loading={isLoading}
			/>
			<MetricCard
				title="Data Feeds"
				value={oracle?.data_feeds || 0}
				icon="📡"
				type="number"
				loading={isLoading}
			/>
			<MetricCard
				title="Last Update"
				value={oracle?.last_update || 'Never'}
				icon="🕐"
				type="time"
				loading={isLoading}
			/>
			<MetricCard
				title="Reliability Score"
				value="{oracle?.reliability_score || 0}%"
				icon="🎯"
				type="percentage"
				status={oracle?.reliability_score > 95 ? 'active' : 'warning'}
				loading={isLoading}
			/>
		</div>
	</section>

	<!-- Security Monitoring -->
	<section class="monitor-section">
		<h3 class="section-title">
			<span class="section-icon">🛡️</span>
			Security & Compliance
		</h3>
		<div class="metrics-grid">
			<MetricCard
				title="Security Rating"
				value="{security?.security_rating || 0}/10"
				icon="🔒"
				type="number"
				status={security?.security_rating >= 9 ? 'active' : 'warning'}
				loading={isLoading}
			/>
			<MetricCard
				title="Post-Quantum"
				value={security?.post_quantum_enabled ? 'Active' : 'Inactive'}
				icon="⚛️"
				type="status"
				status={security?.post_quantum_enabled ? 'active' : 'warning'}
				loading={isLoading}
			/>
			<MetricCard
				title="HTTP Cage"
				value={security?.http_cage_status || 'Unknown'}
				icon="🏰"
				type="status"
				status={security?.http_cage_status === 'Active' ? 'active' : 'warning'}
				loading={isLoading}
			/>
			<MetricCard
				title="Audit Score"
				value="{security?.audit_score || 0}%"
				icon="📋"
				type="percentage"
				status={security?.audit_score > 90 ? 'active' : 'warning'}
				loading={isLoading}
			/>
		</div>
	</section>
</div>

<style>
	.bpi-core-monitor {
		padding: 20px;
		background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
		min-height: 100vh;
		color: white;
	}

	.monitor-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 30px;
		padding: 20px;
		background: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
		border-radius: 15px;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.monitor-title {
		font-size: 2rem;
		font-weight: 700;
		margin: 0;
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.title-icon {
		font-size: 2.5rem;
		animation: pulse 2s infinite;
	}

	.monitor-controls {
		display: flex;
		gap: 12px;
		align-items: center;
	}

	.real-time-toggle {
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		color: white;
		border: none;
		padding: 12px 20px;
		border-radius: 25px;
		cursor: pointer;
		font-weight: 600;
		display: flex;
		align-items: center;
		gap: 8px;
		transition: all 0.3s ease;
	}

	.real-time-toggle.active {
		background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
		box-shadow: 0 4px 15px rgba(240, 147, 251, 0.4);
	}

	.real-time-toggle:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
	}

	.refresh-btn {
		background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
		color: white;
		border: none;
		padding: 12px 20px;
		border-radius: 25px;
		cursor: pointer;
		font-weight: 600;
		display: flex;
		align-items: center;
		gap: 8px;
		transition: all 0.3s ease;
	}

	.refresh-btn:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(79, 172, 254, 0.4);
	}

	.refresh-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.refresh-icon.spinning {
		animation: spin 1s linear infinite;
	}

	.error-banner {
		background: linear-gradient(135deg, #ff6b6b 0%, #ee5a52 100%);
		color: white;
		padding: 15px 20px;
		border-radius: 10px;
		margin-bottom: 20px;
		display: flex;
		align-items: center;
		gap: 10px;
		font-weight: 600;
		box-shadow: 0 4px 15px rgba(255, 107, 107, 0.3);
	}

	.monitor-section {
		margin-bottom: 40px;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 600;
		margin-bottom: 20px;
		display: flex;
		align-items: center;
		gap: 12px;
		color: #ffd700;
		text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
	}

	.section-icon {
		font-size: 1.8rem;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 20px;
	}

	@keyframes pulse {
		0%, 100% { transform: scale(1); }
		50% { transform: scale(1.05); }
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* Enterprise responsive design */
	@media (max-width: 768px) {
		.monitor-header {
			flex-direction: column;
			gap: 15px;
		}

		.monitor-title {
			font-size: 1.5rem;
		}

		.metrics-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (min-width: 1400px) {
		.metrics-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}
</style>
