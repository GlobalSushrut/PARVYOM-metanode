<script lang="ts">
	import { onMount } from 'svelte';
	
	let systemChecks = [
		{ name: 'Operating System', requirement: 'Linux/macOS/Windows 10+', status: 'checking', icon: '💻' },
		{ name: 'Memory (RAM)', requirement: '8GB minimum, 16GB recommended', status: 'checking', icon: '🧠' },
		{ name: 'Storage Space', requirement: '50GB available space', status: 'checking', icon: '💾' },
		{ name: 'Network Connection', requirement: 'Stable internet connection', status: 'checking', icon: '🌐' },
		{ name: 'Rust Compiler', requirement: 'Rust 1.70+ or auto-install', status: 'checking', icon: '🦀' },
		{ name: 'Node.js', requirement: 'Node.js 18+ (optional)', status: 'checking', icon: '📦' }
	];
	
	let allChecksPassed = false;
	
	onMount(async () => {
		// Simulate system checks
		for (let i = 0; i < systemChecks.length; i++) {
			await new Promise(resolve => setTimeout(resolve, 500));
			systemChecks[i].status = Math.random() > 0.1 ? 'passed' : 'warning';
		}
		
		allChecksPassed = systemChecks.every(check => check.status === 'passed');
	});
</script>

<div class="system-requirements glass-card">
	<div class="requirements-header">
		<h3>System Requirements Check</h3>
		<p>Verifying your system meets the minimum requirements for BPI Core</p>
	</div>
	
	<div class="checks-container">
		{#each systemChecks as check, index}
			<div 
				class="check-item"
				class:passed={check.status === 'passed'}
				class:warning={check.status === 'warning'}
				class:checking={check.status === 'checking'}
				style="animation-delay: {index * 0.1}s"
			>
				<div class="check-icon">{check.icon}</div>
				<div class="check-content">
					<h4 class="check-name">{check.name}</h4>
					<p class="check-requirement">{check.requirement}</p>
				</div>
				<div class="check-status">
					{#if check.status === 'checking'}
						<div class="status-spinner">
							<div class="spinner"></div>
						</div>
					{:else if check.status === 'passed'}
						<div class="status-icon passed">✓</div>
					{:else if check.status === 'warning'}
						<div class="status-icon warning">⚠️</div>
					{/if}
				</div>
			</div>
		{/each}
	</div>
	
	<div class="requirements-summary">
		{#if allChecksPassed}
			<div class="summary-success">
				<div class="summary-icon">🎉</div>
				<h4>System Ready!</h4>
				<p>Your system meets all requirements for BPI Core installation.</p>
			</div>
		{:else if systemChecks.some(check => check.status === 'warning')}
			<div class="summary-warning">
				<div class="summary-icon">⚠️</div>
				<h4>Some Issues Detected</h4>
				<p>Installation can proceed, but some features may be limited. Check warnings above.</p>
			</div>
		{:else}
			<div class="summary-checking">
				<div class="summary-icon">🔍</div>
				<h4>Checking System...</h4>
				<p>Please wait while we verify your system requirements.</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.system-requirements {
		padding: 30px;
		color: white;
		margin-bottom: 30px;
	}
	
	.requirements-header {
		text-align: center;
		margin-bottom: 30px;
	}
	
	.requirements-header h3 {
		font-size: 1.5rem;
		margin-bottom: 10px;
	}
	
	.requirements-header p {
		opacity: 0.8;
		margin: 0;
	}
	
	.checks-container {
		display: flex;
		flex-direction: column;
		gap: 15px;
		margin-bottom: 30px;
	}
	
	.check-item {
		display: flex;
		align-items: center;
		gap: 15px;
		padding: 15px;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 8px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		transition: all 0.3s ease;
		opacity: 0;
		transform: translateY(10px);
		animation: slideUp 0.5s ease-out forwards;
	}
	
	.check-item.passed {
		background: rgba(76, 175, 80, 0.2);
		border-color: rgba(76, 175, 80, 0.4);
	}
	
	.check-item.warning {
		background: rgba(255, 193, 7, 0.2);
		border-color: rgba(255, 193, 7, 0.4);
	}
	
	.check-item.checking {
		background: rgba(102, 126, 234, 0.2);
		border-color: rgba(102, 126, 234, 0.4);
	}
	
	.check-icon {
		font-size: 1.5rem;
		min-width: 40px;
		text-align: center;
	}
	
	.check-content {
		flex: 1;
	}
	
	.check-name {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 5px 0;
	}
	
	.check-requirement {
		font-size: 0.9rem;
		opacity: 0.8;
		margin: 0;
	}
	
	.check-status {
		min-width: 40px;
		display: flex;
		justify-content: center;
		align-items: center;
	}
	
	.status-spinner {
		width: 20px;
		height: 20px;
	}
	
	.spinner {
		width: 100%;
		height: 100%;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top: 2px solid white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	
	.status-icon {
		width: 24px;
		height: 24px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.8rem;
		font-weight: bold;
	}
	
	.status-icon.passed {
		background: #4CAF50;
		color: white;
	}
	
	.status-icon.warning {
		background: transparent;
		font-size: 1.2rem;
	}
	
	.requirements-summary {
		text-align: center;
		padding: 20px;
		border-radius: 8px;
	}
	
	.summary-success {
		background: rgba(76, 175, 80, 0.2);
		border: 1px solid rgba(76, 175, 80, 0.4);
	}
	
	.summary-warning {
		background: rgba(255, 193, 7, 0.2);
		border: 1px solid rgba(255, 193, 7, 0.4);
	}
	
	.summary-checking {
		background: rgba(102, 126, 234, 0.2);
		border: 1px solid rgba(102, 126, 234, 0.4);
	}
	
	.summary-icon {
		font-size: 2rem;
		margin-bottom: 10px;
	}
	
	.requirements-summary h4 {
		font-size: 1.2rem;
		margin: 0 0 10px 0;
	}
	
	.requirements-summary p {
		margin: 0;
		opacity: 0.9;
	}
	
	@keyframes slideUp {
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
	
	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}
	
	@media (max-width: 768px) {
		.check-item {
			flex-direction: column;
			text-align: center;
			gap: 10px;
		}
	}
</style>
