<script>
	export let steps;
	export let currentStep;
	export let isInstalling;
	
	$: progress = isInstalling ? ((currentStep + 1) / steps.length) * 100 : 0;
</script>

<div class="installation-progress glass-card">
	<div class="progress-header">
		<h3>Installation Progress</h3>
		<div class="progress-percentage">{Math.round(progress)}%</div>
	</div>
	
	<div class="progress-bar-container">
		<div class="progress-bar">
			<div 
				class="progress-fill" 
				style="width: {progress}%"
			></div>
		</div>
	</div>
	
	<div class="steps-list">
		{#each steps as step, index}
			<div 
				class="step-item"
				class:completed={index < currentStep}
				class:current={index === currentStep}
				class:pending={index > currentStep}
			>
				<div class="step-indicator">
					{#if index < currentStep}
						<div class="step-check">✓</div>
					{:else if index === currentStep && isInstalling}
						<div class="step-spinner">
							<div class="spinner"></div>
						</div>
					{:else}
						<div class="step-number">{index + 1}</div>
					{/if}
				</div>
				<div class="step-name">{step.name}</div>
				{#if index === currentStep && isInstalling}
					<div class="step-status">Installing...</div>
				{:else if index < currentStep}
					<div class="step-status">Complete</div>
				{:else}
					<div class="step-status">Pending</div>
				{/if}
			</div>
		{/each}
	</div>
	
	<div class="installation-details">
		{#if isInstalling}
			<p class="current-action">
				{#if currentStep < steps.length}
					Currently: {steps[currentStep].name}
				{:else}
					Finalizing installation...
				{/if}
			</p>
		{/if}
	</div>
</div>

<style>
	.installation-progress {
		padding: 30px;
		color: white;
	}
	
	.progress-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}
	
	.progress-header h3 {
		font-size: 1.5rem;
		margin: 0;
	}
	
	.progress-percentage {
		font-size: 1.2rem;
		font-weight: 600;
		color: #4CAF50;
	}
	
	.progress-bar-container {
		margin-bottom: 30px;
	}
	
	.progress-bar {
		width: 100%;
		height: 8px;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 4px;
		overflow: hidden;
	}
	
	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #4CAF50, #81C784);
		border-radius: 4px;
		transition: width 0.5s ease;
		position: relative;
	}
	
	.progress-fill::after {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
		animation: shimmer 2s infinite;
	}
	
	.steps-list {
		display: flex;
		flex-direction: column;
		gap: 15px;
		margin-bottom: 20px;
	}
	
	.step-item {
		display: flex;
		align-items: center;
		gap: 15px;
		padding: 15px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 8px;
		transition: all 0.3s ease;
	}
	
	.step-item.current {
		background: rgba(102, 126, 234, 0.2);
		border: 1px solid rgba(102, 126, 234, 0.4);
	}
	
	.step-item.completed {
		background: rgba(76, 175, 80, 0.2);
		border: 1px solid rgba(76, 175, 80, 0.4);
	}
	
	.step-indicator {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		font-weight: 600;
	}
	
	.step-check {
		background: #4CAF50;
		color: white;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
	}
	
	.step-spinner {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	
	.spinner {
		width: 20px;
		height: 20px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top: 2px solid white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	
	.step-number {
		background: rgba(255, 255, 255, 0.2);
		color: white;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
	}
	
	.step-name {
		flex: 1;
		font-weight: 500;
	}
	
	.step-status {
		font-size: 0.9rem;
		opacity: 0.8;
	}
	
	.installation-details {
		text-align: center;
		padding-top: 20px;
		border-top: 1px solid rgba(255, 255, 255, 0.2);
	}
	
	.current-action {
		font-style: italic;
		opacity: 0.9;
		margin: 0;
	}
	
	@keyframes shimmer {
		0% { transform: translateX(-100%); }
		100% { transform: translateX(100%); }
	}
	
	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}
	
	@media (max-width: 768px) {
		.step-item {
			flex-direction: column;
			text-align: center;
			gap: 10px;
		}
	}
</style>
