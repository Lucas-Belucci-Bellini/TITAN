# TITAN — framework de backend em Rust

> Versão `0.1.0`: **esqueleto funcional para APIs HTTP**, ainda não pronto para produção.

TITAN é o framework de backend da família para criar APIs e serviços em Rust, com roteamento, configuração, eventos e pontos de extensão para plugins. O motor HTTP é Axum sobre Tokio, com middleware do Tower, em vez de reimplementar protocolo HTTP e runtime assíncrono.

Os nomes oficiais da API pública incluem `TitanServerConfig`, `TitanEventBus`, `TitanPluginRegistry`, `TitanAppState` e os tipos `TitanApi*`. Os nomes genéricos e aliases Ferrovia continuam disponíveis por compatibilidade. Os pacotes `ferrovia-*` e variáveis `FERROVIA_*` não são renomeados nesta etapa.

## Objetivos

- Fornecer uma base pequena para serviços HTTP e APIs JSON.
- Separar configuração, roteamento, eventos e sistema de plugins.
- Facilitar testes, observabilidade e evolução modular.
- Não acoplar a lógica de domínio ao servidor HTTP.

## O que já existe

- Workspace Cargo com `ferrovia-core` (sem dependência de HTTP) e `ferrovia-server` (biblioteca + binário).
- Configuração por variáveis de ambiente, validada na inicialização: valor inválido impede o servidor de subir.
- Corpo de erro JSON padronizado para 404, 405, 408, 413, 415, 422 e 500.
- Extrator `ApiJson<T>` que converte erros de JSON para o formato padrão, e um DTO de exemplo com validação.
- Middleware: request ID (`x-request-id`), tracing por requisição, timeout, limite de body e CORS opcional só com origens explícitas.
- Event bus assíncrono baseado em `tokio::sync::broadcast`.
- Registro estático de plugins, com detecção de nomes duplicados e capacidades.
- Encerramento gracioso por Ctrl+C (e SIGTERM em Unix).
- Testes unitários e de integração HTTP para o core, os contratos de API e o middleware.
- Smoke test de processo para encerramento gracioso: o CI inicia o binário do servidor, aguarda o endpoint de saúde, envia SIGTERM e exige saída com código zero dentro do prazo, além da mensagem de shutdown esperada. Esse cenário é uma verificação de ciclo de vida em Linux, não um benchmark de carga nem uma garantia de resiliência em produção.
- Plano de produto, arquitetura, roadmap e ADRs em `plans/`.

## O que ainda não existe

Não inclui banco de dados, autenticação, autorização, migrações, limite de taxa, OpenAPI, métricas Prometheus, plugins que registram rotas, plugins dinâmicos nem política de compatibilidade 1.0. Esses recursos não devem ser presumidos seguros para produção.

## Requisitos

- Rust 1.89 ou mais recente, instalado via [rustup](https://rustup.rs/).

## Executar

Na raiz do projeto:

```bash
cargo test --workspace
cargo run -p ferrovia-server
```

Por padrão, o servidor abre `127.0.0.1:3000`.

## Configuração

| Variável | Padrão | Descrição |
| --- | --- | --- |
| `FERROVIA_HOST` | `127.0.0.1` | Endereço de escuta. |
| `FERROVIA_PORT` | `3000` | Porta (0–65535). |
| `FERROVIA_ENV` | `development` | Nome do ambiente, exposto em `/api/hello`. |
| `FERROVIA_REQUEST_TIMEOUT_SECS` | `30` | Tempo máximo por requisição; precisa ser maior que zero. |
| `FERROVIA_BODY_LIMIT_BYTES` | `1048576` | Tamanho máximo do corpo; precisa ser maior que zero. |
| `FERROVIA_CORS_ORIGINS` | vazio | Origens separadas por vírgula, ex.: `https://app.exemplo.com`. Vazio desativa CORS; `*` é recusado. |

**PowerShell**

```powershell
$env:FERROVIA_PORT = "8080"
$env:FERROVIA_CORS_ORIGINS = "http://localhost:5173"
cargo run -p ferrovia-server
```

O nível de log segue `RUST_LOG` (padrão `info`).

## Rotas de exemplo

```bash
curl http://127.0.0.1:3000/health
curl http://127.0.0.1:3000/api/hello
curl -X POST http://127.0.0.1:3000/api/greetings -H "content-type: application/json" -d "{\"name\":\"Lucas\"}"
```

| Rota | Resposta |
| --- | --- |
| `GET /health` | `{"status":"ok","service":"ferrovia"}` |
| `GET /api/hello` | `{"message":"Olá da Ferrovia!","environment":"development"}` |
| `GET /api/v1/hello` | Versioned success envelope containing `data.message`, `data.environment` and `meta.api_version = "v1"`; shared ATLAS/AETHER contract smoke endpoint. |
| `POST /api/greetings` | `{"message":"Olá, Lucas!"}` — `name` é obrigatório, de 1 a 64 caracteres, sem caracteres de controle; campos extras são recusados. |

## Formato de erro

Todo erro devolvido pelo framework tem o mesmo formato:

```json
{"error":{"code":"validation_failed","message":"name não pode ser vazio"}}
```

`code` é fixo e serve para o cliente tratar o erro por programa: `not_found`, `method_not_allowed`, `invalid_body`, `unsupported_media_type`, `payload_too_large`, `validation_failed`, `request_timeout` ou `internal_error`. `message` é texto para pessoas e nunca carrega stack trace nem detalhe interno.

## Usar como biblioteca

`ferrovia_server::app(&config)` devolve o `Router` completo. Para rotas próprias com os mesmos limites, aplique a pilha padrão:

```rust
use axum::{routing::get, Router};
use ferrovia_core::TitanServerConfig;

let config = TitanServerConfig::from_env()?;
let rotas = Router::new().route("/ping", get(|| async { "pong" }));
let app = ferrovia_server::middleware::apply(rotas, &config)?;
```

## Estrutura

```text
crates/
  ferrovia-core/          # configuração, eventos e contratos de plugin (sem HTTP)
  ferrovia-server/
    src/lib.rs            # app(), TitanAppState (alias compatível de AppState)
    src/api.rs            # rotas, DTOs e ApiJson
    src/error.rs          # ApiError e formato de erro
    src/middleware.rs     # request ID, tracing, timeout, body limit, CORS
    src/main.rs           # ponto de entrada executável
    tests/http.rs         # testes de integração HTTP
plans/                    # produto, arquitetura, roadmap e ADRs
```

## Qualidade

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Segurança antes de produção

- Não registrar tokens, cookies, senhas ou payloads sensíveis. Os logs de requisição registram só método, path e request ID, nunca a query string.
- Usar TLS no edge/proxy ou configurar TLS conforme o ambiente.
- Ajustar `FERROVIA_BODY_LIMIT_BYTES` e `FERROVIA_REQUEST_TIMEOUT_SECS` ao caso de uso.
- CORS não substitui autenticação. Implementar autenticação e autorização de forma explícita.
- Configurar limites de conexão e rate limiting.
- Executar testes, análise de dependências e revisão de ameaças.

## Referências técnicas

- [`axum`](https://docs.rs/axum/latest/axum/)
- [`tokio`](https://docs.rs/tokio/latest/tokio/)
- [`tower-http`](https://docs.rs/tower-http/latest/tower_http/)

## Licença

MIT. Ver `LICENSE`.
