# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

**Academia Jedi** is a personal competitive programming training ground for LeetCode problems. Solutions exist in two languages: Python and Rust. Each problem is called a "Trial" and is identified by its LeetCode problem ID. The repository uses automation scripts called "Holocrons" to scaffold, test, and organize work.

## Repository Structure

```
trials/
├── python/              # Python solutions (pytest-based)
│   ├── {ID}.py         # Solution file with tests
│   ├── {ID}/           # (Optional) Multi-file trials
│   │   ├── solution.py
│   │   └── test.py
│   └── ... (20+ problems)
└── rust/               # Rust solutions (cargo-based)
    ├── Cargo.toml      # Single workspace for all binaries
    └── src/bin/
        ├── {ID}_{name}.rs  # One binary per trial
        └── ... (30+ problems)
utils/                  # Shared utilities
├── __init__.py
└── log.py             # Logging utility used in Python tests
docs/                  # Notes and documentation (rust-notes, whiteboards)
```

## Development Workflow

### Python Path (pytest)

**Create a new trial:**
```bash
./holocron_py.sh -m "9. Palindrome Number"
```
Creates `trials/python/9.py` with test scaffold.

**Run tests for a trial:**
```bash
./holocron_py.sh -t 9
```
Or directly:
```bash
pytest trials/python/9.py -v
```

**List all completed trials:**
```bash
./holocron_py.sh -l
```

**Run all Python tests:**
```bash
pytest trials/python/ -v
```

### Rust Path (cargo)

**Create a new trial:**
```bash
./holocron_rs.sh -m "1. Two Sum"
```
Creates `trials/rust/src/bin/1_two_sum.rs` with test scaffold.

**Run tests for a trial:**
```bash
./holocron_rs.sh -t 1
```
Or directly:
```bash
cargo test --manifest-path trials/rust/Cargo.toml --bin 1_two_sum
```

**Execute trial main function:**
```bash
./holocron_rs.sh -r 1
```

**Run all Rust tests:**
```bash
cargo test --manifest-path trials/rust/Cargo.toml
```

**List all completed trials:**
```bash
./holocron_rs.sh -l
```

## Key Technical Details

### Python Setup
- Uses **pytest** for testing with parametrize decorators
- Tests in same file as solution (at bottom of `{ID}.py`)
- Imports `utils.log.Log` for standardized logging (optional but available)
- Project root is on `PYTHONPATH` (configured in `pytest.ini`)
- venv at `.venv/` (pyright configured to use it)

### Rust Setup
- Single Cargo workspace in `trials/rust/` with edition `2024`
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
| Create Python trial | `./holocron_py.sh -m "{ID}. Problem Name"` |
| Create Rust trial | `./holocron_rs.sh -m "{ID}. Problem Name"` |
| Test Python solution | `./holocron_py.sh -t {ID}` or `pytest trials/python/{ID}.py -v` |
| Test Rust solution | `./holocron_rs.sh -t {ID}` |
| Run Rust binary | `./holocron_rs.sh -r {ID}` |
| Test all Python | `pytest trials/python/ -v` |
| Test all Rust | `cargo test --manifest-path trials/rust/Cargo.toml` |

## Code Philosophy

Per the README philosophy:
1. **Clarity over cleverness** — solutions must be readable
2. **Continuous refinement** — a trial is only "mastered" when no further simplification is possible
3. **Process-driven** — each failed test is a lesson; solutions should flow naturally when mastered

## Important Files
- `holocron_py.sh` — scaffolds and runs Python trials
- `holocron_rs.sh` — scaffolds and runs Rust trials
- `trials/roadmap.csv` — tracks problem metadata and progress
- `pytest.ini` — pytest configuration (testpaths, pythonpath, logging)
- `trials/rust/Cargo.toml` — Rust workspace config

## Notes for Future Sessions
- Both language paths run independently; no requirement to do both
- LeetCode problem IDs are used consistently across both paths
- Rust edition 2024 is explicitly set in Cargo.toml (non-standard; verify compatibility when upgrading)
- Test scaffold filenames follow the pattern: Python uses `test_{function_name}`, Rust uses `test_{package_name}`
