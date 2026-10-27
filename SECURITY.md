# Security Policy

## Supported versions

Until a stable release policy is published, security fixes should target the latest development version on the default branch. This is a project policy proposal and does not imply a production-support commitment.

## Reporting a vulnerability

Please do not publish exploit details in a public issue. Use GitHub's private vulnerability reporting or Security Advisories feature for this repository if available. If that feature is unavailable, contact the repository maintainers privately through their GitHub profile and include a minimal reproduction, affected revision, impact, and suggested mitigation where possible.

## Security review priorities

- Do not treat CORS as authentication or authorization.
- Never expose secrets, environment dumps, stack traces, or internal errors in public responses.
- Bound request bodies, timeouts, concurrency, and any client-controlled request identifiers.
- Do not trust forwarded IP/protocol headers unless a trusted proxy is explicitly configured.
- Keep authentication and authorization explicitly absent until implemented and tested; deny access by default for protected routes.
- Run the RustSec advisory audit against Cargo.lock in CI.
- Do not describe the framework as production-ready until security and operational limitations are documented and reviewed.
