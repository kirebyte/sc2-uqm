**The Ur-Quan Masters: Análisis Técnico de Arquitectura y Plan de Migración**

**Resumen Ejecutivo**

The Ur-Quan Masters (UQM) es una versión open-source reimplementada del clásico juego Star Control 2 (1992), originalmente escrito en ensamblador x86 y C. El proyecto sc2-uqm representa un esfuerzo de más de 20 años para portar, documentar y expandir el código fuente original. Este documento proporciona un análisis técnico exhaustivo de la arquitectura del motor de juego, sus módulos componentes, mecánicas de juego, y un plan detallado de migración hacia lenguajes modernos (Rust y Python) que ofrecen mejor mantenibilidad, seguridad de tipos, y ecosistemas contemporáneos.

**1. Arquitectura General del Código Fuente**

**1.1 Estructura de Directorios y Módulos Principales**

El proyecto sc2-uqm está organizado en las siguientes secciones principales:

/src
 /core - Motor de juego central y gestión de estado
 /input - Sistemas de entrada (teclado, joystick, mouse)
 /graphics - Renderizado 2D y gestión de gráficos
 /audio - Sistema de sonido y música
 /resource - Carga y gestión de recursos (sprites, música, datos)
 /libs - Librerías utilitarias (math, strings, memoria)
 /ship - Sistema de naves y comportamiento de IA
 /starmap - Motor de mapa estelar y navegación
 /melee - Sistema de combate directo (Melee Combat)
 /planet - Sistema de aterrizaje y exploración de planetas
 /setup - Inicialización y configuración del juego
 /battle - Orquestación de batallas y encuentros
 /conversation - Sistema de diálogos y comunicaciones

**1.2 Paradigma de Programación**

**Lenguaje Base:** C89/C99 con características procedurales

**Caracter****ísticas Arquitect****ónicas:**

·     **Modelo de Estado Global:** Uso extensivo de variables globales para mantener estado del juego

·     **Gesti****ón Manual de Memoria:** malloc/free para asignación dinámica

·     **Estructuras de Datos Simples:** structs, arrays, listas enlazadas

·     **Callbacks y Punteros a Funciones:** Para implementar polimorfismo en tiempo de ejecución

·     **M****áquinas de Estado:** Implementadas manualmente mediante switch/case para controlar flujos de juego

·     **Rendering Immediate Mode:** Redibujado completo de pantalla cada frame (sin retained-mode graphics)

**1.3 Ciclo Principal de Juego (Game Loop)**

// Pseudocódigo del bucle principal
 void MainGameLoop() {
 while (gameRunning) {
 // 1. Entrada
 ProcessInputEvents();

  // 2. Actualización de Lógica
   UpdateGameState();
   UpdateGameTime();
   
   // 3. Renderizado
   ClearScreen();
   RenderStarmap();   // o RenderMelee, RenderPlanet
   RenderUI();
   FlipBuffers();
   
   // 4. Audio
   UpdateAudio();
   
   // 5. Control de Frame Rate
   LimitFrameRate(60);  // Típicamente 60 FPS
 }
 

}

**Caracter****ísticas Clave:**

·     Frame rate fijo (~60 FPS) con sincronización por tiempo real

·     Procesamiento de eventos de entrada por iteración

·     Renderizado immediate-mode (sin scene graph moderno)

·     Actualización de audio integrada con lógica de juego



**2. Análisis de Módulos Principales**

**2.1 Sistema de Mapa Estelar (Starmap System)**

**Responsabilidades:**

1. Representación del universo (249 sistemas estelares)
2. Navegación de la nave estelar Shofixti
3. Encuentros aleatorios con naves enemigas
4. Gestión de combustible, tripulación y recursos
5. Eventos narrativos y misiones
6. Gráficos del mapa y UI de navegación

**Estructura de Datos:**

typedef struct {
 BYTE star_index;
 BYTE constellation;
 WORD x, y; // Coordenadas galácticas
 BYTE num_planets;
 BYTE color;
 BYTE icon;
 BYTE visited;
 BYTE known;
 } STAR_DESC;

typedef struct {
 STAR_DESC stars[NUM_STAR_SYSTEMS];
 DWORD global_flags; // Eventos completados, naves conocidas
 POINT ship_location;
 WORD fuel;
 WORD crew;
 BYTE day;
 WORD year;
 } UNIVERSE_STATE;

**Flujo de Operación:**

1. **Inicializaci****ón:** Carga datos de sistemas estelares desde recursos compilados
2. **Renderizado:** Dibuja constelaciones, marca sistemas visitados, muestra nave del jugador
3. **Entrada:** Teclado/mouse para seleccionar destinos y acelerar
4. **Viaje:** Interpola movimiento del jugador, calcula encuentros basados en distancia/aleatoriedad
5. **Encuentros:** Transición a sistema melee o planeta según tipo de encuentro
6. **Actualizaci****ón de Recursos:** Consumo de combustible, paso de tiempo

**Algoritmos Clave:**

·     **Generaci****ón de Encuentros:** Basada en tabla de probabilidades según el sistema estelar

·     **C****álculo de Combustible:** Distancia euclidiana entre puntos con consumo lineal

·     **Eventos Narrativos:** Cola de eventos con triggers basados en banderas globales (flags)

**M****ódulos Relacionados:**

·     starmap/starmap.c - Renderizado y lógica principal

·     starmap/events.c - Sistema de eventos y misiones

·     starmap/travel.c - Cálculo de viajes y encuentros

·     starmap/flagship.c - Estado y recursos de la nave estelar



**2.2 Sistema de Combate Cercano (Melee Combat System)**

**Responsabilidades:**

1. Combate de naves 1v1 en vista isométrica
2. Control de movimiento y rotación de nave
3. Sistemas de armas (primarias y secundarias)
4. Gestión de energía y escudos
5. Colisiones y daño
6. Renderizado del campo de batalla

**Estructura de Datos Principal:**

typedef struct {
 BYTE ship_type;
 POINT location;
 POINT velocity;
 BYTE facing; // 0-31 (360 grados en 32 pasos)
 WORD energy;
 WORD max_energy;
 WORD shield_strength;
 BYTE crew;
 BYTE max_crew;
 BYTE flags; // En combate, muerto, especial
 } SHIP_STATE;

typedef struct {
 SHIP_STATE player_ship;
 SHIP_STATE enemy_ship;
 DWORD frame_count;
 BYTE battle_status; // En curso, victoria, derrota
 } MELEE_CONTEXT;

**Mecánicas de Juego:**

| Aspecto                    | Descripción                             | Implementación                                  |
| -------------------------- | --------------------------------------- | ----------------------------------------------- |
| **Movimiento**             | WASD para rotar/acelerar/reversa        | Actualiza velocidad y orientación cada frame    |
| **Armas Primarias**        | Cañones continuos (recursos ilimitados) | Spawn de proyectiles, colisión con sprite       |
| **Armas Secundarias**      | Misiles/especiales limitados            | Cola de misiles, count-down para disponibilidad |
| **Energ****ía**            | Restringe velocidad, armas, especiales  | Sistema de costos por acción + regeneración     |
| **Escudos**                | Reduce daño, consume energía            | Hit-test esférico con multiplicador de daño     |
| **Colisi****ón**           | Daño por impacto directo                | AABB simplificado o radio circular              |
| **Habilidades Especiales** | Únicas por nave (ej: warping, cloaking) | State machine por nave                          |

 

**Flujo de Ejecución (por frame):**

1. Procesar entrada del jugador

o  Rotación: facing = (facing + input) % 32

o  Aceleración: velocity += acceleration * direction_vector

2. Actualizar IA del enemigo

o  Calcula distancia a jugador

o  Selecciona acción (atacar, huir, rotar)

o  Aplica comando a velocidad/rotación

3. Actualizar posiciones

o  player_location += player_velocity * delta_time

o  enemy_location += enemy_velocity * delta_time

o  Clamp a límites del mapa

4. Detectar colisiones

o  Proyectil-nave: aplica daño

o  Nave-nave: rebote elástico

o  Nave-borde: reflejo/límite

5. Actualizar energía

o  Consumo por: velocidad, armas, escudos

o  Regeneración pasiva

6. Renderizar

o  Dibuja fondo (estrellas, planeta)

o  Dibuja naves con orientación correcta

o  Dibuja proyectiles y efectos

o  Dibuja UI (energía, crew, municiones)

7. Evaluar victoria/derrota

o  Si crew == 0: derrota

o  Si enemigo destroyed: victoria

**Clases de Naves (Ejemplos):**

