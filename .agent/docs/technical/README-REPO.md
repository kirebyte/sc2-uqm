# The Ur-Quan Masters (sc2-uqm) Repository Guide

## Quick facts
- **Project goal:** Open-source continuation of *Star Control II* (3DO release) under the name *The Ur-Quan Masters*.
- **Primary language:** C, with POSIX-style build tooling and SDL/OpenAL backends for graphics and audio.
- **License:** GNU GPLv2 (see `sc2/COPYING`). All content assets remain under their original owners' terms.
- **Canonical upstream:** https://sc2.sourceforge.net/ (this tree mirrors the classic open-source drop).

## Getting started fast
1. Install dependencies: SDL2, PNG, Ogg Vorbis, and zlib. (See `sc2/INSTALL` or the platform-specific `INSTALL.*` files for exact package names.)
2. From the `sc2/` directory, run `./build.sh uqm [-jN]`. The script handles configuration, dependency probing, and compilation.
3. Launch the freshly built binary (`sc2/uqm` or `sc2/uqm-debug`). Use `--help` for runtime flags (parsed in `sc2/src/uqm.c`).
4. Supply game data from `sc2/content/base/` (already included here) and drop optional addons or voice packs into `sc2/content/addons/`.

## Repository layout

### Root
| Path | Purpose |
| --- | --- |
| `sc2/` | Main game source, content packs, build system, and documentation. This is the historical upstream tree. |
| `misc/` | Optional voice-over WAV packs (e.g., Melnorme trader speeches) and probe speech assets used when generating custom content. |
| `symbian/` | Legacy Symbian client project files, headers, and documentation for the Series 60 port (includes custom Tremor patch, resources, and SIS packaging helpers). |
| `tools/` | Standalone utilities for converting and packaging legacy assets (ABX↔WAV, AIFF tools, animation extractors, palette editors, resource mappers, etc.). |
| `.gitignore` | VCS hygiene matching generated build artefacts. |

### `sc2/` top-level
| Path | Purpose |
| --- | --- |
| `build/` | Cross-platform build orchestration scripts. Contains interactive Unix build driver, MSVC6 project scaffolding, Windows installer glue, and packaging helpers. |
| `build.sh`, `build.vars.in` | Entry script and template variables for the build system. |
| `ChangeLog`, `WhatsNew`, `BUGS` | Upstream history, release notes, and known issues. |
| `content/` | Packed art, audio, dialogue, and addon definitions consumed at runtime. |
| `doc/` | User manuals (`doc/users/`) and in-depth developer docs (`doc/devel/`) covering engine subsystems, assets, threading, scripting, and tooling. |
| `src/` | All C sources and headers for the engine, subsystems, and platform integration layers. |
| `subst/`, `Makefile.build`, `Makeproject`, `uqm-indent` | Build automation helpers and formatting scripts used by upstream maintainers. |
| `README`, `README-SDL`, `INSTALL*` | Upstream overview plus platform-specific build guides (Win32, macOS, Symbian, pkg guidance). |
| `uqm.lsm`, `AUTHORS`, `Contributing` | Packaging metadata and project governance documents. |

### `sc2/content/`
| Path | Purpose |
| --- | --- |
| `base/` | Canonical shipped assets: battle animations, comm portraits, cutscenes, fonts, lander art, planetary data, ship definitions, and UI sprites. |
| `addons/` | Slot for optional music packs, remixes, or voice data. The build/runtime loader merges addon directories in priority order. |
| `menu.key`, `uqm.key`, `uqm.rmp`, `version` | Resource indices that map runtime identifiers back to packed content files. |

### `sc2/src/` overview
| Path | Purpose |
| --- | --- |
| `uqm.c`, `options.c` | Entry point (`main`) and command-line/config parser. They bootstrap logging, SDL/OpenAL backends, and game flow. |
| `uqm/` | Core game logic for campaign, combat, UI, resource management, dialogue, and subsystems. Subdirectories include `comm/`, `planets/`, `ships/`, and `supermelee/`. |
| `libs/` | Portable engine libraries (graphics, audio, IO, resource, threading, networking, etc.) abstracting platform services away from gameplay code. |
| `abxadec/` | ADPCM decoder used for legacy ABX audio assets. |
| `darwin/`, `symbian/` | Platform glue layers and Objective-C/C++ wrappers for macOS and Symbian builds. |
| `getopt/`, `regex/` | Bundled fallbacks for GNU `getopt` and regex routines when not provided by the host system. |
| `res/` | Platform resource files (icons, manifests, Windows resource scripts). |
| `types.h`, `port.c`, `port.h`, `config_*.h` | Type aliases, platform abstraction helpers, and configure-time header templates. |
| `Makeinfo` | Maintainer metadata for generating dependency trees. |

