<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import { bpiLiveDataStore } from '$lib/stores/bpi-live-data';
	import { authStore } from '$lib/stores/auth';
	import BPICoreComponentsPanel from '$lib/components/dashboard/BPICoreComponentsPanel.svelte';
	import DockLockPanel from '$lib/components/dashboard/DockLockPanel.svelte';
	import SecurityPanel from '$lib/components/dashboard/SecurityPanel.svelte';
	import LiveMetricsPanel from '$lib/components/dashboard/LiveMetricsPanel.svelte';
	import GrafanaPanel from '$lib/components/dashboard/GrafanaPanel.svelte';

	let refreshInterval: ReturnType<typeof setInterval>;
	let logs: Array<{timestamp: string, level: string, message: string, component: string}> = [];

	// Real-time WebSocket logs
	let ws: WebSocket | null = null;

	function initializeWebSocketLogs() {
		try {
			ws = new WebSocket('ws://localhost:8617/ws/bpi/logs');
			
			ws.onmessage = (event) => {
				try {
					const logEntry = JSON.parse(event.data);
					logs = [logEntry, ...logs.slice(0, 99)]; // Keep last 100 logs
				} catch (err) {
					console.error('Failed to parse log entry:', err);
				}
			};

			ws.onclose = () => {
				console.log('Log WebSocket disconnected, attempting reconnect...');
				setTimeout(initializeWebSocketLogs, 5000);
			};

		} catch (err) {
			console.error('Failed to initialize log WebSocket:', err);
			setTimeout(initializeWebSocketLogs, 5000);
		}
	}

	async function executeSystemCommand(command: string) {
		console.log(`Executing BPI Core command: ${command}`);
		
		try {
			const response = await fetch('/api/bpi/system/command', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'Authorization': 'Bearer bpi-core-token',
					'X-BPI-Security-Level': 'MAXIMUM'
				},
				body: JSON.stringify({ command })
			});

			const result = await response.json();
			
			// Add to logs
			logs = [{
				timestamp: new Date().toISOString(),
				level: response.ok ? 'INFO' : 'ERROR',
				message: response.ok ? `Command executed: ${command}` : `Command failed: ${result.error}`,
				component: 'SYSTEM'
			}, ...logs.slice(0, 99)];
			
		} catch (error) {
			logs = [{
				timestamp: new Date().toISOString(),
				level: 'ERROR',
				message: `Command execution failed: ${error}`,
				component: 'SYSTEM'
			}, ...logs.slice(0, 99)];
		}
	}

	onMount(async () => {
		// Initialize BPI live data store
		bpiLiveDataStore.startDataFetching();
		bpiLiveDataStore.initWebSocket();
		
		// Initialize WebSocket logs
		initializeWebSocketLogs();
		
		// Set up periodic refresh
		refreshInterval = setInterval(() => {
			bpiLiveDataStore.refresh();
		}, 10000); // Every 10 seconds
	});

	onDestroy(() => {
		if (refreshInterval) {
			clearInterval(refreshInterval);
		}
		if (ws) {
			ws.close();
		}
		bpiLiveDataStore.cleanup();
	});

	// Navigation functions
	function navigateToWallet() {
		goto('/wallet');
	}

	function navigateToLogin() {
		goto('/login');
	}

	async function handleLogout() {
		await authStore.logout();
		goto('/login');
	}
</script>

<svelte:head>
	<title>BPI Operations Center</title>
	<meta name="description" content="Enterprise Blockchain Infrastructure Dashboard" />
</svelte:head>

