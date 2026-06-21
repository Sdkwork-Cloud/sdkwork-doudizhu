/// <reference types="vite/client" />

import {
  createClient,
  type SdkworkDoudizhuAppClient,
  type SdkworkAppConfig,
} from '@sdkwork-internal/doudizhu-app-sdk-generated';

const DEFAULT_DEV_PRINCIPAL =
  'tenant_id=demo-tenant;user_id=user-1;session_id=session-1;app_id=doudizhu;auth_level=password';

let cachedClient: SdkworkDoudizhuAppClient | null = null;

export function resolveDoudizhuAppSdkConfig(): SdkworkAppConfig {
  const env = import.meta.env;
  const baseUrl =
    env.VITE_SDKWORK_DOUDIZHU_APP_API_BASE_URL ??
    env.VITE_DOUDIZHU_API_BASE_URL ??
    (typeof window !== 'undefined' ? window.location.origin : 'http://localhost:8096');
  const principal =
    env.VITE_DOUDIZHU_DEV_PRINCIPAL ?? env.SDKWORK_ACCESS_TOKEN ?? DEFAULT_DEV_PRINCIPAL;

  return {
    baseUrl,
    authToken: `Bearer ${principal}`,
    accessToken: principal,
  };
}

export function getDoudizhuAppClient(): SdkworkDoudizhuAppClient {
  if (!cachedClient) {
    cachedClient = createClient(resolveDoudizhuAppSdkConfig());
  }
  return cachedClient;
}

export function resetDoudizhuAppClient(): void {
  cachedClient = null;
}
