---
title: Oxid
slug: /intro
---

# Oxid

**Um pequeno motor de jogos em Rust com scripting de gameplay em JavaScript.**

Oxid foi construído em torno de um ciclo curto de desenvolvimento: criar um projeto, escrever JavaScript e executar. O lado nativo cuida da janela, runtime, fronteira de renderização e módulos integrados; o projeto cuida da lógica do jogo.

## O modelo mental

```text
Seu jogo
  ├─ package.json       configuração
  ├─ main.js            ponto de entrada
  ├─ src/...            seus módulos
  └─ assets/...         seus recursos
          │
          ▼
      runtime Oxid
  ┌───────────────────┐
  │ carregador projeto│
  │ runtime JS        │
  │ módulos nativos   │
  │ ciclo de frames   │
  │ fila de render    │
  │ renderer          │
  └───────────────────┘
```

A fronteira importante é: **o código do jogo conversa com o Oxid pela API de scripting**. Você não precisa conhecer QuickJS ou Macroquad para usar o motor. Esses detalhes só são necessários quando você estiver depurando ou estendendo o próprio Oxid.

## Comece aqui

[**Aprender Oxid →**](./learn)

Se este é seu primeiro projeto, siga [Instalação](./getting-started/installation) e depois [Criar um projeto](./getting-started/create-project).

## Camadas da documentação

| Camada | Pergunta que responde |
| --- | --- |
| **Aprender** | O que faço agora? |
| **Scripting** | Como o código do jogo executa e conversa com o motor? |
| **Referência da API** | Quais funções, tipos e globais existem? |
| **Arquitetura** | O que acontece dentro do Oxid? |
| **Informações técnicas** | Como contribuo ou adiciono funcionalidades nativas? |
