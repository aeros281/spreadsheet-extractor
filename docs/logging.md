# Logging

Logging uses the `log` crate macros (`trace!`, `debug!`, `warn!`, `error!`) with `colog` as the backend, initialised in `main.rs`. The default level exposed to users is `warn`; finer levels are opt-in via `--log-level`.

## Files

- [src/main.rs](../src/main.rs) — logger init (`colog::default_builder`) and `--log-level` flag.
- All other source files import macros via `use log::{…}` (not via the crate-level `#[macro_use] extern crate log` in `main.rs`).

## Log levels — when to use each

| Level | Use for |
|-------|---------|
| `error!` | Unrecoverable failures that are about to propagate as an `Err`. Prefer returning the error; only log if the error is swallowed. |
| `warn!` | Unexpected but recoverable situations: empty API responses, missing optional data, edge-case fallbacks the user should know about. |
| `info!` | High-level progress milestones visible at normal verbosity (not currently used — reserve for future user-facing progress). |
| `debug!` | Per-call context useful during development: resolved names, row counts, encoded parameters, flag values. One `debug!` near the start of every public function or `Command::run` entry point. |
| `trace!` | Step-by-step internals: function entry/exit for async helpers, raw API request parameters, intermediate computation state. |

## Rules

1. **Every `Command::run` entry point** opens with a `debug!` line listing the key arguments (sheet_id, gid, range/file, format).
2. **Every public async function** in `sheet_utils.rs` opens with a `trace!` line (function name + identifying parameters).
3. **After each external API call** that returns meaningful metadata (row count, sheet list length, resolved name), log at `debug!`.
4. **Warn on empty or missing data** that is technically valid but likely unintended — e.g. an API response with no values, a CSV with no rows.
5. **Do not log secrets.** Never include token paths' file contents, OAuth tokens, or credential values. Logging the *path* to a credential file is fine.
6. **No `info!` calls** until a user-facing progress mode is designed. The gap between `warn` (default) and `debug` (opt-in) is intentional: normal runs are silent unless something is wrong.
7. **Format strings** — use the `key=value` style for structured fields: `debug!("values_get range={encoded_range}")`. Quote string values with `{foo:?}` when they may contain spaces or special characters.
8. **Import explicitly** in each file: `use log::{debug, trace, warn};` (only import the levels you actually use). Do not rely on the `#[macro_use] extern crate log` re-export from `main.rs` in library-style modules.

## Examples

```rust
// Command entry point
debug!(
    "fetch-sheet: sheet_id={} gid={} range={} format={:?}",
    self.sheet_id, self.gid, self.range, self.format
);

// Async helper entry
trace!("resolve_gid_to_name: spreadsheet_id={spreadsheet_id} gid={gid}");

// Post-API metadata
debug!("spreadsheet has {} sheet(s)", sheets.len());
debug!("gid {gid} resolved to {name:?}");

// Warn on empty/unexpected result
warn!("sheet returned no values for the requested range");
warn!("CSV file {path:?} contains no rows — tab will remain empty after clear");
```

## Runtime control

Users pass `--log-level <LEVEL>` (default `warn`). Valid values match `log::LevelFilter`: `off`, `error`, `warn`, `info`, `debug`, `trace`.

```
# see debug output
spreadsheet-extractor --log-level debug fetch-sheet …

# full trace
spreadsheet-extractor --log-level trace fetch-sheet …
```