### `sc2/src/libs/` highlights
| Library | Focus |
| --- | --- |
| `graphics/`, `gfxlib.h` | Rendering backend with legacy 3DO blitters plus modern `TFB_Draw*` APIs, SDL surface abstraction, canvas management, palette utilities, and draw command queues. |
| `sound/`, `sndlib.h`, `mikmod/` | Audio mixer abstraction (MixSDL/OpenAL), module player integration, streaming buffers, and 3DO/PC music path handling. |
| `input/`, `inplib.h` | Unified input device polling (keyboard, joystick, netplay) with configurable control templates. |
| `video/`, `vidlib.h` | Full-motion video decoding and playback surfaces. |
| `network/`, `net.h`, `callback/`, `alarm.h` | Netplay support (sockets, callbacks, alarms) for SuperMelee multiplayer. |
| `task/`, `threads/`, `threadlib.h`, `time/` | Cooperative threading abstractions (`Task`, `Thread`, mutexes, semaphores) built atop SDL threading primitives. |
| `file/`, `resource/`, `uio/`, `reslib.h` | Virtual file systems, package readers, buffered IO helpers, save/config file management. |
| `memory/`, `heap/`, `memlib.h` | Custom allocators and linear heaps tuned for consoles and memory-constrained devices. |
| `strings/`, `unicode.h`, `strlib.h` | Game-wide string tables, Unicode helpers, and resource name resolution. |
| `log/`, `log.h` | Structured logging with per-module levels and thread-safe sinks. |
| `math/`, `mathlib.h`, `md5/` | Game math helpers (fixed-point arithmetic, vector ops), MD5 utilities for asset verification. |
| `async.h`, `alarm.h`, `timelib.h` | Asynchronous callbacks, timers, and event scheduling. |

### Core gameplay modules (`sc2/src/uqm/`)
The `uqm/` directory implements campaign flow, combat, media presentation, and content integration. The list below consolidates upstream notes (`doc/devel/files`) with additional modules introduced since the original release.

#### Bootstrapping & global systems
- `uqm.c` (root) – program entry: parses CLI, configures logging, loads config/content directories, boots graphics, audio, and the main state machine.
- `init.c` / `cleanup.c` – lifecycle hooks for initializing and tearing down global state, resources, and background tasks.
- `setup.c` / `setupmenu.c` – input setup, key templates, controller configuration, interactive setup menu UI.
- `settings.c` – back-end for persisting user options (`uqm.cfg`) and applying them at startup.
- `options.c` (root) – command-line flag definitions, default options, parsing helpers, and usage output.
- `globdata.c` – seeds global data structures (player ship, radar state), handles save-path discovery.
- `build.c` / `cons_res.c` – construct and register resource packages (images, strings, audio) used during runtime.
- `load.c` / `load_legacy.c` / `save.c` – serialization of campaign saves (modern and legacy formats) including validation and upgrade logic.
- `state.c` – cache of per-planet/per-location state (resources gathered, visited flags) stored in save games.
- `master.c` – preloads ship manifests and melee rosters into `master_q` for quick selection and sorting.
- `clock.c` – advances the in-game calendar, manages pauses, and synchronizes timed events.
- `process.c` – shared combat math routines (damage resolution, status updates).
- `util.c` – utility helpers (pausing, random seeds, string helpers) leveraged throughout gameplay code.
- `displist.c` – reusable doubly linked list with optional preallocated pools, used for game object queues.
- `uqmdebug.c` – runtime debugging helpers (assertions, logging shortcuts).
- `dummy.c` – null-object implementations used to stub out optional features.
- `resinst.h`, `coderes.h`, `gamestr.h`, `nameref.h` – resource index headers generated from content packs.

