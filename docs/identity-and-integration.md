# TITAN family identity and backend integration contract

Status: **proposed architecture decision for review** (2026-10). This document defines intended product boundaries; it does not claim cross-repository adapters or production readiness.

## Official product names

- **AETHER** — modular Rust application/composition framework.
- **ATLAS** — Rust frontend framework for browser interfaces compiled to WebAssembly (WASM).
- **TITAN** — Rust backend HTTP framework for APIs and services.

These are the public product/repository names for the family. This repository currently uses **Ferrovia** and the `ferrovia-*` Cargo crate namespace. Treat these as existing package/API identities during a compatibility transition. Do not rename packages, imports, environment variables or public symbols without a separate migration plan and explicit approval.

## TITAN responsibility

TITAN owns backend HTTP/service concerns: routing, request handling, middleware, validated configuration, error responses, request context, graceful shutdown and extension points. It should expose documented APIs and stable integration contracts without depending on ATLAS.

AETHER and TITAN overlap in some backend-oriented capabilities. The intended distinction is:
- **TITAN**: focused HTTP/service foundation and server pipeline.
- **AETHER**: broader application composition, module lifecycle, background tasks and optional policy-controlled AI workflows.

Do not duplicate these capabilities by default. Before an integration is implemented, map overlapping features and decide which framework owns each responsibility. Prefer an adapter or documented composition pattern over a direct dependency if that preserves independent use and avoids cycles.

## Intended family flow

`ATLAS (browser/WASM) → versioned HTTP/JSON contract → TITAN (backend service)`

AETHER is optional and can orchestrate broader application modules or workflows around stable interfaces. This flow is an intended design, not a claim that a ready-to-use ATLAS/TITAN integration exists.

## Backend contract requirements

1. Publish endpoint, method, request/response schema, status code and error behavior as a versioned contract.
2. Keep machine-readable error codes stable. Map TITAN's current error shape to AETHER's current error shape before declaring interoperability.
3. Define request/correlation ID behavior consistently across middleware and API responses.
4. Treat configuration and secrets as sensitive. Redact credentials and payloads; validate configuration before serving traffic.
5. Define authentication and authorization explicitly. CORS is not authentication; rate limiting, abuse controls and deployment TLS need explicit design and tests.
6. Set and test request body limits, timeouts, concurrency limits and graceful-shutdown behavior.
7. Keep plugin capabilities explicit and deny by default where a capability grants access to sensitive operations.
8. Keep public APIs independent from private implementation types and avoid circular dependencies with AETHER or ATLAS.

## Compatibility and crate naming

- Product/repository brand: TITAN.
- Current package namespace: `ferrovia-*` (observed in the workspace manifest).
- Existing public Rust API remains unchanged until a separate approved migration.
- A future rename must inventory imports, examples, docs, environment-variable prefixes (the README currently documents `FERROVIA_*`), Cargo metadata and downstream use. No rename is included in this proposal.

## Verification gates before production claims

- Unit and integration tests for middleware ordering, error normalization, request IDs, body limits, timeouts and shutdown.
- Security review for authentication/authorization, CORS, secret redaction, rate limiting and dependency advisories.
- Contract tests for compatibility with a documented ATLAS-facing API.
- Load/latency benchmarks with reproducible setup and realistic limits.
- Actual CI results recorded for the supported MSRV and stable toolchain.

These are acceptance criteria, not claims that the features/tests are complete or passing.

## Current evidence and limits

The current README describes `ferrovia-core` and `ferrovia-server`. It explicitly lists authentication, authorization, rate limiting, OpenAPI and Prometheus metrics as absent. This document does not re-run tests or independently validate the README's reported test count.

## Decision record

- Proposed: TITAN = backend HTTP/service framework.
- Proposed: keep the HTTP/API layer distinct from AETHER's broader composition role.
- Deferred: production readiness, shared stable error/API contract, package renaming and a mandatory AETHER dependency.
- Approval: pending maintainer review.
