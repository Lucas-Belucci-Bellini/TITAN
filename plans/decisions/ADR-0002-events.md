# ADR-0002 — Barramento de eventos em memória no MVP

- **Status:** aceito provisoriamente
- **Data:** 2026-10-22

## Decisão

Usar `tokio::sync::broadcast` com eventos tipados (`topic` + `serde_json::Value`).

## Motivo

Dá aos módulos um caminho simples para publicar eventos sem impor um broker externo no primeiro exemplo.

## Limitações

- Eventos não são persistidos.
- Não há replay ou garantias de entrega após falha.
- Um publisher pode não ter receiver ativo; a chamada de publish retorna erro nesse caso.
- Não é apropriado para tarefas críticas sem desenhar persistência, backpressure e retries.
