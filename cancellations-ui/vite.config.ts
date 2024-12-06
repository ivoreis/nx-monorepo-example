/// <reference types='vitest' />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';
import { nxCopyAssetsPlugin } from '@nx/vite/plugins/nx-copy-assets.plugin';
import { federation } from '@module-federation/vite';

export default defineConfig({
  root: __dirname,
  cacheDir: '../node_modules/.vite/cancellations-ui',
  server: {
    port: 4200,
    host: 'localhost',
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:3000', // Replace with your API server
        changeOrigin: true,
        secure: false,
        rewrite: (path) => path.replace(/^\/api/, ''), // Optional: remove '/api' prefix
      },
    },
  },
  preview: {
    port: 4200,
    host: 'localhost',
  },
  plugins: [
    federation({
      name: 'app',
      exposes: {
        './UI': './src/remotes/UI.tsx',
      },
      remotes: {
        remote: {
          type: 'module',
          name: 'remote',
          entry: 'http://localhost:4200/remoteEntry.js',
        },
      },
      filename: 'remoteEntry.js',
      shared: {
        react: {
          singleton: true,
        },
      },
    }),
    react(),
    nxViteTsPaths(),
    nxCopyAssetsPlugin(['*.md']),
  ],
  build: {
    outDir: '../dist/cancellations-ui',
    emptyOutDir: true,
    reportCompressedSize: true,
    commonjsOptions: {
      transformMixedEsModules: true,
    },
    target: 'esnext',
  },
  test: {
    watch: false,
    globals: true,
    environment: 'jsdom',
    include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    reporters: ['default'],
    coverage: {
      reportsDirectory: '../coverage/cancellations-ui',
      provider: 'v8',
    },
  },
});
