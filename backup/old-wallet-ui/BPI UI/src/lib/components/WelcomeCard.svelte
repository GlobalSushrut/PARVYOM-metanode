<script>
	export let title;
	export let description;
	export let icon;
	export let action;
	export let actionText;
	export let disabled = false;
</script>

<div class="welcome-card card" class:disabled>
	<div class="card-header">
		<div class="card-icon">{icon}</div>
		<h3 class="card-title">{title}</h3>
	</div>
	
	<div class="card-content">
		<p class="card-description">{description}</p>
	</div>
	
	<div class="card-footer">
		<button 
			class="btn btn-primary" 
			on:click={action}
			{disabled}
		>
			{actionText}
		</button>
	</div>
</div>

<style>
	.welcome-card {
		padding: 30px;
		text-align: center;
		transition: all 0.3s ease;
		cursor: pointer;
		position: relative;
		overflow: hidden;
	}
	
	.welcome-card::before {
		content: '';
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		background: linear-gradient(90deg, transparent, rgba(255,255,255,0.1), transparent);
		transition: left 0.5s;
	}
	
	.welcome-card:hover::before {
		left: 100%;
	}
	
	.welcome-card.disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
	
	.card-header {
		margin-bottom: 20px;
	}
	
	.card-icon {
		font-size: 3rem;
		margin-bottom: 15px;
		display: block;
	}
	
	.card-title {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}
	
	.card-content {
		margin-bottom: 25px;
	}
	
	.card-description {
		color: var(--text-secondary);
		line-height: 1.6;
		margin: 0;
	}
	
	.card-footer {
		margin-top: auto;
	}
	
	.btn {
		width: 100%;
		justify-content: center;
	}
	
	.disabled .btn {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