typedef struct {
 BYTE ship_id;
 const char *name;
 WORD max_crew;
 WORD max_energy;
 WORD weapon_cooldown;
 SPECIAL_ABILITY special;
 } SHIP_SPECS;

// Ejemplo: Shofixti (nave del jugador)
 SHIP_SPECS shofixti = {
 SHOFIXTI, "Shofixti", 6, 48,
 4, shofixti_warp // Especial: Warp Space
 };

**M****ódulos Relacionados:**

·     melee/melee.c - Bucle principal de combate

·     melee/ship.c - Lógica de naves y movimiento

·     melee/weapons.c - Sistema de armas y proyectiles

·     melee/collision.c - Detección de colisiones

·     melee/ai.c - Inteligencia artificial de enemigos

·     melee/graphics.c - Renderizado isométrico



**2.3 Sistema de Aterrizaje de Planetas (Planet Landing System)**

**Responsabilidades:**

1. Generación procedural de terreno planetario
2. Control de vehículo de exploración (lander)
3. Recolección de recursos (minerales, criaturas)
4. Encuentros con vida alienígena
5. Diálogos con especies inteligentes
6. Renderizado de vista lateral/isométrica

**Estructura de Datos:**

typedef struct {
 BYTE planet_type; // Rocoso, Gaseoso, Acuático, etc.
 BYTE gravity;
 BYTE temperature;
 BYTE life_forms; // Bitmap de tipos de vida presentes
 BYTE minerals[NUM_ELEMENTS]; // Abundancia de minerales
 } PLANET_DESC;

typedef struct {
 POINT lander_pos;
 WORD lander_fuel;
 BYTE lander_crew;
 BYTE hazard_effects; // Daño ambiental
 BYTE collected_items[MAX_INVENTORY];
 } PLANET_CONTEXT;

**Mecánicas de Exploración:**

| Elemento       | Descripción                     | Interacción                               |
| -------------- | ------------------------------- | ----------------------------------------- |
| **Lander**     | Vehículo explorador controlable | WASD para movimiento, click para recoger  |
| **Minerales**  | Recursos valiosos               | Recolección automática en radio cercano   |
| **Criaturas**  | Enemigos o aliados              | Combate automático, conversación, huida   |
| **Artefactos** | Objetos especiales de misión    | Recolección obligatoria para progresión   |
| **Hazards**    | Peligros ambientales            | Daño constante (lava, toxinas, radiación) |

 

**Generación Procedural:**

// Deterministic seeding basado en planeta
 void GeneratePlanetTerrain(PLANET_DESC *planet, uint32_t seed) {
 // 1. Inicializar mapa con ruido Perlin o similitud
 PerlinNoise2D(terrain_height_map, planet->width, planet->height, seed);

// 2. Aplicar características según tipo de planeta
 ApplyPlanetCharacteristics(terrain_height_map, planet);
 
 // 3. Colocar elementos (minerales, criaturas, hazards)
 SpawnMinerals(planet, seed + 1);
 SpawnLifeForms(planet, seed + 2);
 SpawnHazards(planet, seed + 3);
 
 // 4. Validar accesibilidad (sin áreas impossibles)
 ValidateTerrainAccessibility(terrain_height_map);
 

}

**Flujo de Aterrizaje:**

1. **Pre-Aterrizaje:** Seleccionar coordenadas, confirmar
2. **Generaci****ón:** Crear terreno procedural para la ubicación
3. **Exploraci****ón:** Controlar lander, recolectar, combatir
4. **Despegue:** Consumir combustible, retornar a nave estelar
5. **Post-Aterrizaje:** Actualizar inventario, completar objetivos

**M****ódulos Relacionados:**

·     planet/landing.c - Sistema de aterrizaje

·     planet/terrain.c - Generación y renderizado de terreno

·     planet/lander.c - Física y control del lander

·     planet/elements.c - Minerales y artefactos

·     planet/creatures.c - Vida alienígena y combate automático



**2.4 Sistema de Combate Estelar (Space Battle System)**

**Responsabilidades:**

1. Combate con múltiples naves (flotas)
2. Formaciones y tácticas de grupo
3. Escenas cinemáticas de encuentros
4. Gestión de daño a largo plazo
5. Recompensas y botín

**Características:**

El sistema de combate estelar actúa como orquestador que:

·     Determina qué naves están en combate basado en starmap

·     Configura encuentros de melee según probabilidades

·     Gestiona secuencias de múltiples encuentros (flota vs flota)

·     Acumula daño a las naves jugador y enemigas

typedef struct {
 BYTE fleet_id;
 BYTE ship_count[NUM_SHIP_TYPES];
 WORD fleet_strength; // Métrica agregada de poder
 } FLEET_DESC;

typedef struct {
 FLEET_DESC player_fleet;
 FLEET_DESC enemy_fleet;
 BYTE battles_fought;
 BYTE battles_won;
 } SPACE_BATTLE_CONTEXT;

**M****ódulos Relacionados:**

·     battle/encounter.c - Generación de encuentros

·     battle/fleet.c - Gestión de flotas

·     battle/sequencing.c - Secuencia de múltiples encuentros



**2.5 Sistema de Diálogos y Narrativa (Conversation System)**

**Responsabilidades:**

1. Almacenar y renderizar diálogos de NPCs
2. Ramificaciones narrativas (branching dialogues)
3. Banderas narrativas y progresión de historia
4. Localización multiidioma

**Estructura:**

// Árbol de diálogos compilado
 typedef struct {
 BYTE dialogue_id;
 const char *text;
 BYTE num_responses;
 struct {
 const char *response_text;
 WORD next_dialogue_id;
 DWORD flags_required; // Qué eventos deben estar completados
 DWORD flags_set; // Qué eventos se completan al elegir
 } responses[MAX_RESPONSES];
 } DIALOGUE_NODE;

**M****ódulos Relacionados:**

·     conversation/dialogue.c - Renderizado y navegación

·     conversation/subtitles.c - Sistema de subtítulos



**2.6 Sistema de Menú y UI (Menu & UI System)**

**Responsabilidades:**

1. Menús navegables (Starmap, Equipo, Alineaciones, Créditos)
2. Renderizado de fuentes y elementos UI
3. Gestión de entrada en contexto de menú
4. Persistencia y guardado de juego

**Máquina de Estados de UI:**

typedef enum {
 MENU_MAIN,
 MENU_STARMAP,
 MENU_CREW,
 MENU_ROSTER,
 MENU_ALIGNMENTS,
 MENU_SAVE_GAME,
 MENU_LOAD_GAME,
 SCENE_MELEE,
 SCENE_PLANET,
 SCENE_CONVERSATION
 } GAME_STATE;

**M****ódulos Relacionados:**

·     menu/menu.c - Sistema de menús principal

·     menu/screens.c - Pantallas individuales

·     resource/savegame.c - Serialización de estado



**2.7 Capa de Gráficos (Graphics Layer)**

**Características:**

·     **Modo 320x240 (SVGA)** - Resolución clásica

·     **8-bit Paleta Indexada** - 256 colores simultáneos

·     **Sprites Pre-renderizados** - No hay rotación real, múltiples ángulos almacenados

·     **Buffers Doble** - Previene flickering (on-screen, off-screen)

·     **Operaciones por P****íxel B****ásicas** - Fill, blit, alpha blending simple

typedef struct {
 WORD width, height;
 BYTE *pixel_data; // Índices de paleta 0-255
 BYTE transparent_color; // Color transparente
 } SPRITE_T;

typedef struct {
 BYTE palette[256][3]; // RGB para cada color 0-255
 } PALETTE_T;

**M****ódulos Relacionados:**

·     graphics/graphics.c - Funciones básicas de renderizado

·     graphics/colors.c - Gestión de paleta y conversiones

·     graphics/fonts.c - Renderizado de texto bitmap

·     graphics/sprites.c - Carga y blitting de sprites



**3. Flujos Narrativos y Mecánicas del Juego**

**3.1 Estructura Narrativa General**

El juego sigue esta progresión:

1. **Introducci****ón:** Cutscene cinemática, presentación de la Ur-Quan Masters
2. **Exploraci****ón Inicial:** Misiones de reconocimiento de sistemas estelares
3. **Construcci****ón de Alianzas:** Contacto con especies, negociaciones, reclutamiento de naves
4. **Conflicto Escalante:** Encuentros con naves enemigas, batallas progresivamente difíciles
5. **Climax:** Enfrentamiento final con la Ur-Quan Masters
6. **Resoluci****ón:** Cinemática final (victoria/derrota)

**3.2 Relación Código-Mecánicas**

