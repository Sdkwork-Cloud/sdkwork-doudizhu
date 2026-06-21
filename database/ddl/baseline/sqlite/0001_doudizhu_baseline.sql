-- SDKWork doudizhu database baseline (moduleId=doudizhu, prefix=ddz_)

CREATE TABLE IF NOT EXISTS ddz_match (
  id TEXT PRIMARY KEY,
  uuid TEXT NOT NULL UNIQUE,
  tenant_id TEXT NOT NULL,
  organization_id TEXT,
  match_code TEXT NOT NULL,
  title TEXT NOT NULL,
  summary TEXT,
  mode TEXT,
  cover_url TEXT,
  status TEXT NOT NULL DEFAULT 'draft',
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TEXT NOT NULL,
  created_by TEXT,
  updated_at TEXT NOT NULL,
  updated_by TEXT,
  version INTEGER NOT NULL DEFAULT 0,
  deleted_at TEXT,
  deleted_by TEXT,
  UNIQUE (tenant_id, organization_id, match_code)
);

CREATE TABLE IF NOT EXISTS ddz_room (
  id TEXT PRIMARY KEY,
  uuid TEXT NOT NULL UNIQUE,
  tenant_id TEXT NOT NULL,
  organization_id TEXT,
  match_id TEXT NOT NULL,
  room_code TEXT NOT NULL,
  max_players INTEGER NOT NULL DEFAULT 3,
  current_players INTEGER NOT NULL DEFAULT 0,
  status TEXT NOT NULL DEFAULT 'open',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE (tenant_id, room_code)
);

CREATE TABLE IF NOT EXISTS ddz_leaderboard (
  id TEXT PRIMARY KEY,
  uuid TEXT NOT NULL UNIQUE,
  tenant_id TEXT NOT NULL,
  organization_id TEXT,
  match_id TEXT NOT NULL,
  user_id TEXT NOT NULL,
  score BIGINT NOT NULL DEFAULT 0,
  rank_no INTEGER,
  recorded_at TEXT NOT NULL,
  UNIQUE (tenant_id, match_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_ddz_match_tenant_status ON ddz_match (tenant_id, status);
CREATE INDEX IF NOT EXISTS idx_ddz_room_match_status ON ddz_room (match_id, status);
CREATE INDEX IF NOT EXISTS idx_ddz_leaderboard_match_score ON ddz_leaderboard (match_id, score DESC);
