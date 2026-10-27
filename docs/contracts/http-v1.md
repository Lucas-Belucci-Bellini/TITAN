# TITAN HTTP contract v1

Status: initial reference implementation. This is the proposed shared wire contract for ATLAS, TITAN and AETHER. Only the endpoints described below are intended to conform at this stage.

## Successful response envelope

Versioned successful endpoints return JSON shaped like:

```json
{
  "data": {
    "message": "Olá do TITAN!",
    "environment": "development"
  },
  "meta": {
    "api_version": "v1"
  }
}
```

Rules:
- Route-specific payload belongs inside `data`.
- `meta.api_version` is `v1`.
- JSON field names use snake_case.
- Content type is `application/json`.
- Additive response fields are preferred.

## Versioned error envelope

The route-level error contract for paths under `/api/v1` is:

```json
{
  "error": {
    "code": "not_found",
    "message": "recurso não encontrado"
  },
  "meta": {
    "api_version": "v1"
  }
}
```

`error.code` is stable for programmatic handling. `error.message` must remain safe for display and must not contain stack traces, SQL, secrets or internal diagnostics. HTTP status remains authoritative.

The `/api/v1` router uses this envelope for route-level 404 and 405 responses and for request timeouts. The timeout middleware returns a versioned `408` response for paths under `/api/v1`; unversioned paths retain their existing legacy error shape. The timeout test also checks that CORS headers and `x-request-id` remain visible to an allowed browser origin.

## Implemented endpoints in this branch

- `GET /api/v1/hello` — expected `200 OK`, with `data.message`, `data.environment` and `meta.api_version`.
- Unknown paths beneath `/api/v1` return a versioned `404`.
- Unsupported methods on a known `/api/v1` path return a versioned `405`.

The hello endpoint is a contract smoke test, not a production health/readiness check. Existing `/health`, `/api/hello` and `/api/greetings` routes retain their current wire formats.

## Request IDs

The middleware emits/preserves `x-request-id` and exposes it to browser clients when explicit CORS origins are configured. Use it for diagnostics; do not log secrets or sensitive payloads.

## Security and compatibility

- Use HTTPS in production.
- CORS is not authentication or authorization.
- Add explicit auth/authz before protected endpoints are deployed.
- Do not share Rust structs between repositories as an implicit wire format.
- Breaking API changes require a versioning/migration decision.

## Verification status

The dedicated contract tests cover the success envelope, versioned route-level 404/405 responses, versioned timeout responses, and browser-visible request-ID headers. CI must verify each commit before it is considered validated.
