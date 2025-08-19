/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	darkMode: 'class',
	theme: {
		extend: {
			// BPI Design System Colors
			colors: {
				// Primary Brand Colors
				'bpi-deep-blue': '#0A1628',
				'quantum-silver': '#8B9DC3',
				'neural-gray': '#1E2329',
				
				// Functional Colors - Success Spectrum
				'success-primary': '#00D4AA',
				'success-secondary': '#00A085',
				'success-background': '#004D40',
				
				// Warning Spectrum
				'warning-primary': '#FFB020',
				'warning-secondary': '#FF8F00',
				'warning-background': '#FF6F00',
				
				// Error Spectrum
				'error-primary': '#FF4757',
				'error-secondary': '#FF3742',
				'error-background': '#2C1810',
				
				// Information Spectrum
				'info-primary': '#3742FA',
				'info-secondary': '#2F3542',
				'info-background': '#1A1D23',
				
				// Extended palette for components
				'bpi': {
					50: '#F0F4F8',
					100: '#D9E2EC',
					200: '#BCCCDC',
					300: '#9FB3C8',
					400: '#829AB1',
					500: '#627D98',
					600: '#486581',
					700: '#334E68',
					800: '#243B53',
					900: '#0A1628', // bpi-deep-blue
				}
			},
			
			// Typography System
			fontFamily: {
				'sans': ['Inter', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'sans-serif'],
				'mono': ['JetBrains Mono', 'SF Mono', 'Monaco', 'Cascadia Code', 'monospace']
			},
			
			// Type Scale
			fontSize: {
				'xs': ['0.75rem', { lineHeight: '1.3' }],      // 12px
				'sm': ['0.875rem', { lineHeight: '1.4' }],     // 14px
				'base': ['1rem', { lineHeight: '1.5' }],       // 16px
				'lg': ['1.125rem', { lineHeight: '1.6' }],     // 18px
				'xl': ['1.25rem', { lineHeight: '1.4' }],      // 20px
				'2xl': ['1.5rem', { lineHeight: '1.3' }],      // 24px
				'3xl': ['2rem', { lineHeight: '1.25' }],       // 32px
				'4xl': ['2.5rem', { lineHeight: '1.2' }],      // 40px
			},
			
			// Spacing for 12-column grid
			spacing: {
				'18': '4.5rem',
				'88': '22rem',
				'128': '32rem'
			},
			
			// Animation for enterprise feel
			animation: {
				'fade-in': 'fadeIn 0.3s ease-in-out',
				'slide-up': 'slideUp 0.3s ease-out',
				'pulse-soft': 'pulseSoft 2s cubic-bezier(0.4, 0, 0.6, 1) infinite'
			},
			
			keyframes: {
				fadeIn: {
					'0%': { opacity: '0' },
					'100%': { opacity: '1' }
				},
				slideUp: {
					'0%': { transform: 'translateY(10px)', opacity: '0' },
					'100%': { transform: 'translateY(0)', opacity: '1' }
				},
				pulseSoft: {
					'0%, 100%': { opacity: '1' },
					'50%': { opacity: '0.7' }
				}
			},
			
			// Box shadows for depth
			boxShadow: {
				'bpi-card': '0 4px 6px -1px rgba(10, 22, 40, 0.1), 0 2px 4px -1px rgba(10, 22, 40, 0.06)',
				'bpi-elevated': '0 10px 15px -3px rgba(10, 22, 40, 0.1), 0 4px 6px -2px rgba(10, 22, 40, 0.05)'
			}
		}
	},
	plugins: [
		// Add custom utilities for BPI design system
		function({ addUtilities }) {
			const newUtilities = {
				'.text-gradient-bpi': {
					'background': 'linear-gradient(135deg, #0A1628 0%, #8B9DC3 100%)',
					'-webkit-background-clip': 'text',
					'-webkit-text-fill-color': 'transparent',
					'background-clip': 'text'
				},
				'.bg-gradient-bpi': {
					'background': 'linear-gradient(135deg, #0A1628 0%, #1E2329 100%)'
				}
			}
			addUtilities(newUtilities)
		}
	]
}