**Exploración → Starmap Module**

·     Navegar entre 249 sistemas estelares

·     Gestión de combustible limita velocidad de exploración

·     Encuentros aleatorios disparan transiciones a Melee/Planet

**Combate Táctico → Melee Module**

·     Cada encuentro de nave es 1v1 (melee directo)

·     Victoria obtiene experiencia y recompensas narrativas

·     Derrota reinicia el encuentro o termina el juego (modo hardcore)

**Exploración Planetaria → Planet Landing Module**

·     Recolectar recursos específicos según objetivos de misión

·     Encuentros con forma de vida alienígena

·     Algunos planetas tienen artefactos únicos

**Narrativa Ramificada → Conversation Module**

·     Diálogos con cada especie determinan alianzas

·     Elecciones abren/cierran misiones

·     Banderas narrativas afectan encuentros posteriores

**3.3 Sistemas de Progresión**

**Progresi****ón Temporal:**
 // Estructura de tiempo del juego
 typedef struct {
 BYTE day; // 0-30
 BYTE month; // 1-12
 WORD year; // 2155-2159
 DWORD total_ticks; // Acumulativo
 } GAME_TIME;

**Progresi****ón de Recursos:**

·     **Combustible:** Se consume viajando, reabastecido en bases amigas

·     **Tripulaci****ón:** Perdida en combate, reclutada en planetas

·     **Naves:** Reclutadas progresivamente mediante alianzas

·     **Tecnolog****ía:** Obtenida de artefactos, desbloquea capacidades

**Banderas Narrativas (Global Flags):**
 \#define FLAGS_SHOFIXTI_ALLY 0x0001
 \#define FLAGS_THRADDASH_ALLY 0x0002
 \#define FLAGS_SPATHI_ALLY 0x0004
 \#define FLAGS_ARILOU_ALIVE 0x0008
 // ... 500+ banderas adicionales ...



**4. Análisis Técnico Profundo del Código Fuente**

**4.1 Patrones de Programación Utilizados**

**4.1.1 Máquinas de Estado Explícitas**

Casi todos los módulos implementan máquinas de estado manualmente:

typedef enum {
 STATE_IDLE,
 STATE_MOVING,
 STATE_ATTACKING,
 STATE_FLEEING,
 STATE_DEAD
 } SHIP_STATE_T;

void UpdateShip(SHIP_T *ship) {
 switch(ship->current_state) {
 case STATE_IDLE:
 // Seleccionar acción
 break;
 case STATE_MOVING:
 // Actualizar posición
 break;
 case STATE_ATTACKING:
 // Disponer de armas
 break;
 // ...
 }
 }

**Problema:** Difícil de mantener, propenso a errores de transición de estado

**4.1.2 Objetos Simulados mediante Structs + Callbacks**

typedef struct {
 void *instance_data;
 void (*update_func)(void *self);
 void (*render_func)(void *self);
 void (*destroy_func)(void *self);
 } GAME_OBJECT;

**Problema:** No hay herencia real, tipos débiles, propenso a casting incorrecto

**4.1.3 Gestión Manual de Recursos**

// Cargar recurso
 SPRITE_T *sprite = LoadSpriteFromResource(SPRITE_ID_SHOFIXTI);

// Usar
 RenderSprite(screen, sprite, x, y);

// Liberar (fácil olvidar)
 FreeSprite(sprite);

**Problema:** Fugas de memoria comunes, no hay RAII o garbage collection

**4.1.4 Aritmética de Punteros para Acceso a Memoria**

// Acceder a pixel en buffer de pantalla
 BYTE *framebuffer = (BYTE *)0xA0000; // Dirección de video legacy
 BYTE *pixel = framebuffer + (y * 320) + x;
 *pixel = color_index;

**Problema:** Propenso a buffer overflow, no portable, difícil de debuguear

**4.2 Análisis de Antipatrones y Deuda Técnica**

| Antipatrón                  | Ubicación              | Impacto                                  | Migración                       |
| --------------------------- | ---------------------- | ---------------------------------------- | ------------------------------- |
| **Goto statements**         | ship.c, melee.c        | Control flow oscuro                      | Refactorizar a loops/returns    |
| **Globals impl****ícitos**  | starmap.c, battle.c    | Estado oculto, reentrancia imposible     | Encapsular en structs           |
| **Macros complejos**        | defs.h                 | Debugging difícil, incompatibilidad tipo | Reemplazar con funciones inline |
| **Buffer overflows**        | string handling        | Security risk                            | Usar arrays con bounds checking |
| **Hardcoded magic numbers** | Everywhere             | Mantenibilidad baja                      | Constantes nombradas            |
| **Pointer casting**         | graphics.c, resource.c | Type safety comprometida                 | Usar tipos genéricos            |

 

**4.3 Puntos de Dependencia Crítica**

┌─────────────────────────────────────┐
 │ Game Loop (core/main.c) │
 └──────────────┬──────────────────────┘
 │
 ┌──────────┼──────────┬──────────┐
 │ │ │ │
 Input Starmap Melee Planet
 System Logic System System
 │ │ │ │
 ├──────────┼──────────┼──────────┤
 │ │
 └──────────→ Graphics Layer ←─────┘
 │
 ├→ Audio Engine
 ├→ Resource Manager
 └→ Save/Load System

**Cr****íticos para migraci****ón:**

·     Graphics Layer (toca todo)

·     Resource Manager (dependencia global)

·     Input System (sincronización de frame)

·     Audio Engine (integración con lógica)

**4.4 Complejidad Ciclomática por Módulo**

Basado en análisis típico de código C de era 90s:

| Módulo                  | Est. Complejidad | Crítico |
| ----------------------- | ---------------- | ------- |
| melee/ai.c              | 45-60            | Alto    |
| starmap/events.c        | 35-50            | Alto    |
| planet/terrain.c        | 30-40            | Medio   |
| conversation/dialogue.c | 25-35            | Bajo    |
| graphics/graphics.c     | 50-70            | Crítico |

 



**5. Análisis de Lenguajes Destino: Rust vs Python**

**5.1 Comparativa Técnica**

| Criterio                    | Rust                       | Python                 |
| --------------------------- | -------------------------- | ---------------------- |
| **Seguridad de Memoria**    | ⭐⭐⭐⭐⭐ (Compiletime)        | ⭐⭐ (Runtime GC)        |
| **Rendimiento**             | ⭐⭐⭐⭐⭐ (Native, zero-cost)  | ⭐⭐ (Interpretado, GIL) |
| **Curva de Aprendizaje**    | Difícil (borrow checker)   | Fácil (dinámico)       |
| **Velocidad de Desarrollo** | Lento (compilación, tipos) | Rápido (prototipado)   |
| **Ecosystem Juegos**        | Bueno (bevy, ggez)         | Medio (pygame, pyglet) |
| **Concurrencia**            | ⭐⭐⭐⭐⭐                      | ⭐⭐ (GIL limita)        |
| **Mantenibilidad**          | ⭐⭐⭐⭐                       | ⭐⭐⭐⭐                   |
| **Tama****ño Binario**      | ~5-30 MB                   | ~30-150 MB (+ runtime) |
| **Tiempo de Inicio**        | <100ms                     | 100-500ms              |

 

**5.2 Recomendación para UQM**

**RUST es la opci****ón recomendada** por:

1. **Eliminaci****ón de Clases de Bugs:**

o  Memory safety sin garbage collector

o  Type safety garantizado en compiletime

o  No hay buffer overflows, use-after-free, data races

2. **Rendimiento Cr****ítico:**

o  Generación procedural en tiempo real

o  Renderizado 60 FPS requiere eficiencia

o  Concurrencia para cargar recursos sin lag

3. **Ecosistema Moderno:**

o  **Bevy:** Motor de juegos ECS-first, excelente para UQM

o  **winit:** Manejo de ventanas cross-platform

o  **wgpu:** Graphics API moderna (reemplaza legacy SVGA)

**Python es viable como capa prototipo:**

·     Desarrollo rápido inicial

·     Fácil testing de lógica de juego

·     Migración posterior a Rust optimizado

**5.3 Rust - Stack Tecnológico Propuesto**

[dependencies]
 bevy = "0.14"
 wgpu = "0.21"
 winit = "0.30"
 serde = { version = "1.0", features = ["derive"] }
 serde_json = "1.0"
 rand = "0.8"
 nalgebra = "0.33"
 specs = "0.20" # ECS alternativo a bevy

**Ventajas Rust para cada módulo:**

