# .agent Directory - Project Context System

This directory contains all project context, documentation, and development history for sc2-uqm (The Ur-Quan Masters).

## Purpose

The `.agent` directory serves as the single source of truth for:
- Project specifications and architecture
- Development history and decisions
- Documentation and guides
- Implementation plans and tasks
- Open questions and research

## Structure

### Core Files (Read in Order)
1. **00_manifest.md** - Framework overview and reading order
2. **10_spec.md** - Project specification (COMPLETE)
3. **20_architecture.md** - System architecture (COMPLETE)
4. **21_api_contract.md** - API contracts (TODO)
5. **22_standards.md** - Coding standards (TODO)
6. **30_adrs/** - Architecture Decision Records
7. **35_research.md** - Research notes and findings
8. **40_tasks.md** - Current task list
9. **45_implementation_plan.md** - Phased implementation plan (ACTIVE)
10. **50_worklog.md** - Complete development history (COMPLETE)
11. **60_open_questions.md** - Pending questions

### Documentation Directory (docs/)
- **INDEX.md** - Complete documentation index
- **guides/** - How-to guides for developers and users
  - CHEAT_MODE_README.md
  - STAR_WIZARD_TOOL.md
- **technical/** - Implementation details and specifications
  - HOW_TO_ADD_STAR_SYSTEM.md (169 lines)
  - HOW_TO_ADD_MENU_OPTIONS.md (394 lines)
  - KITSUNE_SYSTEM.md
  - CHEAT_MODE_IMPLEMENTATION.md
  - All sc2/doc/ files (copied for centralization)
- **reports/** - Status and completion reports
  - COMPILATION_SUCCESS.md
  - CLEANUP_SUMMARY.md
  - WARNING_RESOLUTION_REPORT.md

### Archive
Older documentation and phase reports are stored in `archive/` to keep the main directory focused on current work.

## Quick Start

### For New Team Members
1. Read 00_manifest.md
2. Read 10_spec.md (project overview)
3. Read 20_architecture.md (system design)
4. Read 50_worklog.md (last 50 lines for recent history)
5. Check 40_tasks.md for current work
6. Browse docs/INDEX.md for specific topics

### For Development
- Adding a star system? → docs/guides/STAR_WIZARD_TOOL.md
- Adding a menu option? → docs/technical/HOW_TO_ADD_MENU_OPTIONS.md
- Understanding cheat mode? → docs/guides/CHEAT_MODE_README.md

### For Context Updates
- Log all work in 50_worklog.md (append-only)
- Update 45_implementation_plan.md for task changes
- Add open questions to 60_open_questions.md
- Create ADRs in 30_adrs/ for significant decisions

## Key Information

### Project Status (2026-01-23)
- **Version**: 0.8.0
- **Status**: Active Development
- **Branch**: modernization (consolidated and tested)
- **Build**: ✅ Successful (5.6MB executable)
- **Features**: Cheat mode, Kitsune system, Spanish localization, logging
- **Platform**: Ubuntu WSL2

### Recent Achievements
- ✅ Zero compiler warnings
- ✅ 34% size reduction (549MB → 360MB)
- ✅ Comprehensive documentation (8+ guides)
- ✅ Python wizard tool for content creation
- ✅ Successfully consolidated 3 development branches

### Known Issues
- Minor segfault on exit (doesn't affect gameplay)
- Starmap JSON export disabled (needs reimplementation)

## Maintenance

### Context Budget
Keep total context under 40% of model context window. When files grow too large:
1. Summarize old sections
2. Move to archive/
3. Keep INDEX.md updated

### File Ownership
- **Human-controlled**: 10_spec.md, 21_api_contract.md, 22_standards.md, 30_adrs/*
- **AI-writable (with review)**: 35_research.md, 40_tasks.md, 45_implementation_plan.md, 50_worklog.md, 60_open_questions.md
- **Both**: 20_architecture.md (collaborative evolution)

### Workflow
1. Research (35_research.md)
2. Plan (45_implementation_plan.md)
3. Implement (code changes)
4. Document (50_worklog.md, relevant guides)
5. Review (update specs/architecture if needed)

## Contact

Project Owner: Kira
Repository: https://github.com/kirebyte/sc2-uqm
Branch: modernization (current)
