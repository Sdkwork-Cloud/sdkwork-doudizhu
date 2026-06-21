import type { DoudizhuMatchItem } from './doudizhu-match-item';

export interface DoudizhuMatchPage {
  items: DoudizhuMatchItem[];
  total: number;
  page: number;
  pageSize: number;
}
