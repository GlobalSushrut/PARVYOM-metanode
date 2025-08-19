<script lang="ts">
	import { onMount } from 'svelte';
	import { walletStore } from '$lib/stores/wallet';
	import { authStore } from '$lib/stores/auth';
	import type { Transaction, MultiSigRequest, ComplianceAlert } from '$lib/stores/wallet';

	// UI state
	let activeTab: 'overview' | 'transactions' | 'multisig' | 'compliance' | 'settings' = 'overview';
	let showSendModal = false;
	let showCreateWalletModal = false;
	let isLoading = false;

	// Send transaction form
	let sendForm = {
		recipient: '',
		amount: '',
		memo: '',
		priority: 'normal' as 'low' | 'normal' | 'high'
	};

	// Create wallet form
	let createWalletForm = {
		name: '',
		type: 'bank_stamped' as 'bank_stamped' | 'government_stamped' | 'standard',
		requiredSignatures: 2,
		totalSigners: 3
	};

	// Reactive statements
	$: wallet = $walletStore.currentWallet;
	$: transactions = $walletStore.transactions;
	$: multiSigRequests = $walletStore.multiSigRequests;
	$: complianceAlerts = $walletStore.complianceAlerts;
	$: isConnected = $walletStore.isConnected;

	onMount(() => {
		// Load wallet data if authenticated
		if ($authStore.isAuthenticated && !isConnected) {
			walletStore.connectWallet();
		}
	});

	async function handleSendTransaction() {
		if (!sendForm.recipient || !sendForm.amount) return;
		
		isLoading = true;
		const success = await walletStore.sendTransaction({
			recipient: sendForm.recipient,
			amount: parseFloat(sendForm.amount),
			memo: sendForm.memo,
			priority: sendForm.priority
		});
		
		if (success) {
			showSendModal = false;
			sendForm = { recipient: '', amount: '', memo: '', priority: 'normal' };
		}
		
		isLoading = false;
	}

	async function handleCreateWallet() {
		if (!createWalletForm.name) return;
		
		isLoading = true;
		const success = await walletStore.createWallet(createWalletForm);
		
		if (success) {
			showCreateWalletModal = false;
			createWalletForm = { name: '', type: 'bank_stamped', requiredSignatures: 2, totalSigners: 3 };
		}
		
		isLoading = false;
	}

	async function signMultiSigRequest(requestId: string) {
		await walletStore.signMultiSigRequest(requestId);
	}

	function formatBalance(balance: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 2,
			maximumFractionDigits: 6
		}).format(balance);
	}

	function formatDate(timestamp: number): string {
		return new Date(timestamp).toLocaleString();
	}

	function getTransactionStatusColor(status: string): string {
		switch (status) {
			case 'confirmed': return 'text-green-400';
			case 'pending': return 'text-yellow-400';
			case 'failed': return 'text-red-400';
			default: return 'text-gray-400';
		}
	}

	function getComplianceAlertColor(severity: string): string {
		switch (severity) {
			case 'critical': return 'border-red-500 bg-red-500/10';
			case 'warning': return 'border-yellow-500 bg-yellow-500/10';
			case 'info': return 'border-blue-500 bg-blue-500/10';
			default: return 'border-gray-500 bg-gray-500/10';
		}
	}
</script>

