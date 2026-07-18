import type { DoudizhuMatchData } from './doudizhu-match-data';

export interface DoudizhuMatchResponse {
  code: 0;
  data: unknown & DoudizhuMatchData;
  traceId: string;
}
