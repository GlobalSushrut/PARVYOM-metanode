import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		// Static adapter for offline/air-gapped deployment
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html',
			precompress: true,
			strict: true
		}),
		
		// Configure for local serving by Rust/Axum gateway
		paths: {
			base: '',
			assets: ''
		},

		// Prerender all routes for static deployment
		prerender: {
			handleHttpError: 'warn',
			handleMissingId: 'warn'
		},

		// CSP for security (will be served by Rust gateway)
		csp: {
			mode: 'auto',
			directives: {
				'default-src': ['self'],
				'script-src': ['self'],
				'style-src': ['self', 'unsafe-inline'],
				'img-src': ['self', 'data:'],
				'font-src': ['self'],
				'connect-src': ['self']
			}
		}
	}
};

export default config;
