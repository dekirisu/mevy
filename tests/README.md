# Tests

## Structure

```
tests/
├── shared/          # Shared macro crate (156 tests)
│   ├── Cargo.toml
│   └── src/lib.rs   # macro_rules! shared_tests!()
├── test_018/        # Bevy 0.18 tests (158 total)
│   ├── Cargo.toml
│   └── src/lib.rs   # shared_tests!() + 2 version-specific tests
└── test_019/        # Bevy 0.19 tests (163 total)
    ├── Cargo.toml
    └── src/lib.rs   # shared_tests!() + 7 version-specific tests
```

## How It Works

**`shared/`** is a regular library crate containing a `macro_rules!` macro (`shared_tests!()`) with all tests that are 100% identical between Bevy 0.18 and 0.19. The macro injects test code into each per-version crate at compile time.

Each per-version crate (`test_018`, `test_019`) depends on `shared` and invokes `shared_tests!();` to pull in the shared tests. They also contain:
- The 2 assertions that differ between versions (`overflow_clip_margin` and `font_size`)
- Any exclusive tests for that Bevy version (e.g., 0.19-only `EditableText` fields)

## Why Two Separate Crates?

We can't test both versions in a single compilation unit — each `test_01X/Cargo.toml` pins a different Bevy version. The shared macro avoids duplicating ~156 tests while still compiling against each version independently.

## Running

```bash
make test-018    # Bevy 0.18 — 158 tests
make test-019    # Bevy 0.19 — 163 tests
make test-all    # Both
```
