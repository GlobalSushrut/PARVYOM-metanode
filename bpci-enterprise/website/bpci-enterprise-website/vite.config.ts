import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  define: {
    global: 'globalThis',
    'process.env': {},
  },
  build: {
    target: 'es2015',
    rollupOptions: {
      output: {
        manualChunks: undefined,
        entryFileNames: 'assets/bpci-testnet-[hash].js',
        chunkFileNames: 'assets/bpci-chunk-[hash].js',
        assetFileNames: 'assets/bpci-asset-[hash].[ext]',
      },
    },
  },
})
