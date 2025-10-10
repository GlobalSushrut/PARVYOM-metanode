<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Logo from '$lib/components/Logo.svelte';
	import { 
		walletData, 
		walletLoading, 
		walletError, 
		bpiVmStatus, 
		totalBalance, 
		totalBalanceUSD, 
		networkStatus,
		bpiActions 
	} from '$lib/stores/bpi';
	
	let showSendModal = false;
	let showReceiveModal = false;
	let sendAddress = '';
	let sendAmount = '';
	let gasFee = 0.01;
	let notification = { show: false, message: '', type: 'success' };
	
	$: wallet = $walletData;
	$: vmStatus = $bpiVmStatus;
	$: loading = $walletLoading;
	$: error = $walletError;
	$: balance = $totalBalance;
	$: balanceUSD = $totalBalanceUSD;
	$: network = $networkStatus;
	
	onMount(() => {
		// Start real-time updates when component mounts
		return bpiActions.startRealTimeUpdates();
	});
	
	function openSendModal() {
		if (!isLedgerOperationAllowed('send')) {
			showNotification('BPI ledger operations require proper registration', 'error');
			return;
		}
		showSendModal = true;
	}
	
	function openReceiveModal() {
		showReceiveModal = true;
	}
	
	function closeSendModal() {
		showSendModal = false;
		sendAddress = '';
		sendAmount = '';
	}
	
	function closeReceiveModal() {
		showReceiveModal = false;
	}
	
	async function sendTransaction() {
		if (!wallet || !sendAddress || !sendAmount) {
			showNotification('Please fill in all fields', 'error');
			return;
		}
		
		const amount = parseFloat(sendAmount);
		if (amount <= 0 || amount > wallet.bpiBalance) {
			showNotification('Invalid amount', 'error');
			return;
		}
		
		try {
			await bpiActions.sendTransaction(sendAddress, amount, gasFee);
			showNotification('Transaction sent successfully!', 'success');
			closeSendModal();
		} catch (error) {
			showNotification(`Transaction failed: ${error}`, 'error');
		}
	}
	
	function copyReceiveAddress() {
		if (wallet?.address) {
			navigator.clipboard.writeText(wallet.address);
			showNotification('Address copied to clipboard!', 'success');
		}
	}
	
	function isLedgerOperationAllowed(operation) {
		if (!wallet) return false;
		
		const hasRegistry = wallet.registryAddress && wallet.registryToken;
		const isRegistered = wallet.bpciRegistered;
		const ledgerActive = wallet.ledgerActivated;
		
		return hasRegistry && isRegistered && ledgerActive;
	}
	
	function showNotification(message, type = 'success') {
		notification = { show: true, message, type };
		setTimeout(() => {
			notification.show = false;
		}, 5000);
	}
	
	function goToDashboard() {
		goto('/dashboard');
	}
	
	function goHome() {
		goto('/');
	}
	
	function connectToBPCI() {
		// This would trigger BPCI connection process
		showNotification('BPCI connection not implemented yet', 'warning');
	}
	
	function buyTestnetTokens() {
		if (wallet && wallet.networkType === 'testnet') {
			// Simulate testnet token purchase
			showNotification('Testnet tokens have no real value - for testing only', 'warning');
		}
	}
</script>

<svelte:head>
	<title>BPI Wallet - Digital Asset Management</title>
</svelte:head>