#### Universe simulation & campaign progression
- `galaxy.c` – defines hyperspace maps, star neighborhoods, and background rendering cues.
- `hyper.c` – player travel in hyperspace, random encounter hooks, and starmap navigation logic.
- `starmap.c` – star lookup by coordinates, procedural name construction, and UI overlays.
- `encount.c` – determines encounter outcomes (alien communications, battles, random events).
- `intel.c` – routes story progression input (computer or player), updates quest triggers.
- `gameev.c` – schedules timed events (Kohr-Ah march, Thraddash missions, rebellion triggers).
- `ipdisp.c` – draws spheres of influence and discovered planets on the starmap.
- `starcon.c` – high-level campaign events (Arilou portals, genocides, Umgah prank, Ur-Quan war clock).
- `starbase.c` – Earth Starbase interactions, including upgrade menus and story gating.
- `sis.c` – status/info screen overlays (ship stats, coordinates, credits, cargo, time).
- `outfit.c` – outfitting workflow for modules, landers, and resources aboard the flagship.
- `shipyard.c` – manages escort roster, ship purchasing, and crew/fuel logistics.
- `pickship.c` – full-game ship selection UI (distinct from SuperMelee fleet editor).
- `menu.c` / `restart.c` – title screen, new/load game handling, entry points into campaign vs. SuperMelee.
- `demo.c` – handles attract mode, journal playback, automated demo sequences.
- `credits.c` – endgame credits sequencing.
- `intro.c` – alternative intro/outro sequences (3DO vs PC variants).
- `planets/` – entire planetary subsystem: surface generation (`planets.c`, `plangen.c`, `generate/`), orbit calculations (`orbits.c`), lander simulation (`lander.c`, `surface.c`), scanning (`scan.c`), and resource reports (`report.c`).
- `plandata.c`, `sundata.h`, `elemdata.h` – static definitions mapping stars, elements, planetary hazards, and loot tables.
- `gameinp.c`, `getchar.c` – input routing for campaign UI, name entry with joystick/keyboard support.

#### Combat, physics, and AI
- `battle.c` – orchestrates real-time combat loop, updates ships, shots, and effects each frame.
- `battlecontrols.c` – maps control templates to combat commands, handles pause, menu, and AI toggles.
- `ship.c` / `weapon.c` / `tactrans.c` – ship kinematics, weapon firing logic, combat transitions (explosions, respawns, ion trails, escape pods).
- `velocity.c` / `gravity.c` – physics helpers computing thrust, inertia, and gravitational pull around celestial objects.
- `collide.c` – collision detection/resolution between ships, projectiles, asteroids, and debris.
- `misc.c` – assorted combat support (camera shake, debris, special effects not tied directly to weapon definitions).
- `cyborg.c` – human/cyborg AI routines for combat opponents.
- `shipstat.c` / `status.c` / `border.c` – draws combat HUD (crew, energy, ship icons) and screen borders.
- `sounds.c` / `flash.c` – synchronizes sound effects and screen flashes with combat events.
- `process.c` (shared) – calculates combat state updates (damage application, cooldowns).
- `supermelee/` – SuperMelee menus, roster build UI (`buildpick.c`), fleet loading (`loadmele.c`), netplay client (`netplay/`), and vs. setup (`meleesetup.c`).
- `pickmele.c` / `melee.c` – SuperMelee selection and main loop (distinct from campaign ship selection).
- `ships/` – per-species ship implementations (movement stats, special abilities, captain voice hooks, AI overrides).

#### Narrative, media, and communications
- `comm/` (plus `comm.c`, `commanim.c`, `commglue.c`) – dialogue trees, alien portraits, lip-sync & animation sequencing, speech playback, and conversation scripting glue.
- `oscill.c` – oscilloscope display and speech progress sliders during conversations.
- `fmv.c` – full-motion video playback support (3DO intro/outro, cutscenes).
- `flash.c` – handles cinematic flashes and transitions reused in cutscenes and combat.
- `sounds.c` – overall sound trigger dispatch (battle, UI, comms) linking to `libs/sound` mixers.
- `cnctdlg.c` / `confirm.c` – modal dialogues for confirmation prompts, network connect dialogues.
- `menu.c` / `setupmenu.c` – interactive menus (options, load/save lists, key bindings).
- `credits.c`, `intro.c`, `demo.c` – cinematic and story presentation wrappers.

#### Support utilities & bridging code
- `grpinfo.c` – manages combat group metadata for roster displays.
- `gendef.c` – tables of function pointers used to generate solar systems procedurally.
- `gameopt.c` – general options menu including save/load screens.
- `sounds.c`, `imusicre.h`, `isndres.h` – registers music/sound resources, integrates with resource indices.
- `uqmdebug.c` – debug printouts and asserts (mirrors `libs/log`).
- `trans.c` – integer-based `atan2` helper used in tight loops.
- `intel.h`, `controls.h`, `shipcont.h`, `units.h`, `colors.h` – headers describing shared structures, control constants, and drawing palettes.

