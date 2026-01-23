# Guía de Desarrollo: Cómo añadir un sistema estelar en UQM

Esta guía explica, paso a paso, cómo agregar un sistema estelar nuevo a The Ur-Quan Masters (Star Control 2) en este repo. Cubre arquitectura de archivos, nombres, starmap, generadores de planetas, recursos y verificaciones críticas para evitar fallos.

Usamos el sistema “Kitsune” como ejemplo práctico.

## Visión general

Para agregar un sistema estelar necesitas tocar tres áreas principales:
- Starmap (base de datos de estrellas): agrega una entrada en el arreglo de estrellas con coordenadas, tipo/color, nombre y generador del sistema.
- Generador del sistema: código que define los planetas/lunas, sus órbitas y, si quieres, biomas, minerales, artefactos, etc.
- Strings (nombres): asegúrate de que el nombre de la constelación exista y que los conteos encajen con `gamestr.h`.

## Archivos y directorios clave

- Base de datos del starmap (lista de estrellas):
  - `sc2/src/uqm/plandata.c` → `STAR_DESC starmap_array[]`
- Tipos, estructuras, conteos:
  - `sc2/src/uqm/planets/planets.h` (estructuras `STAR_DESC` y `PLANET_DESC`)
  - `sc2/src/uqm/starmap.h` (define `NUM_SOLAR_SYSTEMS`)
  - `sc2/src/uqm/gamestr.h` (conteos de strings y bases)
- Nombres (strings de juego):
  - `sc2/content/base/gamestrings.txt` (nombres de constelaciones, prefijos, etc.)
- Renderizado, búsqueda y nombres de estrellas:
  - `sc2/src/uqm/starmap.c` (ensambla nombre con prefijo/postfijo)
  - `sc2/src/uqm/planets/pstarmap.c` (dibujo, búsqueda y selección en el starmap)
- Generadores de sistemas (planetas y lunas):
  - `sc2/src/uqm/planets/generate/` (crear `gen<tu_sistema>.c` y `.h`)
  - `sc2/src/uqm/gendef.h` (enum de generadores)
  - `sc2/src/uqm/gendef.c` (mapea enum → `GenerateFunctions*`)
  - `sc2/src/uqm/planets/generate/Makeinfo` (añadir tu archivo `.c` a la build)

## Starmap: añadir una estrella

En `plandata.c`, cada estrella es un elemento del arreglo `starmap_array[]` de tipo `STAR_DESC`.

Formato por campo:
- `{ {x, y}, MAKE_STAR(star_type, color, owner), alien_presence, prefix, postfix }`
  - `x, y` (coordenadas HyperSpace): 0..9999. Debes mantener el arreglo ordenado por Y ascendente (crítico: se usa búsqueda binaria).
  - `MAKE_STAR(star_type, color, owner)`:
    - `star_type`: `DWARF_STAR`, `GIANT_STAR`, `SUPER_GIANT_STAR`.
    - `color`: `RED_BODY`, `ORANGE_BODY`, `YELLOW_BODY`, `GREEN_BODY`, `BLUE_BODY`, `WHITE_BODY`...
    - `owner`: hoy no se usa en starmap, deja `-1`.
  - `alien_presence`: enum que activa un generador específico (ver Generadores). Usa `0` si usas el generador por defecto; usa tu constante si quieres contenido especial.
  - `prefix`: índice del prefijo (0 = sin prefijo; 1 = “Alpha”, 2 = “Beta”, ... ver STAR_NUMBER_BASE).
  - `postfix`: índice del nombre de constelación (entero en la sección STAR_STRING_BASE de `gamestrings.txt`).

Reglas críticas:
- Orden del arreglo: por `y` ascendente. Inserta tu estrella donde corresponda por su `y` respecto a las vecinas. Si rompes este orden, la selección en el mapa falla.
- Centinela de fin de starmap: justo después de la última estrella hay una entrada con `{ MAX_X_UNIVERSE << 1, MAX_Y_UNIVERSE << 1 }`. Tu estrella debe ir antes de ese centinela.
- `NUM_SOLAR_SYSTEMS` debe coincidir con el número de estrellas antes del centinela. Si agregas una estrella, típicamente incrementa `#define NUM_SOLAR_SYSTEMS` en `starmap.h`.

