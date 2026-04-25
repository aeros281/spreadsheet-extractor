# Logging

Logging uses the `tracing` crate macros (`trace!`, `debug!`, `warn!`, `error!`) with `tracing-subscriber` as the backend, initialised in `main.rs`. The default level exposed to users is `warn`; finer levels are opt-in via `--log-level`.

## Files

- [src/main.rs](../src/main.rs) — subscriber init (`tracing_subscriber::fmt`) and `--log-level` flag.
- [src/sheet_utils.rs](../src/sheet_utils.rs) — all public async functions are instrumented with `#[tracing::instrument]`.
- All other source files import macros explicitly via `use tracing::{…}`.

## Spans

Every public async function in `sheet_utils.rs` is wrapped with `#[tracing::instrument(skip(config), err)]`. This:

- Opens a named span on function entry carrying the identifying arguments as fields (e.g. `sheet_id`, `range`, `gid`).
- Attaches those fields automatically to every log line emitted inside the function, including nested calls.
- Emits an ERROR event automatically if the function returns `Err(...)`.
- Skips `config` so OAuth credentials never appear in span fields.

When adding a new public async function to `sheet_utils.rs`, always add `#[tracing::instrument(skip(config), err)]`. Do **not** add a manual entry-point `trace!` or `debug!` line — the span covers it.

## Log levels — when to use each

| Level | Use for |
|-------|---------|
| `error!` | Unrecoverable failures that are about to propagate as an `Err`. Prefer returning the error; only log if the error is swallowed. |
| `warn!` | Unexpected but recoverable situations: empty API responses, missing optional data, edge-case fallbacks the user should know about. |
| `info!` | High-level progress milestones visible at normal verbosity (not currently used — reserve for future user-facing progress). |
| `debug!` | Per-call context useful during development: resolved names, row counts, encoded parameters, flag values. One `debug!` near the start of every `Command::run` entry point. |
| `trace!` | Step-by-step internals: encoded parameters, intermediate computation state inside async helpers. |

## Rules

1. **Every `Command::run` entry point** opens with a `debug!` line listing the key arguments (sheet_id, gid, range/file, format).
2. **Every public async function** in `sheet_utils.rs` uses `#[tracing::instrument(skip(config), err)]`. Do not add a manual entry-point log line.
3. **After each external API call** that returns meaningful metadata (row count, sheet list length, resolved name), log at `debug!`.
4. **Warn on empty or missing data** that is technically valid but likely unintended — e.g. an API response with no values, a CSV with no rows.
5. **Do not log secrets.** Never include token paths' file contents, OAuth tokens, or credential values. Logging the *path* to a credential file is fine.
6. **No `info!` calls** until a user-facing progress mode is designed. The gap between `warn` (default) and `debug` (opt-in) is intentional: normal runs are silent unless something is wrong.
7. **Format strings** — use the `key=value` style for structured fields: `debug!("values_get range={encoded_range}")`. Quote string values with `{foo:?}` when they may contain spaces or special characters.
8. **Import explicitly** in each file: `use tracing::{debug, trace, warn};` (only import the levels you actually use).

## Examples

```rust
// Command entry point
debug!(
    "fetch-sheet: sheet_id={} gid={} range={} format={:?}",
    self.sheet_id, self.gid, self.range, self.format
);

// Async helper — span is automatic, no entry log needed
#[tracing::instrument(skip(config), err)]
pub async fn get_sheet_data(config: &Config, sheet_id: &str, range: &str) -> Result<...> {
    debug!("values_get range={encoded_range}");  // fields from span appear automatically
}

// Post-API metadata
debug!("spreadsheet has {} sheet(s)", sheets.len());
debug!("gid {gid} resolved to {name:?}");

// Warn on empty/unexpected result
warn!("sheet returned no values for the requested range");
warn!("CSV file {path:?} contains no rows — tab will remain empty after clear");
```

## Runtime control

Users pass `--log-level <LEVEL>` (default `warn`). Valid values match `tracing_subscriber::filter::LevelFilter`: `off`, `error`, `warn`, `info`, `debug`, `trace`.

```
# see debug output with span fields
spreadsheet-extractor --log-level debug fetch-sheet …

# full trace including span entry/exit
spreadsheet-extractor --log-level trace fetch-sheet …
```
