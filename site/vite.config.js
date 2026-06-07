import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  base: '/busbar-sf-types/',
  build: {
    outDir: '../docs',
    emptyOutDir: true,
  },
})
