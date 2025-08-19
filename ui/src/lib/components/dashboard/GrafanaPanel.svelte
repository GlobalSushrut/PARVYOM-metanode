<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import type { ComponentType } from 'svelte';

	export let title: string;
	export let subtitle: string = '';
	export let height: string = 'h-64';
	export let refreshInterval: number = 10000; // 10 seconds
	export let loading: boolean = false;
	export let error: string | null = null;
	export let lastUpdate: Date | null = null;
	export let actions: Array<{label: string, action: () => void}> = [];
	export let fullscreen: boolean = false;

	let refreshTimer: ReturnType<typeof setInterval> | null = null;
	let panelElement: HTMLElement;

	function formatLastUpdate(date: Date | null): string {
		if (!date) return 'Never';
		const now = new Date();
		const diff = now.getTime() - date.getTime();
		const seconds = Math.floor(diff / 1000);
		const minutes = Math.floor(seconds / 60);
		const hours = Math.floor(minutes / 60);

		if (seconds < 60) return `${seconds}s ago`;
		if (minutes < 60) return `${minutes}m ago`;
		if (hours < 24) return `${hours}h ago`;
		return date.toLocaleString();
	}

	function toggleFullscreen() {
		fullscreen = !fullscreen;
	}

	onMount(() => {
		if (refreshInterval > 0) {
			refreshTimer = setInterval(() => {
				// Emit refresh event
				panelElement?.dispatchEvent(new CustomEvent('refresh'));
			}, refreshInterval);
		}
	});

	onDestroy(() => {
		if (refreshTimer) {
			clearInterval(refreshTimer);
		}
	});
</script>

<div 
	bind:this={panelElement}
	class="bg-gray-900 border border-gray-700 rounded-lg shadow-lg transition-all duration-300 {fullscreen ? 'fixed inset-4 z-50' : height}"
	class:animate-pulse={loading}
>
	<!-- Panel Header -->
	<div class="flex items-center justify-between p-4 border-b border-gray-700">
		<div class="flex-1">
			<h3 class="text-lg font-semibold text-white font-mono">{title}</h3>
			{#if subtitle}
				<p class="text-sm text-gray-400 mt-1">{subtitle}</p>
			{/if}
		</div>
		
		<div class="flex items-center space-x-2">
			<!-- Status Indicator -->
			<div class="flex items-center space-x-2">
				{#if loading}
					<div class="w-2 h-2 bg-yellow-400 rounded-full animate-pulse"></div>
					<span class="text-xs text-yellow-400 font-mono">UPDATING</span>
				{:else if error}
					<div class="w-2 h-2 bg-red-400 rounded-full"></div>
					<span class="text-xs text-red-400 font-mono">ERROR</span>
				{:else}
					<div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
					<span class="text-xs text-green-400 font-mono">LIVE</span>
				{/if}
			</div>

			<!-- Last Update -->
			{#if lastUpdate}
				<span class="text-xs text-gray-500 font-mono">
					{formatLastUpdate(lastUpdate)}
				</span>
			{/if}

			<!-- Actions -->
			{#each actions as action}
				<button
					on:click={action.action}
					class="px-2 py-1 text-xs bg-gray-800 hover:bg-gray-700 text-gray-300 rounded border border-gray-600 font-mono transition-colors"
				>
					{action.label}
				</button>
			{/each}

			<!-- Fullscreen Toggle -->
			<button
				on:click={toggleFullscreen}
				class="p-1 text-gray-400 hover:text-white transition-colors"
				title={fullscreen ? 'Exit Fullscreen' : 'Fullscreen'}
			>
				{#if fullscreen}
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
					</svg>
				{:else}
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4"></path>
					</svg>
				{/if}
			</button>
		</div>
	</div>

	<!-- Panel Content -->
	<div class="p-4 {fullscreen ? 'h-full overflow-auto' : 'h-full'} relative">
		{#if error}
			<div class="flex items-center justify-center h-full">
				<div class="text-center">
					<div class="text-red-400 mb-2">
						<svg class="w-12 h-12 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
						</svg>
					</div>
					<p class="text-red-400 font-mono text-sm">{error}</p>
				</div>
			</div>
		{:else if loading && !$$slots.default}
			<div class="flex items-center justify-center h-full">
				<div class="text-center">
					<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-400 mx-auto mb-2"></div>
					<p class="text-gray-400 font-mono text-sm">Loading data...</p>
				</div>
			</div>
		{:else}
			<slot />
		{/if}
	</div>
</div>

<style>
	/* Custom scrollbar for fullscreen mode */
	.overflow-auto::-webkit-scrollbar {
		width: 6px;
	}
	.overflow-auto::-webkit-scrollbar-track {
		background: #374151;
	}
	.overflow-auto::-webkit-scrollbar-thumb {
		background: #6b7280;
		border-radius: 3px;
	}
	.overflow-auto::-webkit-scrollbar-thumb:hover {
		background: #9ca3af;
	}
</style>
