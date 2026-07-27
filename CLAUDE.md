# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

**Academia Jedi** is a personal competitive programming and language-learning training ground. Solutions exist in two languages: Python and Rust. Each problem is called a "Trial" and is identified by its LeetCode problem ID. The repository uses automation scripts called "Holocrons" to scaffold, test, and organize work.

The repo is organized **language-first**. Within Rust there are two tracks: `fundamentals/` (mastering the language via algorithms) and `building/` (applied projects — APIs, data analysis).

## Repository Structure

```
rust/
├── fundamentals/           # Mastering the language
│   ├── notes/              # Slidev study deck (pnpm project)
│   │   ├── slides.md       # Entry point; includes pages/ via `src:`
│   │   └── pages/          # A0X syntax, B0X memory safety, C0X enums,
│   │                       # D0X data structures, E0X appendices
│   └── trials/             # Cargo package "katas" (LeetCode in Rust)
│       ├── Cargo.toml
│       ├── holocron.sh     # Scaffolds/tests/runs trials
│       └── src/
│           ├── lib.rs      # `mod macros;`
│           ├── macros.rs   # exports the `s!` macro
│           └── bin/{ID}_{name}.rs   # one binary per trial (34)
└── building/               # Applied projects (notes/ + projects/) — planned
python/
├── trials/                 # Python solutions (pytest-based, 21)
│   ├── {ID}.py             # Solution file with tests
│   └── {ID}/main.py        # (Optional) Multi-file trials
├── utils/                  # log.py — logging helper used in tests
└── holocron.sh             # Scaffolds/tests Python trials
docs/whiteboards/           # Cross-language sketches
roadmap.csv                 # Problem metadata + progress (spans both languages)
pytest.ini                  # Stays at root — see "Python Setup"
```

## Development Workflow

Both holocrons resolve their own location, so they work from the repo root **or** from their own directory.

### Python Path (pytest)

**Create a new trial:**
```bash
python/holocron.sh -m "9. Palindrome Number"
```
Creates `python/trials/9.py` with test scaffold.

**Run tests for a trial:**
```bash
python/holocron.sh -t 9
```
Or directly:
```bash
pytest python/trials/9.py -v
```

**List all completed trials:**
```bash
python/holocron.sh -l
```

**Run all Python tests:**
```bash
pytest
```

### Rust Path (cargo)

**Create a new trial:**
```bash
rust/fundamentals/trials/holocron.sh -m "1. Two Sum"
```
Creates `rust/fundamentals/trials/src/bin/1_two_sum.rs` with test scaffold.

**Run tests for a trial:**
```bash
rust/fundamentals/trials/holocron.sh -t 1
```
Or directly:
```bash
cargo test --manifest-path rust/fundamentals/trials/Cargo.toml --bin 1_two_sum
```

**Execute trial main function:**
```bash
rust/fundamentals/trials/holocron.sh -r 1
```

**Run all Rust tests:**
```bash
cargo test --manifest-path rust/fundamentals/trials/Cargo.toml
```

**List all completed trials:**
```bash
rust/fundamentals/trials/holocron.sh -l
```

### Study deck (Slidev)

```bash
pnpm --dir rust/fundamentals/notes run dev
```
Also available as the `rust-notes` config in `.claude/launch.json` (port 3031).

## Key Technical Details

### Python Setup
- Uses **pytest** for testing with parametrize decorators
- Tests in same file as solution (at bottom of `{ID}.py`)
- Imports `utils.log.Log` for standardized logging (optional but available)
- `python/` is on `PYTHONPATH` (via `pythonpath = python` in `pytest.ini`), which is what makes `from utils.log import Log` resolve
- **`pytest.ini` deliberately stays at the repo root.** If moved into `python/`, running `pytest` from the root would let the root `pyproject.toml` win as rootdir and silently discard all config. At the root it behaves identically from either directory.
- venv at `.venv/` (pyright configured to use it)

### Rust Setup
- Single Cargo **package** (`katas`, not a workspace) in `rust/fundamentals/trials/` with edition `2024`
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
| Create Python trial | `python/holocron.sh -m "{ID}. Problem Name"` |
| Create Rust trial | `rust/fundamentals/trials/holocron.sh -m "{ID}. Problem Name"` |
| Test Python solution | `python/holocron.sh -t {ID}` or `pytest python/trials/{ID}.py -v` |
| Test Rust solution | `rust/fundamentals/trials/holocron.sh -t {ID}` |
| Run Rust binary | `rust/fundamentals/trials/holocron.sh -r {ID}` |
| Test all Python | `pytest` |
| Test all Rust | `cargo test --manifest-path rust/fundamentals/trials/Cargo.toml` |
| Serve study deck | `pnpm --dir rust/fundamentals/notes run dev` |

## Code Philosophy

Per the README philosophy:
1. **Clarity over cleverness** — solutions must be readable
2. **Continuous refinement** — a trial is only "mastered" when no further simplification is possible
3. **Process-driven** — each failed test is a lesson; solutions should flow naturally when mastered

## Important Files
- `python/holocron.sh` — scaffolds and runs Python trials
- `rust/fundamentals/trials/holocron.sh` — scaffolds and runs Rust trials
- `roadmap.csv` — tracks problem metadata and progress (spans both languages, hence at root)
- `pytest.ini` — pytest configuration (testpaths, pythonpath, logging)
- `rust/fundamentals/trials/Cargo.toml` — Rust package config
- `rust/fundamentals/notes/slides.md` — study deck entry point

## Notes for Future Sessions
- Both language paths run independently; no requirement to do both
- LeetCode problem IDs are used consistently across both paths
- Rust edition 2024 is explicitly set in Cargo.toml (non-standard; verify compatibility when upgrading)
- Test scaffold filenames follow the pattern: Python uses `test_{function_name}`, Rust uses `test_{package_name}`
- The Slidev deck's images use root-absolute paths (`/images/...`) served from `notes/public`; its page includes use `./pages/...`. Both are internal, so the deck can be moved as a unit.
- In Slidev pages, sibling `<div>`s containing markdown need blank lines around them, or the build fails with `Element is missing end tag`.

## Failing tests (pre-existing; unrelated to the language-first refactor)

`cargo test` is **fail-fast**: a single failing target stops the run, so use
`--no-fail-fast` to see the whole picture. Four targets fail today:

| Trial | Symptom | Likely cause |
|---|---|---|
| 241 Different Ways to Add Parentheses | `[]` vs `[11]` | real bug: single-number input returns nothing |
| 347 Top K Frequent Elements | `[2, 1]` vs `[1, 2]` | **order only** — the answer is right; the test asserts an exact order that LeetCode doesn't require |
| 49 Group Anagrams | groups right, order differs | **order only** — same as above, at both the group and element level |
| 48 Rotate Image | panics at `48_rotate_image.rs:86` | needs a look (no left/right in the assert output) |

All four are marked `ya_en_rust=Si` in `roadmap.csv`. They stayed invisible for a
while because a compile error in 241 aborted the suite before reaching them.

`python/trials/888.py` also has one failing parametrized case
(`test_fairCandySwap[...expected1]`).

## Environment gotcha

The `.venv/bin/*` console scripts have a dead shebang pointing at
`/Users/giovannychavez/developments/...` (plural) while the repo lives at
`.../development/...` — the venv was built before the repo moved. So invoking
bare `pytest` fails even with the venv activated. Use `.venv/bin/python -m
pytest` (which is what `python/holocron.sh` does), or recreate the venv from
`requeriments.txt`.