<div class="min-h-screen bg-slate-900 text-white">
	<!-- Navigation Header -->
	<div class="bg-slate-800 border-b border-slate-700 px-6 py-4">
		<div class="flex items-center justify-between">
			<div class="flex items-center space-x-4">
				<div class="w-10 h-10 bg-gradient-to-r from-blue-500 to-purple-600 rounded-xl flex items-center justify-center">
					<svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
						<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
					</svg>
				</div>
				<div>
					<h1 class="text-xl font-bold">BPI Operations Center</h1>
					<p class="text-sm text-gray-400">Enterprise Blockchain Infrastructure</p>
				</div>
			</div>
			
			<!-- Navigation Menu -->
			<div class="flex items-center space-x-4">
				<h1 class="text-xl font-bold text-white">BPI Dashboard</h1>
				<nav class="flex space-x-6">
					<button 
						on:click={() => goto('/')}
						class="text-blue-400 hover:text-blue-300 transition-colors font-medium"
					>
						Dashboard
					</button>
					<button 
						on:click={() => goto('/metrics')}
						class="text-gray-300 hover:text-white transition-colors font-medium"
					>
						System Metrics
					</button>
					<button 
						on:click={() => goto('/wallet')}
						class="text-gray-300 hover:text-white transition-colors font-medium"
					>
						Wallet
					</button>
					<button 
						on:click={() => goto('/mesh')}
						class="text-gray-300 hover:text-white transition-colors font-medium"
					>
						Mesh
					</button>
					<button 
						on:click={() => goto('/docklock')}
						class="text-gray-300 hover:text-white transition-colors font-medium"
					>
						DockLock
					</button>
				</nav>

				<!-- Status and User Menu -->
				<div class="flex items-center space-x-3">
					<div class="flex items-center space-x-2 bg-green-500/20 border border-green-500/30 rounded-lg px-3 py-1">
						<div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
						<span class="text-sm text-green-300">BPI Dashboard Online</span>
					</div>

					{#if $authStore.isAuthenticated}
						<!-- Authenticated User Menu -->
						<div class="flex items-center space-x-3">
							<div class="flex items-center space-x-2 text-sm">
								<div class="w-8 h-8 bg-gradient-to-r from-purple-500 to-pink-600 rounded-full flex items-center justify-center">
									<svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
									</svg>
								</div>
								<span class="text-gray-300">{$authStore.user?.username || 'User'}</span>
							</div>
							<button
								on:click={handleLogout}
								class="bg-red-600 hover:bg-red-700 px-3 py-1 rounded-lg text-sm font-medium transition-colors"
							>
								Logout
							</button>
						</div>
					{:else}
						<!-- Login Button -->
						<button
							on:click={navigateToLogin}
							class="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg font-medium transition-colors"
						>
							Login
						</button>
					{/if}
				</div>
				<!-- Refresh Button -->
				<button
					on:click={() => bpiLiveDataStore.refresh()}
					class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded font-mono text-sm transition-colors"
					disabled={$bpiLiveDataStore.loading}
				>
					{$bpiLiveDataStore.loading ? '⟳' : '↻'} Refresh
				</button>
			</div>
		</div>
	</div>

	<!-- Grafana-Style Dashboard Grid -->
	<div class="p-6 space-y-6">
		<!-- Top Row - Live Metrics and Core Components -->
		<div class="grid grid-cols-1 xl:grid-cols-2 gap-6">
			<LiveMetricsPanel />
			<BPICoreComponentsPanel />
		</div>

		<!-- Second Row - DockLock and Security -->
		<div class="grid grid-cols-1 xl:grid-cols-2 gap-6">
			<DockLockPanel />
			<SecurityPanel />
		</div>

		<!-- Third Row - System Logs and Actions -->
		<div class="grid grid-cols-1 xl:grid-cols-2 gap-6">
			<!-- System Logs Panel -->
			<GrafanaPanel
				title="Live System Logs"
				subtitle="Real-time BPI Core system events and alerts"
				height="h-96"
				loading={false}
				lastUpdate={logs.length > 0 ? new Date(logs[0].timestamp) : null}
			>
				<div class="bg-black rounded-lg p-4 h-full overflow-y-auto font-mono text-sm">
					{#if logs.length === 0}
						<div class="flex items-center justify-center h-full">
							<p class="text-gray-500">Waiting for log entries...</p>
						</div>
					{:else}
						{#each logs as log}
							<div class="flex items-start space-x-2 mb-1 hover:bg-gray-800 px-2 py-1 rounded">
								<span class="text-gray-500 text-xs whitespace-nowrap">
									{new Date(log.timestamp).toLocaleTimeString()}
								</span>
								<span class="text-xs {log.level === 'ERROR' ? 'text-red-400' : log.level === 'WARN' ? 'text-yellow-400' : log.level === 'INFO' ? 'text-green-400' : 'text-blue-400'} whitespace-nowrap">
									[{log.level}]
								</span>
								<span class="text-xs text-blue-400 whitespace-nowrap">[{log.component}]</span>
								<span class="text-gray-300 text-xs">{log.message}</span>
							</div>
						{/each}
					{/if}
				</div>
			</GrafanaPanel>

			<!-- System Actions Panel -->
			<GrafanaPanel
				title="System Actions & Controls"
				subtitle="BPI Core system management and control operations"
				height="h-96"
				loading={false}
			>
				<div class="space-y-6">
					<!-- Quick Actions -->
					<div>
						<h4 class="text-lg font-semibold text-white font-mono mb-4">🚀 Quick Actions</h4>
						<div class="grid grid-cols-2 gap-3">
							<button 
								on:click={() => executeSystemCommand('fips_validate')}
								class="px-4 py-3 bg-green-600 hover:bg-green-700 text-white rounded font-mono text-sm transition-colors"
							>
								🔒 FIPS Validate
							</button>
							<button 
								on:click={() => executeSystemCommand('quantum_init')}
								class="px-4 py-3 bg-purple-600 hover:bg-purple-700 text-white rounded font-mono text-sm transition-colors"
							>
								⚛️ Quantum Init
							</button>
							<button 
								on:click={() => executeSystemCommand('security_audit')}
								class="px-4 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded font-mono text-sm transition-colors"
							>
								🔍 Security Audit
							</button>
							<button 
								on:click={() => executeSystemCommand('emergency_shutdown')}
								class="px-4 py-3 bg-red-600 hover:bg-red-700 text-white rounded font-mono text-sm transition-colors"
							>
								🛑 Emergency Stop
							</button>
						</div>
					</div>

					<!-- Container Actions -->
					<div>
						<h4 class="text-lg font-semibold text-white font-mono mb-4">📦 Container Management</h4>
						<div class="grid grid-cols-2 gap-3">
							<button 
								on:click={() => executeSystemCommand('docklock_deploy')}
								class="px-4 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded font-mono text-sm transition-colors"
							>
								🚀 Deploy Container
							</button>
							<button 
								on:click={() => executeSystemCommand('cluster_scale')}
								class="px-4 py-3 bg-green-600 hover:bg-green-700 text-white rounded font-mono text-sm transition-colors"
							>
								📈 Scale Cluster
							</button>
							<button 
								on:click={() => executeSystemCommand('encryption_rotate')}
								class="px-4 py-3 bg-purple-600 hover:bg-purple-700 text-white rounded font-mono text-sm transition-colors"
							>
								🔐 Rotate Keys
							</button>
							<button 
								on:click={() => executeSystemCommand('container_restart')}
								class="px-4 py-3 bg-yellow-600 hover:bg-yellow-700 text-white rounded font-mono text-sm transition-colors"
							>
								🔄 Restart All
							</button>
						</div>
					</div>

					<!-- System Status -->
					<div class="bg-gray-800 rounded-lg p-4">
						<h5 class="text-sm font-mono text-gray-300 mb-3">📊 SYSTEM STATUS</h5>
						<div class="grid grid-cols-2 gap-4 text-xs">
							<div class="flex justify-between">
								<span class="text-gray-400 font-mono">Data Store:</span>
								<span class="font-mono {$bpiLiveDataStore.connected ? 'text-green-400' : 'text-red-400'}">
									{$bpiLiveDataStore.connected ? 'CONNECTED' : 'DISCONNECTED'}
								</span>
							</div>
							<div class="flex justify-between">
								<span class="text-gray-400 font-mono">WebSocket:</span>
								<span class="font-mono {ws ? 'text-green-400' : 'text-red-400'}">
									{ws ? 'ACTIVE' : 'INACTIVE'}
								</span>
							</div>
							<div class="flex justify-between">
								<span class="text-gray-400 font-mono">Log Entries:</span>
								<span class="text-blue-400 font-mono">{logs.length}</span>
							</div>
							<div class="flex justify-between">
								<span class="text-gray-400 font-mono">Auto-Refresh:</span>
								<span class="text-green-400 font-mono">ENABLED</span>
							</div>
						</div>
					</div>
				</div>
			</GrafanaPanel>
		</div>
	</div>
</main>
