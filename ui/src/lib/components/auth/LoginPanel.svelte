<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth';
	import { goto } from '$app/navigation';

	// Login form state
	let username = '';
	let password = '';
	let mfaCode = '';
	let rememberMe = false;
	let showPassword = false;
	let loginStep: 'credentials' | 'mfa' | 'wallet' = 'credentials';

	// UI state
	let isLoading = false;
	let shake = false;

	// Reactive statements
	$: canSubmit = username.trim() && password.trim() && !isLoading;
	$: canSubmitMFA = mfaCode.trim().length === 6 && !isLoading;

	async function handleLogin() {
		if (!canSubmit) return;
		
		isLoading = true;
		const success = await authStore.login(username, password);
		
		if (success) {
			goto('/dashboard');
		} else if ($authStore.mfaRequired) {
			loginStep = 'mfa';
		} else {
			// Shake animation on error
			shake = true;
			setTimeout(() => shake = false, 500);
		}
		
		isLoading = false;
	}

	async function handleMFA() {
		if (!canSubmitMFA) return;
		
		isLoading = true;
		const success = await authStore.completeMFA(mfaCode);
		
		if (success) {
			goto('/dashboard');
		} else {
			shake = true;
			setTimeout(() => shake = false, 500);
		}
		
		isLoading = false;
	}

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			if (loginStep === 'credentials') {
				handleLogin();
			} else if (loginStep === 'mfa') {
				handleMFA();
			}
		}
	}

	function goBackToCredentials() {
		loginStep = 'credentials';
		mfaCode = '';
		authStore.clearError();
	}

	onMount(() => {
		// Clear any existing errors
		authStore.clearError();
		
		// Focus on username field
		const usernameInput = document.getElementById('username');
		if (usernameInput) {
			usernameInput.focus();
		}
	});
</script>

