import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { defineConfig, loadEnv } from 'vite';

const doudizhuAppSdkRoot = path.resolve(
  __dirname,
  '../../sdks/sdkwork-doudizhu-app-sdk/sdkwork-doudizhu-app-sdk-typescript/src/index.ts',
);

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, '.', '');
  return {
    plugins: [react(), tailwindcss()],
    define: {
      'process.env.GEMINI_API_KEY': JSON.stringify(env.GEMINI_API_KEY),
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ''),
    },
    resolve: {
      alias: {
        '@': path.resolve(__dirname, '.'),
        '@sdkwork-internal/doudizhu-app-sdk-generated': path.resolve(
          doudizhuAppSdkRoot,
          'src/index.ts',
        ),
      },
    },
    server: {
      hmr: process.env.DISABLE_HMR !== 'true',
      proxy: {
        '/app/v3/api': {
          target: env.VITE_DOUDIZHU_API_BASE_URL ?? 'http://127.0.0.1:8096',
          changeOrigin: true,
        },
      },
    },
  };
});
