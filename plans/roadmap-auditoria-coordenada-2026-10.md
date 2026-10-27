# Roadmap técnico — TITAN (auditoria coordenada, outubro de 2026)

> Estado desta revisão: inspeção estática de manifests, README e fontes principais via GitHub. **Nenhum comando de teste/build foi executado nesta auditoria.**
>
> Branch de trabalho: `roadmap/coordinated-audit-2026-10`. A branch `main` não foi alterada.

## 1. Papel do TITAN no ecossistema

TITAN é o framework backend HTTP de base: configuração, roteamento, middleware e contratos de extensão. O código e README usam o nome interno Ferrovia e crates `ferrovia-*`. Preservar essa identidade até uma decisão explícita. TITAN deve permanecer utilizável sozinho; AETHER pode oferecer uma camada mais integrada de módulos/tarefas/IA, sem criar dependência circular ou duplicar sem propósito.

## 2. Estado de código observado

- Workspace Cargo resolver 2, edition 2021, licença MIT, `rust-version = "1.89"`; `Cargo.lock` existe.
- `ferrovia-core`: configuração de servidor, EventBus baseado em `tokio::sync::broadcast` e registry estático de plugins/capabilities, sem HTTP.
- `ferrovia-server`: biblioteca e binário; `app(config)` compõe router de exemplo e middleware padrão.
- Rotas atuais em `api.rs`: `GET /health`, `GET /api/hello`, `POST /api/greetings`; DTO de greeting recusa campos desconhecidos e valida nome vazio, caracteres de controle e limite de 64 caracteres.
- Middleware observado: request ID, tracing sem query string, timeout, limite de body e CORS opcional por origens explícitas.
- O binário lê configuração do ambiente e encerra graciosamente com Ctrl+C e SIGTERM em Unix.
- O plugin registry é estático; o trait permite registro de capabilities, mas não foi observado contrato para plugins adicionarem rotas nem carregamento dinâmico.
- O EventBus é volátil e em memória. O README explicitamente exclui banco, auth/authz, migrações, rate limiting, OpenAPI e métricas Prometheus.
- Os caminhos `.github/workflows/ci.yml` e `.github/workflows/rust.yml` não foram encontrados; confirmar se há workflows com outros nomes.
- O README declara 22 testes, mas essa contagem não foi reexecutada nesta revisão.

## 3. Lacunas e riscos para validar

1. Rodar baseline de formatação, testes, clippy e build locked; verificar a contagem real de testes.
2. Validar erros para 404/405/408/413/415/422/500 e garantir que middleware/extractors mantêm o mesmo envelope JSON.
3. Auditar CORS para origens inválidas, métodos/headers necessários e comportamento quando vazio; CORS não é mecanismo de autenticação.
4. Rever a confiança no `x-request-id` fornecido pelo cliente: preservar pode ser útil para tracing distribuído, mas validar comprimento/caracteres e evitar spoofing/abuso.
5. Rate limiting, limites de conexão e backpressure ainda não foram identificados na implementação.
6. Não existe autenticação/autorização no estado descrito; não expor endpoints de domínio como prontos para produção.
7. Definir contratos reais de plugin: ciclo de vida, falhas, capacidades, contribuição de rotas e isolamento; evitar sugerir que registry estático equivale a sandbox.
8. Definir garantias do EventBus, incluindo eventos perdidos quando não há subscribers ou quando buffer transborda.
9. Verificar se a config expõe ambiente em respostas públicas; decidir se `/api/hello` é apenas demo.
10. Documentar deployment/TLS e política de compatibilidade antes de chamar o framework de production-ready.

## 4. Roadmap por fases

### Fase A — Baseline e CI (P0)
- Rodar `cargo fmt --all -- --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo check --workspace --locked`.
- Testar com Rust 1.89 e toolchain stable; corrigir/documentar o MSRV conforme resultado real.
- Configurar CI para pull requests e pushes na branch principal, com permissões mínimas e cache seguro.
- Validar os caminhos de documentação e exemplos de curl/PowerShell.

**Aceite:** baseline e logs anexáveis; MSRV reproduzível; CI cobre fmt, clippy, check e testes.

