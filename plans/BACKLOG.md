# Backlog — TITAN (pacotes atuais: ferrovia-*)

As tarefas preservam os IDs históricos `RB-` para não quebrar referências. Status reflete o estado atual do código e dos workflows.

## RB-001 — CI e qualidade
**Prioridade:** P0 · **Status:** concluído

- Workflow executa `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.
- MSRV `rust-version = "1.89"` verificada em workflow dedicado.
- Auditoria RustSec de dependências executada em workflow separado.

**Evidência:** workflows `Rust CI` e `Dependency Security Audit`; consultar resultados pelo SHA do commit.

## RB-002 — Testes de integração HTTP
**Prioridade:** P0 · **Status:** concluído (`crates/ferrovia-server/tests/http.rs`)

- Testar status, headers e payloads de `/health` e `/api/hello`.
- Criação do router permite testes sem abrir porta real.
- Contrato versionado `/api/v1/hello` também é exercitado no workflow de integração com ATLAS.

## RB-003 — Erros padronizados e validação
**Prioridade:** P1 · **Status:** concluído

- Corpo de erro estável, `ApiError`, `ApiJson` e validação de `POST /api/greetings`.
- Payload inválido retorna 4xx sem expor stack traces ou detalhes internos.

## RB-004 — Middleware básico
**Prioridade:** P1 · **Status:** concluído

- Request ID, tracing, timeout, limite de body e CORS opcional com origens explícitas.

## RB-005 — Registro de plugins com rotas
**Prioridade:** P1 · **Status:** em andamento / parcial

- Registro estático de plugins e capacidades existe.
- Falta contrato completo para plugins registrarem rotas/serviços e lifecycle configure/start/shutdown com rollback.

**Aceite:** plugin de demonstração registra rota, e erros de inicialização são reportados com contexto.

## RB-006 — Persistência opcional
**Prioridade:** P2 · **Status:** não iniciado

- Escolher banco por necessidade real e comparar opções de driver/ORM.
- Definir repositórios, migrations, pool e testes de integração.

**Aceite:** módulo de exemplo persiste dados com consultas parametrizadas e testes reproduzíveis.

## RB-007 — Revisão de segurança de pré-produção
**Prioridade:** P1 antes de produção · **Status:** parcial / bloqueador de produção

- Definir autenticação/autorização, limites de taxa e política de segredos.
- Rever logs, timeouts, limites de body, CORS e dependências.
- Adicionar threat model e checklist de deploy.

**Aceite:** checklist documentado, testes pertinentes e nenhum segredo de exemplo comprometedor no repositório.
