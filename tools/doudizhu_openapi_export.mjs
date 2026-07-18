#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { isBlank, joinPath } from '@sdkwork/utils';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const checkMode = process.argv.includes('--check');

const OWNER = 'sdkwork-doudizhu';
const DOMAIN = 'game';

const outputPaths = {
  appAuthority: joinPath('apis', 'app-api', 'game', 'doudizhu-app-api.openapi.json'),
  backendAuthority: joinPath('apis', 'backend-api', 'game', 'doudizhu-backend-api.openapi.json'),
  generatedApp: joinPath('generated', 'openapi', 'doudizhu-app-api.openapi.json'),
  generatedBackend: joinPath('generated', 'openapi', 'doudizhu-backend-api.openapi.json'),
};

const sharedComponents = {
  securitySchemes: {
    AuthToken: {
      type: 'http',
      scheme: 'bearer',
      bearerFormat: 'SDKWork-Auth-Token',
      description: 'SDKWork dual-token auth principal (Authorization bearer).',
    },
    AccessToken: {
      type: 'apiKey',
      in: 'header',
      name: 'Access-Token',
      description: 'SDKWork dual-token access credential header.',
    },
  },
  parameters: {
    MatchIdPath: {
      name: 'matchId',
      in: 'path',
      required: true,
      schema: { type: 'string' },
    },
    PageQuery: {
      name: 'page',
      in: 'query',
      required: false,
      schema: { type: 'integer', minimum: 1, default: 1 },
    },
    PageSizeQuery: {
      name: 'page_size',
      in: 'query',
      required: false,
      schema: { type: 'integer', minimum: 1, maximum: 200, default: 20 },
    },
    StatusQuery: {
      name: 'status',
      in: 'query',
      required: false,
      schema: { type: 'string' },
    },
  },
  responses: {
    ProblemDetailResponse: {
      description: 'RFC 9457 problem details.',
      content: {
        'application/problem+json': {
          schema: { $ref: '#/components/schemas/ProblemDetail' },
        },
      },
    },
  },
  schemas: {
    ProblemDetail: {
      type: 'object',
      additionalProperties: true,
      required: ['type', 'title', 'status', 'code', 'traceId'],
      properties: {
        type: { type: 'string', format: 'uri-reference' },
        title: { type: 'string' },
        status: { type: 'integer', minimum: 100, maximum: 599 },
        detail: { type: 'string' },
        instance: { type: 'string' },
        code: { type: 'integer', format: 'int32', minimum: 40001, maximum: 79999 },
        traceId: { type: 'string', format: 'uuid' },
      },
    },
    SdkWorkApiResponse: {
      type: 'object',
      additionalProperties: false,
      required: ['code', 'data', 'traceId'],
      properties: {
        code: { type: 'integer', format: 'int32', enum: [0], default: 0 },
        data: {},
        traceId: { type: 'string', format: 'uuid' },
      },
    },
    PageInfo: {
      type: 'object',
      additionalProperties: false,
      required: ['mode', 'page', 'pageSize', 'totalItems', 'totalPages'],
      properties: {
        mode: { type: 'string', enum: ['offset', 'cursor'] },
        page: { type: 'integer', minimum: 1 },
        pageSize: { type: 'integer', minimum: 1, maximum: 200 },
        totalItems: { type: 'string', pattern: '^[0-9]+$' },
        totalPages: { type: 'integer', minimum: 0 },
        nextCursor: { type: ['string', 'null'] },
        hasMore: { type: 'boolean' },
      },
    },
    DoudizhuMatchItem: {
      type: 'object',
      additionalProperties: false,
      required: ['id', 'gameCode', 'title', 'status'],
      properties: {
        id: { type: 'string' },
        gameCode: { type: 'string' },
        title: { type: 'string' },
        summary: { type: 'string' },
        genre: { type: 'string' },
        status: { type: 'string' },
      },
    },
    DoudizhuMatchData: {
      type: 'object',
      additionalProperties: false,
      required: ['item'],
      properties: {
        item: { $ref: '#/components/schemas/DoudizhuMatchItem' },
      },
    },
    DoudizhuMatchResponse: {
      allOf: [
        { $ref: '#/components/schemas/SdkWorkApiResponse' },
        {
          type: 'object',
          properties: { data: { $ref: '#/components/schemas/DoudizhuMatchData' } },
        },
      ],
    },
    DoudizhuMatchListData: {
      type: 'object',
      additionalProperties: false,
      required: ['items', 'pageInfo'],
      properties: {
        items: {
          type: 'array',
          items: { $ref: '#/components/schemas/DoudizhuMatchItem' },
        },
        pageInfo: { $ref: '#/components/schemas/PageInfo' },
      },
    },
    DoudizhuMatchListResponse: {
      allOf: [
        { $ref: '#/components/schemas/SdkWorkApiResponse' },
        {
          type: 'object',
          properties: { data: { $ref: '#/components/schemas/DoudizhuMatchListData' } },
        },
      ],
    },
  },
};

