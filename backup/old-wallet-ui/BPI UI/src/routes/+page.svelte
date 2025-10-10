<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Logo from '$lib/components/Logo.svelte';
	import WelcomeCard from '$lib/components/WelcomeCard.svelte';
	
	let showContent = false;
	
	onMount(() => {
		setTimeout(() => {
			showContent = true;
		}, 300);
	});
	
	function startInstallation() {
		goto('/installer');
	}
	
	function viewDashboard() {
		goto('/dashboard');
	}
	
	function viewWallet() {
		goto('/wallet');
	}
</script>

<svelte:head>
	<title>Welcome to Parvyom Metanode - BPI Installer</title>
</svelte:head>

<div class="welcome-container">
	<div class="container">
		{#if showContent}
			<div class="welcome-header slide-up">
				<Logo size="large" />
				<h1 class="welcome-title">Welcome to Parvyom Metanode</h1>
				<p class="welcome-subtitle">This is just for BPI installer</p>
				<div class="status-indicator">
					<div class="status-dot"></div>
					<span>System Ready</span>
				</div>
			</div>
			
			<div class="welcome-content fade-in">
				<WelcomeCard 
					title="BPI Core Installation"
					description="Install and configure your BPI (Blockchain Protocol Infrastructure) node"
					icon="🚀"
					action={startInstallation}
					actionText="Start Installation"
				/>
				
				<WelcomeCard 
					title="System Dashboard"
					description="Monitor your BPI node performance and network status"
					icon="📊"
					action={viewDashboard}
					actionText="View Dashboard"
				/>
				
				<WelcomeCard 
					title="BPI Wallet"
					description="Manage your digital assets and blockchain transactions"
					icon="💼"
					action={viewWallet}
					actionText="Open Wallet"
				/>
			</div>
			
			<div class="installation-preview fade-in">
				<div class="installation-info">
					<h3>Quick Start Guide</h3>
					<p>Use the cards above to get started with your PARVYOM Metanode BPI system.</p>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.welcome-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 20px;
	}
	
	.welcome-header {
		text-align: center;
		margin-bottom: 50px;
		color: white;
	}
	
	.welcome-title {
		font-size: 3rem;
		font-weight: 700;
		margin: 20px 0 10px 0;
		text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
	}
	
	.welcome-subtitle {
		font-size: 1.2rem;
		opacity: 0.9;
		margin-bottom: 30px;
	}
	
	.status-indicator {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 10px;
		font-size: 1rem;
		opacity: 0.8;
	}
	
	.welcome-content {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: 30px;
		margin-bottom: 50px;
	}
	
	.installation-preview {
		max-width: 800px;
		margin: 0 auto;
	}
	
	@media (max-width: 768px) {
		.welcome-title {
			font-size: 2rem;
		}
		
		.welcome-content {
			grid-template-columns: 1fr;
			gap: 20px;
		}
	}
</style>
