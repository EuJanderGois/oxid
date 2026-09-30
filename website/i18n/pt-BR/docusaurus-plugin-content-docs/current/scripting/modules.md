---
title: Módulos
slug: /scripting/modules
---

# Módulos

Oxid usa a sintaxe ES module padrão. APIs nativas usam o namespace `oxid/*`; código do projeto usa caminhos de arquivo.

```js
import { Vector2D } from "oxid/math";
import { drawCircle } from "oxid/shapes";
```

Imports relativos ficam dentro da raiz do projeto. O resolver rejeita caminhos que escapem dessa raiz.

Os módulos nativos são registrados pelo registry de plugins e suas declarações também entram no `oxid.d.ts`.
