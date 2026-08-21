# Doudizhu Deployments

Deployment descriptors and packaging handoff for `sdkwork-doudizhu`.

## Supported profiles

| Profile id | Notes |
| --- | --- |
| `standalone.development` / `standalone.production` | Local browser + API server on one host |
| `cloud.development` / `cloud.production` | API server container with PostgreSQL |

## Templates

- `templates/server.env.example` — cloud container server env for `DOUDIZHU_*` runtime
- `docker/Dockerfile.doudizhu-api` — release API server image

## Topology

- Dev: `etc/topology/standalone.development.env`
- No RPC split deployment yet; `sdkwork-discovery` is deferred until gRPC services are introduced.

## Packaging

- GitHub workflow: `sdkwork.workflow.json` + `.github/workflows/package.yml`
- Release artifact: `target/release/sdkwork-api-doudizhu-standalone-gateway`