<div class="min-h-screen bg-slate-900 text-white">
	<!-- Header -->
	<div class="bg-slate-800 border-b border-slate-700 px-6 py-4">
		<div class="flex items-center justify-between">
			<div class="flex items-center space-x-4">
				<div class="w-10 h-10 bg-gradient-to-r from-purple-500 to-pink-600 rounded-xl flex items-center justify-center">
					<svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
					</svg>
				</div>
				<div>
					<h1 class="text-xl font-bold">BPI Wallet</h1>
					<p class="text-sm text-gray-400">Enterprise Digital Asset Management</p>
				</div>
			</div>
			
			<div class="flex items-center space-x-3">
				{#if isConnected}
					<div class="flex items-center space-x-2 bg-green-500/20 border border-green-500/30 rounded-lg px-3 py-1">
						<div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
						<span class="text-sm text-green-300">Connected</span>
					</div>
				{:else}
					<div class="flex items-center space-x-2 bg-red-500/20 border border-red-500/30 rounded-lg px-3 py-1">
						<div class="w-2 h-2 bg-red-400 rounded-full"></div>
						<span class="text-sm text-red-300">Disconnected</span>
					</div>
				{/if}
				
				<button
					on:click={() => showCreateWalletModal = true}
					class="bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg font-medium transition-colors"
				>
					Create Wallet
				</button>
				
				<button
					on:click={() => showSendModal = true}
					disabled={!isConnected}
					class="bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed px-4 py-2 rounded-lg font-medium transition-colors"
				>
					Send
				</button>
			</div>
		</div>
	</div>

	<!-- Navigation Tabs -->
	<div class="bg-slate-800 border-b border-slate-700">
		<div class="px-6">
			<nav class="flex space-x-8">
				{#each [
					{ id: 'overview', label: 'Overview', icon: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z' },
					{ id: 'transactions', label: 'Transactions', icon: 'M9 5H7a2 2 0 00-2 2v6a2 2 0 002 2h2m0-8h2a2 2 0 012 2v6a2 2 0 01-2 2H9m0-8V3m0 8v2' },
					{ id: 'multisig', label: 'Multi-Sig', icon: 'M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z' },
					{ id: 'compliance', label: 'Compliance', icon: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z' },
					{ id: 'settings', label: 'Settings', icon: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z' }
				] as tab}
					<button
						on:click={() => activeTab = tab.id}
						class="flex items-center space-x-2 py-4 px-1 border-b-2 font-medium text-sm transition-colors {activeTab === tab.id ? 'border-purple-500 text-purple-400' : 'border-transparent text-gray-400 hover:text-gray-300'}"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={tab.icon}></path>
						</svg>
						<span>{tab.label}</span>
					</button>
				{/each}
			</nav>
		</div>
	</div>

	<!-- Main Content -->
	<div class="p-6">
		{#if activeTab === 'overview'}
			<!-- Overview Tab -->
			<div class="space-y-6">
				<!-- Wallet Cards -->
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
					<!-- Balance Card -->
					<div class="bg-gradient-to-r from-purple-600 to-pink-600 rounded-2xl p-6 text-white">
						<div class="flex items-center justify-between mb-4">
							<h3 class="text-lg font-semibold">Total Balance</h3>
							<svg class="w-8 h-8 opacity-80" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"></path>
							</svg>
						</div>
						<div class="text-3xl font-bold mb-2">
							{wallet ? formatBalance(wallet.balance) : '$0.00'}
						</div>
						<div class="text-purple-200 text-sm">
							+2.5% from last month
						</div>
					</div>

					<!-- Pending Transactions -->
					<div class="bg-slate-800 border border-slate-700 rounded-2xl p-6">
						<div class="flex items-center justify-between mb-4">
							<h3 class="text-lg font-semibold text-white">Pending</h3>
							<svg class="w-8 h-8 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
							</svg>
						</div>
						<div class="text-3xl font-bold text-white mb-2">
							{transactions.filter(t => t.status === 'pending').length}
						</div>
						<div class="text-gray-400 text-sm">
							Awaiting confirmation
						</div>
					</div>

					<!-- Multi-Sig Requests -->
					<div class="bg-slate-800 border border-slate-700 rounded-2xl p-6">
						<div class="flex items-center justify-between mb-4">
							<h3 class="text-lg font-semibold text-white">Multi-Sig</h3>
							<svg class="w-8 h-8 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path>
							</svg>
						</div>
						<div class="text-3xl font-bold text-white mb-2">
							{multiSigRequests.filter(r => r.status === 'pending').length}
						</div>
						<div class="text-gray-400 text-sm">
							Awaiting signatures
						</div>
					</div>
				</div>

				<!-- Recent Activity -->
				<div class="bg-slate-800 border border-slate-700 rounded-2xl p-6">
					<h3 class="text-lg font-semibold text-white mb-4">Recent Activity</h3>
					<div class="space-y-4">
						{#each transactions.slice(0, 5) as transaction}
							<div class="flex items-center justify-between p-4 bg-slate-700 rounded-lg">
								<div class="flex items-center space-x-4">
									<div class="w-10 h-10 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center">
										<svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"></path>
										</svg>
									</div>
									<div>
										<div class="font-medium text-white">{transaction.type === 'send' ? 'Sent' : 'Received'}</div>
										<div class="text-sm text-gray-400">{formatDate(transaction.timestamp)}</div>
									</div>
								</div>
								<div class="text-right">
									<div class="font-medium {transaction.type === 'send' ? 'text-red-400' : 'text-green-400'}">
										{transaction.type === 'send' ? '-' : '+'}{formatBalance(transaction.amount)}
									</div>
									<div class="text-sm {getTransactionStatusColor(transaction.status)}">
										{transaction.status}
									</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			</div>

		{:else}
			<!-- Other tabs placeholder -->
			<div class="bg-slate-800 border border-slate-700 rounded-2xl p-6">
				<h3 class="text-lg font-semibold text-white mb-4">{activeTab.charAt(0).toUpperCase() + activeTab.slice(1)}</h3>
				<p class="text-gray-400">Content for {activeTab} tab coming soon...</p>
			</div>
		{/if}
	</div>
</div>
