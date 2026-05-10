# Contributing

Issues and pull requests are welcome. This guide covers local development, adding new kata templates, and how releases work.

## Development setup

```bash
git clone https://github.com/aldevv/katac
cd katac
cargo build
cargo test
cargo fmt --all
cargo clippy -- -D warnings
```

CI runs `cargo fmt --check` and `cargo clippy -- -D warnings` on Linux, then `cargo build` + `cargo test` on Linux/macOS/Windows. Both must pass before a PR can merge.

For a containerized run, `make test` (uses `Dockerfile.tests`, alpine + cargo test) and `make build` (multi-arch via `tonistiigi/xx`) are available.

## Adding a kata template

Templates live under `example-katas/<language>/<KataName>/` and are baked into the binary at compile time via the `include_dir!` macro in `src/lib.rs`. They show up in `katac init` automatically — no registration needed.

### Layout

```
example-katas/
├── go/
│   └── Queue/
│       ├── Queue.go        # skeleton with the function the user implements
│       ├── Queue_test.go   # tests against the user's implementation
│       ├── go.mod
│       ├── go.sum
│       └── Makefile        # required — must define a `run:` target
└── python/
    ├── dsa_base.py         # shared helpers (file at language root, not a kata)
    └── Queue/
        ├── queue.py
        ├── test_queue.py
        └── Makefile
```

### Requirements

- The template directory name is what shows up in `katac init` (e.g. `Queue`, `LRU`, `BinarySearchList`). Use PascalCase to match existing katas.
- Every template **must** have a `Makefile` with a `run:` target. `katac run` invokes `make run -s` first; without `make`, it falls back to `run.sh` (Unix) or `run.bat` (Windows).
- The skeleton should be runnable on copy — leaving function bodies empty is fine, but unresolved imports or syntax errors are not. The user should be able to `katac run` immediately and see a no-op or failing test, never a parse error.
- For a brand-new language, just add a sibling directory under `example-katas/` (e.g. `example-katas/rust/`). `katac init` discovers languages by listing top-level directories — no code change required.

### Reference solutions

If you have a working solution, mirror it at `example-implementations/<language>/<KataName>/` with the same structure. This tree is **not** embedded in the binary — it exists for repo readers who want to see one possible implementation. Don't `include_dir!` it; that would bloat every release binary.

### Naming collisions

If you add a kata that shares a name across languages (e.g. `go/Foo` and `python/Foo`), `katac init` automatically renames the second selection to `<language>_Foo` to avoid collision. The existing `test_init_command_with_duplicates` integration test covers this — no extra test work needed for the duplicate case.

## Submitting changes

- One logical change per PR; keep diffs scoped.
- Run `cargo fmt --all` and `cargo clippy -- -D warnings` before pushing.
- Add tests when behavior changes; reuse fixtures in `tests/example_katas/` where possible.
- Update [`docs/usage.md`](usage.md) if the change is user-facing.
- Don't bump the version in `Cargo.toml` — that's part of the release flow.

## Releases (maintainers)

Tagging is the trigger.

1. Bump `version` in `Cargo.toml`.
2. `cargo build` to refresh `Cargo.lock`, then commit both files.
3. `make release` — verifies a clean working tree, creates an annotated `vX.Y.Z` tag, and pushes it.
4. `.github/workflows/release.yml` then creates the GitHub release, publishes to crates.io, and uploads cross-compiled binaries for all release targets.

`make untag` removes a botched tag locally and on origin. Both `katac upgrade` and `install.sh` pull from the GitHub release assets, so target naming in `get_rust_target()` (`src/lib.rs`) and `get_target()` (`install.sh`) must stay in sync with the release matrix.
