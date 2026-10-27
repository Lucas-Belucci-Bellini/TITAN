# ADR-0003 — Plugins estáticos antes de plugins dinâmicos

- **Status:** aceito
- **Data:** 2026-10-22

## Decisão

A primeira versão usa o trait `Plugin` e registro em tempo de compilação. O registro identifica plugins e capacidades e rejeita nomes duplicados.

## Motivo

Carregar código binário dinâmico aumenta a complexidade de ABI, segurança, versões e encerramento de recursos. O caso de uso não está definido o suficiente para assumir esse custo.

## Reavaliação

Só considerar plugins dinâmicos com requisitos concretos de isolamento, compatibilidade de ABI e modelo de confiança.
