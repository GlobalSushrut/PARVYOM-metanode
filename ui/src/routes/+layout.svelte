<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import Header from '$lib/components/layout/Header.svelte';
	import { systemStore } from '$lib/stores/system';
	import { websocketStore } from '$lib/stores/websocket';

	// Initialize system monitoring and WebSocket connection
	onMount(() => {
		systemStore.initialize();
		websocketStore.connect();
		
		// Cleanup on unmount
		return () => {
			websocketStore.disconnect();
		};
	});
</script>

<div class="min-h-screen bg-gray-900 text-white">
	<!-- Main Layout Grid -->
	<div class="grid grid-cols-12 min-h-screen">
		<!-- Sidebar Navigation -->
		<div class="col-span-2 bg-gray-900 border-r border-gray-700">
			<Sidebar />
		</div>
		
		<!-- Main Content Area -->
		<div class="col-span-10 flex flex-col">
			<!-- Header -->
			<Header />
			
			<!-- Page Content -->
			<main class="flex-1 p-6 overflow-y-auto">
				<div class="max-w-7xl mx-auto">
					<!-- Page content will be rendered here -->
					<slot />
				</div>
			</main>
		</div>
	</div>
</div>

<!-- Global loading indicator -->
{#if $systemStore.loading}
	<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
		<div class="bg-gray-800 rounded-lg p-6 flex items-center space-x-3">
			<div class="animate-spin rounded-full h-6 w-6 border-b-2 border-quantum-silver"></div>
			<span class="text-white">Loading...</span>
		</div>
	</div>
{/if}

<!-- Global error notifications -->
{#if $systemStore.error}
	<div class="fixed top-4 right-4 bg-error-primary text-white p-4 rounded-lg shadow-lg z-50 animate-slide-up">
		<div class="flex items-center justify-between">
			<span>{$systemStore.error}</span>
			<button 
				class="ml-4 text-white hover:text-gray-200"
				on:click={() => systemStore.clearError()}
			>
				×
			</button>
		</div>
	</div>
{/if}
