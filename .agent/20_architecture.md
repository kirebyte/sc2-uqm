# Architecture

Overview
--------
System Context: Classic C-based game engine with SDL2 frontend, content-driven gameplay

Actors:
- Player: Interacts via keyboard/mouse, explores galaxy, manages resources
- Game Engine: Manages state, physics, AI, rendering
- Content System: Loads assets (.ani, .ct, .rmp, .ogg files)
- Audio System: PulseAudio backend via SDL2
- Platform: Ubuntu WSL2, X11/Wayland for graphics

Main Flows:
1. Initialization → Resource Loading → Main Menu → New Game/Load Game → Gameplay Loop
2. HyperSpace Navigation → Star System → Planet Scan → Landing → Resource Collection
3. Combat System → Ship Selection → Battle → Victory/Defeat → Rewards
4. Dialogue System → NPC Interaction → Choice Selection → Story Progression

Key Components
--------------

### Core Engine (sc2/src/uqm/)
- **starmap.c**: Galaxy navigation, star system management (503 systems)
- **planets/**: Planet generation, scanning, landing mechanics
  - `generate/`: Procedural generation for each star system
  - `genkitsune.c`: Custom Kitsune system generator
- **ships/**: 25 ship types with unique combat mechanics
- **comm/**: Dialogue system for alien races
- **battle.c**: Real-time combat engine
- **gameopt.c**: Settings and configuration management
- **menu.c**: UI navigation and display
- **settings.c/h**: Cheat mode and global settings

### Content System (sc2/content/)
- **base/**: Core game assets
  - gamestrings.txt: All text strings (dialogue, UI, 134 star names)
  - ships/: Ship graphics and stats (25 types)
  - planets/: Planet textures and data
  - comm/: Alien portraits and dialogue
- **addons/**: Optional content packs
  - 3domusic: High-quality music tracks (.ogg)
  - 3dovoice: Voice acting
  - 3dovideo: Cutscene videos

### Libraries (sc2/src/libs/)
- **graphics/**: SDL2 rendering, OpenGL support, scalers
- **sound/**: Audio engine, OGG decoder, MixSDL
- **input/**: Keyboard/joystick handling
- **uio/**: Universal I/O (file system abstraction, ZIP support)
- **network/**: Netplay for multiplayer melee
- **log/**: Logging system for debugging

### Build System (sc2/build/unix/)
- Shell-based configuration and compilation
- Dependency detection (SDL2, libpng, vorbis, zlib)
- Makefiles for incremental builds
- Supports debug and release builds

### Developer Tools (tools/)
- **add_star_wizard.py**: Interactive tool to create new star systems
  - Generates C code for planet/moon configurations
  - Updates plandata.c, gendef.c/h, gamestrings.txt
  - Creates generator files automatically

Data Model
----------

### Core Entities

**STAR_DESC** (plandata.c):
- Coordinates (x, y): HyperSpace position (0-9999)
- Type: DWARF_STAR, GIANT_STAR, SUPER_GIANT_STAR
- Color: RED_BODY, ORANGE_BODY, YELLOW_BODY, GREEN_BODY, BLUE_BODY, WHITE_BODY
- Index: Generator function identifier (presence constant)
- Prefix: Name prefix index (0=none, 1=Alpha, 2=Beta...)
- Postfix: Constellation name index (points to gamestrings.txt)

**PLANET_DESC** (planets.h):
- data_index: Planet type (WATER_WORLD, TREASURE_WORLD, RAINBOW_WORLD, etc.)
- radius: Orbital distance (multiples of EARTH_RADIUS)
- location: Angular position (COSINE/SINE macros)
- NumPlanets: Moon count
- rand_seed: Generation seed for reproducibility
- PLANET_SHIELDED flag: Indicates slave-shielded worlds

**SOLARSYS_STATE** (solarsys.h):
- SysInfo: Current system data
- PlanetInfo: Active planet details (tectonics, weather, resources)
- MoonDesc[]: Array of moons (up to MAX_MOONS)
- genFuncs: Function pointers (generatePlanets, generateMoons, etc.)
- pOrbitalDesc: Currently selected planet/moon

**Ship Types** (25 unique):
- Earthling Cruiser, Arilou Skiff, Chenjesu Broodhome
- Ur-Quan Dreadnought, Kohr-Ah Marauder
- Each with: crew, battery, max speed, turning rate
- Weapon systems, special abilities
- AI behavior patterns

Quality Attributes
------------------

### Reliability
- Deterministic planet generation (seed-based, reproducible)
- Save game system with validation
- Error handling for missing content
- Graceful degradation (warnings for missing files)
- Binary search on sorted starmap (performance + correctness)

### Maintainability
- Zero compiler warnings (achieved in modernization branch)
- Comprehensive documentation (8 HOW_TO guides in sc2/doc/)
- Modular generator system (easy to add star systems)
- Clear separation: engine / content / platform layers
- Consistent naming conventions

### Performance
- Optimized asset loading (lazy loading, caching)
- 60 FPS target achieved
- Efficient planet generation (on-demand, not pre-computed)
- Audio streaming for music (not loaded fully into RAM)
- OpenGL hardware acceleration

### Extensibility
- Generator function pointers allow custom systems
- String table system for easy localization
- Addon system for optional content (detected automatically)
- Menu system extensible (demonstrated by cheat mode addition)
- Python tools for content creation

Risks & Mitigations
-------------------

1. **Legacy Code Complexity**
   - Risk: Hard to understand/modify 20+ year old codebase
   - Mitigation: ✅ Comprehensive documentation created, gradual refactoring, preserve working features

2. **Platform Dependencies**
   - Risk: SDL2/OpenGL versions, audio drivers vary
   - Mitigation: ✅ Version detection in build system, fallback renderers, multiple audio backends

3. **Content Integrity**
   - Risk: Missing or corrupted game assets
   - Mitigation: ✅ .rmp index validation, graceful fallbacks, clear error messages

4. **Memory Management**
   - Risk: Manual C memory management, potential leaks
   - Mitigation: Consistent patterns, wrapper functions (HMalloc/HFree), debug builds with tracking

5. **Segfault on Exit**
   - Risk: Known issue with cleanup sequence (observed 2026-01-23)
   - Mitigation: Gameplay stable, cleanup can be fixed in future iteration
   - Priority: Low (doesn't affect playability)