**Starmap Module:**
 // Type-safe ship states
 \#[derive(Clone, Copy, Debug)]
 enum ShipState {
 Idle,
 Moving { velocity: Vec2 },
 InCombat { enemy_id: ShipId },
 }

// Garantizado: no estados inválidos
 struct Ship {
 id: ShipId,
 position: Vec2,
 state: ShipState, // Exhaustive pattern matching
 }

impl Ship {
 fn update(&mut self, dt: f32) {
 match &mut self.state {
 ShipState::Idle => { /* ... 
 
 
 
 */ }ShipState::Moving { velocity } => {self.position += velocity \* dt;}ShipState::InCombat { .. } => { /* ... */ }
 }
 }
 }

**Melee Module:**
 // Traits para polimorfismo type-safe
 trait Weapon {
 fn fire(&self, origin: Vec2, direction: Vec2) -> ProjectileBundle;
 fn cooldown(&self) -> Duration;
 }

struct PrimaryWeapon { /* ... 
 */ }struct SpecialAbility { /* ... */ }

impl Weapon for PrimaryWeapon { /* ... 
 */ }impl Weapon for SpecialAbility { /* ... */ }

**Memory Safety Example:**
 // NO buffer overflow possible
 let mut pixels: Vec<u8> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

// Bounds check automático, no segmentation fault
 for y in 0..SCREEN_HEIGHT {
 for x in 0..SCREEN_WIDTH {
 let idx = y * SCREEN_WIDTH + x;
 pixels[idx] = color; // Panics if out of bounds, never silent overflow
 }
 }

**5.4 Python - Stack Tecnológico Alternativo**

**Prototipado rápido**

pygame
 2.5.0numpy1.24.0
 dataclasses-json==0.6.0

**Ventaja:** Iteración rápida, fácil testing, pero:

·     No apto para build final (performance)

·     Útil como "golden reference" para comportamiento



**6. Plan de Migración: De C a Rust**

**6.1 Estrategia General**

**Enfoque:** Migración iterativa por módulos, testeo exhaustivo al final de cada fase.

**Principios:**

1. **Phase Isolation:** Cada módulo migrarse de forma independiente
2. **Golden Behavior:** Python prototipo define comportamiento esperado
3. **Gradual Integration:** Módulos Rust se integran uno por uno con C legacy
4. **Test-Driven:** Escribir tests ANTES de migración
5. **Performance Validation:** Cada módulo debe cumplir benchmarks

**6.2 Fases de Migración (8 Fases)**

**Fase 0: Preparación y Setup (2-3 semanas)**

**Objetivos:**

·     [ ] Establecer proyecto Rust con estructura modular

·     [ ] Crear test framework

·     [ ] Documentar API C original

·     [ ] Crear Python golden reference

**Tareas:**
 cargo new uqm-rust --lib

**Estructura inicial**

