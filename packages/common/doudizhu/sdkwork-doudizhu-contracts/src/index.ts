import { slugify } from '@sdkwork/utils';

export type DoudizhuMatchStatus = 'draft' | 'published' | 'archived';

export interface DoudizhuMatchSummary {
  id: string;
  matchCode: string;
  title: string;
  status: DoudizhuMatchStatus;
}

export function normalizeMatchCode(title: string): string {
  return slugify(title);
}

export const DOUDIZHU_APP_API_PREFIX = '/app/v3/api';
export const DOUDIZHU_BACKEND_API_PREFIX = '/backend/v3/api';
