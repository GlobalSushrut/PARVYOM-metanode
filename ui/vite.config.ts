import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	
	// Build configuration for static deployment
	build: {
		target: 'es2020',
		minify: 'esbuild',
		sourcemap: false,
		rollupOptions: {
			output: {
				// Asset hashing for cache busting
				assetFileNames: 'assets/[name]-[hash][extname]',
				chunkFileNames: 'assets/chunk-[hash].js',
				entryFileNames: 'assets/app-[hash].js'
			}
		}
	},

	// Development server configuration
	server: {
		port: 5173,
		host: '127.0.0.1',
		strictPort: true,
		// Proxy API calls to BPI gateway during development
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:8617',
				changeOrigin: true
			},
			'/_bpi': {
				target: 'http://127.0.0.1:8617',
				changeOrigin: true
			}
		}
	},

	// Optimize dependencies
	optimizeDeps: {
		include: ['chart.js', 'date-fns'],
		exclude: ['@codemirror/lang-yaml', '@codemirror/lang-json']
	},

	// Define for environment variables
	define: {
		__BPI_VERSION__: JSON.stringify(process.env.npm_package_version || '1.0.0'),
		__BUILD_TIME__: JSON.stringify(new Date().toISOString())
	}
});