uqm-rust/
 ├── src/
 │ ├── [lib.rs](http://lib.rs) (API pública)
 │ ├── core/
 │ │ ├── [mod.rs](http://mod.rs)
 │ │ ├── [types.rs](http://types.rs) (Tipos principales)
 │ │ └── [state.rs](http://state.rs) (Estado global)
 │ ├── graphics/
 │ ├── input/
 │ ├── audio/
 │ └── ...
 ├── tests/
 │ ├── integration_tests.rs
 │ └── fixtures/
 ├── Cargo.toml
 └── golden_reference/ (Python prototipo)
 └── uqm_golden.py

**Configuraci****ón Cargo.toml:**
 [package]
 name = "uqm-rust"
 version = "0.1.0"
 edition = "2021"

[lib]
 name = "uqm"
 path = "src/lib.rs"

[[bin]]
 name = "uqm-game"
 path = "src/main.rs"

[dependencies]
 bevy = { version = "0.14", features = ["dynamic_linking"] }
 wgpu = "0.21"
 winit = "0.30"
 serde = { version = "1.0", features = ["derive"] }
 nalgebra = "0.33"
 rand = "0.8"

[dev-dependencies]
 criterion = "0.5" # Benchmarking

**Python Golden Reference:**

**uqm_golden.py - Define comportamiento esperado**

from dataclasses import dataclass
 from enum import Enum
 import math

class ShipState(Enum):
 IDLE = 0
 MOVING = 1
 IN_COMBAT = 2

@dataclass
 class Ship:
 ship_id: int
 x: float
 y: float
 vx: float
 vy: float
 state: ShipState

def update(self, dt: float):
   if self.state == ShipState.MOVING:
     self.x += self.vx * dt
     self.y += self.vy * dt
   
 def distance_to(self, other: 'Ship') -> float:
   dx = self.x - other.x
   dy = self.y - other.y
   return math.sqrt(dx*dx + dy*dy)
 

**Test cases**

def test_ship_movement():
 ship = Ship(1, 0.0, 0.0, 10.0, 0.0, ShipState.MOVING)
 ship.update(1.0)
 assert ship.x == 10.0, "Ship should move 10 units in x direction"
 assert ship.y == 0.0

**Deliverables Fase 0:**

·     ✅ Repositorio Rust configurado

·     ✅ Test infrastructure (unit + integration)

·     ✅ Python golden reference con 50+ test cases

·     ✅ Documentación de API C original (generada con Doxygen)

·     ✅ Benchmarks baseline del código C original



**Fase 1: Core Tipos y Estructuras (2-3 semanas)**

**Objetivo:** Migrar tipos de datos y crear abstracciones type-safe

**M****ódulos Afectados:**

·     core/types.rs - Tipos principales

·     core/state.rs - Gestión de estado global

**Cambios Clave:**

**Antes (C):**
 // core/types.h
 typedef unsigned char BYTE;
 typedef unsigned short WORD;
 typedef unsigned int DWORD;
 typedef signed char SBYTE;

typedef struct {
 BYTE x, y;
 WORD coords[2];
 } POINT;

typedef struct {
 BYTE star_index;
 WORD x, y;
 BYTE num_planets;
 // ... 10 campos más
 } STAR_DESC;

**Despu****és (Rust):**
 // src/core/types.rs
 \#[derive(Clone, Copy, Debug, PartialEq, Eq)]
 pub struct StarIndex(pub u8);

\#[derive(Clone, Copy, Debug, PartialEq)]
 pub struct Point {
 pub x: i16,
 pub y: i16,
 }

\#[derive(Clone, Debug)]
 pub struct Star {
 pub index: StarIndex,
 pub position: Point,
 pub num_planets: u8,
 pub constellation: u8,
 pub color: u8,
 pub visited: bool,
 // ... REST
 }

impl Star {
 pub fn distance_to(&self, other: &Star) -> f32 {
 let dx = (self.position.x - other.position.x) as f32;
 let dy = (self.position.y - other.position.y) as f32;
 (dx * dx + dy * dy).sqrt()
 }
 }

**Ventajas:**

·     Strong typing previene conversiones inválidas

·     bool explícito en lugar de BYTE ambiguo

·     Métodos asociados más claros que extern functions

**Tests:**
 \#[cfg(test)]
 mod tests {
 use super::*;

\#[test]
 fn test_star_distance() {
   let star1 = Star { position: Point { x: 0, y: 0 }, .. };
   let star2 = Star { position: Point { x: 3, y: 4 }, .. };
   let dist = star1.distance_to(&star2);
   assert!((dist - 5.0).abs() < 0.01);
 }
 

}

**Deliverables Fase 1:**

·     ✅ core/types.rs completo con 100+ tipos

·     ✅ Métodos asociados para operaciones comunes

·     ✅ Test coverage >90%

·     ✅ Comparación con golden reference (Python)



**Fase 2: Graphics Layer Refactor (3-4 semanas)**

**Objetivo:** Reemplazar legacy graphics (SVGA paleta 8-bit) con wgpu moderno

**M****ódulos Afectados:**

·     graphics/mod.rs

·     graphics/renderer.rs

·     graphics/sprite.rs

**Cambios Cr****íticos:**

**Antes (C):**
 // Legacy 320x240 paleta indexada
 void DrawPixel(int x, int y, BYTE color_index) {
 BYTE *framebuffer = (BYTE *)0xA0000;
 framebuffer[y * 320 + x] = color_index;
 }

void DrawSprite(SPRITE_T *sprite, int x, int y) {
 for (int sy = 0; sy < sprite->height; sy++) {
 for (int sx = 0; sx < sprite->width; sx++) {
 BYTE pixel = sprite->pixels[sy * sprite->width + sx];
 if (pixel != sprite->transparent_color) {
 DrawPixel(x + sx, y + sy, pixel);
 }
 }
 }
 }

**Despu****és (Rust + wgpu):**
 // src/graphics/renderer.rs
 use wgpu::*;

pub struct Renderer {
 device: Device,
 queue: Queue,
 surface: Surface,
 config: SurfaceConfiguration,
 }

impl Renderer {
 pub async fn new(window: &Window) -> Self {
 let instance = Instance::new(InstanceDescriptor {
 backends: Backends::all(),
 ..Default::default()
 });

  let surface = unsafe { instance.create_surface(&window).unwrap() };
   let adapter = instance.request_adapter(&RequestAdapterOptions {
     power_preference: PowerPreference::HighPerformance,
     ..Default::default()
   }).await.unwrap();
   
   let (device, queue) = adapter.request_device(&DeviceDescriptor::default(), None)
     .await.unwrap();
   
   // ... Initialize render pipeline ...
   
   Self { device, queue, surface, config }
 }
 
 pub fn render_sprite(&mut self, sprite: &Sprite, x: i32, y: i32) {
   // Modern batch rendering, no pixel-by-pixel operations
   // Uses GPU shaders for transformation, blitting, blending
 }
 

}

pub struct Sprite {
 pub texture: Texture,
 pub width: u32,
 pub height: u32,
 pub transparent_color: Option<u32>,
 }

**Ventajas:**

·     Renderizado GPU en lugar de CPU pixel-by-pixel

·     Soporte para resoluciones modernas (no limitado a 320x240)

·     Shaders permiten efectos visuales avanzados

·     Hardware acceleration

**Benchmarks:**
 \#[cfg(test)]
 mod benches {
 use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_render(c: &mut Criterion) {
   c.bench_function("render_1000_sprites", |b| {
     b.iter(|| {
       // Renderizar 1000 sprites
       // C legacy: ~50ms
       // Rust wgpu: <5ms (10x mejora)
     })
   });
 }
 

}

**Deliverables Fase 2:**

·     ✅ wgpu renderer funcional

·     ✅ Sprite loading y rendering

·     ✅ Palette support (backwards compatible con art assets legacy)

·     ✅ Benchmarks 10x mejora vs C original

·     ✅ Unit tests para transformaciones, blitting, blending



**Fase 3: Input System (1-2 semanas)**

**Objetivo:** Modernizar manejo de input, mantener API simple

**M****ódulos Afectados:**

·     input/mod.rs

**Cambios:**

**Antes (C):**
 // Polling en cada frame
 if (CheckKey(KEY_UP)) {
 ship_velocity_y--;
 }
 if (CheckKey(KEY_SPACE)) {
 FireWeapon();
 }

**Despu****és (Rust):**
 // Event-driven + polling
 use winit::event::{Event, VirtualKeyCode};

pub struct InputState {
 keys_pressed: HashSet<VirtualKeyCode>,
 mouse_pos: (f32, f32),
 mouse_clicked: bool,
 }

impl InputState {
 pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
 self.keys_pressed.contains(&key)
 }

pub fn handle_event(&mut self, event: &Event<()>) {
   match event {
     Event::WindowEvent { event, .. } => {
       match event {
         WindowEvent::KeyboardInput { input, .. } => {
           match input.state {
             ElementState::Pressed => {
               if let Some(key) = input.virtual_keycode {
                 self.keys_pressed.insert(key);
               }
             }
             ElementState::Released => {
               if let Some(key) = input.virtual_keycode {
                 self.keys_pressed.remove(&key);
               }
             }
           }
         }
         // ... Mouse handling ...
         _ => {}
       }
     }
     _ => {}
   }
 }
 

}

**Ventajas:**

·     Event-driven system más moderno

·     Mejor performance (no polling constante)

·     Soporte para múltiples dispositivos (joystick, gamepad)

**Deliverables Fase 3:**

·     ✅ Input handler completo

·     ✅ Keyboard, mouse, gamepad support

·     ✅ Input rebinding system

·     ✅ Tests para mapeo de teclas



**Fase 4: Audio System (1-2 semanas)**

**Objetivo:** Reemplazar audio legacy con sistema moderno

**M****ódulos Afectados:**

·     audio/mod.rs

**Stack:**

·     rodio - Audio playback Rust puro

·     O FMOD - Binding C++ (compatible con legacy)

// src/audio/manager.rs
 use rodio::{Sink, Source, Decoder};
 use std::io::BufReader;

pub struct AudioManager {
 stream_handle: StreamHandle,
 music_sink: Sink,
 effects_sink: Sink,
 }

impl AudioManager {
 pub fn new() -> Self {
 let device = Device::default();
 let (stream_handle, stream) = rodio::ThreadedStream::new();

  let music_sink = Sink::try_new(&stream_handle).unwrap();
   let effects_sink = Sink::try_new(&stream_handle).unwrap();
   
   Self {
     stream_handle,
     music_sink,
     effects_sink,
   }
 }
 
 pub fn play_music(&self, path: &str) {
   let file = std::fs::File::open(path).unwrap();
   let source = Decoder::new(BufReader::new(file)).unwrap();
   self.music_sink.append(source);
 }
 
 pub fn play_effect(&self, path: &str) {
   let file = std::fs::File::open(path).unwrap();
   let source = Decoder::new(BufReader::new(file)).unwrap();
   self.effects_sink.append(source);
 }
 

}

**Deliverables Fase 4:**

·     ✅ Audio manager with music + effects

·     ✅ Volume control

·     ✅ Looping support

·     ✅ Tests de playback timing



**Fase 5: Starmap Module Completa (4-5 semanas)**

**Objetivo:** Migrar lógica starmap desde C a Rust

**M****ódulos Afectados:**

·     starmap/mod.rs

·     starmap/map.rs

·     starmap/navigation.rs

·     starmap/encounters.rs

**Estructura Rust:**

// src/starmap/mod.rs
 pub mod map;
 pub mod navigation;
 pub mod encounters;
 pub mod events;

use crate::core::types::*;

pub struct StarMap {
 stars: Vec<Star>,
 player_position: Point,
 player_fuel: u32,
 player_crew: u32,
 time: GameTime,
 global_flags: GameFlags,
 }

impl StarMap {
 pub fn new() -> Self {
 let stars = Self::load_stars();
 Self {
 stars,
 player_position: Point { x: 0, y: 0 },
 player_fuel: 1000,
 player_crew: 50,
 time: GameTime::default(),
 global_flags: GameFlags::new(),
 }
 }

fn load_stars() -> Vec<Star> {
   // Cargar datos compilados o asset file
   // 249 sistemas estelares
   todo!()
 }
 
 pub fn travel_to(&mut self, destination: StarIndex) -> Result<(), TravelError> {
   let source = self.stars[self.player_position.to_index()];
   let dest = self.stars[destination.0 as usize];
   
   let distance = source.distance_to(&dest);
   let fuel_cost = (distance * FUEL_CONSUMPTION_RATE) as u32;
   
   if self.player_fuel < fuel_cost {
     return Err(TravelError::InsufficientFuel);
   }
   
   self.player_fuel -= fuel_cost;
   self.player_position = dest.position;
   self.time.advance_days(calculate_travel_time(distance));
   
   // Check for encounter
   if let Some(encounter) = self.check_encounter() {
     return Err(TravelError::EncounterOccurred(encounter));
   }
   
   Ok(())
 }
 
 fn check_encounter(&self) -> Option<Encounter> {
   // Basado en probabilidades y estado
   todo!()
 }
 
 pub fn update(&mut self, dt: f32) {
   // Update time-based events, regeneration, etc.
 }
 

}

// Golden reference tests
 \#[cfg(test)]
 mod tests {
 use super::*;

\#[test]
 fn test_travel_fuel_consumption() {
   let mut map = StarMap::new();
   map.player_fuel = 100;
   
   let initial_fuel = map.player_fuel;
   // Travel between nearby stars
   let _ = map.travel_to(StarIndex(1));
   
   assert!(map.player_fuel < initial_fuel);
   assert!(map.player_fuel > 0); // Didn't deplete completely
 }
 
 \#[test]
 fn test_insufficient_fuel() {
   let mut map = StarMap::new();
   map.player_fuel = 1; // Almost empty
   
   let result = map.travel_to(StarIndex(100)); // Distant star
   assert!(matches!(result, Err(TravelError::InsufficientFuel)));
 }
 

}

**Comparaci****ón con Golden Reference (Python):**

**golden_reference/starmap_golden.py**

import math

class StarMap:
 def **init**(self):
 self.stars = self.load_stars()
 self.player_pos = (0, 0)
 self.fuel = 1000

def travel_to(self, dest_idx):
   src = self.stars[0]
   dest = self.stars[dest_idx]
   
   distance = math.sqrt((src[0] - dest[0])**2 + (src[1] - dest[1])**2)
   fuel_cost = int(distance * 0.5) # FUEL_CONSUMPTION_RATE = 0.5
   
   if self.fuel < fuel_cost:
     raise ValueError("Insufficient fuel")
   
   self.fuel -= fuel_cost
   self.player_pos = dest
   return True
 

**Test**

def test_starmap():
 m = StarMap()
 assert m.fuel == 1000
 m.travel_to(1)
 assert m.fuel < 1000

**Deliverables Fase 5:**

·     ✅ StarMap logic (navigation, travel, encounters)

·     ✅ Event system (banderas narrativas)

·     ✅ Time system (day, month, year)

·     ✅ Complete test coverage vs golden reference

·     ✅ Performance: cargo test --release <10ms



**Fase 6: Melee Combat System (5-6 semanas)**

**Objetivo:** Migrar sistema de combate directo, el módulo más complejo

**M****ódulos Afectados:**

·     melee/mod.rs

·     melee/ship.rs

·     melee/weapons.rs

·     melee/ai.rs

·     melee/collision.rs

·     melee/battle.rs

**Estructura:**

// src/melee/ship.rs
 \#[derive(Clone, Copy, Debug, PartialEq)]
 pub enum ShipState {
 Idle,
 Moving { velocity: Vec2, facing: u8 },
 Attacking { target_id: ShipId },
 Fleeing { target_id: ShipId },
 Dead,
 }

\#[derive(Clone, Debug)]
 pub struct Ship {
 pub id: ShipId,
 pub ship_type: ShipType,
 pub position: Vec2,
 pub state: ShipState,
 pub energy: u16,
 pub max_energy: u16,
 pub crew: u8,
 pub max_crew: u8,
 }

impl Ship {
 pub fn update(&mut self, dt: f32) {
 // Consumo de energía
 if let ShipState::Moving { velocity, .. } = self.state {
 let speed = velocity.magnitude();
 self.energy = self.energy.saturating_sub((speed * 0.1) as u16);
 }

  // Regeneración pasiva
   let regen = (5.0 * dt) as u16;
   self.energy = self.energy.saturating_add(regen).min(self.max_energy);
   
   // Match state and apply updates
   match &mut self.state {
     ShipState::Moving { velocity, facing } => {
       self.position += *velocity * dt;
       // Clamp to battle bounds
       self.position.x = self.position.x.clamp(0.0, BATTLE_WIDTH as f32);
       self.position.y = self.position.y.clamp(0.0, BATTLE_HEIGHT as f32);
     }
     ShipState::Attacking { .. } => {
       // AI selects weapon and fires
     }
     _ => {}
   }
 }
 
 pub fn take_damage(&mut self, damage: u16, penetration: f32) {
   // Calculate actual damage with shield
   let shield_reduction = (damage as f32 * (1.0 - penetration)) as u16;
   let actual_damage = damage - shield_reduction;
   
   self.crew = self.crew.saturating_sub(actual_damage as u8);
 }
 

}

// src/melee/weapons.rs
 \#[derive(Clone, Debug)]
 pub enum Weapon {
 Primary(PrimaryWeapon),
 Special(SpecialWeapon),
 }

\#[derive(Clone, Debug)]
 pub struct PrimaryWeapon {
 pub damage: u16,
 pub cooldown: f32,
 pub time_since_last_fire: f32,
 }

\#[derive(Clone, Debug)]
 pub struct SpecialWeapon {
 pub name: String,
 pub damage: u16,
 pub energy_cost: u16,
 pub cooldown: f32,
 pub ammo: u8,
 pub max_ammo: u8,
 pub time_since_last_fire: f32,
 }

impl Weapon {
 pub fn can_fire(&self) -> bool {
 match self {
 Weapon::Primary(w) => w.time_since_last_fire >= w.cooldown,
 Weapon::Special(w) => {
 w.time_since_last_fire >= w.cooldown && w.ammo > 0
 }
 }
 }

pub fn fire(&mut self) -> Option<(u16, u16)> { // damage, energy_cost
   match self {
     Weapon::Primary(w) => {
       if w.can_fire() {
         w.time_since_last_fire = 0.0;
         Some((w.damage, 0))
       } else {
         None
       }
     }
     Weapon::Special(w) => {
       if w.can_fire() {
         w.time_since_last_fire = 0.0;
         w.ammo -= 1;
         Some((w.damage, w.energy_cost))
       } else {
         None
       }
     }
   }
 }
 

}

// src/melee/ai.rs
 pub struct AIController {
 decision_timer: f32,
 current_action: AIAction,
 }

\#[derive(Clone, Copy, Debug)]
 enum AIAction {
 Attack,
 Evade,
 Flee,
 Idle,
 }

impl AIController {
 pub fn decide(&mut self, player: &Ship, enemy: &Ship, dt: f32) -> ShipCommand {
 self.decision_timer -= dt;

  if self.decision_timer <= 0.0 {
     // Tomar decisión basada en distancia, energía, crew
     let distance = (player.position - enemy.position).magnitude();
     let health_percent = enemy.crew as f32 / enemy.max_crew as f32;
     let energy_percent = enemy.energy as f32 / enemy.max_energy as f32;
     
     self.current_action = if health_percent < 0.25 {
       AIAction::Flee
     } else if distance < 100.0 && energy_percent > 0.5 {
       AIAction::Attack
     } else if distance > 200.0 {
       AIAction::Attack // Approach
     } else {
       AIAction::Evade
     };
     
     self.decision_timer = 2.0; // Decidir cada 2 segundos
   }
   
   match self.current_action {
     AIAction::Attack => {
       let direction = (player.position - enemy.position).normalize();
       ShipCommand {
         acceleration: Some(direction),
         rotation: Some((direction.y.atan2(direction.x) * 32.0) as u8),
         fire_primary: true,
         fire_special: energy_percent > 0.8 && rand() > 0.7,
       }
     }
     AIAction::Evade => {
       // Perpendicular movement
       let direction = (player.position - enemy.position).normalize();
       let perpendicular = Vec2::new(-direction.y, direction.x);
       ShipCommand {
         acceleration: Some(perpendicular),
         rotation: None,
         fire_primary: false,
         fire_special: false,
       }
     }
     _ => ShipCommand::default(),
   }
 }
 

}

// src/melee/collision.rs
 pub struct CollisionDetector;

impl CollisionDetector {
 pub fn check_projectile_ship(
 projectile: &Projectile,
 ship: &Ship,
 ) -> bool {
 let dist = (projectile.position - ship.position).magnitude();
 dist < SHIP_COLLISION_RADIUS
 }

pub fn check_ship_ship(ship1: &Ship, ship2: &Ship) -> bool {
   let dist = (ship1.position - ship2.position).magnitude();
   dist < SHIP_COLLISION_RADIUS * 2.0
 }
 

}

// src/melee/battle.rs
 pub struct BattleContext {
 player: Ship,
 enemy: Ship,
 projectiles: Vec<Projectile>,
 ai_controller: AIController,
 frame_count: u32,
 }

impl BattleContext {
 pub fn update(&mut self, input: &InputState, dt: f32) {
 // 1. Player input
 let player_cmd = self.handle_player_input(input);

  // 2. AI decision
   let enemy_cmd = self.ai_controller.decide(&self.player, &self.enemy, dt);
   
   // 3. Update ships
   self.player.update(dt);
   self.enemy.update(dt);
   
   // 4. Apply commands
   self.apply_command(&mut self.player, player_cmd, dt);
   self.apply_command(&mut self.enemy, enemy_cmd, dt);
   
   // 5. Fire weapons
   if player_cmd.fire_primary {
     if let Some(projectile) = self.create_projectile(&self.player) {
       self.projectiles.push(projectile);
     }
   }
   
   // 6. Update projectiles
   self.projectiles.iter_mut().for_each(|p| p.update(dt));
   
   // 7. Check collisions
   self.projectiles.retain_mut(|projectile| {
     if CollisionDetector::check_projectile_ship(projectile, &self.enemy) {
       let damage = projectile.damage();
       self.enemy.take_damage(damage, projectile.penetration);
       false // Remove projectile
     } else {
       true
     }
   });
   
   // 8. Check win/loss conditions
   if self.enemy.crew == 0 {
     // Victory
   }
   if self.player.crew == 0 {
     // Defeat
   }
   
   self.frame_count += 1;
 }
 

}

**Comparaci****ón con Golden Reference:**

**golden_reference/melee_golden.py**

class Ship:
 def **init**(self, name, max_crew, max_energy):
 [self.name](http://self.name) = name
 self.crew = max_crew
 self.energy = max_energy
 self.max_crew = max_crew
 self.max_energy = max_energy
 self.pos = [0, 0]
 self.vel = [0, 0]

def take_damage(self, dmg):
   self.crew = max(0, self.crew - dmg)
 
 def update(self, dt):
   self.pos[0] += self.vel[0] * dt
   self.pos[1] += self.vel[1] * dt
   
   \# Regenerate energy
   self.energy = min(self.max_energy, self.energy + 5 * dt)
 

**Tests que ambas versiones deben pasar**

def test_ship_takes_damage():
 s = Ship("Test", crew=10, energy=50)
 s.take_damage(3)
 assert s.crew == 7

def test_energy_regeneration():
 s = Ship("Test", crew=10, energy=50)
 s.update(1.0) # 1 segundo
 assert s.energy == min(50, 50 + 5*1.0) # 55 pero capped

**Deliverables Fase 6:**

·     ✅ Ship state machine completa

·     ✅ AI combate balanceado

·     ✅ Weapon system con primary + special

·     ✅ Collision detection

·     ✅ ~100% test coverage vs golden reference

·     ✅ Performance: 60 FPS stable



**Fase 7: Planet Landing System (3-4 semanas)**

**Objetivo:** Migrar sistema de exploración planetaria

**M****ódulos Afectados:**

·     planet/mod.rs

·     planet/terrain.rs

·     planet/lander.rs

·     planet/creatures.rs

// src/planet/terrain.rs
 use noise::{Perlin, NoiseFn};

pub struct TerrainGenerator {
 perlin: Perlin,
 }

impl TerrainGenerator {
 pub fn generate(&self, planet: &PlanetDesc, seed: u32) -> TerrainMap {
 let mut terrain = vec![vec![0; 512]; 512];

  for y in 0..512 {
     for x in 0..512 {
       let nx = x as f64 / 64.0;
       let ny = y as f64 / 64.0;
       let value = self.perlin.get([nx, ny, seed as f64 / 1000.0]);
       
       // Normalize to 0-255
       let height = ((value + 1.0) / 2.0 * 255.0) as u8;
       terrain[y][x] = height;
     }
   }
   
   TerrainMap { data: terrain }
 }
 

}

pub struct Lander {
 pub position: Vec2,
 pub velocity: Vec2,
 pub fuel: u16,
 pub crew_aboard: u8,
 pub inventory: Vec<CollectedItem>,
 }

impl Lander {
 pub fn update(&mut self, input: &InputState, terrain: &TerrainMap, dt: f32) {
 // Movement
 let mut accel = Vec2::ZERO;
 if input.is_key_pressed(VirtualKeyCode::W) {
 accel.y -= 100.0;
 }
 if input.is_key_pressed(VirtualKeyCode::A) {
 accel.x -= 100.0;
 }
 // ... etc ...

  self.velocity += accel * dt;
   self.velocity *= 0.95; // Friction
   self.position += self.velocity * dt;
   
   // Gravity
   let ground_height = terrain.height_at(self.position);
   if self.position.y > ground_height as f32 {
     self.velocity.y += 200.0 * dt; // 200 units/s^2 gravity
   }
   
   // Fuel consumption
   let speed = self.velocity.magnitude();
   if speed > 1.0 {
     self.fuel = self.fuel.saturating_sub((speed * 0.1) as u16);
   }
 }
 

}

**Deliverables Fase 7:**

·     ✅ Terrain generation (Perlin noise)

·     ✅ Lander physics y control

·     ✅ Resource collection system

·     ✅ Creature encounters

·     ✅ Tests vs golden reference



**Fase 8: Integración Final y Polish (2-3 semanas)**

**Objetivo:** Integración de todos los módulos, testing, optimización

**Tareas:**

·     [ ] Integración de UI menu system

·     [ ] Save/load system

·     [ ] Conversation/dialogue system

·     [ ] Full game integration testing

·     [ ] Performance profiling y optimization

·     [ ] Bug fixing

·     [ ] Documentation final

·     [ ] Entrega release

**Estructura Final:**
 uqm-rust/
 ├── src/
 │ ├── [main.rs](http://main.rs) (Punto de entrada)
 │ ├── [lib.rs](http://lib.rs) (API pública)
 │ ├── core/
 │ │ ├── [mod.rs](http://mod.rs)
 │ │ ├── [types.rs](http://types.rs) ✅ Fase 1
 │ │ ├── [state.rs](http://state.rs) ✅ Fase 1
 │ │ └── [flags.rs](http://flags.rs)
 │ ├── graphics/
 │ │ ├── [mod.rs](http://mod.rs)
 │ │ ├── [renderer.rs](http://renderer.rs) ✅ Fase 2
 │ │ ├── [sprite.rs](http://sprite.rs) ✅ Fase 2
 │ │ └── [ui.rs](http://ui.rs) ✅ Fase 8
 │ ├── input/
 │ │ └── [mod.rs](http://mod.rs) ✅ Fase 3
 │ ├── audio/
 │ │ └── [manager.rs](http://manager.rs) ✅ Fase 4
 │ ├── starmap/
 │ │ ├── [mod.rs](http://mod.rs) ✅ Fase 5
 │ │ ├── [map.rs](http://map.rs) ✅ Fase 5
 │ │ ├── [navigation.rs](http://navigation.rs) ✅ Fase 5
 │ │ ├── [encounters.rs](http://encounters.rs) ✅ Fase 5
 │ │ └── [events.rs](http://events.rs) ✅ Fase 5
 │ ├── melee/
 │ │ ├── [mod.rs](http://mod.rs) ✅ Fase 6
 │ │ ├── [ship.rs](http://ship.rs) ✅ Fase 6
 │ │ ├── [weapons.rs](http://weapons.rs) ✅ Fase 6
 │ │ ├── [ai.rs](http://ai.rs) ✅ Fase 6
 │ │ ├── [collision.rs](http://collision.rs) ✅ Fase 6
 │ │ └── [battle.rs](http://battle.rs) ✅ Fase 6
 │ ├── planet/
 │ │ ├── [mod.rs](http://mod.rs) ✅ Fase 7
 │ │ ├── [terrain.rs](http://terrain.rs) ✅ Fase 7
 │ │ ├── [lander.rs](http://lander.rs) ✅ Fase 7
 │ │ └── [creatures.rs](http://creatures.rs) ✅ Fase 7
 │ ├── conversation/
 │ │ └── [mod.rs](http://mod.rs) ✅ Fase 8
 │ ├── menu/
 │ │ └── [mod.rs](http://mod.rs) ✅ Fase 8
 │ └── resource/
 │ └── [manager.rs](http://manager.rs) ✅ Fase 8
 ├── tests/
 │ ├── integration_tests.rs
 │ ├── melee_tests.rs
 │ ├── starmap_tests.rs
 │ ├── planet_tests.rs
 │ └── fixtures/
 ├── golden_reference/ (Python reference)
 │ ├── uqm_golden.py
 │ ├── melee_golden.py
 │ ├── starmap_golden.py
 │ └── planet_golden.py
 └── Cargo.toml



**7. Plan de Testing Integral**

**7.1 Estrategia de Testing Multinivel**

┌──────────────────────────────────────────┐
 │ Golden Reference (Python) │
 │ - Define comportamiento esperado │
 │ - 200+ test cases documentados │
 └──────────┬───────────────────────────────┘
 │
 ├──────────────────┬─────────────────────┐
 │ │ │
 ┌──────▼──────┐ ┌─────▼──────┐ ┌──────▼──────┐
 │ Unit Tests │ │ Integration│ │ System Tests │
 │ (Rust) │ │ Tests │ │ (Rust) │
 │ │ │ (Rust) │ │ │
 │ - Types │ │ │ │ - Full game │
 │ - Algorithms│ │ - Modules │ │ - Platforms │
 │ - Edge cases│ │ - Interfaces │ - Performance
 └─────────────┘ └────────────┘ └──────────────┘
 │ │ │
 └──────────────────┼─────────────────────┘
 │
 ┌───────▼────────┐
 │ CI/CD (GitHub │
 │ Actions) │
 │ │
 │ - Build check │
 │ - All tests │
 │ - Benchmarks │
 │ - Coverage │
 └────────────────┘

**7.2 Test Organization**

**Unit Tests (per module):**
 cargo test --lib # Run all unit tests

**Ejemplo: tests/melee_unit_tests.rs**

\#[cfg(test)]
 mod melee_tests {
 use uqm::melee::*;

\#[test]
 fn test_ship_energy_consumption() { ... }
 
 \#[test]
 fn test_weapon_cooldown() { ... }
 
 \#[test]
 fn test_ai_decision_attack() { ... }
 

}

**Integration Tests:**
 cargo test --test '*' # Run integration tests

**Ejemplo: tests/starmap_integration_test.rs**

\#[test]
 fn test_full_travel_sequence() {
 // Start new game
 let mut game = GameState::new();

// Travel from Sol to Alpha Centauri
 game.travel(StarIndex(1)).unwrap();
 
 // Verify state updated correctly
 assert_eq!(game.current_location(), StarIndex(1));
 assert!(game.fuel() < 1000);
 

}

**Property-based Tests (con proptest):**
 \#[cfg(test)]
 mod property_tests {
 use proptest::prelude::*;

proptest! {
   \#[test]
   fn prop_fuel_never_negative(
     initial_fuel in 0u32..10000,
     distances in prop::collection::vec(0.0..1000.0, 1..100)
   ) {
     // Test property: fuel should never go negative
     // even with random travel distances
   }
 }
 

}

**7.3 Validation vs Golden Reference**

**Script de comparaci****ón:**

**tests/golden_validation.py**

import subprocess
 import json

def test_behavior_matches_golden():
 """Rust implementation debe producir resultados idénticos a Python"""

test_cases = [
   {"ship_id": 1, "distance": 100.0, "expected_fuel": 750},
   {"ship_id": 1, "distance": 50.0, "expected_fuel": 875},
   {"ship_id": 2, "distance": 100.0, "expected_fuel": 400},
 ]
 
 for case in test_cases:
   \# Run Python golden reference
   py_result = run_python_golden(case)
   
   \# Run Rust implementation
   rust_result = run_rust_binary(case)
   
   \# Compare results
   assert py_result == rust_result, \
     f"Mismatch: Python {py_result} != Rust {rust_result}"
 

**7.4 Benchmarking y Performance Targets**

**Cargo.toml:**
 [[bench]]
 name = "melee_benchmark"
 harness = false

[[bench]]
 name = "starmap_benchmark"
 harness = false

**Benches/melee_benchmark.rs:**
 use criterion::{black_box, criterion_group, criterion_main, Criterion};
 use uqm::melee::*;

fn bench_battle_update(c: &mut Criterion) {
 c.bench_function("battle_update_60fps", |b| {
 b.iter(|| {
 // Simular 1 frame de combate
 // Target: <16ms (para 60 FPS)
 });
 });
 }

criterion_group!(benches, bench_battle_update);
 criterion_main!(benches);

**Resultados esperados (vs C legacy):**

| Métrica                 | C Original | Rust  | Target    |
| ----------------------- | ---------- | ----- | --------- |
| Battle frame update     | 12ms       | <8ms  | 10x mejor |
| Terrain generation      | 150ms      | <30ms | 5x mejor  |
| Sprite rendering (1000) | 45ms       | <3ms  | 15x mejor |
| Game startup            | 2-3s       | <0.5s | 5x mejor  |

 



**8. Consideraciones Especiales para Migración**

**8.1 Manejo de Assets Legacy**

**El c****ódigo C carga assets compilados. En Rust:**

// Opción 1: Cargar desde archivos
 impl ResourceManager {
 pub fn load_sprite(&self, id: SpriteId) -> Result<Sprite> {
 let path = format!("assets/sprites/{}.png", id.0);
 let image = image::open(path)?;
 Ok(Sprite::from_image(image))
 }
 }

// Opción 2: Embeber assets en binary (release)
 \#[cfg(not(debug_assertions))]
 const SPRITE_SHOFIXTI: &[u8] = include_bytes!("../assets/ships/shofixti.png");

**8.2 Compatibilidad de Plataforma**

**C legacy soportaba:**

·     DOS (assembly)

·     Windows 95/98

·     Linux x86

·     macOS PPC

**Rust moderno soporta:**

·     Windows (x64, ARM64)

·     Linux (x64, ARM64)

·     macOS (Intel, Apple Silicon)

·     Web (WebAssembly - bonus)

**Estrategia:**
 [target.x86_64-unknown-linux-gnu]

**Linux 64-bit**

[target.x86_64-apple-darwin]

**macOS Intel**

[target.aarch64-apple-darwin]

**macOS Apple Silicon**

[target.wasm32-unknown-unknown]

**Web browser (bonus)**

**8.3 Migración Incremental de FFI (si necesario)**

Si requiere integración con código C existente:

// src/ffi/mod.rs
 \#[link(name = "uqm_legacy")]
 extern "C" {
 pub fn legacy_load_sprite(id: c_uint) -> *const c_void;
 pub fn legacy_free_sprite(sprite: *const c_void);
 }

pub struct LegacySprite(*const c_void);

impl LegacySprite {
 pub unsafe fn from_id(id: u32) -> Self {
 LegacySprite(legacy_load_sprite(id))
 }
 }

impl Drop for LegacySprite {
 fn drop(&mut self) {
 unsafe {
 legacy_free_sprite(self.0);
 }
 }
 }



**9. Estimaciones de Esfuerzo**

**9.1 Timeline por Fase**

| Fase      | Descripción              | Duración          | Esfuerzo (Dev-hours) |
| --------- | ------------------------ | ----------------- | -------------------- |
| 0         | Setup & Golden Reference | 2-3 semanas       | 80-120               |
| 1         | Core Types & Structures  | 2-3 semanas       | 60-90                |
| 2         | Graphics Layer           | 3-4 semanas       | 120-160              |
| 3         | Input System             | 1-2 semanas       | 40-60                |
| 4         | Audio System             | 1-2 semanas       | 40-60                |
| 5         | Starmap Module           | 4-5 semanas       | 160-200              |
| 6         | Melee Combat             | 5-6 semanas       | 200-280              |
| 7         | Planet Landing           | 3-4 semanas       | 120-160              |
| 8         | Integration & Polish     | 2-3 semanas       | 100-140              |
| **TOTAL** |                          | **24-32 semanas** | **920-1290 hours**   |

 

**9.2 Equipo Recomendado**

·     **1 Rust Architect** (tiempo completo) - Diseño, revisión, decisiones técnicas

·     **1-2 Rust Developers** (tiempo completo) - Implementación de módulos

·     **1 QA/Tester** (tiempo parcial) - Testing, validación

·     **1 DevOps** (tiempo parcial) - CI/CD, builds, infraestructura

**Total:** 3-4 personas a tiempo completo durante 6-8 meses



**10. Riesgos y Mitigación**

| Riesgo                                     | Probabilidad | Impacto | Mitigación                         |
| ------------------------------------------ | ------------ | ------- | ---------------------------------- |
| **Aprendizaje Rust lento**                 | Alta         | Alto    | Training previo, pair programming  |
| **Cambios scope mid-proyecto**             | Media        | Alto    | Strict version control, milestones |
| **Performance issues**                     | Media        | Medio   | Benchmarking temprano, profiling   |
| **Incompatibilidad assets  legacy**        | Baja         | Medio   | Validación en Fase 0               |
| **Integraci****ón final problem****ática** | Baja         | Alto    | Integration tests constantes       |

 



**11. Criterios de Éxito**

·     ✅ **Funcionalidad:** 100% feature parity con C original

·     ✅ **Performance:** Todas las operaciones <60 FPS baseline (16.67ms/frame)

·     ✅ **Testing:** >85% code coverage, todos los golden reference tests pasan

·     ✅ **C****ódigo:** Zero unsafe code (excepto FFI necesario), clippy clean

·     ✅ **Documentaci****ón:** Rust-doc para todas las APIs públicas

·     ✅ **Entrega:** Build final <30MB, startup <500ms



**Conclusión**

La migración de The Ur-Quan Masters de C a Rust es **t****écnicamente viable y recomendada**. El proyecto beneficiará de:

1. **Seguridad de memoria garantizada** sin GC overhead
2. **Modern ecosystem** (Bevy, wgpu, tokio)
3. **Better maintainability** mediante type system fuerte
4. **Extensibilidad** para nuevas características (WebAssembly, networking)
5. **Performance** superior al original

La estrategia de 8 fases con golden reference en Python proporciona **validaci****ón rigurosa** en cada paso. El esfuerzo estimado (900-1300 horas) es **justificado** por el beneficio a largo plazo.

**Recomendaci****ón final:** Proceder con Fase 0 inmediatamente. Si en Fase 2 los benchmarks confirman >5x mejora de rendering, continuar con confianza hacia Fases 3-8.