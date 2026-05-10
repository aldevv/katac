# katac

[![Crates.io](https://img.shields.io/crates/v/katac)](https://crates.io/crates/katac)
[![License](https://img.shields.io/crates/l/katac)](#license)

Practice data structures — Queue, Trie, LRU, Map, UnionFind — by reimplementing them from a clean template. `katac` handles the boilerplate: copies the kata into `days/dayN/`, runs the tests, gets out of the way.

![katac demo](docs/demo.gif)

## Quickstart

```console
$ katac init                  # pick a language, then katas to seed katas/
$ katac Queue                 # copies katas/Queue into days/day1/Queue
$ katac run                   # runs everything in the latest day folder
```

That's the inner loop: `init` once, `katac <kata>` to start a day, `katac run` while you work.

## Install

```bash
# crates.io (recommended for Rust users)
cargo install katac

# Pre-built binary
curl -fsSL https://raw.githubusercontent.com/aldevv/katac/main/install.sh | bash
```

Or grab a binary directly from the [releases page](https://github.com/aldevv/katac/releases). On native Windows, use `cargo install katac` — `install.sh` requires bash.

## Commands

| Command                  | What it does                                                                                                |
| ------------------------ | ----------------------------------------------------------------------------------------------------------- |
| `katac init`             | Interactively seed templates into `katas/` from embedded examples (`--examples-dir <path>` for your own).   |
| `katac <kata>...`        | Copy katas into the next `days/dayN/`. Sugar for `katac start`.                                             |
| `katac start <kata>...`  | Same as the bare form, explicit.                                                                            |
| `katac run [kata]...`    | Run katas in the latest `dayN/`; defaults to all. Pass `-c <cmd>` to override the run command.              |
| `katac new <name>`       | Scaffold a new kata in `katas/`.                                                                            |
| `katac random <N>`       | Copy `N` randomly-picked katas into the next day.                                                           |
| `katac upgrade`          | Self-update to the latest GitHub release.                                                                   |

`katac init` ships embedded templates for **Go** and **Python**. Adding a template — in a new language or for an existing one — is a great first contribution; see [docs/contributing.md](docs/contributing.md).

## Documentation

- [docs/tutorial.md](docs/tutorial.md) — walk through scaffolding your own kata, failing the test, and implementing it.
- [docs/usage.md](docs/usage.md) — `katac.toml` configuration, restricting the random pool, and per-kata Makefile recipes.

## Contributing

Issues and pull requests welcome. See [docs/contributing.md](docs/contributing.md) for development setup, how to add new kata templates, and the release process.

## License

Dual-licensed under either of:

- [MIT license](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

at your option.
