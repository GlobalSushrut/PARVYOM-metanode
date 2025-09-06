<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	
	let mounted = false;
	
	onMount(() => {
		mounted = true;
	});
</script>

<svelte:head>
	<title>BPI Installer - Parvyom Metanode</title>
</svelte:head>

<div class="app">
	{#if mounted}
		<main class="fade-in">
			<slot />
		</main>
	{/if}
</div>

<style>
	.app {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}
	
	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		width: 100%;
	}
</style>
