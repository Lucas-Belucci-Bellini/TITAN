# TITAN HTTP contract v1

Status: initial reference implementation. This is the proposed shared wire contract for ATLAS, TITAN and AETHER. Only the endpoint listed below is intended to conform at this stage.

## Successful response envelope

Versioned endpoints return JSON shaped like:

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

## Implemented endpoint in this branch

`GET /api/v1/hello` — expected `200 OK`, with `data.message`, `data.environment` and `meta.api_version`. The endpoint is for contract smoke tests, not a production health/readiness check.

Existing `/health`, `/api/hello` and `/api/greetings` routes remain unchanged.

## Errors and request IDs

Error bodies are not yet unified with AETHER. Existing `ApiError` bodies use `error.code` and `error.message`. Clients must not assume every non-2xx response uses the success envelope. The middleware exposes `x-request-id`; clients may use it for diagnostics. Do not log secrets or sensitive payloads.

## Security and compatibility

- Use HTTPS in production.
- CORS is not authentication or authorization.
- Add explicit auth/authz before protected endpoints are deployed.
- Do not share Rust structs between repositories as an implicit wire format.
- Breaking API changes require a versioning/migration decision.

## Verification status

The endpoint uses the local typed `ApiEnvelope<T>` and `ApiMeta` response types. A dedicated HTTP integration test is added for the versioned endpoint. This document does not assert that the test has been run or passed; CI/local execution must verify it.
