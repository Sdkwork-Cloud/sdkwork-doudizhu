import type { DoudizhuMatchListData } from './doudizhu-match-list-data';

export interface DoudizhuMatchListResponse {
  code: 0;
  data: unknown & DoudizhuMatchListData;
  traceId: string;
}
