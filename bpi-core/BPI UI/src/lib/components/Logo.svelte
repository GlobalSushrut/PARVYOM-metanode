<script>
	export let size = 'medium';
	export let showText = true;
	
	const sizes = {
		small: { width: 40, height: 40, fontSize: '1rem' },
		medium: { width: 60, height: 60, fontSize: '1.5rem' },
		large: { width: 100, height: 100, fontSize: '2rem' }
	};
	
	$: currentSize = sizes[size] || sizes.medium;
</script>

<div class="logo-container" style="font-size: {currentSize.fontSize}">
	<div class="logo-icon" style="width: {currentSize.width}px; height: {currentSize.height}px;">
		<!-- PARVYOM Metanode Logo -->
		<img 
			src="/parvyom-logo.png" 
			alt="PARVYOM Metanode Logo"
			style="width: 100%; height: 100%; object-fit: contain; border-radius: 8px;"
		/>
	</div>
	
	{#if showText}
		<div class="logo-text">
			<div class="brand-name">PARVYOM Metanode</div>
			<div class="subtitle">BPI Core System</div>
		</div>
	{/if}
</div>

<style>
	.logo-container {
		display: flex;
		align-items: center;
		gap: 15px;
	}
	
	.logo-icon svg {
		filter: drop-shadow(0 4px 8px rgba(0,0,0,0.2));
	}
	
	.logo-small .logo-icon svg {
		width: 40px;
		height: 40px;
	}
	
	.logo-medium .logo-icon svg {
		width: 60px;
		height: 60px;
	}
	
	.logo-large .logo-icon svg {
		width: 80px;
		height: 80px;
	}
	
	.logo-text {
		color: white;
		text-align: left;
	}
	
	.logo-title {
		font-size: 1.5em;
		font-weight: 700;
		margin: 0;
		text-shadow: 1px 1px 2px rgba(0,0,0,0.3);
	}
	
	.logo-subtitle {
		font-size: 0.9em;
		opacity: 0.8;
		margin: 0;
		font-weight: 400;
	}
	
	.logo-large .logo-title {
		font-size: 2em;
	}
	
	.logo-large .logo-subtitle {
		font-size: 1.1em;
	}
	
	.logo-small .logo-title {
		font-size: 1.2em;
	}
	
	.logo-small .logo-subtitle {
		font-size: 0.8em;
	}
</style>