Ejemplo (Kitsune):
- Entrada en `plandata.c` (ya insertada y ordenada):
  - `{ {9749, 9250}, MAKE_STAR(SUPER_GIANT_STAR, BLUE_BODY, -1), KITSUNE_DEFINED, 0, 133 }`
  - Prefijo `0` significa que el nombre será solo “Kitsune”.

## Nombres: prefijo y constelación

El nombre de una estrella se compone así (`starmap.c:GetClusterName`):
- Si `Prefix > 0`, se antepone el prefijo: `GAME_STRING(STAR_NUMBER_BASE + Prefix - 1)` por ejemplo, 1 = “Alpha”, 2 = “Beta”, etc.
- Luego se concatena el “postfix” (constelación): `GAME_STRING(Postfix)` usando el índice en la sección `STAR_STRING_BASE` de `gamestrings.txt`.

Dónde editar:
- `sc2/content/base/gamestrings.txt`:
  - Al comienzo están los nombres de constelación (STAR_STRING_BASE). Por ejemplo: Vega, Antliae, ... Groombridge, ... UNKNOWN, Kitsune.
  - Asegúrate de que tu nombre exista; si agregas uno nuevo, aumenta `#define STAR_STRING_COUNT` en `gamestr.h`.
  - Mantén consistencia: si mueves/insertas en el medio, cambian los índices. Verifica todos los usos de índices de constelación en `plandata.c`.
- Prefijos (STAR_NUMBER_BASE): están más abajo en el archivo, comentados como `#(Alpha) -- STAR_NUMBER_BASE`. Índices: 1=Alpha, 2=Beta, etc. Prefijo 0 = sin prefijo.

Recomendación:
- Para una estrella única, usa `prefix = 0`.
- Para grupos de estrellas (clusters) que comparten constelación (p.ej., Groombridge), usa el mismo `postfix` con `prefix` 1/2/3… para Alpha/Beta/Gamma.

## Generadores: contenido del sistema (planetas y lunas)

Para que una estrella tenga planetas específicos, crea un generador dedicado:
- Crea archivos:
  - `sc2/src/uqm/planets/generate/gen<nombre>.c`
  - `sc2/src/uqm/planets/generate/gen<nombre>.h`
- Define una constante en `sc2/src/uqm/gendef.h` (enum), por ejemplo `KITSUNE_DEFINED`.
- En `sc2/src/uqm/gendef.c`, devuelve tus funciones:
  - En `getGenerateFunctions(...)` añade `case KITSUNE_DEFINED: return &generateKitsuneFunctions;`
- Añade tu `.c` a `sc2/src/uqm/planets/generate/Makeinfo` para que compile.

Estructura mínima del generador (`GenerateFunctions`):
- `generatePlanets`: fija `SunDesc[0].NumPlanets` y define cada `PlanetDesc[i]`:
  - `data_index` (tipo: WATER_WORLD, TREASURE_WORLD, RAINBOW_WORLD, VIO_GAS_GIANT, etc.)
  - `NumPlanets` (número de lunas)
  - `radius` (distancia orbital: usa múltiplos de `EARTH_RADIUS`)
  - `location` con `COSINE/SINE(angle, radius)` (elige un ángulo aleatorio)
- `generateMoons`: si necesitas lunas especiales, crea entradas en `MoonDesc[]` del planeta actual.
- Opcionalmente sobreescribe `generateMinerals/generateEnergy/generateLife` si quieres nodos/biomas concretos.

Ejemplo (Kitsune):
- 9 planetas, espaciados a radios aprox: 1.40x, 1.80x, 2.30x, 2.80x, 3.30x, 3.80x, 4.30x, 4.80x, 5.30x.
- Planeta 7 = Treasure World, Planeta 8 = Rainbow World.
- Planeta 0 = Water World grande con 1 luna (con escudo en la luna como adorno).

Notas de implementación:
- Puedes llamar primero a `GenerateDefault_generatePlanets(solarSys)` y luego sobreescribir tus `PlanetDesc[i]` y `SunDesc[0].NumPlanets`.
- No excedas límites de arrays (ver `planets.h`).

## Recursos y tipos de planetas

