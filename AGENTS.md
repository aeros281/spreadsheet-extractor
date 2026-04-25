# AGENTS.md

Entry point for AI agents working on `spreadsheet-extractor` — a Rust CLI that reads/writes Google Sheets ranges.

last-synced-commit: 9b85ccf0d2a2198f2809e97187191e3b25185c15

## How to use this file

Load only the domain doc(s) relevant to the user's prompt. Do not read every doc up-front.

- Subcommand, CLI flag, or `main.rs` changes → [docs/cli-commands.md](docs/cli-commands.md)
- Config file format, figment sources, or the `Config` struct → [docs/configuration.md](docs/configuration.md)
- OAuth, `Sheets` hub, or any `spreadsheets.values.*` call → [docs/google-sheets.md](docs/google-sheets.md)
- Output formats, CSV/JSON formatters, `reader.rs` → [docs/formatting.md](docs/formatting.md)
- Adding or reviewing `trace!`/`debug!`/`warn!` calls → [docs/logging.md](docs/logging.md)
- Updating this file after new commits land → [docs/update-agents.md](docs/update-agents.md)
- Cutting a release, tag format, changelog preview → [docs/releasing.md](docs/releasing.md)

## Domain map (path → doc)

| Path | Doc |
|---|---|
| `src/main.rs`, `src/commands/**` | [docs/cli-commands.md](docs/cli-commands.md) |
| `src/config.rs` | [docs/configuration.md](docs/configuration.md) |
| `src/sheet_utils.rs` | [docs/google-sheets.md](docs/google-sheets.md) |
| `src/format.rs`, `src/formatter/**`, `src/reader.rs` | [docs/formatting.md](docs/formatting.md) |
| `Cargo.toml`, `Cargo.lock` | cross-cutting — update whichever domain(s) the new/removed dep touches |
| `log` calls anywhere | [docs/logging.md](docs/logging.md) |
| `.github/workflows/releases.yml`, `cliff.toml` | [docs/releasing.md](docs/releasing.md) |

## Project-wide conventions

- Edition 2024, `anyhow::Result` in command entry points, `log`/`colog` for logging.
- Async is confined to [src/sheet_utils.rs](src/sheet_utils.rs); commands drive it with a fresh `tokio` runtime.
- Release profile uses LTO + `opt-level = "s"` + `strip = true` — avoid adding heavyweight dependencies without reason.

## Staying current

Before non-trivial work, follow [docs/update-agents.md](docs/update-agents.md) to diff the repo against `last-synced-commit` and refresh any affected domain doc, then bump the hash.
