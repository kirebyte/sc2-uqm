# Documentation Index

Last Updated: 2026-01-23

## Directory Structure

```
.agent/docs/
├── INDEX.md (this file)
├── guides/           # How-to guides and tutorials
├── technical/        # Technical implementation details
└── reports/          # Status reports and summaries
```

## Guides (How-To Documentation)

### Development Guides
- **CHEAT_MODE_README.md**: End-user guide for using cheat mode features
- **STAR_WIZARD_TOOL.md**: How to use add_star_wizard.py for creating star systems

### In technical/ (developer guides)
- **HOW_TO_ADD_STAR_SYSTEM.md**: Complete guide to adding custom star systems (169 lines)
  - Architecture overview, file structure, step-by-step instructions
  - Kitsune system as reference example
  
- **HOW_TO_ADD_MENU_OPTIONS.md**: Complete guide to adding menu options (394 lines)
  - Menu system architecture, enum indexing, gamestrings.txt coordination
  - Cheat mode as reference example

- **MENU_OPTIONS_QUICK_REF.md**: Quick reference for menu constants and indices
- **README_MENU_DOCS.md**: Overview of menu documentation

## Technical Documentation

### Architecture & Implementation
- **README-REPO.md**: Repository structure and organization
- **README.md**: General project documentation
- **CHEAT_MODE_IMPLEMENTATION.md**: Technical implementation details of cheat mode
  - Files modified, code changes, lessons learned
  
- **CHEAT_STARMAP_SOUND.md**: Starmap sound testing feature documentation
- **KITSUNE_SYSTEM.md**: Complete documentation of Kitsune star system
  - 9 planets, coordinates, file changes, technical details

### Tools & Utilities
- **README_STARMAP_EXPORT.md**: Documentation for starmap export functionality (if implemented)

## Reports & Status

### Completion Reports
- **COMPILATION_SUCCESS.md**: Compilation achievement report
  - Dependencies, build process, verification, optimization results
  
- **CLEANUP_SUMMARY.md**: Project cleanup and optimization report
  - Files removed, size reduction (549MB → 360MB)
  - Platform-specific code elimination
  
- **WARNING_RESOLUTION_REPORT.md**: Compiler warning resolution summary
  - Types of warnings fixed, approach taken

## Related Files in Repository Root

### Active Documentation
- **README.md**: Main project readme (visible on GitHub)
- **README-REFACTORED.md**: Refactoring notes
- **REFACTORING_PLAN.md**: Refactoring strategy

### Developer Documentation in sc2/doc/
All files have been copied to .agent/docs/technical/ for centralization.
Original files remain in sc2/doc/ for backward compatibility.

## Usage Patterns

### For New Developers
1. Start with README.md (project overview)
2. Read HOW_TO_ADD_STAR_SYSTEM.md or HOW_TO_ADD_MENU_OPTIONS.md
3. Review KITSUNE_SYSTEM.md or CHEAT_MODE_IMPLEMENTATION.md as examples
4. Use STAR_WIZARD_TOOL.md for quick content creation

### For End Users
1. CHEAT_MODE_README.md for gameplay enhancements
2. COMPILATION_SUCCESS.md for build instructions

### For Project Understanding
1. CLEANUP_SUMMARY.md (what was removed and why)
2. WARNING_RESOLUTION_REPORT.md (code quality improvements)
3. Architecture docs in ../.agent/ parent directory
