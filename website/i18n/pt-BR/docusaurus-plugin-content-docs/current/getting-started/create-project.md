---
title: Criar um projeto
slug: /getting-started/create-project
---

# Criar um projeto

Crie um projeto com:

```bash
oxid new meu-jogo
```

O scaffold atual inclui `package.json`, `main.js`, `oxid.d.ts` e `tsconfig.json`. O `package.json` concentra a configuração do runtime no campo `oxid`, incluindo `entry`, `title`, `width`, `height` e `locale`.

`oxid.d.ts` é gerado a partir dos metadados da API nativa. Ele existe para autocomplete e verificação no editor; o runtime continua sendo JavaScript.
