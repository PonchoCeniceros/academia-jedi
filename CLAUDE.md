# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

**Estudio de programación competitiva** is a personal competitive programming and language-learning repo. Solutions exist in two languages: Python and Rust. Each problem is identified by its LeetCode problem ID. A single script, `./problema` (which delegates to one `problema.sh` per language), scaffolds, tests, and lists problems. (The project used to be called "Academia Jedi", with *trials* and *holocrons*; those terms were retired, so old notes and commits may still use them.)

The repo is organized **language-first**. Within Rust there are two tracks: `fundamentals/` (mastering the language via algorithms) and `building/` (applied projects — APIs, data analysis).

## Repository Structure

```
problema                   # Entry point: ./problema --py|--rs -n|-t|-r|-l
rust/
├── fundamentals/           # Mastering the language
│   ├── notes/              # "Rust · Cuaderno de campo" — Slidev, ONE deck.
│   │   ├── slides.md       # Entry point; includes pages/ via `src:`
│   │   ├── export-sections.py  # One PDF per section (splits the deck) + standalone decks
│   │   ├── export-navegable.py # One navigable PDF: bookmarks, per-section topic page, working TOC links, back buttons
│   │   ├── sync-indice.py  # Re-syncs the TOC's slide numbers. RUN AFTER EDITING pages/
│   │   ├── ejercicios.md   # Standalone deck: 80 exercises (its own PDF, not in slides.md)
│   │   ├── GUIA-DE-TONO.md # Writing/tone rules for the deck — READ BEFORE EDITING
│   │   ├── _multideck/     # Abandoned per-section entry files. Safe to delete.
│   │   └── pages/          # Numbered by section (topological order, see below):
│   │                       # 1.1-tokens.md … 9.6-un-servidor-minimo.md,
│   │                       # ap0-cheatsheet.md, ap2-aplicacion.md, ejercicios.md
│   └── problems/           # Cargo package "katas" (LeetCode in Rust)
│       ├── Cargo.toml
│       ├── problema.sh     # Scaffolds/tests/runs problems
│       └── src/
│           ├── lib.rs      # `mod macros;`
│           ├── macros.rs   # exports the `s!` macro
│           └── bin/{ID}_{name}.rs   # one binary per problem (34)
└── building/               # Applied Cargo projects — planned. No separate
                             # slide deck; concepts live in fundamentals/notes
                             # (prefix F) — only per-project crate details
                             # belong here, in that project's own README
python/
├── problems/               # Python solutions (pytest-based, 21)
│   ├── {ID}.py             # Solution file with tests
│   └── {ID}/main.py        # (Optional) Multi-file problems
├── utils/                  # log.py — logging helper used in tests
└── problema.sh             # Scaffolds/tests Python problems
docs/whiteboards/           # Cross-language sketches
roadmap.csv                 # Problem metadata + progress (spans both languages)
pytest.ini                  # Stays at root — see "Python Setup"
```

## Development Workflow

`./problema` at the repo root takes `--py` or `--rs` and delegates to that language's `problema.sh`; `./problema -l` lists both. Each `problema.sh` resolves its own location, so they work from the repo root **or** from their own directory. Options: `-n` new (alias `-m`), `-t` test (alias `--train`), `-r` run (Rust only), `-l` list. The Rust case index for `-r` is passed to the binary as the `CASO` env var.

### Python Path (pytest)

**Create a new problem:**
```bash
python/problema.sh -n "9. Palindrome Number"
```
Creates `python/problems/9.py` with test scaffold.

**Run tests for a problem:**
```bash
python/problema.sh -t 9
```
Or directly:
```bash
pytest python/problems/9.py -v
```

**List all completed problems:**
```bash
python/problema.sh -l
```

**Run all Python tests:**
```bash
pytest
```

### Rust Path (cargo)

**Create a new problem:**
```bash
rust/fundamentals/problems/problema.sh -n "1. Two Sum"
```
Creates `rust/fundamentals/problems/src/bin/1_two_sum.rs` with test scaffold.

**Run tests for a problem:**
```bash
rust/fundamentals/problems/problema.sh -t 1
```
Or directly:
```bash
cargo test --manifest-path rust/fundamentals/problems/Cargo.toml --bin 1_two_sum
```

**Execute the problem's main function:**
```bash
rust/fundamentals/problems/problema.sh -r 1
```

**Run all Rust tests:**
```bash
cargo test --manifest-path rust/fundamentals/problems/Cargo.toml
```

