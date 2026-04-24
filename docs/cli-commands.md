# CLI & Commands

Binary entrypoint and subcommand dispatch.

## Files
- [src/main.rs](../src/main.rs) — `Cli` parser, logger init, rustls crypto provider install, config parse, dispatch.
- [src/commands/mod.rs](../src/commands/mod.rs) — `Command` trait, `re_export!` macro, `register_commands!` macro generating the `Commands` enum with a `Deref<Target = dyn Command>` impl.
- [src/commands/show_config.rs](../src/commands/show_config.rs) — prints resolved config path and Google section.
- [src/commands/fetch_sheet.rs](../src/commands/fetch_sheet.rs) — resolves sheet GID to name, reads a range, optionally forward-fills repeat columns, outputs in the requested `Format`.
- [src/commands/push_sheet.rs](../src/commands/push_sheet.rs) — resolves sheet GID to name, clears the tab, and appends rows from a CSV file.

## Conventions
- Each subcommand is a struct deriving `clap::Args`, implementing `Command::run(&self, &Config) -> Result<()>`.
- To add a subcommand: create `src/commands/<name>.rs`, add the module to `re_export!` in [src/commands/mod.rs](../src/commands/mod.rs), and add the struct name to `register_commands!` in [src/main.rs](../src/main.rs).
- Async work uses `tokio::runtime::Runtime::new()?.block_on(...)` inside `run` (the `Command` trait is sync).

## `fetch-sheet` args
- `sheet_id` — spreadsheet ID.
- `gid` — numeric sheet tab GID (visible in the URL as `&gid=...`).
- `range` — cell range, e.g. `G4:Q7`. When `--headers` is used, omit the header row from the range.
- `--format` — `json` (default), `csv`, `table`.
- `--repeat-columns A,C,...` — columns whose empty cells inherit the previous row's value (applied after fetch, before output).
- `--headers A:HeaderForA,C:HeaderForC` — explicit column-to-header mapping. Only the listed columns appear in the output. The column letters are relative to the fetched data (A = first column of the range). No header row should be included in the range when this flag is set.

## `push-sheet` args
- `input_file` — CSV file to upload.
- `sheet_id` — spreadsheet ID.
- `gid` — numeric sheet tab GID.
- `--format` — `csv` (default); only CSV is implemented.

## Global flags
- `-c/--config <path>` — explicit config file (otherwise cfgdir lookup).
- `-l/--log-level <level>` — `log::LevelFilter`, defaults to `warn`.
