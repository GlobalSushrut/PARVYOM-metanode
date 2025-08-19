<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authStore } from '$lib/stores/auth';
	import SystemMetricsDashboard from '$lib/components/dashboard/SystemMetricsDashboard.svelte';

	// Redirect to login if not authenticated
	onMount(() => {
		const unsubscribe = authStore.subscribe(auth => {
			if (!auth.isAuthenticated) {
				goto('/login');
			}
		});
		return unsubscribe;
	});
</script>

<svelte:head>
	<title>System Metrics - BPI Dashboard</title>
	<meta name="description" content="Real-time system monitoring and performance metrics for BPI infrastructure" />
</svelte:head>

<SystemMetricsDashboard />
