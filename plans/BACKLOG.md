# Backlog inicial — Ferrovia

As tarefas abaixo já podem virar issues. Marque-as concluídas apenas depois de executar os critérios de aceite. Os IDs `RB-` foram mantidos do nome de trabalho anterior para não quebrar referências.

## RB-001 — CI e qualidade
**Prioridade:** P0 · **Status:** parcial

- Configurar `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings` e `cargo test --workspace`.
- Registrar versões suportadas do Rust.

**Aceite:** pipeline executa para cada pull request e não ignora falhas.

**Feito:** os três comandos passam localmente; `rust-version = "1.89"` declarado no workspace (piso das dependências atuais; validado só com 1.99). **Falta:** o workflow de CI, adiado enquanto a conta do GitHub está em revisão.

## RB-002 — Testes de integração HTTP
**Prioridade:** P0 · **Status:** concluído (`crates/ferrovia-server/tests/http.rs`)

- Testar status, headers e payloads de `/health` e `/api/hello`.
- Extrair a criação do router para permitir testes sem abrir porta real.

**Aceite:** testes verificam respostas e formato JSON sem depender de um servidor manual.

## RB-003 — Erros padronizados e validação
**Prioridade:** P1 · **Status:** concluído (`ApiError`, `ApiJson`, `POST /api/greetings`)

- Definir corpo de erro estável e mapeamento de erros de domínio para HTTP.
- Adicionar DTOs de exemplo e validação de entrada.

**Aceite:** payload inválido retorna 4xx sem expor stack traces ou detalhes internos.

## RB-004 — Middleware básico
**Prioridade:** P1 · **Status:** concluído (`crates/ferrovia-server/src/middleware.rs`)

- Avaliar Tower HTTP para request ID, tracing, timeout, limite de body e CORS opcional.
- Definir defaults seguros e permitir sobrescrita explícita.

**Aceite:** middleware tem testes de integração e a política CORS não permite origens arbitrárias por padrão.

## RB-005 — Registro de plugins com rotas
**Prioridade:** P1

- Desenhar ciclo de vida `configure`/`start`/`shutdown`.
- Permitir que plugins estáticos registrem rotas e serviços sem criar dependência circular.
- Definir comportamento quando um plugin falhar parcialmente.

**Aceite:** plugin de demonstração registra uma rota, e erros de inicialização são reportados com contexto.

## RB-006 — Persistência opcional
**Prioridade:** P2

- Escolher banco por necessidade real e comparar as opções de driver/ORM.
- Definir repositórios, migrations, pool e testes de integração.

**Aceite:** módulo de exemplo persiste dados com consultas parametrizadas e testes reproduzíveis.

## RB-007 — Revisão de segurança de pré-produção
**Prioridade:** P1 antes de produção

- Definir autenticação/autorização, limites de taxa e política de segredos.
- Rever logs, timeouts, limites de body, CORS e dependências.
- Adicionar threat model e checklist de deploy.

**Aceite:** checklist documentado, testes pertinentes e nenhum segredo de exemplo comprometedor no repositório.
