/// <reference types='vitest' />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';
import { nxCopyAssetsPlugin } from '@nx/vite/plugins/nx-copy-assets.plugin';
import { federation } from '@module-federation/vite';

const DEFAULT_PORT = 4200;
const DEFAULT_HOST = 'localhost';
const REMOTE_ENTRY_NAME = 'remoteEntry.js';
const LOCAL_API = 'http://localhost:3000';

function generateEntry(serverName: string, serverPort: number, ssl: boolean) {
  const proto = ssl ? 'https' : 'http';
  const entry = `${proto}://${serverName}:${serverPort}/${REMOTE_ENTRY_NAME}`;

  return entry;
}

export default defineConfig({
  root: __dirname,
  cacheDir: '../node_modules/.vite/cancellations-ui',
  server: {
    port: DEFAULT_PORT,
    host: DEFAULT_HOST,
    proxy: {
      '/api': {
        target: LOCAL_API,
        changeOrigin: true,
        secure: false,
        rewrite: (path) => path.replace(/^\/api/, ''), // Optional: remove '/api' prefix
      },
    },
  },
  preview: {
    port: DEFAULT_PORT,
    host: DEFAULT_HOST,
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
          entry: generateEntry(
            process.env.SERVER_NAME || DEFAULT_HOST,
            (process.env.SERVER_PORT &&
              parseInt(process.env.SERVER_PORT, 10)) ||
              DEFAULT_PORT,
            process.env.ENABLE_SSL === 'true'
          ),
        },
      },
      filename: REMOTE_ENTRY_NAME,
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
