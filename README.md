# Oxid Framework

> Oxid — Build fast games with Rust performance and JavaScript simplicity

**Oxid** é um framework moderno para desenvolvimento de jogos que combina a **performance de Rust** com a **produtividade de JavaScript**.

Ele fornece uma arquitetura simples baseada em **componentes e sistemas**, permitindo organizar lógica de jogo, atualização e renderização de forma clara.

Multiplataforma, rápido e eficiente, **Oxid** foi projetado para oferecer uma experiência de desenvolvimento fluida sem sacrificar performance.

---

## Features

- Engine escrita em **Rust**
- Scripting em **JavaScript / TypeScript**
- Renderização baseada em **Macroquad**
- Arquitetura modular baseada em **Componentes**
- CLI simples para criação e build de projetos
- Binários **nativos e standalone**
- Multiplataforma

---
## Olá Mundo

``` javascript
import { Component } from "oxid";
import { Transform2D } from "oxid/math";
import { drawCircle } from "oxid/graphics";

export class MyApp extends Component {
	constructor() {
		super();
		this.jogador = new Transform2D(100.0, 100.0);
		this.velocidade = 120.0;
	}

	update(dt) {
		this.jogador.x += this.velocidade * dt;

		if (this.jogador.x > 800.0) {
			this.jogador.x = 0;
		}
	}
	
	draw() {
		drawCircle(this.jogador.x, this.jogador.y, 25.0);
	}
}
```

---
## Por que usar Oxid?

### Simplicidade

Depois de instalado, criar um novo projeto é extremamente simples:

```bash
oxid new my-project
cd my-project
oxid run
```

Este comando cria toda a estrutura necessária para começar.

Para gerar um build de distribuição:

```bash
oxid build --release
```

## Arquitetura moderna

**Oxid** utiliza **JavaScript** como linguagem de scripting, permitindo desenvolvimento rápido e flexível.

Você também pode usar **TypeScript** ou qualquer outra linguagem capaz de compilar para **JavaScript (ES6)**.

Apesar disso, **Oxid não é uma engine de navegador.**

O núcleo da engine é escrito em **Rust**, oferecendo alto desempenho e controle de baixo nível.

A comunicação com a camada de renderização é feita através de uma **API robusta baseada em Macroquad**, garantindo performance e simplicidade.

O resultado é um sistema que:

- executa **fora do navegador**
- compila para **binários nativos**
- gera **aplicações leves e standalone**