**List all completed problems:**
```bash
rust/fundamentals/problems/problema.sh -l
```

### Study deck (Slidev)

```bash
pnpm --dir rust/fundamentals/notes run dev
```
One deck, ~330 slides (called *láminas* in the deck's own language), served whole on port 3031, plus a standalone `ejercicios.md`
(161 slides: cover + 80 exercise/answer pairs) that is served with `pnpm dev:ej` (port 3032) and exported separately. The split into sections happens only at
PDF export time: `pnpm export:sections` produces one PDF per section in `exports/`,
and `pnpm export:nav` produces the whole deck as one navigable PDF.
The deck's TOC links to slide numbers, so **run `python3 sync-indice.py` after
adding, removing or reordering any page** or the links go stale. Also available as the `rust-notes` config in `.claude/launch.json`.

### Learning line (topological)

Sections are ordered so that nothing is used before it is explained:
`1 Fundamentos` (tokens, escalares, variables, expresiones, funciones, control de
flujo) · `2 Tipos compuestos` (tuplas/arreglos, structs *sin métodos*, enums,
patrones) · `3 Memoria` (stack/heap, ownership, borrowing, impl y métodos,
lifetimes) · `4 Abstracciones de costo cero` (traits, generics, closures) · `5 Vocabulario
stdlib` (Option, Result, Vec, HashMap, otras colecciones, iteradores) ·
`6 Indirección` (Box, Rc/RefCell, dyn) · `7 Proyectos` (módulos, Cargo, tests,
errores, I/O, macros) · `8 Concurrencia` (hilos, Send/Sync, Arc/Mutex, canales,
rayon) · `9 Async` (async/await, Future, runtime, Send en tareas).

Sections 8 and 9 close the fundamentals for the applied tracks the user is heading
into: web APIs, databases, Polars, parallel compute. Async is taught at the language
level only — the runtime (tokio) and frameworks belong in `rust/building/`.

Consequences worth knowing before editing: **`impl`/methods are split from `struct`**
(datos in 2.2, métodos in 3.4, because `&self` is a borrow); **traits come before
generics** (bounds are traits); and the two "ya lo viste" tables in 4.1/4.2 are now
**forward maps** ("dónde vas a encontrarlos"). Two forward references are declared
explicitly in 1.2 and allowed everywhere: string literals (`&str`) and `String`.
Adding a page? Check nothing it uses is introduced later.

A per-section multi-deck layout was tried and reverted: reading the notes wants one
continuous deck, and only the PDFs need to be separate. The abandoned entry files
are in `_multideck/`.

## Key Technical Details

### Python Setup
- Uses **pytest** for testing with parametrize decorators
- Tests in same file as solution (at bottom of `{ID}.py`)
- Imports `utils.log.Log` for standardized logging (optional but available)
- `python/` is on `PYTHONPATH` (via `pythonpath = python` in `pytest.ini`), which is what makes `from utils.log import Log` resolve
- **`pytest.ini` deliberately stays at the repo root.** If moved into `python/`, running `pytest` from the root would let the root `pyproject.toml` win as rootdir and silently discard all config. At the root it behaves identically from either directory.
- venv at `.venv/` (pyright configured to use it)

### Rust Setup
- Single Cargo **package** (`katas`, not a workspace) in `rust/fundamentals/problems/` with edition `2024`
- Each solution is a separate binary: `cargo run --bin {ID}_{name}`
- Dependencies available: `regex`, `colored`
- Tests embedded in binary via `#[cfg(test)]` modules
- Common utilities imported via `use katas::s;` (internal crate reference)

### Test Conventions
- **Python**: Use `@pytest.mark.parametrize` with tuples of (input, expected)
- **Rust**: Use `#[test]` fn with loop over test cases, `assert_eq!` macro, print errors with `.red().italic().underline()`

## Common Commands

| Task | Command |
|------|---------|
| Create Python problem | `python/problema.sh -n "{ID}. Problem Name"` |
| Create Rust problem | `rust/fundamentals/problems/problema.sh -n "{ID}. Problem Name"` |
| Test Python solution | `python/problema.sh -t {ID}` or `pytest python/problems/{ID}.py -v` |
| Test Rust solution | `rust/fundamentals/problems/problema.sh -t {ID}` |
| Run Rust binary | `rust/fundamentals/problems/problema.sh -r {ID}` |
| Test all Python | `pytest` |
| Test all Rust | `cargo test --manifest-path rust/fundamentals/problems/Cargo.toml` |
| Serve study deck | `pnpm --dir rust/fundamentals/notes run dev` |
| One PDF per section | `pnpm --dir rust/fundamentals/notes run export:sections` |
| One navigable PDF | `pnpm --dir rust/fundamentals/notes run export:nav` |
| Serve exercises deck | `pnpm --dir rust/fundamentals/notes run dev:ej` |

## Code Philosophy

Per the README philosophy:
1. **Clarity over cleverness** — solutions must be readable
2. **Continuous refinement** — a problem is only "mastered" when no further simplification is possible
3. **Process-driven** — each failed test is a lesson; solutions should flow naturally when mastered

## Important Files
- `./problema` — single entry point (`--py` / `--rs`)
- `python/problema.sh` — scaffolds and runs Python problems
- `rust/fundamentals/problems/problema.sh` — scaffolds and runs Rust problems
- `roadmap.csv` — tracks problem metadata and progress (spans both languages, hence at root)
- `pytest.ini` — pytest configuration (testpaths, pythonpath, logging)
- `rust/fundamentals/problems/Cargo.toml` — Rust package config
- `rust/fundamentals/notes/slides.md` — study deck entry point (the only one)
- `rust/fundamentals/notes/GUIA-DE-TONO.md` — tone rules for the decks; read it before writing or editing any page

## Notes for Future Sessions
- Both language paths run independently; no requirement to do both
- LeetCode problem IDs are used consistently across both paths
- Rust edition 2024 is explicitly set in Cargo.toml (non-standard; verify compatibility when upgrading)
- Test scaffold filenames follow the pattern: Python uses `test_{function_name}`, Rust uses `test_{package_name}`
- The Slidev deck's images use root-absolute paths (`/images/...`) served from `notes/public`; its page includes use `./pages/...`. Both are internal, so the deck can be moved as a unit.
- In Slidev pages, sibling `<div>`s containing markdown need blank lines around them, or the build fails with `Element is missing end tag`.
- `rust/building/notes` (a separate "Rust Aplicado" deck) was merged into `rust/fundamentals/notes` as the `F0X` pages (Módulos, Crates y Cargo) — there is now a single Slidev deck for all of Rust. `rust/building/` keeps only future Cargo projects.
- `slidev export --range "X-Y"` silently produces blank-page PDFs/PNGs in this Slidev version (Playwright rendering bug). `export-sections.py` works around it by exporting the full deck once and splitting the result with `pypdf`, verifying each PDF's content streams aren't suspiciously small before trusting the output. It was updated for the new numbering: sections are keyed `0`–`9`, `cheat`, `ap`, `ej` (see `section_key()`).
- The deck was split and reordered on 2026-08-31 (`A01`…`Z02` became `1.1`…`9.6` plus appendices; `B02` became `2.3-borrowing`/`2.4-lifetimes`, later renumbered). The placeholder pages from that split have since been written. Pages prefixed `_` (`_*.md`, `_*.md.bak`) are drafts or backups that `slides.md` does not include.

## Failing tests (pre-existing; unrelated to the language-first refactor)

`cargo test` is **fail-fast**: a single failing target stops the run, so use
`--no-fail-fast` to see the whole picture. Four targets fail today:

| Problem | Symptom | Likely cause |
|---|---|---|
| 241 Different Ways to Add Parentheses | `[]` vs `[11]` | real bug: single-number input returns nothing |
| 347 Top K Frequent Elements | `[2, 1]` vs `[1, 2]` | **order only** — the answer is right; the test asserts an exact order that LeetCode doesn't require |
| 49 Group Anagrams | groups right, order differs | **order only** — same as above, at both the group and element level |
| 48 Rotate Image | panics at `48_rotate_image.rs:86` | needs a look (no left/right in the assert output) |

All four are marked `ya_en_rust=Si` in `roadmap.csv`. They stayed invisible for a
while because a compile error in 241 aborted the suite before reaching them.

`python/problems/888.py` also has one failing parametrized case
(`test_fairCandySwap[...expected1]`).

## Environment gotcha

The `.venv/bin/*` console scripts have a dead shebang pointing at
`/Users/giovannychavez/developments/...` (plural) while the repo lives at
`.../development/...` — the venv was built before the repo moved. So invoking
bare `pytest` fails even with the venv activated. Use `.venv/bin/python -m
pytest` (which is what `python/problema.sh` does), or recreate the venv from
`requeriments.txt`.
