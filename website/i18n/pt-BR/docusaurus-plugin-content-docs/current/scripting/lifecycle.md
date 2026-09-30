---
title: Ciclo de vida
slug: /scripting/lifecycle
---

# Ciclo de vida

O modelo de scripting do Oxid é orientado a frames. O `main()` fornece uma `Entity` e o runtime chama seus métodos de ciclo de vida.

- `onInit()` executa uma vez.
- `onUpdate(dt)` executa a cada frame para simulação e estado.
- `onDraw()` executa a cada frame para produzir comandos de renderização.

As APIs de desenho alimentam uma fila ativa durante `onDraw()`. O renderer consome essa fila depois que a fase de scripting termina.
