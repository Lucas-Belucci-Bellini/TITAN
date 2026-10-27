# Arquitetura — Ferrovia

## Fluxo de uma requisição

```text
Cliente HTTP
    ↓
Axum Router / handlers
    ↓
Estado compartilhado da aplicação
    ↓
Serviços de domínio (a criar)
    ↓
Banco, APIs externas ou eventos (a integrar)
```

## Crates

### `ferrovia-core`

Contém configuração, `EventBus`, `AppEvent` e contratos do registro de plugins. Não conhece rotas HTTP. A configuração vem de variáveis de ambiente com defaults para desenvolvimento local.

### `ferrovia-server`

Biblioteca e binário. A biblioteca expõe `app(&ServerConfig)`, que monta o router com a pilha de middleware, e `middleware::apply`, para aplicar os mesmos limites a rotas próprias. Também define `ApiError` (corpo de erro estável) e o extrator `ApiJson`. O binário (`main.rs`) só lê a configuração, abre a porta e cuida do encerramento. As rotas atuais são exemplos, não convenções definitivas.

Ordem do middleware, de fora para dentro: request ID → tracing → propagação do request ID → timeout (408 em JSON) → CORS (se configurado) → limite de body → handler.

## Decisões arquiteturais

- Axum para rotas e handlers; Tokio para runtime async.
- `tower`/`tower-http` fornecem o middleware comum (request ID, tracing, timeout, CORS); compressão fica para depois. Nada de middleware genérico reimplementado.
- Eventos iniciais usam broadcast em memória; isso não é uma fila persistente e pode perder mensagens para consumidores que não estavam ativos.
- Plugins são compilados estaticamente na versão inicial; não há carregamento de DLLs, dylibs nem execução de scripts.
- Estado de aplicação deve ser injetado em handlers; segredos e configuração não devem ser globais mutáveis.

## Camadas futuras

- `ferrovia-config`: configuração tipada e validação completa.
- `ferrovia-http`: configuração comum de router e middleware.
- `ferrovia-observability`: tracing, métricas e health/readiness.
- `ferrovia-db`: integrações opcionais por feature.
- `ferrovia-auth`: autenticação/autorização como módulos separados e revisados.
- `ferrovia-openapi`: geração de especificação e documentação.
- `ferrovia-cli`: criação de projeto, migrations e comandos de operação.

## Segurança e confiabilidade

1. Configuração deve falhar rápido para parâmetros inválidos.
2. Definir timeouts e limites de body antes de disponibilizar publicamente.
3. Não assumir que CORS substitui autenticação.
4. Usar consultas parametrizadas e limites de conexão em integrações com banco.
5. Redigir política de erros sem expor detalhes internos ao cliente.
6. Ter testes de integração para rotas, validação e encerramento.
7. Revisar dependências e avisos de segurança antes de cada release.