## Runtime architecture (high level)
1. **Bootstrap:** `main` in `sc2/src/uqm.c` parses command-line options (`options.c`), initializes logging, SDL, audio drivers, and cooperative threading (`libs/tasklib`, `libs/threads`).
2. **Resource registration:** `build.c`, `cons_res.c`, `loadship.c`, and `master.c` populate resource tables (images, strings, ships). The virtual filesystem (`libs/reslib`) maps logical identifiers to packed assets in `content/`.
3. **Task scheduling:** The engine uses cooperative "Task" objects (documented in `doc/devel/threads`) to run the main loop, audio mixer, graphics queues, and background timers. Synchronization relies on mutexes, semaphores, and condition variables supplied by `libs/threads`.
4. **Main loop:** Controlled by `StarCon2Main` (in `uqm/`), which advances the global clock, processes input, updates world state (`hyper.c`, `gameev.c`, `starcon.c`), and invokes the appropriate scene stack (navigation, conversations, combat, SuperMelee) depending on player location and story triggers.
5. **Rendering pipeline:** Gameplay modules emit drawing commands through the legacy frame system (`battle.c`, `sis.c`, etc.) which now routes through `TFB_Draw*` APIs and the draw command queue (`libs/graphics`). Background threads flush the queue to SDL surfaces for display.
6. **Audio/video:** Music and speech playback (`libs/sound`, `fmv.c`, `comm/`) stream via SDL/OpenAL; MixSDL is the default but can be swapped for OpenAL using command-line flags.
7. **Persistence:** Saves and configuration files are serialized using the resource system (`save.c`, `state.c`, `uuq.cfg`) and stored under the user's config path determined at bootstrap.

## Content and tooling pipeline
- **Core assets:** `sc2/content/base/` stores all canonical art/audio; indices (`*.rmp`, `*.key`) describe package layouts consumed by `libs/reslib`.
- **Voice packs:** Optional speech in `misc/` can be imported or remixed using converters in `tools/`.
- **Conversion utilities:** The `tools/` directory includes dozens of format-specific helpers:
  - `abx/`, `wav2abx.c`, `abx2wav.c` – transform 3DO ABX audio.
  - `aif/` – AIFF ⇄ WAV conversion.
  - `ani/`, `dos-ani/`, `mng/` – animation decoders and converters.
  - `ctconv/`, `ctmap.sh` – colour table manipulation for sprites.
  - `pkg/`, `resmap/`, `rmpver/` – rebuild resource bundles.
  - `maskimg/`, `map/`, `keys/`, etc. – assorted asset helpers.
- **Developer docs:** `sc2/doc/devel/` contains subsystem-specific guides (graphics library, threading, save files, plugin architecture, scripting, audio). `doc/users/` includes installation and gameplay manuals.

## Platform ports
- **Unix/macOS:** Primary build path via `sc2/build/unix/`. Scripts support native and cross-builds; see `README`, `README.crossbuild`, and `README.packages` for dependency hints.
- **Windows:** Legacy MSVC6 projects (`sc2/build/msvc6/`) and NSIS-based installer files under `sc2/build/win32_install/`.
- **Symbian:** Dedicated source under `symbian/` with platform UI, resource headers (`inc/`), and SIS packaging instructions (`doc/`). Includes a Tremor Ogg Vorbis patch.
- **Legacy consoles/mac:** `sc2/src/darwin/` and `sc2/src/symbian/` host platform glue for historical ports.

## Developer checkpoints
- **Testing:** Automated tests are minimal; rely on manual playtesting, though `tools/valgrind/` and `tools/tcpdump/` assist with diagnostics. Always run `./build.sh uqm clean && ./build.sh uqm` before committing engine changes.
- **Code style:** See `uqm-indent` and `Contributing` for formatting and patch submission rules.
- **Localization & strings:** Managed via string tables referenced in `gamestr.h` and `doc/devel/strtab`.

## Requirements coverage
- Project analysis ✅
- Directory structure documentation ✅
- Module design summary for core game systems ✅

## Quality gates
- **Build:** ✅ **SUCCESSFUL** - Compiled without errors on Ubuntu WSL2
- **Runtime:** ✅ **WORKING** - Game launches and loads resources correctly
- **Dependencies:** ✅ **RESOLVED** - All required libraries installed and linked

## Compilation Results ✅

### Successfully compiled on Ubuntu WSL2 with:

- **System:** Ubuntu on Windows WSL2
- **Compiler:** GCC with C99 standard
