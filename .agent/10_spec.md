# Project Specification

Version: 0.8.0
Status: Active Development
Last Updated: 2026-01-23
Owner: Kira

Project Overview
----------------
Name: sc2-uqm (The Ur-Quan Masters)
Description: Modern port and enhancements to the classic space adventure game "The Ur-Quan Masters" (Star Control 2)
Original Source: Forked from SourceForge project sc2 (version 0.8.0)
Platform: Ubuntu WSL2, Linux

Target Users:
- Classic game enthusiasts wanting to replay UQM on modern systems
- Developers extending and modding the game
- Spanish-speaking players (partial translation included)

Business Objectives
1) Migrate the game to modern platform (WSL2/Linux) with optimized build
2) Cleanup and modernize C codebase (eliminate warnings, improve maintainability)
3) Add developer-friendly features (cheat mode, documentation, modding tools)
4) Preserve original gameplay while enabling future enhancements
5) Create comprehensive documentation for future development

Core Features Implemented
- ✅ Full compilation on Ubuntu WSL2
- ✅ Cheat Mode system for testing and debugging
- ✅ Custom star system (Kitsune) with 9 planets
- ✅ Star wizard tool for easy system creation
- ✅ Spanish translations for key dialogues
- ✅ Logging system for debugging
- ✅ Warning-free compilation
- ✅ Project size optimization (549MB → 360MB, -34%)

User Stories
- As a developer, I want comprehensive documentation so that I can add new star systems easily
  - ✅ HOW_TO_ADD_STAR_SYSTEM.md created
  - ✅ HOW_TO_ADD_MENU_OPTIONS.md created
  - ✅ Python wizard tool (add_star_wizard.py) functional

- As a tester, I want cheat mode so that I can quickly test game features
  - ✅ Toggle from main menu
  - ✅ Enhanced lander speed
  - ✅ Visual shield effects
  - ✅ Cursor movement enhancements
  - ✅ Starmap sound testing

- As a Spanish speaker, I want localized content so that I can enjoy the game in my language
  - ✅ Commander dialogues translated
  - ✅ Ur-Quan dialogues translated
  - ✅ Ilwrath dialogues translated
  - ✅ Starbase dialogues translated
  - ✅ Moonbase dialogues translated

Non-Functional Requirements
- Performance: 60 FPS gameplay, optimized asset loading
- Compatibility: SDL2 2.32.4, OpenGL renderer, PulseAudio
- Build: Clean compilation with GCC, no warnings
- Size: Optimized content delivery (removed 189MB of unnecessary files)
- Observability: Logging system integrated, debug build available
