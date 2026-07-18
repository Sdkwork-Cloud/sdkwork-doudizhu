import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkAppConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { DoudizhuApi, createDoudizhuApi } from './api/doudizhu';

export class SdkworkDoudizhuAppClient {
  private httpClient: HttpClient;

  public readonly doudizhu: DoudizhuApi;

  constructor(config: SdkworkAppConfig) {
    this.httpClient = createHttpClient(config);
    this.doudizhu = createDoudizhuApi(this.httpClient);
  }
  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.httpClient.setAccessToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }

  get http(): HttpClient {
    return this.httpClient;
  }
}

export function createClient(config: SdkworkAppConfig): SdkworkDoudizhuAppClient {
  return new SdkworkDoudizhuAppClient(config);
}

export default SdkworkDoudizhuAppClient;
