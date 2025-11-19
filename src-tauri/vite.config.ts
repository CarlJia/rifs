import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './'),
    },
  },
  // Vite options tailored for Tauri to prevent too much magic
  // Detail: https://tauri.app/v1/guides/getting-started/setup/vite
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      // Using polling since fsEvents may not provide events quickly enough,
      // especially for embedded files in containers.
      usePolling: true,
      interval: 100,
    },
  },
  build: {
    target: 'ES2020',
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
})
