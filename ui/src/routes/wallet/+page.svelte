<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth';
	import WalletDashboard from '$lib/components/wallet/WalletDashboard.svelte';

	onMount(() => {
		// If user is not authenticated, redirect to login
		if (!$authStore.isAuthenticated) {
			goto('/login');
		}
	});
</script>

<svelte:head>
	<title>Wallet - BPI Operations Center</title>
	<meta name="description" content="BPI Wallet - Enterprise Digital Asset Management" />
</svelte:head>

{#if $authStore.isAuthenticated}
	<WalletDashboard />
{:else}
	<div class="min-h-screen bg-slate-900 flex items-center justify-center">
		<div class="text-white text-center">
			<div class="animate-spin w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full mx-auto mb-4"></div>
			<p>Loading wallet...</p>
		</div>
	</div>
{/if}
