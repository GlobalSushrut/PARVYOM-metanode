<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth';
	import LoginPanel from '$lib/components/auth/LoginPanel.svelte';

	onMount(() => {
		// If user is already authenticated, redirect to dashboard
		if ($authStore.isAuthenticated) {
			goto('/dashboard');
		}
	});
</script>

<svelte:head>
	<title>Login - BPI Operations Center</title>
	<meta name="description" content="Secure login to BPI Operations Center - Enterprise Blockchain Infrastructure" />
</svelte:head>

<LoginPanel />
