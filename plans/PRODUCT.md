# Plano de produto — Ferrovia

## Problema

Criar uma base reutilizável para APIs e serviços em Rust que evite concentrar configuração, roteamento, eventos e extensões no `main.rs`.

## Usuários-alvo

- Desenvolvedores Rust que criam APIs HTTP JSON.
- Equipes que querem módulos de domínio com limites explícitos.
- Projetos internos que precisam de uma base extensível e testável.

## Proposta de valor

- Menos boilerplate repetido entre serviços.
- Separação do core de aplicação e transporte HTTP.
- Contratos claros para configuração, eventos e plugins.
- Integração com middleware do ecossistema Rust em vez de inventar uma pilha HTTP própria.

## MVP incluído

1. Inicializar uma API e ler configuração do ambiente.
2. Expor endpoints de saúde e exemplo.
3. Publicar eventos em memória.
4. Registrar plugins estáticos e capacidades.
5. Encerrar de forma graciosa e emitir logs.

## Fora do escopo da versão 0.1

- Banco de dados e migrations.
- Autenticação/autorizações prontas.
- Sistema de plugins dinâmicos ou executar código externo.
- Clustering, filas persistentes ou garantias de entrega de eventos.
- OpenAPI gerado e contrato 1.0 estável.

## Critérios de sucesso do MVP

- `cargo test --workspace` passa.
- Servidor sobe com configuração padrão em loopback.
- `/health` e `/api/hello` retornam JSON.
- Configuração inválida retorna erro de inicialização em vez de ser ignorada silenciosamente.
- Eventos e plugins têm testes unitários.
