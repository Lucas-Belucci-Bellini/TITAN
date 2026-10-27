# ADR-0001 — Axum e Tokio para HTTP assíncrono

- **Status:** aceito
- **Data:** 2026-10-22

## Contexto

O projeto precisa de roteamento HTTP confiável sem gastar as primeiras fases reinventando o protocolo, as conexões e o runtime assíncrono.

## Decisão

Usar Axum para roteamento e handlers, com Tokio como runtime. Considerar Tower/Tower HTTP para middleware comum.

## Consequências

- Reutiliza componentes e padrões conhecidos do ecossistema Rust.
- O framework adiciona suas convenções por cima desses componentes, não os substitui.
- Atualizações de dependências e compatibilidade devem ser testadas pelo CI.