### Fase B — Contratos HTTP e segurança (P1)
- Criar testes de integração para todos os códigos de erro documentados, JSON inválido, content-type incorreto, body acima do limite e timeout.
- Testar request ID gerado e recebido; documentar regra de confiança e limitar/normalizar entrada.
- Adicionar limites configuráveis de concorrência e rate limiting com estratégia clara (por IP, chave ou middleware externo), evitando confiar em headers de proxy sem configuração explícita.
- Documentar TLS como responsabilidade do proxy/edge ou planejar suporte nativo por ADR.
- Remover ambiente de respostas públicas por padrão ou deixar explícito que endpoint é somente exemplo.
- Adicionar revisão de dependências/advisories e threat model de API.

**Aceite:** testes de limites e erros; defaults seguros; nenhum segredo/stack trace em resposta; documentação sem ambiguidade sobre auth.

### Fase C — Autenticação e autorização (P1)
- Definir extension points sem impor um único provedor: middleware de identidade, principal/request context e autorização por rota/ação.
- Começar por integração documentada (por exemplo, OIDC/JWT via biblioteca madura), com validação de issuer, audience, expiração, algoritmo e rotação de chaves.
- Definir RBAC/permissions apenas se necessário ao escopo, negar por padrão e testar 401/403.
- Não implementar criptografia/tokenização manualmente.
- Criar exemplos que distingam endpoint público, autenticado e autorizado.

**Aceite:** contrato claro de identidade/claims, testes negativos, documentação de deployment e sem segredo hardcoded.

### Fase D — Observabilidade e API contract (P2)
- Adicionar métricas de requests/latência/erros e health/readiness diferenciados, sem labels de alta cardinalidade.
- Gerar OpenAPI a partir de rotas/DTOs e testar que a especificação não diverge do comportamento.
- Definir tracing propagado, formato de logs e redaction.
- Considerar migrações/DB somente como crates/adapters opcionais, sem dependência do core.

**Aceite:** métricas sem dados sensíveis, OpenAPI validado por teste e dependências opcionais.

### Fase E — Plugins de verdade (P2)
- Definir contrato de plugin estático primeiro: metadata/version, dependências/capabilities, register/start/shutdown e rollback em falha.
- Integrar rotas apenas com tipo/escopo de estado explícito e teste de conflito de rota.
- Manter carregamento dinâmico fora do escopo até haver modelo de segurança e ABI estável.
- Criar ADR comparando plugins compilados com extensões externas via processo/HTTP.

**Aceite:** plugins estáticos documentados e testados; nenhuma alegação de isolamento/sandbox sem implementação.

### Fase F — Compatibilidade e maturidade 1.0 (P3)
- Definir SemVer, depreciação, política de breaking changes e MSRV.
- Fazer benchmarks e teste de carga antes de otimizar.
- Criar matriz de deployment, shutdown, limites e recuperação.
- Revisar se recursos devem ficar no TITAN ou ser fornecidos por AETHER como camada superior.

## 5. Ordem recomendada de execução

1. Baseline + CI.
2. Contratos de erro e middleware/request ID.
3. Hardening de API e rate limiting.
4. Contratos de auth/authz.
5. Métricas/OpenAPI.
6. Plugins com ciclo de vida.
7. Compatibilidade 1.0 e benchmarks.

## 6. Integração com AETHER e ATLAS

- TITAN continua backend autônomo e não depende de AETHER.
- AETHER pode compor serviços e automações acima do TITAN, mas o contrato entre ambos deve ser documentado e opcional.
- ATLAS consome HTTP; não deve depender das crates `ferrovia-*`.
- Se compartilharem envelope de erro ou schema, prefira especificação/fixture de contrato a uma crate comum prematura.

## 7. Fora de escopo por enquanto

- Autenticação proprietária/criptografia feita à mão.
- Banco de dados obrigatório no core.
- Plugins dinâmicos ou alegações de sandbox.
- Declarar o framework pronto para produção antes de fechar auth, rate limiting, observabilidade, threat model e CI.

## 8. Critério de conclusão da auditoria

Marcar cada capacidade como confirmada no código, parcial, ausente ou não verificada. O roadmap descreve prioridades; não afirma que testes foram executados nem que recursos ausentes foram implementados.
