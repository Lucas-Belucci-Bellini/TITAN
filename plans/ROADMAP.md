# Roadmap — Ferrovia

## Fase 0 — Fundação (agora)

- [x] Workspace e divisão core/server.
- [x] Configuração por ambiente e validação da porta.
- [x] Event bus em memória.
- [x] Registro estático de plugins.
- [x] Rotas de exemplo, logging e Ctrl+C.
- [x] Rodar `cargo test --workspace` e validar respostas HTTP em ambiente com Rust.
- [ ] Adicionar CI (fmt, clippy, test). Os comandos já passam localmente; falta o pipeline (pausado enquanto a conta do GitHub está em revisão).

**Saída:** skeleton de API reproduzível.

## Fase 1 — Contratos HTTP

- [x] Modelo padronizado de erro JSON.
- [x] Validação de payloads e exemplos de DTO.
- [x] Testes de integração para status codes e formato das respostas.
- [x] Middleware de request ID, timeout, limite de body e tracing.
- [x] Política CORS opcional, com origens explícitas.

**Saída:** API robusta em ambiente de desenvolvimento.

## Fase 2 — Extensibilidade real

- [ ] Contrato estável para registrar rotas e estado a partir de plugins.
- [ ] Ordem de inicialização e encerramento de plugins.
- [ ] Configuração de capacidades e conflito de nomes.
- [ ] Testes de plug-in falhando parcialmente e rollback da inicialização.

**Saída:** plugins estáticos úteis sem dependência circular.

## Fase 3 — Dados e domínio

- [ ] Trait de serviço/repositório para separar domínio de HTTP.
- [ ] Escolher integração inicial com banco com base no uso real.
- [ ] Pool de conexões, migrations e consultas parametrizadas.
- [ ] Transações e testes com banco efêmero.

**Saída:** API CRUD exemplo com persistência.

## Fase 4 — Segurança e observabilidade

- [ ] Modelo de autenticação e autorização separado.
- [ ] Rate limiting e limites de concorrência.
- [ ] Métricas, readiness e liveness.
- [ ] OpenAPI/documentação de endpoints.
- [ ] Testes de carga e threat modeling.

**Saída:** release candidata para aplicações de baixo risco, após revisão.

## Fase 5 — Ferramentas e versão 1.0

- [ ] CLI para criar módulos e serviços.
- [ ] Política de compatibilidade semântica.
- [ ] Guia de upgrade e exemplos de produção.
- [ ] Auditoria de dependências e checklist operacional.
- [ ] Publicar `1.0` apenas após critérios de segurança, testes e estabilidade de API.
