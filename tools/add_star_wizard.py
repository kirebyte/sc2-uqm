#!/usr/bin/env python3
"""
Interactive wizard to add a new star to UQM (and keep tables in sync).

Features:
- Validates and updates:
  * sc2/content/base/gamestrings.txt (STAR_STRING_BASE section)
  * sc2/src/uqm/gamestr.h (STAR_STRING_COUNT)
  * sc2/src/uqm/plandata.c (insert star in Y-order; before sentinel)
  * sc2/src/uqm/starmap.h (NUM_SOLAR_SYSTEMS)
- Safety: --dry-run (no writes) and .bak backups when writing.
- Diagnostics: --validate to only parse and show current counts, verifying sentinel.

Note: This version focuses on safe star insertion. Generator scaffolding can be
added after confirming your flow; the script architecture allows extending it.
"""

import argparse
import os
import re
import sys
from typing import List, Tuple

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))

PLANDATA = os.path.join(REPO_ROOT, "sc2", "src", "uqm", "plandata.c")
STARMAP_H = os.path.join(REPO_ROOT, "sc2", "src", "uqm", "starmap.h")
GAMESTR_H = os.path.join(REPO_ROOT, "sc2", "src", "uqm", "gamestr.h")
GAMESTRINGS = os.path.join(REPO_ROOT, "sc2", "content", "base", "gamestrings.txt")


def eprint(*a, **k):
    print(*a, file=sys.stderr, **k)


def read_text(path: str) -> str:
    with open(path, "r", encoding="utf-8") as f:
        return f.read()


def write_text(path: str, content: str, dry_run: bool):
    if dry_run:
        return
    if os.path.exists(path):
        with open(path + ".bak", "w", encoding="utf-8") as b:
            b.write(read_text(path))
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


# ---------------- gamestrings -----------------

def parse_star_names(gamestrings: str) -> Tuple[List[str], int, int]:
    lines = gamestrings.splitlines(True)
    start = None
    end = None
    for i, ln in enumerate(lines):
        if "-- STAR_STRING_BASE" in ln:
            start = i
            break
    if start is None:
        raise ValueError("STAR_STRING_BASE not found in gamestrings.txt")
    # find next section marker
    for j in range(start + 1, len(lines)):
        if lines[j].startswith("#(") and "--" in lines[j] and "STAR_STRING_BASE" not in lines[j]:
            end = j
            break
    if end is None:
        end = len(lines)
    names: List[str] = []
    i = start + 1
    while i < end:
        if lines[i].strip().startswith("#("):
            # next non-empty is the value
            k = i + 1
            while k < end and lines[k].strip() == "":
                k += 1
            if k < end:
                names.append(lines[k].strip())
            i = k + 1
        else:
            i += 1
    return names, start, end


def ensure_star_name(name: str, gamestrings: str) -> Tuple[str, int]:
    names, start, end = parse_star_names(gamestrings)
    if name in names:
        return gamestrings, names.index(name)
    # insert after UNKNOWN if present, else append
    try:
        insert_after = names.index("UNKNOWN") + 1
    except ValueError:
        insert_after = len(names)
    block = f"#({name})\n{name}\n\n\n"
    lines = gamestrings.splitlines(True)
    count = 0
    pos = start + 1
    while pos < end and count < insert_after:
        if lines[pos].strip().startswith("#("):
            # skip header plus value and following blank(s)
            pos2 = pos + 1
            while pos2 < end and lines[pos2].strip() == "":
                pos2 += 1
            pos2 += 1
            count += 1
            pos = pos2
        else:
            pos += 1
    new_content = "".join(lines[:pos]) + block + "".join(lines[pos:])
    new_names, _, _ = parse_star_names(new_content)
    return new_content, new_names.index(name)


def set_define_value(text: str, name: str, value: int) -> str:
    return re.sub(rf"(#define\s+{re.escape(name)}\s+)\d+", rf"\g<1>{value}", text)


