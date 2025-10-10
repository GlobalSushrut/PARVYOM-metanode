<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Logo from '$lib/components/Logo.svelte';
	import InstallationProgress from '$lib/components/InstallationProgress.svelte';
	import SystemRequirements from '$lib/components/SystemRequirements.svelte';
	
	let currentStep = 0;
	let isInstalling = false;
	let installationComplete = false;
	
	const installationSteps = [
		{ name: 'System Check', duration: 2000 },
		{ name: 'Download BPI Core', duration: 5000 },
		{ name: 'Configure Network', duration: 3000 },
		{ name: 'Initialize Wallet', duration: 2000 },
		{ name: 'Start Node', duration: 3000 }
	];
	
	async function startInstallation() {
		isInstalling = true;
		
		for (let i = 0; i < installationSteps.length; i++) {
			currentStep = i;
			await new Promise(resolve => setTimeout(resolve, installationSteps[i].duration));
		}
		
		installationComplete = true;
		setTimeout(() => {
			goto('/dashboard');
		}, 2000);
	}
	
	function goBack() {
		goto('/');
	}
</script>

<svelte:head>
	<title>BPI Installer - Installation Process</title>
</svelte:head>

<div class="installer-container">
	<div class="container">
		<div class="installer-header">
			<Logo size="medium" />
			<h1>BPI Core Installation</h1>
			<p>Setting up your Blockchain Protocol Infrastructure</p>
		</div>
		
		{#if !isInstalling && !installationComplete}
			<div class="pre-installation fade-in">
				<SystemRequirements />
				
				<div class="installation-actions">
					<button class="btn btn-secondary" on:click={goBack}>
						← Back to Welcome
					</button>
					<button class="btn btn-primary" on:click={startInstallation}>
						🚀 Start Installation
					</button>
				</div>
			</div>
		{:else if isInstalling}
			<div class="installation-active slide-up">
				<InstallationProgress 
					steps={installationSteps}
					{currentStep}
					{isInstalling}
				/>
			</div>
		{:else if installationComplete}
			<div class="installation-complete fade-in">
				<div class="success-message glass-card">
					<div class="success-icon">✅</div>
					<h2>Installation Complete!</h2>
					<p>Your BPI node has been successfully installed and configured.</p>
					<p>Redirecting to dashboard...</p>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.installer-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 20px;
	}
	
	.installer-header {
		text-align: center;
		margin-bottom: 40px;
		color: white;
	}
	
	.installer-header h1 {
		font-size: 2.5rem;
		font-weight: 700;
		margin: 20px 0 10px 0;
		text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
	}
	
	.installer-header p {
		font-size: 1.1rem;
		opacity: 0.9;
	}
	
	.pre-installation {
		max-width: 800px;
		margin: 0 auto;
	}
	
	.installation-actions {
		display: flex;
		gap: 20px;
		justify-content: center;
		margin-top: 30px;
	}
	
	.installation-active {
		max-width: 900px;
		margin: 0 auto;
	}
	
	.installation-complete {
		max-width: 600px;
		margin: 0 auto;
	}
	
	.success-message {
		padding: 40px;
		text-align: center;
		color: white;
	}
	
	.success-icon {
		font-size: 4rem;
		margin-bottom: 20px;
	}
	
	.success-message h2 {
		font-size: 2rem;
		margin-bottom: 15px;
	}
	
	.success-message p {
		opacity: 0.9;
		margin-bottom: 10px;
	}
	
	@media (max-width: 768px) {
		.installer-header h1 {
			font-size: 2rem;
		}
		
		.installation-actions {
			flex-direction: column;
			align-items: center;
		}
		
		.installation-actions .btn {
			width: 200px;
		}
	}
</style>
