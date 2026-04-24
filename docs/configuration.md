# Configuration

Figment-based multi-source config loader.

## Files
- [src/config.rs](../src/config.rs)

## Structure
- `Config { google: Option<Google> }`
- `Google { client_secret_path: String, token_storage_path: String }`

## Resolution order
`Config::parse(opt_path)`:
- If a path is passed (via `--config`), `parse_from_file` dispatches on extension (`toml` / `yaml` / `yml` / `json`). Any other extension errors out.
- Otherwise `parse_from_cfgdir` merges, in order:
  1. `./spreadsheet-extractor.toml|yaml|json` (cwd, name from `CARGO_PKG_NAME`)
  2. `$HOME/.config/spreadsheet-extractor/config.toml|yaml|json`

Later sources override earlier ones (figment merge semantics).

## Adding a config field
Add the field to `Config` or a nested struct with `#[derive(Deserialize)]`. Path fields consumed by the Google layer get tilde-expanded at use-time, not at parse-time — see [google-sheets.md](google-sheets.md).
