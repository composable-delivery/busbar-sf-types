import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { resolve } from 'path';

export default defineConfig({
  plugins: [react()],
  root: './site',
  publicDir: './public',
  build: {
    outDir: '../docs',
    emptyOutDir: true,
    rollupOptions: {
      input: resolve(__dirname, 'site/index.html'),
    },
  },
  base: '/busbar-sf-types/',
});
