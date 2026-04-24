# CLI & Commands

Binary entrypoint and subcommand dispatch.

## Files
- [src/main.rs](../src/main.rs) — `Cli` parser, logger init, rustls crypto provider install, config parse, dispatch.
- [src/commands/mod.rs](../src/commands/mod.rs) — `Command` trait, `re_export!` macro, `register_commands!` macro generating the `Commands` enum with a `Deref<Target = dyn Command>` impl.
- [src/commands/show_config.rs](../src/commands/show_config.rs) — prints resolved config path and Google section.
- [src/commands/fetch_sheet.rs](../src/commands/fetch_sheet.rs) — reads a range, outputs in the requested `Format`.
- [src/commands/push_sheet.rs](../src/commands/push_sheet.rs) — clears a tab and appends rows from a CSV file.

## Conventions
- Each subcommand is a struct deriving `clap::Args`, implementing `Command::run(&self, &Config) -> Result<()>`.
- To add a subcommand: create `src/commands/<name>.rs`, add the module to `re_export!` in [src/commands/mod.rs](../src/commands/mod.rs), and add the struct name to `register_commands!` in [src/main.rs](../src/main.rs).
- Async work uses `tokio::runtime::Runtime::new()?.block_on(...)` inside `run` (the `Command` trait is sync).

## Global flags
- `-c/--config <path>` — explicit config file (otherwise cfgdir lookup).
- `-l/--log-level <level>` — `log::LevelFilter`, defaults to `warn`.
