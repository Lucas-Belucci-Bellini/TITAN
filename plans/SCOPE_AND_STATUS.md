# Escopo e status

Este pacote entrega o planejamento, um core pequeno e uma API de demonstração com contratos HTTP básicos. Não é uma release 1.0 e não deve ser exposto à internet sem completar a revisão de segurança (RB-007).

## Validado em 2026-10-25 (Rust 1.99, Windows)

- `cargo fmt --all -- --check`: sem diferenças.
- `cargo clippy --workspace --all-targets -- -D warnings`: sem avisos.
- `cargo test --workspace`: 22 testes passando (7 no core, 15 de integração HTTP).

## Prioridade atual

1. RB-001: workflow de CI. Adiado enquanto a conta do GitHub está em revisão; os comandos já passam localmente.
2. RB-005: plugins estáticos que registram rotas, com ciclo de vida `configure`/`start`/`shutdown`.
3. RB-007: revisão de segurança de pré-produção (autenticação, rate limiting, threat model).
