# Tools

Interactive helpers for UQM modding.

- add_star_wizard.py
  - Adds a new star to the HyperSpace starmap and keeps counts in sync
  - Updates:
    - `content/base/gamestrings.txt` (adds name under STAR_STRING_BASE)
    - `src/uqm/gamestr.h` (STAR_STRING_COUNT)
    - `src/uqm/plandata.c` (inserts star in Y-order before sentinel)
    - `src/uqm/starmap.h` (NUM_SOLAR_SYSTEMS)
  - Safety: `--dry-run` prevents writes; backups are saved as `.bak`
  - Diagnostics: `--validate` prints current counts and confirms sentinel detection

Examples:

```bash
# Validate parsing only
python3 tools/add_star_wizard.py --validate

# Run interactive wizard (no writes)
python3 tools/add_star_wizard.py --dry-run

# Run and write changes
python3 tools/add_star_wizard.py
```