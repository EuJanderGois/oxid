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
- Input nativo exposto via **`oxid/input`**
- Texto 2D exposto via **`oxid/text`**
- Texturas 2D expostas via **`oxid/texture`**
- Arquitetura modular baseada em **Componentes**
- CLI simples para criação e build de projetos
- Binários **nativos e standalone**
- Multiplataforma

---
## Olá Mundo

``` javascript
import { GameObject } from "oxid/core";
import { Transform2D } from "oxid/math";
import { isKeyDown, isMouseButtonDown, mousePosition } from "oxid/input";
import { drawCircle, drawRectangle, drawArc } from "oxid/shapes";
import { drawText, drawMultilineText, measureText } from "oxid/text";
import { loadTexture, drawTextureScaled } from "oxid/texture";
import { Color } from "oxid/color";

const RED = new Color(1.0, 0.0, 0.0, 1.0);
const WHITE = new Color(1.0, 1.0, 1.0, 1.0);
const LABEL = "PLAYER";

export class MyApp extends GameObject {
	constructor() {
		super();
		this.jogador = new Transform2D(100.0, 100.0);
		this.arco = new Transform2D(80.0, 80.0);
		this.ui = new Transform2D(20.0, 30.0);
		this.mascotePos = new Transform2D(320.0, 120.0);
		this.mascoteSize = new Transform2D(160.0, 160.0);
		this.mascoteRotacao = 0.0;
		this.velocidade = 180.0;
	}

	onInit() {
		this.mascote = loadTexture("src/assets/oxil.png");
	}

	onUpdate(dt) {
		const step = this.velocidade * dt;

		if (isKeyDown("ArrowRight") || isKeyDown("D")) this.jogador.x += step;
		if (isKeyDown("ArrowLeft") || isKeyDown("A")) this.jogador.x -= step;
		if (isKeyDown("ArrowDown") || isKeyDown("S")) this.jogador.y += step;
		if (isKeyDown("ArrowUp") || isKeyDown("W")) this.jogador.y -= step;

		this.arco = mousePosition();
		this.mascoteRotacao += dt * 0.6;
	}

	onDraw() {
		const labelMetrics = measureText(LABEL, 24.0);
		const labelPosition = new Transform2D(
			this.jogador.x - labelMetrics.width / 2.0,
			this.jogador.y - 35.0,
		);

		drawMultilineText(
			"WASD/setas movem.\nSegure o mouse esquerdo para desenhar o arco.\nO mascote usa oxid/texture.",
			this.ui,
			24.0,
			WHITE,
			1.2,
		);
		drawText(LABEL, labelPosition, 24.0, WHITE);
		drawCircle(this.jogador.x, this.jogador.y, 25.0, RED);
		drawRectangle(50.0, 50.0, 100.0, 20.0, RED);
		drawTextureScaled(
			this.mascote,
			this.mascotePos,
			this.mascoteSize,
			this.mascoteRotacao,
		);

		if (isMouseButtonDown("left")) {
			drawArc(this.arco, 64, 40.0, 0.0, 8.0, 140.0, RED);
		}
	}
}

export function main() {
	return new MyApp();
}
```

### Input API

O módulo `oxid/input` aceita nomes de teclas como `string`.

- Ignora maiúsculas/minúsculas, espaços, `_` e `-`
- Exemplos válidos: `"A"`, `"ArrowLeft"`, `"Space"`, `"Enter"`, `"Escape"`, `"LeftShift"`, `"F1"`
- Botões do mouse aceitos: `"left"`, `"middle"` e `"right"`

### Text API

O módulo `oxid/text` expõe:

- `drawText(text, position, fontSize, color)`
- `drawMultilineText(text, position, fontSize, color, lineDistance?)`
- `measureText(text, fontSize)`

- `position` usa `Transform2D`
- `fontSize` é definido em pixels
- a coordenada `y` representa a baseline do texto
- `measureText` mede uma única linha e retorna `TextMetrics` com `width`, `height` e `offset_y`

### Texture API

O módulo `oxid/texture` expõe:

- `loadTexture(path)`
- `drawTexture(texture, position)`
- `drawTextureScaled(texture, position, size, rotation?)`

- `loadTexture` carrega a imagem uma vez e retorna um objeto `Texture2D`
- `Texture2D` expõe `path`, `width` e `height`
- `position` e `size` usam `Transform2D`
- `rotation` usa radianos
- caminhos relativos usam o diretório de trabalho atual

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
