<script lang="ts">
	import { onMount } from 'svelte';
	
	let currentStep = 0;
	let animateSteps = false;
	
	const steps = [
		{
			title: "System Check",
			description: "Verify system requirements and dependencies",
			icon: "🔍",
			status: "completed"
		},
		{
			title: "Download BPI Core",
			description: "Download the latest BPI blockchain infrastructure",
			icon: "⬇️",
			status: "current"
		},
		{
			title: "Configure Network",
			description: "Set up network connections and peer discovery",
			icon: "🌐",
			status: "pending"
		},
		{
			title: "Initialize Wallet",
			description: "Create and secure your BPI wallet",
			icon: "💼",
			status: "pending"
		},
		{
			title: "Start Node",
			description: "Launch your BPI node and join the network",
			icon: "🚀",
			status: "pending"
		}
	];
	
	onMount(() => {
		setTimeout(() => {
			animateSteps = true;
		}, 500);
	});
</script>

<div class="installation-steps glass-card">
	<div class="steps-header">
		<h3>Installation Process</h3>
		<p>Follow these steps to set up your BPI node</p>
	</div>
	
	<div class="steps-container" class:animate={animateSteps}>
		{#each steps as step, index}
			<div 
				class="step-item" 
				class:completed={step.status === 'completed'}
				class:current={step.status === 'current'}
				class:pending={step.status === 'pending'}
				style="animation-delay: {index * 0.1}s"
			>
				<div class="step-icon">
					{step.icon}
				</div>
				<div class="step-content">
					<h4 class="step-title">{step.title}</h4>
					<p class="step-description">{step.description}</p>
				</div>
				<div class="step-status">
					{#if step.status === 'completed'}
						<div class="status-check">✓</div>
					{:else if step.status === 'current'}
						<div class="status-loading">
							<div class="loading-spinner"></div>
						</div>
					{:else}
						<div class="status-pending">○</div>
					{/if}
				</div>
			</div>
		{/each}
	</div>
</div>

<style>
	.installation-steps {
		padding: 30px;
		margin: 20px 0;
	}
	
	.steps-header {
		text-align: center;
		margin-bottom: 30px;
	}
	
	.steps-header h3 {
		font-size: 1.8rem;
		margin-bottom: 10px;
		color: white;
	}
	
	.steps-header p {
		opacity: 0.8;
		color: white;
	}
	
	.steps-container {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}
	
	.step-item {
		display: flex;
		align-items: center;
		gap: 20px;
		padding: 20px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		border: 1px solid rgba(255, 255, 255, 0.2);
		transition: all 0.3s ease;
		opacity: 0;
		transform: translateX(-20px);
	}
	
	.animate .step-item {
		opacity: 1;
		transform: translateX(0);
		animation: slideInLeft 0.6s ease-out forwards;
	}
	
	.step-item.current {
		background: rgba(102, 126, 234, 0.2);
		border-color: rgba(102, 126, 234, 0.4);
		box-shadow: 0 0 20px rgba(102, 126, 234, 0.3);
	}
	
	.step-item.completed {
		background: rgba(76, 175, 80, 0.2);
		border-color: rgba(76, 175, 80, 0.4);
	}
	
	.step-icon {
		font-size: 2rem;
		min-width: 60px;
		text-align: center;
	}
	
	.step-content {
		flex: 1;
		color: white;
	}
	
	.step-title {
		font-size: 1.2rem;
		font-weight: 600;
		margin-bottom: 5px;
	}
	
	.step-description {
		opacity: 0.8;
		margin: 0;
	}
	
	.step-status {
		min-width: 40px;
		text-align: center;
	}
	
	.status-check {
		width: 30px;
		height: 30px;
		background: #4CAF50;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		font-weight: bold;
	}
	
	.status-loading {
		width: 30px;
		height: 30px;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	
	.loading-spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top: 2px solid white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	
	.status-pending {
		width: 30px;
		height: 30px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: rgba(255, 255, 255, 0.5);
		font-size: 1.5rem;
	}
	
	@keyframes slideInLeft {
		from {
			opacity: 0;
			transform: translateX(-20px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}
	
	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}
	
	@media (max-width: 768px) {
		.step-item {
			flex-direction: column;
			text-align: center;
			gap: 15px;
		}
		
		.step-content {
			text-align: center;
		}
	}
</style>
