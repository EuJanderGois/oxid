---
title: Metadados da API e documentação gerada
slug: /scripting/api-generation
---

# Metadados da API e documentação gerada

O Oxid descreve sua API pública de scripting uma vez, em metadata Rust próxima do binding nativo. Essa metadata alimenta tanto o `oxid.d.ts` quanto a referência web.

```text
ModuleMeta / FunctionMeta / TypeMeta
             │
       ┌─────┴─────┐
       ▼           ▼
  oxid.d.ts    páginas da API
```

A referência gerada cuida da estrutura da API. As páginas manuais continuam responsáveis por explicar semântica, ciclo de vida, unidades, cache e decisões arquiteturais.

Para regenerar:

```bash
oxid docs
```
