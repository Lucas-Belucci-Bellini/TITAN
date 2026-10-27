# ADR-0004 — Nome definitivo: Ferrovia

- **Status:** aceito
- **Data:** 2026-10-25

## Decisão

O framework passa a se chamar **Ferrovia**, substituindo o nome de trabalho "RustBack". Crates: `ferrovia-core` e `ferrovia-server`. Variáveis de ambiente: prefixo `FERROVIA_`.

## Motivo

- *Ferro* remete a Rust (ferrugem) e *via* a rotas, que é o núcleo do framework.
- Palavra em português, curta e fácil de pronunciar; combina com a documentação em português.
- "RustBack" descrevia a tecnologia, não o produto, e soa genérico.
- Em 2026-10-25, `cargo search` não encontrou crate chamada `ferrovia` nem `ferrovia-core` no crates.io. Isso não reserva o nome: a disponibilidade só é garantida ao publicar.

## Consequências

- Os IDs do backlog (`RB-001`…) foram mantidos para não quebrar referências.
- A pasta do repositório local ainda se chama `rustback-framework`; renomear a pasta ou o repositório remoto é uma ação separada.
