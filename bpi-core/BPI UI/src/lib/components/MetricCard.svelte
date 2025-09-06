<script>
	export let title;
	export let value;
	export let icon;
	export let type = 'number';
	export let status = 'active';
	export let loading = false;
	export let clickable = false;
	export let onClick = undefined;
	
	$: statusClass = `metric-${status}`;
	$: typeClass = `metric-${type}`;
</script>

<div 
	class="metric-card glass-card {statusClass} {typeClass}" 
	class:clickable
	class:loading
	on:click={onClick}
	role={clickable ? 'button' : undefined}
	tabindex={clickable ? 0 : undefined}
>
	<div class="metric-header">
		<div class="metric-icon">{icon}</div>
		<h3 class="metric-title">{title}</h3>
	</div>
	
	<div class="metric-content">
		{#if loading}
			<div class="metric-loading">
				<div class="loading-spinner"></div>
				<span>Loading...</span>
			</div>
		{:else}
			<div class="metric-value" class:pulsing={type === 'status'}>
				{value}
			</div>
		{/if}
	</div>
	
	{#if type === 'status'}
		<div class="status-indicator">
			<div class="status-dot" class:active={status === 'active'}></div>
		</div>
	{/if}
</div>

<style>
	.metric-card {
		padding: 25px;
		color: white;
		position: relative;
		transition: all 0.3s ease;
		cursor: default;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}
	
	.metric-card.clickable {
		cursor: pointer;
	}
	
	.metric-card.clickable:hover {
		transform: translateY(-5px);
		box-shadow: 0 15px 30px rgba(0, 0, 0, 0.2);
		border-color: rgba(255, 255, 255, 0.4);
	}
	
	.metric-card.loading {
		opacity: 0.7;
	}
	
	.metric-header {
		display: flex;
		align-items: center;
		gap: 15px;
		margin-bottom: 20px;
	}
	
	.metric-icon {
		font-size: 2rem;
		min-width: 40px;
	}
	
	.metric-title {
		font-size: 1.2rem;
		font-weight: 600;
		margin: 0;
		color: #ffd700;
	}
	
	.metric-content {
		text-align: center;
	}
	
	.metric-value {
		font-size: 2.5rem;
		font-weight: 700;
		margin: 10px 0;
		transition: all 0.3s ease;
	}
	
	.metric-value.pulsing {
		animation: pulse 2s infinite;
	}
	
	.metric-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
		padding: 20px 0;
	}
	
	.loading-spinner {
		width: 30px;
		height: 30px;
		border: 3px solid rgba(255, 255, 255, 0.3);
		border-top: 3px solid white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	
	.status-indicator {
		position: absolute;
		top: 15px;
		right: 15px;
	}
	
	.status-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: #dc3545;
		transition: all 0.3s ease;
	}
	
	.status-dot.active {
		background: #4CAF50;
		box-shadow: 0 0 10px rgba(76, 175, 80, 0.5);
	}
	
	/* Status-based styling */
	.metric-active {
		border-color: rgba(76, 175, 80, 0.4);
		background: rgba(76, 175, 80, 0.1);
	}
	
	.metric-inactive {
		border-color: rgba(220, 53, 69, 0.4);
		background: rgba(220, 53, 69, 0.1);
	}
	
	.metric-warning {
		border-color: rgba(255, 193, 7, 0.4);
		background: rgba(255, 193, 7, 0.1);
	}
	
	.metric-error {
		border-color: rgba(220, 53, 69, 0.6);
		background: rgba(220, 53, 69, 0.2);
	}
	
	/* Type-based styling */
	.metric-performance .metric-value {
		color: #00f2fe;
	}
	
	.metric-time .metric-value {
		color: #ffd700;
	}
	
	.metric-percentage .metric-value {
		color: #4CAF50;
	}
	
	.metric-number .metric-value {
		color: white;
	}
	
	.metric-status .metric-value {
		font-size: 1.8rem;
		text-transform: uppercase;
		letter-spacing: 1px;
	}
	
	@keyframes pulse {
		0% { opacity: 1; }
		50% { opacity: 0.7; }
		100% { opacity: 1; }
	}
	
	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}
	
	@media (max-width: 768px) {
		.metric-card {
			padding: 20px;
		}
		
		.metric-value {
			font-size: 2rem;
		}
		
		.metric-icon {
			font-size: 1.5rem;
		}
	}
</style>
