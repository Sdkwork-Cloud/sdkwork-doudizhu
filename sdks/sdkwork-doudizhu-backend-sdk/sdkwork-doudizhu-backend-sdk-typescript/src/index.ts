import {
  createClient as createGeneratedBackendClient,
  SdkworkDoudizhuBackendClient,
} from '../generated/server-openapi/src/index';
import type { SdkworkBackendConfig } from '../generated/server-openapi/src/types/common';

export { SdkworkDoudizhuBackendClient, createGeneratedBackendClient };
export type { SdkworkBackendConfig };
export * from '../generated/server-openapi/src/types';
export * from '../generated/server-openapi/src/api';
export * from '../generated/server-openapi/src/http';
export * from '../generated/server-openapi/src/auth';

export type SdkworkBackendClient = SdkworkDoudizhuBackendClient;

export function createClient(config: SdkworkBackendConfig): SdkworkBackendClient {
  return createGeneratedBackendClient(config);
}
