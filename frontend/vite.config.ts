import { defineConfig } from 'vite'
import { resolve } from 'path'

export default defineConfig({
  root: '.',
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        owner: resolve(__dirname, 'owner.html'),
        franchise: resolve(__dirname, 'franchise.html'),
        transparency: resolve(__dirname, 'transparency.html'),
        video: resolve(__dirname, 'video.html'),
        tablet: resolve(__dirname, 'tablet.html')
      }
    }
  },
  server: {
    port: 3001,
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true
      }
    }
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src')
    }
  }
})
