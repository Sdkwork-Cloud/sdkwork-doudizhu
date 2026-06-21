import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { DoudizhuHealthResponse } from '../types';


export class HealthDoudizhuReadyApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async check(): Promise<DoudizhuHealthResponse> {
    return this.client.get<DoudizhuHealthResponse>(appApiPath(`/system/ready`));
  }
}

export class HealthDoudizhuHealthApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async check(): Promise<DoudizhuHealthResponse> {
    return this.client.get<DoudizhuHealthResponse>(appApiPath(`/system/health`));
  }
}

export class HealthDoudizhuApi {
  private client: HttpClient;
  public readonly health: HealthDoudizhuHealthApi;
  public readonly ready: HealthDoudizhuReadyApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.health = new HealthDoudizhuHealthApi(client);
    this.ready = new HealthDoudizhuReadyApi(client);
  }

}

export class HealthApi {
  private client: HttpClient;
  public readonly doudizhu: HealthDoudizhuApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.doudizhu = new HealthDoudizhuApi(client);
  }

}

export function createHealthApi(client: HttpClient): HealthApi {
  return new HealthApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
