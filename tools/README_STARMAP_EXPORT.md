# Exportar Starmap a JSON

El binario `uqm-debug` soporta un flag para exportar TODO el starmap (estrellas, planetas y lunas) a JSON reutilizando el generador interno del juego. Esto garantiza que lo exportado coincide con lo que verías en juego.

## Uso rápido

```bash
./sc2/uqm-debug --dumpstarmap starmap.json --safe
```

## Esquema del JSON

- Objeto raíz: `{ "stars": [ ... ] }`
- Estrella:
  - `x`, `y`: coordenadas HyperSpace enteras
  - `type`: "dwarf" | "giant" | "super giant"
  - `color`: "blue" | "green" | "orange" | "red" | "white" | "yellow"
  - `presence`: cadena descriptiva del índice especial (puede ser "")
  - `prefix`, `postfix`: índices de nombre (prefijo Alpha/Beta..., sufijo constelación)
  - `planets`: arreglo de planetas
- Planeta:
  - `name`: nombre generado según reglas del juego
  - `type`: tipo del planeta (ej. "Water World", "Treasure World", etc.)
  - `shielded`: booleano
  - `radius`: distancia al sol (aprox via PlanetToSunDist)
  - `moons`: arreglo de lunas
- Luna:
  - `type`: tipo de luna (o "StarBase"/"Sa-Matra" en casos especiales)
  - `shielded`: booleano

## Notas

- El volcado se realiza temprano en la ejecución y el programa termina con código 0.
- Usa `--safe` para minimizar efectos de configuración local.
- La generación invoca los mismos generadores de contenido que el juego, por lo que incluye modificaciones y sistemas especiales.
