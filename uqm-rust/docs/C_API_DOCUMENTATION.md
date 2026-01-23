# C API Documentation

This document provides detailed documentation of the original C codebase APIs
to facilitate the Rust migration.

## Core Types (sc2/src/types.h)

### Basic Types

```c
typedef unsigned char BYTE;    // 8-bit unsigned
typedef unsigned short WORD;   // 16-bit unsigned
typedef unsigned int DWORD;    // 32-bit unsigned
typedef signed char SBYTE;     // 8-bit signed
typedef signed short SWORD;    // 16-bit signed
typedef signed int SDWORD;     // 32-bit signed
```

### Point Structure

```c
typedef struct {
    BYTE x, y;
    WORD coords[2];
} POINT;
```

**Rust Equivalent:**
```rust
pub struct Point {
    pub x: i16,
    pub y: i16,
}
```

## Star System API (sc2/src/uqm/planets/plandata.c)

### STAR_DESC Structure

```c
typedef struct star_desc {
    BYTE star_index;        // Unique identifier
    WORD x, y;              // HyperSpace coordinates (0-9999)
    BYTE type;              // DWARF_STAR, GIANT_STAR, SUPER_GIANT_STAR
    BYTE color;             // RED_BODY, ORANGE_BODY, YELLOW_BODY, etc.
    BYTE prefix;            // Name prefix (0=none, 1=Alpha, 2=Beta...)
    BYTE postfix;           // Constellation name index
    BYTE star_size;         // Visual size
} STAR_DESC;
```

**Constants:**
- `DWARF_STAR = 0`
- `GIANT_STAR = 1`
- `SUPER_GIANT_STAR = 2`

**Colors:**
- `BLUE_BODY = 0`
- `GREEN_BODY = 1`
- `ORANGE_BODY = 2`
- `RED_BODY = 3`
- `WHITE_BODY = 4`
- `YELLOW_BODY = 5`

**Rust Equivalent:**
```rust
pub struct Star {
    pub index: StarIndex,
    pub position: Point,
    pub star_type: StarType,
    pub color: StarColor,
    pub name: String,
    pub num_planets: u8,
}

pub enum StarType {
    Dwarf,
    Giant,
    SuperGiant,
}

pub enum StarColor {
    Blue, Green, Orange, Red, White, Yellow,
}
```

## Planet API (sc2/src/uqm/planets/planets.h)

### PLANET_DESC Structure

```c
typedef struct planet_desc {
    BYTE data_index;        // Planet type index
    BYTE radius;            // Orbital distance
    BYTE angle;             // Angular position
    BYTE rand_seed;         // Generation seed
    COUNT NumPlanets;       // Number of moons
    BYTE flags;             // PLANET_SHIELDED, etc.
} PLANET_DESC;
```

**Planet Types:**
- `ROCKY_WORLD = 0`
- `GAS_GIANT = 1`
- `WATER_WORLD = 2`
- `ICE_WORLD = 3`
- `TREASURE_WORLD = 4`
- `RAINBOW_WORLD = 5`

**Rust Equivalent:**
```rust
pub struct Planet {
    pub planet_type: PlanetType,
    pub orbital_radius: f32,
    pub angle: f32,
    pub seed: u32,
    pub moons: Vec<Moon>,
    pub shielded: bool,
}
```

## Ship API (sc2/src/uqm/ships/)

### SHIP_INFO Structure

```c
typedef struct ship_info {
    BYTE ship_type;         // Ship type ID (0-24)
    BYTE crew;              // Current crew
    BYTE max_crew;          // Maximum crew capacity
    BYTE energy;            // Current energy
    BYTE max_energy;        // Maximum energy capacity
    POINT location;         // Current position
    POINT velocity;         // Current velocity vector
    BYTE facing;            // Direction (0-31, 360°/32)
    BYTE flags;             // Status flags
} SHIP_INFO;
```

**Ship Types (25 types):**
0. Earthling Cruiser
1. Ur-Quan Dreadnought
2. Spathi Eluder
3. Arilou Skiff
4. ...and 21 more

**Rust Equivalent:**
```rust
pub struct Ship {
    pub ship_type: ShipType,
    pub crew: u8,
    pub max_crew: u8,
    pub energy: u16,
    pub max_energy: u16,
    pub position: Vec2,
    pub velocity: Vec2,
    pub facing: u8,
    pub state: ShipState,
}
```

## Game State API (sc2/src/uqm/gamestate.c)

### GLOBAL_STATE Structure

```c
typedef struct global_state {
    POINT ship_location;    // Current location in HyperSpace
    DWORD fuel;             // Available fuel
    WORD crew;              // Crew count
    DWORD resource_units;   // Collected resources
    BYTE month;             // Current month (0-11)
    BYTE day;               // Current day (0-29)
    WORD year;              // Current year
    DWORD flags;            // Global progression flags
} GLOBAL_STATE;
```

**Key Functions:**

```c
void InitGameState(void);
BOOLEAN SaveGame(const char *filename);
BOOLEAN LoadGame(const char *filename);
void UpdateTime(DWORD days);
BOOLEAN ConsumeResource(RESOURCE_TYPE type, DWORD amount);
```

**Rust Equivalent:**
```rust
pub struct GameState {
    pub location: Point,
    pub fuel: u32,
    pub crew: u32,
    pub resource_units: u32,
    pub time: GameTime,
    pub flags: GameFlags,
}

impl GameState {
    pub fn new() -> Self;
    pub fn save(&self, filename: &str) -> Result<()>;
    pub fn load(filename: &str) -> Result<Self>;
    pub fn consume_fuel(&mut self, amount: u32) -> bool;
}
```

## Graphics API (sc2/src/libs/graphics/)

### Legacy 8-bit Palette System

```c
void DrawPixel(int x, int y, BYTE color_index);
void DrawSprite(SPRITE_T *sprite, int x, int y);
void SetPalette(BYTE *palette, int num_colors);
void FlipBuffers(void);
```

**Modern Rust Replacement:**
Uses wgpu for GPU-accelerated rendering (see Phase 2 implementation plan).

## Audio API (sc2/src/libs/sound/)

```c
void PlayMusic(const char *filename);
void PlaySound(SOUND_ID sound_id);
void SetVolume(float volume);
void StopMusic(void);
```

**Rust Equivalent:**
```rust
impl AudioManager {
    pub fn play_music(&self, path: &str) -> Result<()>;
    pub fn play_effect(&self, path: &str) -> Result<()>;
    pub fn set_volume(&mut self, volume: f32);
    pub fn stop_music(&mut self);
}
```

## Key Differences in Rust Implementation

### Memory Management
- **C**: Manual `malloc()`/`free()`
- **Rust**: Ownership system, automatic deallocation

### Error Handling
- **C**: Return codes, `NULL` pointers
- **Rust**: `Result<T, E>` and `Option<T>`

### Type Safety
- **C**: Implicit conversions, weak typing
- **Rust**: Strong typing, explicit conversions

### Concurrency
- **C**: Manual synchronization
- **Rust**: Compile-time race condition prevention

## Migration Notes

### Critical Functions to Port First (Phase 1)
1. Star system data structures
2. Planet generation seeds
3. Game state management
4. Time system

### Functions Requiring Modern Replacement
1. Graphics system (wgpu instead of direct framebuffer)
2. Input system (winit event loop)
3. Audio system (rodio instead of legacy sound APIs)

### Compatibility Concerns
- Asset file formats (.ani, .ct, .rmp) must remain compatible
- Save game format should support migration from C version
- Planet generation must use same algorithms for deterministic results
