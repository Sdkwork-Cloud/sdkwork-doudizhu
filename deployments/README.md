# Doudizhu Deployments

Deployment descriptors and packaging handoff for `sdkwork-doudizhu`.

## Supported profiles

| Profile | Layout | Notes |
| --- | --- | --- |
| `standalone` | unified-process | Local browser + API server on one host |
| `cloud` | container | API server container with PostgreSQL |

## Templates

- `templates/server.env.example` — cloud container server env for `DOUDIZHU_*` runtime
- `docker/Dockerfile.doudizhu-api` — release API server image

## Topology

- Dev: `configs/topology/standalone.unified-process.development.env`
- No RPC split-service deployment yet; `sdkwork-discovery` is deferred until gRPC services are introduced.

## Packaging

- GitHub workflow: `sdkwork.workflow.json` + `.github/workflows/package.yml`
- Release artifact: `target/release/sdkwork-doudizhu-standalone-gateway`