Los tipos de planetas y gas gigantes usan paletas y tablas existentes del contenido base:
- Tablas de color y traducción (xlat) ya existen para los tipos estándar (agua, primordial, oolite, shattered, treasure, rainbow, gas giants, etc.).
- Si usas tipos custom inexistentes, necesitas definir su paleta y xlat en contenidos y enlazarlas — para sistemas estándar no hace falta.

## Build y ejecución

- Construir desde `sc2/`:
  - `./build.sh uqm`
- Ejecutar (script en la raíz de este repo):
  - `./run_uqm.sh`

## Verificación en juego

- En el starmap, usa las coordenadas HyperSpace de tu estrella.
- Comprueba que:
  - El punto de la estrella aparece y tiene el color/tamaño esperado.
  - El nombre se muestra correctamente (con o sin prefijo según tu `prefix`).
  - Se puede seleccionar con el cursor y el autopiloto se fija.
  - Puedes entrar al sistema; allí ves tus planetas/lunas en las órbitas con el espaciado esperado.

## Solución de problemas (pitfalls comunes)

1) La estrella aparece pero no se puede seleccionar o no muestra nombre
- Causa típica: el arreglo `starmap_array[]` no está ordenado por Y ascendente.
- Arregla el orden, recompila y prueba otra vez.

2) Crash o comportamiento raro al iniciar / abrir starmap
- Revisa conteos de strings:
  - `gamestr.h`: `STAR_STRING_COUNT` debe igualar la cantidad real de `#(…)` en la sección de estrellas en `gamestrings.txt`.
  - Si agregaste un nombre nuevo de constelación, sube `STAR_STRING_COUNT` y verifica que no moviste otros índices en uso.
- Verifica que `NUM_SOLAR_SYSTEMS` en `starmap.h` coincide con la cantidad de estrellas reales antes del centinela en `plandata.c`.

3) El nombre no cuadra (“Alpha”/“Beta” equivocados)
- Recuerda: `prefix = 0` → sin prefijo. `1`=Alpha, `2`=Beta, `3`=Gamma… (ver STAR_NUMBER_BASE en `gamestrings.txt`).
- Para estrellas individuales, usa `0`.

4) El sistema aparece vacío o con planetas aleatorios
- Asegúrate de haber:
  - Definido la constante de generador en `gendef.h` y registrado el case en `gendef.c`.
  - Puesto esa constante en el campo `alien_presence` de la entrada de `plandata.c` para esa estrella.
  - Añadido tu `gen<nombre>.c` en `planets/generate/Makeinfo`.

5) Órbitas muy juntas o que se solapan
- Ajusta `radius` de cada planeta en tu generador. Usa múltiplos de `EARTH_RADIUS` con incrementos crecientes.
- Puedes variar los ángulos para separar visualmente.

## Checklist rápido

- [ ] `plandata.c`: entrada con coordenadas, `MAKE_STAR`, `alien_presence=TU_ENUM`, `prefix`, `postfix` (ordenada por Y).
- [ ] `starmap.h`: `NUM_SOLAR_SYSTEMS` actualizado.
- [ ] `gamestrings.txt`: nombre de constelación existe; `STAR_STRING_COUNT` actualizado si agregaste uno.
- [ ] `gendef.h`: nueva constante `*_DEFINED`.
- [ ] `gendef.c`: `case *_DEFINED: return &generate<Nombre>Functions;`
- [ ] `planets/generate/Makeinfo`: agregado tu `.c`.
- [ ] Generador: `SunDesc[0].NumPlanets`, `PlanetDesc[i]` con `data_index`, `NumPlanets`, `radius`, `location`, lunas si aplica.
- [ ] Compila, corre, verifica en starmap y dentro del sistema.

## Ejemplo resumido (Kitsune)

- Starmap: `{ {9749, 9250}, MAKE_STAR(SUPER_GIANT_STAR, BLUE_BODY, -1), KITSUNE_DEFINED, 0, 133 }`
- Strings: `gamestrings.txt` tiene “Kitsune” en la sección de estrellas; `STAR_STRING_COUNT` ajustado.
- Generador: `genkitsune.c/.h` con 9 planetas (últimos dos Treasure y Rainbow), órbitas espaciadas y 1 luna en el primero.
- Registro: `KITSUNE_DEFINED` en `gendef.h` y `gendef.c`; `genkitsune.c` añadido a `Makeinfo`.