# ---------------- starmap/plandata -----------------

SENTINEL_RE = re.compile(r"\{\{\s*MAX_X_UNIVERSE\s*<<\s*1\s*,\s*MAX_Y_UNIVERSE\s*<<\s*1\s*\}\s*,")


def parse_stars(plandata: str) -> Tuple[List[Tuple[int, int, str]], int, int]:
    lines = plandata.splitlines(True)
    # find starmap_array start line index
    start = None
    for i, ln in enumerate(lines):
        if ln.strip().startswith("STAR_DESC starmap_array[]"):
            start = i
            break
    if start is None:
        raise ValueError("starmap_array not found in plandata.c")
    # find the first '{' after this line
    while start < len(lines) and "{" not in lines[start]:
        start += 1
    if start >= len(lines):
        raise ValueError("Malformed starmap_array block")

    entries: List[Tuple[int, int, str]] = []
    sentinel_line = None
    i = start
    while i < len(lines):
        ln = lines[i]
        if SENTINEL_RE.search(ln):
            sentinel_line = i
            break
        if "MAKE_STAR" in ln and "{{" in ln:
            # accumulate entry until its own closing line ending with "},"
            # Use a separate index so we don't skip checking the next line
            # (which might be the sentinel) on the next iteration.
            j = i
            buf = [lines[j]]
            while j < len(lines) and not lines[j].strip().endswith("},"):
                j += 1
                if j < len(lines):
                    buf.append(lines[j])
            entry = "".join(buf)
            m = re.search(r"\{\{\s*(\d+)\s*,\s*(\d+)\s*\}\s*\}", entry)
            if not m:
                # more tolerant: match the first coordinate pair
                m = re.search(r"\{\{\s*(\d+)\s*,\s*(\d+)\s*\}\s*", entry)
            if not m:
                raise ValueError("Failed to parse star coords in entry:\n" + entry)
            x = int(m.group(1)); y = int(m.group(2))
            entries.append((x, y, entry))
            # advance to the end of this entry; the outer loop will +1
            i = j
        i += 1
    if sentinel_line is None:
        raise ValueError("Sentinel not found in starmap_array")
    return entries, start, sentinel_line


def assemble_star_entry(x: int, y: int, star_type: str, color: str, generator: str, prefix: int, postfix: int) -> str:
    return (
        f"\t{{{{{x:4d}, {y:5d}}}, MAKE_STAR ({star_type}, {color}, -1), {generator}, {prefix}, {postfix}}},\n"
    )


def insert_star(plandata: str, new_line: str, new_y: int) -> Tuple[str, int]:
    entries, start, sentinel = parse_stars(plandata)
    ys = [y for _, y, _ in entries]
    insert_at = 0
    for i, y in enumerate(ys):
        if y > new_y:
            insert_at = i
            break
    else:
        insert_at = len(entries)
    lines = plandata.splitlines(True)
    count = 0
    pos = start
    while pos < sentinel and count < insert_at:
        if "MAKE_STAR" in lines[pos] and "{{" in lines[pos]:
            while pos < sentinel and not lines[pos].strip().endswith("},"):
                pos += 1
            pos += 1
            count += 1
        else:
            pos += 1
    new_content = "".join(lines[:pos]) + new_line + "".join(lines[pos:])
    return new_content, len(entries) + 1


# ---------------- wizard flow -----------------

STAR_TYPES = ["DWARF_STAR", "GIANT_STAR", "SUPER_GIANT_STAR"]
COLORS = ["RED_BODY", "ORANGE_BODY", "YELLOW_BODY", "GREEN_BODY", "BLUE_BODY", "WHITE_BODY"]


def choose_from(prompt: str, options: List[str], default_index: int = 0) -> str:
    for i, o in enumerate(options):
        print(f"  {i+1}) {o}")
    while True:
        ans = input(f"{prompt} [1-{len(options)}] (default {default_index+1}): ").strip()
        if not ans:
            return options[default_index]
        if ans.isdigit():
            idx = int(ans) - 1
            if 0 <= idx < len(options):
                return options[idx]
        print("Invalid choice.")


