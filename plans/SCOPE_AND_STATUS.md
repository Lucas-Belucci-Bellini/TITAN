# Escopo e status

TITAN é um framework experimental `0.1.0` para APIs HTTP. Não é uma release 1.0 e não deve ser considerado pronto para exposição pública sem autenticação/autorização, rate limiting e revisão de segurança apropriados ao deployment.

## Status verificado em 2026-10-30

- CI Rust: formatação, workspace check, testes e Clippy.
- MSRV declarada `rust-version = "1.89"` verificada em workflow próprio.
- Auditoria de dependências RustSec integrada ao CI.
- Smoke test de processo em Linux: inicia o binário, aguarda o endpoint de saúde, envia SIGTERM e exige saída com código zero dentro do prazo e mensagem esperada de shutdown.
- Testes de integração HTTP e contrato de sucesso versionado em `/api/v1/hello`.
- Workflow de integração da família executa contrato HTTP contra o servidor TITAN.

Os resultados devem ser conferidos pelo SHA da execução. O smoke test de shutdown é limitado ao cenário de processo descrito; não é teste de carga ou prova de resiliência de produção.

## Implementado

- Core e servidor separados.
- Configuração validada, roteamento e middleware.
- Envelope de erro JSON e validação de payload.
- Event bus em memória e registro estático de plugins.
- Encerramento gracioso por Ctrl+C e SIGTERM em Unix.

## Ainda não implementado

- Plugins que registram rotas/serviços com lifecycle completo.
- Persistência, pool de conexões e migrations.
- Autenticação/autorização e rate limiting.
- Métricas Prometheus, OpenAPI e política de estabilidade 1.0.

## Prioridades atuais

1. Definir contrato de plugins com ciclo de vida e rollback.
2. Concluir revisão de segurança antes de qualquer alegação de produção.
3. Avaliar persistência e observabilidade conforme casos de uso reais.
