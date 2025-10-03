# Moved: Repository Guide

**This file has been moved to `sc2/doc/README-REPO.md`**

Please refer to the centralized documentation in the `sc2/doc/` directory for the complete repository guide and project documentation.
- **Libraries:** SDL2 2.32.4, PNG 1.6.48, Ogg Vorbis 1.3.7, zlib 1.3.1
- **Audio:** PulseAudio integration working
- **Graphics:** OpenGL renderer functional
- **Content:** All base assets loading correctly (25 ship types, UI, fonts, audio)
- **Addons:** 3DO music, voice, and video packs detected and loading

### Build artifacts:
- `sc2/uqm-debug` - Debug build executable (5.7MB)
- `run_uqm.sh` - Convenience script for execution
- Configuration stored in `~/.uqm/`
- Save games in `~/.uqm/save/`

### Usage:
```bash
# Direct execution
cd sc2 && ./uqm-debug -w -r 800x600 --safe

# Using convenience script
./run_uqm.sh
```

## Follow-up ideas

- Extract module dependency graphs into Mermaid diagrams for visual learners.
- Port developer docs (`doc/devel/*`) into Markdown within this repository for easier browsing.
- Automate sanity checks (content index validation, save-file schema tests) to accompany future engine changes.
- Set up automated build pipeline for multiple platforms.