function buildOpenApi(title, serverUrl, operations) {
  return {
    openapi: '3.1.2',
    jsonSchemaDialect: 'https://json-schema.org/draft/2020-12/schema',
    info: {
      title,
      version: '0.1.0',
      description: 'SDKWork doudizhu platform HTTP contract.',
      'x-sdkwork-owner': OWNER,
      'x-sdkwork-domain': DOMAIN,
    },
    servers: [{ url: serverUrl, description: 'SDKWork API root' }],
    paths: operations,
    components: sharedComponents,
  };
}

const appOperations = {
  '/app/v3/api/doudizhu/matches': {
    get: {
      operationId: 'doudizhu.match.list',
      tags: ['doudizhu'],
      'x-sdkwork-request-context': 'WebRequestContext',
      'x-sdkwork-api-surface': 'app-api',
      'x-sdkwork-owner': OWNER,
      'x-sdkwork-domain': DOMAIN,
      security: [{ AuthToken: [], AccessToken: [] }],
      parameters: [
        { $ref: '#/components/parameters/PageQuery' },
        { $ref: '#/components/parameters/PageSizeQuery' },
        { $ref: '#/components/parameters/StatusQuery' },
      ],
      responses: {
        200: {
          description: 'OK',
          content: {
            'application/json': {
              schema: { $ref: '#/components/schemas/DoudizhuMatchListResponse' },
            },
          },
        },
        default: { $ref: '#/components/responses/ProblemDetailResponse' },
      },
    },
  },
  '/app/v3/api/doudizhu/matches/{matchId}': {
    get: {
      operationId: 'doudizhu.match.retrieve',
      tags: ['doudizhu'],
      'x-sdkwork-request-context': 'WebRequestContext',
      'x-sdkwork-api-surface': 'app-api',
      'x-sdkwork-owner': OWNER,
      'x-sdkwork-domain': DOMAIN,
      security: [{ AuthToken: [], AccessToken: [] }],
      parameters: [{ $ref: '#/components/parameters/MatchIdPath' }],
      responses: {
        200: {
          description: 'OK',
          content: {
            'application/json': {
              schema: { $ref: '#/components/schemas/DoudizhuMatchResponse' },
            },
          },
        },
        default: { $ref: '#/components/responses/ProblemDetailResponse' },
      },
    },
  },
};

const backendOperations = {
  '/backend/v3/api/doudizhu/matches': {
    get: {
      operationId: 'backend.doudizhu.match.list',
      tags: ['doudizhu'],
      'x-sdkwork-request-context': 'WebRequestContext',
      'x-sdkwork-api-surface': 'backend-api',
      'x-sdkwork-owner': OWNER,
      'x-sdkwork-domain': DOMAIN,
      security: [{ AuthToken: [], AccessToken: [] }],
      parameters: [
        { $ref: '#/components/parameters/PageQuery' },
        { $ref: '#/components/parameters/PageSizeQuery' },
        { $ref: '#/components/parameters/StatusQuery' },
      ],
      responses: {
        200: {
          description: 'OK',
          content: {
            'application/json': {
              schema: { $ref: '#/components/schemas/DoudizhuMatchListResponse' },
            },
          },
        },
        default: { $ref: '#/components/responses/ProblemDetailResponse' },
      },
    },
  },
};

if (isBlank(OWNER)) {
  throw new Error('owner must be non-empty');
}

const appDoc = buildOpenApi('SDKWork Doudizhu App API', '/app/v3/api', appOperations);
const backendDoc = buildOpenApi('SDKWork Doudizhu Backend API', '/backend/v3/api', backendOperations);

const expected = {
  [outputPaths.appAuthority]: `${JSON.stringify(appDoc, null, 2)}\n`,
  [outputPaths.backendAuthority]: `${JSON.stringify(backendDoc, null, 2)}\n`,
  [outputPaths.generatedApp]: `${JSON.stringify(appDoc, null, 2)}\n`,
  [outputPaths.generatedBackend]: `${JSON.stringify(backendDoc, null, 2)}\n`,
};

function assertMaterialized(relativePath, content) {
  const fullPath = path.join(root, relativePath);
  if (!fs.existsSync(fullPath)) {
    console.error(`[doudizhu-openapi] missing materialized document: ${relativePath}`);
    process.exit(1);
  }
  const actual = fs.readFileSync(fullPath, 'utf8');
  if (actual !== content) {
    console.error(`[doudizhu-openapi] stale materialized document: ${relativePath}`);
    process.exit(1);
  }
}

if (checkMode) {
  for (const [relativePath, content] of Object.entries(expected)) {
    assertMaterialized(relativePath, content);
  }
  process.stdout.write('[doudizhu-openapi] materialized OpenAPI documents are aligned\n');
} else {
  for (const relativePath of Object.keys(expected)) {
    fs.mkdirSync(path.dirname(path.join(root, relativePath)), { recursive: true });
  }
  for (const [relativePath, content] of Object.entries(expected)) {
    fs.writeFileSync(path.join(root, relativePath), content);
  }
  process.stdout.write('[doudizhu-openapi] exported app and backend OpenAPI documents\n');
}