<div class="wallet-container">
	<div class="container">
		<div class="wallet-wrapper card">
			<!-- Header -->
			<div class="wallet-header">
				<Logo size="small" showText={false} />
				<div class="header-content">
					<h1 class="wallet-title">BPI Wallet</h1>
					<p class="wallet-subtitle">Digital Asset Management</p>
				</div>
				<div class="network-indicator">
					<div class="status-dot" class:active={network !== 'Disconnected'}></div>
					<span>{network}</span>
				</div>
			</div>
			
			<!-- Wallet Content -->
			<div class="wallet-content">
				{#if loading}
					<div class="loading-section">
						<div class="loading-spinner"></div>
						<p>Connecting to BPI network...</p>
					</div>
				{:else if error}
					<div class="error-section">
						<div class="error-icon">⚠️</div>
						<h3>Connection Error</h3>
						<p>{error}</p>
						<button class="btn btn-primary" on:click={() => bpiActions.fetchVmStatus()}>
							🔄 Retry Connection
						</button>
					</div>
				{:else if wallet}
					<!-- Account Section -->
					<div class="account-section">
						<div class="account-address" on:click={copyReceiveAddress}>
							{wallet.address}
						</div>
						
						<!-- Balance Display -->
						<div class="balance-display">
							<div class="main-balance">{balance.toFixed(2)} BPI</div>
							<div class="balance-usd">${balanceUSD.toFixed(2)} USD</div>
						</div>
						
						<!-- Balance Grid -->
						<div class="balance-grid">
							<div class="balance-card">
								<div class="balance-label">BPCI Balance</div>
								<div class="balance-amount">{(wallet.balance?.bpci || 0).toFixed(2)}</div>
							</div>
							<div class="balance-card">
								<div class="balance-label">ETH Balance</div>
								<div class="balance-amount">{(wallet.balance?.eth || 0).toFixed(2)}</div>
							</div>
							<div class="balance-card">
								<div class="balance-label">BTC Balance</div>
								<div class="balance-amount">{(wallet.balance?.btc || 0).toFixed(2)}</div>
							</div>
							<div class="balance-card">
								<div class="balance-label">Wallet Type</div>
								<div class="balance-amount" style="font-size: 0.8em;">{wallet.walletType}</div>
							</div>
						</div>
					</div>
					
					<!-- Action Buttons -->
					<div class="action-buttons">
						<button 
							class="action-btn" 
							class:disabled={!isLedgerOperationAllowed('send')}
							on:click={openSendModal}
						>
							📤 Send
						</button>
						<button class="action-btn" on:click={openReceiveModal}>
							📥 Receive
						</button>
						{#if wallet.networkType === 'testnet'}
							<button class="action-btn secondary" on:click={buyTestnetTokens}>
								🪙 Get Testnet
							</button>
						{:else if !wallet.bpciConnected}
							<button class="action-btn success" on:click={connectToBPCI}>
								🔗 Connect BPCI
							</button>
						{/if}
					</div>
					
					<!-- Status Section -->
					<div class="status-section">
						<div class="section-title">🔒 Connection Status</div>
						<div class="status-grid">
							<div class="status-item">
								<span class="status-label">BPCI Server:</span>
								<span class="status-value" class:connected={wallet.bpciConnected}>
									{wallet.bpciConnected ? 'Connected' : 'Disconnected'}
								</span>
							</div>
							<div class="status-item">
								<span class="status-label">Registry:</span>
								<span class="status-value" class:connected={wallet.bpciRegistered}>
									{wallet.bpciRegistered ? 'Registered' : 'Not Registered'}
								</span>
							</div>
							<div class="status-item">
								<span class="status-label">Ledger:</span>
								<span class="status-value" class:connected={wallet.ledgerActivated}>
									{wallet.ledgerActivated ? 'Active' : 'Inactive'}
								</span>
							</div>
						</div>
					</div>
					
					<!-- Transaction Section -->
					<div class="transaction-section">
						<div class="section-title">📊 Recent Transactions</div>
						<div class="transaction-list">
							{#if wallet.transactions.length > 0}
								{#each wallet.transactions as tx}
									<div class="transaction-item">
										<div class="transaction-info">
											<div class="transaction-type">{tx.type}</div>
											<div class="transaction-time">{tx.time}</div>
										</div>
										<div class="transaction-amount" class:positive={tx.amount > 0} class:negative={tx.amount < 0}>
											{tx.amount > 0 ? '+' : ''}{tx.amount.toFixed(2)} BPI
										</div>
									</div>
								{/each}
							{:else}
								<div class="no-transactions">
									<p>No transactions yet</p>
									<p class="hint">Send or receive BPI to see transaction history</p>
								</div>
							{/if}
						</div>
					</div>
				{/if}
				
				<!-- Navigation -->
				<div class="wallet-navigation">
					<button class="nav-btn" on:click={goHome}>
						🏠 Home
					</button>
					<button class="nav-btn" on:click={goToDashboard}>
						📊 Dashboard
					</button>
				</div>
			</div>
		</div>
	</div>
</div>

<!-- Send Modal -->
{#if showSendModal}
	<div class="modal" on:click={closeSendModal}>
		<div class="modal-content" on:click|stopPropagation>
			<div class="modal-title">📤 Send BPI Tokens</div>
			<div class="form-group">
				<label class="form-label">To Address</label>
				<input 
					type="text" 
					class="form-input" 
					bind:value={sendAddress}
					placeholder="bpi_recipient_address"
				/>
			</div>
			<div class="form-group">
				<label class="form-label">Amount</label>
				<input 
					type="number" 
					class="form-input" 
					bind:value={sendAmount}
					placeholder="0.00"
					max={wallet?.bpiBalance || 0}
				/>
			</div>
			<div class="form-group">
				<label class="form-label">Gas Fee</label>
				<input 
					type="number" 
					class="form-input" 
					bind:value={gasFee}
					readonly
				/>
			</div>
			<div class="modal-buttons">
				<button class="modal-btn secondary" on:click={closeSendModal}>Cancel</button>
				<button class="modal-btn primary" on:click={sendTransaction}>Send</button>
			</div>
		</div>
	</div>
{/if}

<!-- Receive Modal -->
{#if showReceiveModal}
	<div class="modal" on:click={closeReceiveModal}>
		<div class="modal-content" on:click|stopPropagation>
			<div class="modal-title">📥 Receive BPI Tokens</div>
			<p style="text-align: center; margin-bottom: 20px; color: #666;">
				Share your wallet address to receive BPI tokens
			</p>
			<div class="form-group">
				<label class="form-label">Your Wallet Address</label>
				<input 
					type="text" 
					class="form-input" 
					value={wallet?.address || ''}
					readonly
				/>
			</div>
			<div class="modal-buttons">
				<button class="modal-btn secondary" on:click={closeReceiveModal}>Close</button>
				<button class="modal-btn primary" on:click={copyReceiveAddress}>Copy Address</button>
			</div>
		</div>
	</div>
{/if}

<!-- Notification -->
{#if notification.show}
	<div class="notification {notification.type}">
		{notification.message}
	</div>
{/if}

<style>
	.wallet-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 20px;
	}
	
	.wallet-wrapper {
		width: 400px;
		max-width: 100%;
	}
	
	.wallet-header {
		background: var(--primary-gradient);
		color: white;
		padding: 20px;
		text-align: center;
		position: relative;
		display: flex;
		align-items: center;
		gap: 15px;
	}
	
	.header-content {
		flex: 1;
	}
	
	.wallet-title {
		font-size: 1.5em;
		font-weight: 600;
		margin: 0 0 5px 0;
	}
	
	.wallet-subtitle {
		opacity: 0.9;
		font-size: 0.9em;
		margin: 0;
	}
	
	.network-indicator {
		position: absolute;
		top: 15px;
		right: 15px;
		background: rgba(255, 255, 255, 0.2);
		padding: 5px 10px;
		border-radius: 15px;
		font-size: 0.8em;
		display: flex;
		align-items: center;
		gap: 5px;
	}
	
	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #dc3545;
	}
	
	.status-dot.active {
		background: #4CAF50;
		animation: pulse 2s infinite;
	}
	
	.wallet-content {
		padding: 25px;
	}
	
	.loading-section,
	.error-section {
		text-align: center;
		padding: 40px 20px;
	}
	
	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #f3f3f3;
		border-top: 3px solid var(--accent-color);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 20px;
	}
	
	.error-icon {
		font-size: 3rem;
		margin-bottom: 15px;
	}
	
	.account-section {
		text-align: center;
		margin-bottom: 25px;
	}
	
	.account-address {
		background: var(--bg-light);
		padding: 12px;
		border-radius: 10px;
		font-family: 'Monaco', monospace;
		font-size: 0.9em;
		color: var(--text-secondary);
		margin-bottom: 15px;
		cursor: pointer;
		transition: background 0.3s;
	}
	
	.account-address:hover {
		background: #e9ecef;
	}
	
	.balance-display {
		margin-bottom: 20px;
	}
	
	.main-balance {
		font-size: 2.5em;
		font-weight: 700;
		color: var(--text-primary);
		margin-bottom: 5px;
	}
	
	.balance-usd {
		color: var(--text-secondary);
		font-size: 1.1em;
	}
	
	.balance-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 15px;
		margin-bottom: 25px;
	}
	
	.balance-card {
		background: var(--bg-light);
		padding: 15px;
		border-radius: 12px;
		text-align: center;
	}
	
	.balance-label {
		font-size: 0.8em;
		color: var(--text-secondary);
		margin-bottom: 5px;
		text-transform: uppercase;
		font-weight: 600;
	}
	
	.balance-amount {
		font-size: 1.2em;
		font-weight: 600;
		color: var(--text-primary);
	}
	
	.action-buttons {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: 10px;
		margin-bottom: 25px;
	}
	
	.action-btn {
		background: var(--primary-gradient);
		color: white;
		border: none;
		padding: 12px 8px;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: var(--transition);
		font-size: 0.9em;
	}
	
	.action-btn:hover:not(.disabled) {
		transform: translateY(-2px);
		box-shadow: 0 5px 15px rgba(102, 126, 234, 0.4);
	}
	
	.action-btn.secondary {
		background: #6c757d;
	}
	
	.action-btn.success {
		background: #28a745;
	}
	
	.action-btn.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
	
	.status-section,
	.transaction-section {
		border-top: 1px solid #e9ecef;
		padding-top: 20px;
		margin-bottom: 20px;
	}
	
	.section-title {
		font-size: 1.1em;
		font-weight: 600;
		margin-bottom: 15px;
		color: var(--text-primary);
	}
	
	.status-grid {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
	
	.status-item {
		display: flex;
		justify-content: space-between;
		padding: 8px 0;
	}
	
	.status-label {
		color: var(--text-secondary);
	}
	
	.status-value {
		font-weight: 600;
		color: #dc3545;
	}
	
	.status-value.connected {
		color: #28a745;
	}
	
	.transaction-list {
		max-height: 200px;
		overflow-y: auto;
	}
	
	.transaction-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 0;
		border-bottom: 1px solid #f1f3f4;
	}
	
	.transaction-item:last-child {
		border-bottom: none;
	}
	
	.transaction-type {
		font-weight: 600;
		font-size: 0.9em;
		margin-bottom: 2px;
	}
	
	.transaction-time {
		font-size: 0.8em;
		color: var(--text-secondary);
	}
	
	.transaction-amount {
		font-weight: 600;
		font-size: 0.9em;
	}
	
	.transaction-amount.positive {
		color: #28a745;
	}
	
	.transaction-amount.negative {
		color: #dc3545;
	}
	
	.no-transactions {
		text-align: center;
		padding: 30px 20px;
		color: var(--text-secondary);
	}
	
	.hint {
		font-size: 0.9em;
		opacity: 0.8;
	}
	
	.wallet-navigation {
		display: flex;
		gap: 10px;
		padding-top: 20px;
		border-top: 1px solid #e9ecef;
	}
	
	.nav-btn {
		flex: 1;
		padding: 12px;
		background: var(--bg-light);
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: var(--transition);
		font-weight: 500;
	}
	
	.nav-btn:hover {
		background: #e9ecef;
	}
	
	/* Modal Styles */
	.modal {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}
	
	.modal-content {
		background: white;
		padding: 30px;
		border-radius: 15px;
		width: 90%;
		max-width: 400px;
		box-shadow: var(--shadow-lg);
	}
	
	.modal-title {
		font-size: 1.3em;
		font-weight: 600;
		margin-bottom: 20px;
		text-align: center;
		color: var(--text-primary);
	}
	
	.form-group {
		margin-bottom: 15px;
	}
	
	.form-label {
		display: block;
		margin-bottom: 5px;
		font-weight: 600;
		color: var(--text-primary);
	}
	
	.form-input {
		width: 100%;
		padding: 12px;
		border: 2px solid #e9ecef;
		border-radius: 8px;
		font-size: 1em;
		transition: border-color 0.3s;
	}
	
	.form-input:focus {
		outline: none;
		border-color: var(--accent-color);
	}
	
	.modal-buttons {
		display: flex;
		gap: 10px;
		margin-top: 20px;
	}
	
	.modal-btn {
		flex: 1;
		padding: 12px;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.3s;
	}
	
	.modal-btn.primary {
		background: var(--accent-color);
		color: white;
	}
	
	.modal-btn.secondary {
		background: #e9ecef;
		color: var(--text-primary);
	}
	
	.notification {
		position: fixed;
		top: 20px;
		right: 20px;
		background: #28a745;
		color: white;
		padding: 15px 20px;
		border-radius: 10px;
		box-shadow: var(--shadow);
		z-index: 1001;
		animation: slideIn 0.3s ease-out;
	}
	
	.notification.error {
		background: #dc3545;
	}
	
	.notification.warning {
		background: #ffc107;
		color: #333;
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
	
	@keyframes slideIn {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}
	
	@media (max-width: 768px) {
		.wallet-wrapper {
			width: 100%;
		}
		
		.balance-grid {
			grid-template-columns: 1fr;
		}
		
		.action-buttons {
			grid-template-columns: 1fr;
		}
	}
</style>