<div class="min-h-screen bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 flex items-center justify-center p-4">
	<!-- Background Animation -->
	<div class="absolute inset-0 overflow-hidden">
		<div class="absolute -inset-10 opacity-20">
			<div class="absolute top-1/4 left-1/4 w-96 h-96 bg-blue-500 rounded-full mix-blend-multiply filter blur-xl animate-blob"></div>
			<div class="absolute top-1/3 right-1/4 w-96 h-96 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl animate-blob animation-delay-2000"></div>
			<div class="absolute bottom-1/4 left-1/3 w-96 h-96 bg-pink-500 rounded-full mix-blend-multiply filter blur-xl animate-blob animation-delay-4000"></div>
		</div>
	</div>

	<!-- Login Card -->
	<div class="relative w-full max-w-md">
		<!-- Main Card -->
		<div class="bg-white/10 backdrop-blur-xl rounded-2xl shadow-2xl border border-white/20 p-8 {shake ? 'animate-shake' : ''}">
			<!-- Logo and Header -->
			<div class="text-center mb-8">
				<div class="w-16 h-16 bg-gradient-to-r from-blue-500 to-purple-600 rounded-2xl flex items-center justify-center mx-auto mb-4 shadow-lg">
					<svg class="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 24 24">
						<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
					</svg>
				</div>
				<h1 class="text-2xl font-bold text-white mb-2">BPI Operations Center</h1>
				<p class="text-gray-300 text-sm">Enterprise Blockchain Infrastructure</p>
			</div>

			{#if loginStep === 'credentials'}
				<!-- Credentials Form -->
				<form on:submit|preventDefault={handleLogin} class="space-y-6">
					<!-- Username Field -->
					<div class="space-y-2">
						<label for="username" class="text-sm font-medium text-gray-200">Username</label>
						<div class="relative">
							<input
								id="username"
								type="text"
								bind:value={username}
								on:keypress={handleKeyPress}
								placeholder="Enter your username"
								class="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
								disabled={isLoading}
								autocomplete="username"
							/>
							<div class="absolute inset-y-0 right-0 flex items-center pr-3">
								<svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
								</svg>
							</div>
						</div>
					</div>

					<!-- Password Field -->
					<div class="space-y-2">
						<label for="password" class="text-sm font-medium text-gray-200">Password</label>
						<div class="relative">
							<input
								id="password"
								type={showPassword ? 'text' : 'password'}
								bind:value={password}
								on:keypress={handleKeyPress}
								placeholder="Enter your password"
								class="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
								disabled={isLoading}
								autocomplete="current-password"
							/>
							<button
								type="button"
								on:click={() => showPassword = !showPassword}
								class="absolute inset-y-0 right-0 flex items-center pr-3 text-gray-400 hover:text-white transition-colors"
							>
								{#if showPassword}
									<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21"></path>
									</svg>
								{:else}
									<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
									</svg>
								{/if}
							</button>
						</div>
					</div>

					<!-- Remember Me -->
					<div class="flex items-center justify-between">
						<label class="flex items-center">
							<input
								type="checkbox"
								bind:checked={rememberMe}
								class="rounded border-white/20 bg-white/10 text-blue-500 focus:ring-blue-500 focus:ring-offset-0"
								disabled={isLoading}
							/>
							<span class="ml-2 text-sm text-gray-300">Remember me</span>
						</label>
						<button type="button" class="text-sm text-blue-400 hover:text-blue-300 transition-colors">
							Forgot password?
						</button>
					</div>

					<!-- Error Message -->
					{#if $authStore.error}
						<div class="bg-red-500/20 border border-red-500/30 rounded-lg p-3">
							<div class="flex items-center">
								<svg class="w-5 h-5 text-red-400 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
								</svg>
								<p class="text-sm text-red-300">{$authStore.error}</p>
							</div>
						</div>
					{/if}

					<!-- Login Button -->
					<button
						type="submit"
						disabled={!canSubmit}
						class="w-full bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 disabled:from-gray-500 disabled:to-gray-600 text-white font-semibold py-3 px-4 rounded-xl transition-all duration-200 transform hover:scale-[1.02] disabled:scale-100 disabled:cursor-not-allowed shadow-lg"
					>
						{#if isLoading}
							<div class="flex items-center justify-center">
								<svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
									<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
									<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
								</svg>
								Signing in...
							</div>
						{:else}
							Sign In
						{/if}
					</button>
				</form>

			{:else if loginStep === 'mfa'}
				<!-- MFA Form -->
				<div class="space-y-6">
					<div class="text-center">
						<div class="w-12 h-12 bg-blue-500/20 rounded-full flex items-center justify-center mx-auto mb-4">
							<svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
							</svg>
						</div>
						<h2 class="text-xl font-semibold text-white mb-2">Two-Factor Authentication</h2>
						<p class="text-gray-300 text-sm">Enter the 6-digit code from your authenticator app</p>
					</div>

					<form on:submit|preventDefault={handleMFA} class="space-y-6">
						<!-- MFA Code Input -->
						<div class="space-y-2">
							<label for="mfaCode" class="text-sm font-medium text-gray-200">Authentication Code</label>
							<input
								id="mfaCode"
								type="text"
								bind:value={mfaCode}
								on:keypress={handleKeyPress}
								placeholder="000000"
								maxlength="6"
								class="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 text-center text-2xl font-mono tracking-widest"
								disabled={isLoading}
								autocomplete="one-time-code"
							/>
						</div>

						<!-- Error Message -->
						{#if $authStore.error}
							<div class="bg-red-500/20 border border-red-500/30 rounded-lg p-3">
								<div class="flex items-center">
									<svg class="w-5 h-5 text-red-400 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"></path>
									</svg>
									<p class="text-sm text-red-300">{$authStore.error}</p>
								</div>
							</div>
						{/if}

						<!-- Buttons -->
						<div class="space-y-3">
							<button
								type="submit"
								disabled={!canSubmitMFA}
								class="w-full bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 disabled:from-gray-500 disabled:to-gray-600 text-white font-semibold py-3 px-4 rounded-xl transition-all duration-200 transform hover:scale-[1.02] disabled:scale-100 disabled:cursor-not-allowed shadow-lg"
							>
								{#if isLoading}
									<div class="flex items-center justify-center">
										<svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
											<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
											<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
										</svg>
										Verifying...
									</div>
								{:else}
									Verify Code
								{/if}
							</button>

							<button
								type="button"
								on:click={goBackToCredentials}
								class="w-full bg-white/10 hover:bg-white/20 text-white font-medium py-3 px-4 rounded-xl transition-all duration-200 border border-white/20"
								disabled={isLoading}
							>
								Back to Login
							</button>
						</div>
					</form>
				</div>
			{/if}

			<!-- Footer -->
			<div class="mt-8 pt-6 border-t border-white/10">
				<div class="flex items-center justify-center space-x-4 text-xs text-gray-400">
					<span>BPI v2.0.0</span>
					<span>•</span>
					<span>Military-Grade Security</span>
					<span>•</span>
					<span>FIPS 140-2</span>
				</div>
			</div>
		</div>

		<!-- Security Badge -->
		<div class="absolute -bottom-4 left-1/2 transform -translate-x-1/2">
			<div class="bg-green-500/20 border border-green-500/30 rounded-full px-4 py-2 backdrop-blur-sm">
				<div class="flex items-center space-x-2">
					<div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
					<span class="text-xs text-green-300 font-medium">Secure Connection</span>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	@keyframes blob {
		0% {
			transform: translate(0px, 0px) scale(1);
		}
		33% {
			transform: translate(30px, -50px) scale(1.1);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.9);
		}
		100% {
			transform: translate(0px, 0px) scale(1);
		}
	}
	
	.animate-blob {
		animation: blob 7s infinite;
	}
	
	.animation-delay-2000 {
		animation-delay: 2s;
	}
	
	.animation-delay-4000 {
		animation-delay: 4s;
	}
	
	@keyframes shake {
		0%, 100% { transform: translateX(0); }
		10%, 30%, 50%, 70%, 90% { transform: translateX(-5px); }
		20%, 40%, 60%, 80% { transform: translateX(5px); }
	}
	
	.animate-shake {
		animation: shake 0.5s ease-in-out;
	}
</style>
