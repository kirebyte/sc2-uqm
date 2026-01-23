# Sistema Estelar Kitsune

## Ubicación
- **Coordenadas HyperSpace:** 9749, 9250
- **Tipo de Estrella:** Super Gigante Azul (SUPER_GIANT_STAR, BLUE_BODY)
- **Número de Planetas:** 9
 - **Nombre visible:** "Kitsune" (prefijo 0 = sin "Alpha/Beta")
 - **Índice de nombre (postfijo en gamestrings):** 133

## Planetas del Sistema

### Planeta 0: Mundo de Agua (Water World)
- **Tipo:** WATER_WORLD (Grande)
- **Tamaño:** Big
- **Características:** Planeta con vida abundante
- **Lunas:** 1
  - **Luna 0:** Mundo Selenico pequeño con Escudo Esclavo (PLANET_SHIELDED)
  - La luna tiene un escudo de protección slave shield

### Planeta 1: Mundo Vinilogoso (Vinylogous World)
- **Tipo:** VINYLOGOUS_WORLD
- **Lunas:** 0
- **Características:** Planeta rocoso grande con compuestos orgánicos

### Planeta 2: Mundo Destrozado (Shattered World)
- **Tipo:** SHATTERED_WORLD
- **Lunas:** 0
- **Características:** Planeta rocoso grande con superficie fragmentada

### Planeta 3: Mundo Primordial (Primordial World)
- **Tipo:** PRIMORDIAL_WORLD
- **Lunas:** 0
- **Características:** Planeta rocoso grande en etapas tempranas de formación

### Planeta 4: Mundo Oolítico (Oolite World)
- **Tipo:** OOLITE_WORLD
- **Lunas:** 0
- **Características:** Planeta rocoso pequeño con depósitos valiosos (distribución estándar del tipo OOLITE_WORLD)

### Planeta 5: Gigante Gaseoso Violeta (Violet Gas Giant)
- **Tipo:** VIO_GAS_GIANT
- **Lunas:** 0
- **Características:** Gigante gaseoso de color violeta

### Planeta 6: Mundo Púrpura (Purple World)
- **Tipo:** PURPLE_WORLD
- **Lunas:** 0
- **Características:** Planeta rocoso pequeño de color púrpura

### Planeta 7: Mundo Tesoro (Treasure World)
- **Tipo:** TREASURE_WORLD
- **Lunas:** 0
- **Características:** Planeta rico en metales preciosos (distribución estándar del tipo TREASURE_WORLD)

### Planeta 8: Mundo Arcoíris (Rainbow World)
- **Tipo:** RAINBOW_WORLD
- **Lunas:** 0
- **Características:** Planeta rocoso grande de múltiples colores y alto valor

## Archivos Modificados/Creados

### Archivos Creados:
1. `/sc2/src/uqm/planets/generate/genkitsune.c` - Generador del sistema
2. `/sc2/src/uqm/planets/generate/genkitsune.h` - Archivo de cabecera

### Archivos Modificados:
1. `/sc2/src/uqm/gendef.h` - Agregada constante `KITSUNE_DEFINED` (activador de generador)
2. `/sc2/src/uqm/gendef.c` - Registrado el generador (`case KITSUNE_DEFINED: return &generateKitsuneFunctions;`)
3. `/sc2/src/uqm/plandata.c` - Agregada estrella al starmap: `{ {9749, 9250}, MAKE_STAR(SUPER_GIANT_STAR, BLUE_BODY, -1), KITSUNE_DEFINED, 0, 133 }` (postfijo 133 = "Kitsune", prefijo 0 = sin "Alpha") y ubicada respetando el orden por Y
4. `/sc2/src/uqm/planets/generate/Makeinfo` - Agregado `genkitsune.c` a la compilación
5. `/sc2/content/base/gamestrings.txt` - Agregado nombre "Kitsune" (postfijo 133) después de "UNKNOWN"
6. `/sc2/src/uqm/starmap.h` - Actualizado `#define NUM_SOLAR_SYSTEMS` a 503 (coincide con entradas antes del centinela)
7. `/sc2/src/uqm/gamestr.h` - Actualizado `#define STAR_STRING_COUNT` a 134 (coincide con cantidad real en `gamestrings.txt`)

## Compilación

Para compilar el juego con el nuevo sistema:

```bash
cd /home/kirebyte/Projects/FoxyByte/sc2-uqm/sc2
./build.sh uqm
```

Para ejecutar rápidamente desde la raíz del repo:

```bash
cd /home/kirebyte/Projects/FoxyByte/sc2-uqm
./run_uqm.sh
```

## Notas de Diseño

- El sistema está diseñado para ser un sistema rico en recursos
- El planeta 0 con vida ofrece oportunidades de bio-colección
- La luna protegida con escudo agrega un elemento de desafío
- Los planetas 4 y 7 son especialmente valiosos por sus depósitos estándar de sus tipos
- El planeta 8 (Rainbow World) es un objetivo de alto valor en el juego

## Detalles Técnicos y Buenas Prácticas

- El arreglo `starmap_array[]` en `plandata.c` debe estar ordenado por coordenada Y ascendente; inserta la estrella en la posición correcta por Y.
- La entrada centinela `{ MAX_X_UNIVERSE << 1, MAX_Y_UNIVERSE << 1 }` marca el fin de la lista; tu estrella debe ir antes.
- `NUM_SOLAR_SYSTEMS` en `starmap.h` debe igualar el número de estrellas reales antes del centinela (503 en este repo tras agregar Kitsune).
- `STAR_STRING_COUNT` en `gamestr.h` debe igualar la cantidad de nombres de constelación en `gamestrings.txt` (134 tras agregar "Kitsune").
- Prefijos (Alpha/Beta/Gamma...): prefijo 0 = sin prefijo; si defines múltiples estrellas de un mismo clúster, usa prefijos 1, 2, 3 con el mismo postfijo.
- Órbitas: las distancias se expresan como múltiplos de `EARTH_RADIUS`. En Kitsune se aumentó el espaciado para mejorar la legibilidad (p.ej. ~1.40x hasta ~5.30x).
