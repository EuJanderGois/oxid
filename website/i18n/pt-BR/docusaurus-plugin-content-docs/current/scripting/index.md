---
title: Scripting
slug: /scripting
---

# Scripting

Oxid usa JavaScript para a lógica do jogo e expõe funcionalidades nativas por imports ES module.

## Modelo de execução

```text
main.js → módulos JavaScript → Entity
                              ├─ onInit()
                              ├─ onUpdate(dt)
                              └─ onDraw() → fila de render → renderer
```

Leia nesta ordem: [Ciclo de vida](./lifecycle), [Módulos](./modules), [Entity](./entity), [Tipos](./types) e [Metadados da API](./api-generation).