def prompt_int(prompt: str, default: int, lo: int, hi: int) -> int:
    while True:
        ans = input(f"{prompt} (default {default}): ").strip()
        if not ans:
            return default
        try:
            v = int(ans)
        except ValueError:
            print("Enter an integer.")
            continue
        if v < lo or v > hi:
            print(f"Value must be in [{lo}, {hi}].")
            continue
        return v


def prompt_yes_no(msg: str, default: bool = True) -> bool:
    suf = "[Y/n]" if default else "[y/N]"
    while True:
        ans = input(f"{msg} {suf} ").strip().lower()
        if not ans:
            return default
        if ans in ("y", "yes"):
            return True
        if ans in ("n", "no"):
            return False
        print("Please answer y or n.")


def validate_only():
    games = read_text(GAMESTRINGS)
    names, _, _ = parse_star_names(games)
    print(f"STAR_STRING_COUNT (actual): {len(names)}")
    plan = read_text(PLANDATA)
    entries, _, _ = parse_stars(plan)
    print(f"NUM_SOLAR_SYSTEMS (actual before sentinel): {len(entries)}")
    print("Sentinel detection: OK")


def run_wizard(dry_run: bool):
    print("=== Add Star Wizard ===\n")
    name = input("Constellation/star name (e.g., Kitsune): ").strip()
    x = prompt_int("HyperSpace X (0-9999)", 9749, 0, 9999)
    y = prompt_int("HyperSpace Y (0-9999)", 9250, 0, 9999)
    star_type = choose_from("Star type:", STAR_TYPES, 2)
    color = choose_from("Star color:", COLORS, 4)
    prefix = prompt_int("Prefix index (0=none, 1=Alpha, 2=Beta, ...)", 0, 0, 20)

    # Load files
    gamestr = read_text(GAMESTRINGS)
    gh = read_text(GAMESTR_H)
    pd = read_text(PLANDATA)
    sh = read_text(STARMAP_H)

    # Ensure name
    new_gamestr, postfix = ensure_star_name(name, gamestr)
    star_count = len(parse_star_names(new_gamestr)[0])

    # Assemble and insert star (default generator 0)
    new_line = assemble_star_entry(x, y, star_type, color, "0", prefix, postfix)
    new_pd, solar_count = insert_star(pd, new_line, y)

    # Update defines
    new_gh = set_define_value(gh, "STAR_STRING_COUNT", star_count)
    new_sh = set_define_value(sh, "NUM_SOLAR_SYSTEMS", solar_count)

    # Summary
    print("\nSummary:")
    print(f"- Postfix index for '{name}': {postfix}")
    print(f"- New star inserted at y={y} (x={x}) before sentinel; new count={solar_count}")
    print(f"- STAR_STRING_COUNT -> {star_count}; NUM_SOLAR_SYSTEMS -> {solar_count}")
    if not prompt_yes_no("Proceed with file updates?", not dry_run):
        print("Aborted.")
        return

    write_text(GAMESTRINGS, new_gamestr, dry_run)
    write_text(GAMESTR_H, new_gh, dry_run)
    write_text(PLANDATA, new_pd, dry_run)
    write_text(STARMAP_H, new_sh, dry_run)
    print("Done." + (" (dry-run)" if dry_run else ""))


def main():
    p = argparse.ArgumentParser(description="Interactive wizard to add a star/system to UQM")
    p.add_argument("--dry-run", action="store_true", help="Show changes without writing files")
    p.add_argument("--validate", action="store_true", help="Only parse and print current counts")
    args = p.parse_args()

    try:
        if args.validate:
            validate_only()
            return
        run_wizard(args.dry_run)
    except Exception as ex:
        eprint("Error:", ex)
        sys.exit(1)


if __name__ == "__main__":
    main()
