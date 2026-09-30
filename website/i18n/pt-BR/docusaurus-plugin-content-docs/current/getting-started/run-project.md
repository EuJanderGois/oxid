---
title: Executar um projeto
slug: /getting-started/run-project
---

# Executar um projeto

Dentro do projeto:

```bash
oxid run
```

Também é possível informar outro diretório:

```bash
oxid run caminho/para/meu-jogo
```

O Oxid lê `package.json`, carrega a configuração `oxid`, resolve `oxid.entry`, inicializa o runtime JavaScript e abre a janela nativa.

As mensagens do CLI usam esta ordem de locale: `--lang`, `OXID_LANG`, `oxid.locale` e, por fim, o locale padrão. Isso afeta as mensagens do Oxid, não os textos do seu jogo.
