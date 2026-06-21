# Tools

Generation and materialization utilities for APIs, SDKs, and database contracts.

- `doudizhu_openapi_export.mjs`: materialize OpenAPI authorities from route manifests
- `doudizhu_route_manifest_check.mjs`: validate route manifest metadata
- `doudizhu_sdk_generate.mjs`: SDK generation and alignment checks
- `materialize_doudizhu_database_contract.mjs`: regenerate database contract artifacts

Do not hand-edit generated output under `sdks/` or `database/ddl/generated/`.
