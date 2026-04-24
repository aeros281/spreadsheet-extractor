# Formatting & IO

Output format selection and CSV input helpers.

## Files
- [src/format.rs](../src/format.rs) — `Format` enum (`Json`, `Csv`, `Table`), used by `clap::ValueEnum` on subcommand `--format` flags. `Table` is not implemented and prints a placeholder.
- [src/formatter/mod.rs](../src/formatter/mod.rs) — re-exports `csv` and `json`.
- [src/formatter/csv.rs](../src/formatter/csv.rs) — `print_csv(values, writer)`.
- [src/formatter/json.rs](../src/formatter/json.rs) — `convert_to_json(values)` and `print_json(writer, parsed)`.
- [src/reader.rs](../src/reader.rs) — `read_csv_to_map` returns `Vec<IndexMap<String, Value>>`. Currently `#[allow(dead_code)]` — reserved for future use.

## Adding a format
1. Add a variant to `Format` in [src/format.rs](../src/format.rs) (and its `Display` match arm).
2. Add a module under [src/formatter/](../src/formatter/) and re-export it from `mod.rs`.
3. Handle the new variant in the subcommand's `match self.format` (e.g. [src/commands/fetch_sheet.rs](../src/commands/fetch_sheet.rs)).
