# Google Sheets Integration

OAuth setup and Sheets API calls.

## Files
- [src/sheet_utils.rs](../src/sheet_utils.rs)

## Auth
`build_hub(&Config)`:
- Reads `config.google.client_secret_path` (installed-app OAuth client secret JSON).
- Uses `yup_oauth2::InstalledFlowAuthenticator` with `InstalledFlowReturnMethod::Interactive` — the user completes the OAuth flow in a browser on first run.
- Persists tokens to `config.google.token_storage_path`.
- Both paths go through `simple_expand_tilde::expand_tilde` (so `~/...` works cross-platform).
- HTTPS client: `hyper_util` + `hyper_rustls` with native roots, http1 only.
- `rustls::crypto::ring::default_provider().install_default()` must be called once at startup — currently done in [src/main.rs](../src/main.rs).

## Operations
- `get_sheet_data(config, sheet_id, range) -> (Response, ValueRange)` — `spreadsheets.values.get`.
- `clear_tab(config, sheet_id, tab_name) -> ClearValuesResponse` — `spreadsheets.values.clear` with an empty `ClearValuesRequest`.
- `write_page(config, sheet_id, tab_name, csv_path)` — clears the tab, reads the CSV (no header) with `csv::ReaderBuilder`, sends `spreadsheets.values.append` with `valueInputOption=USER_ENTERED`.

## Notes
- Return types mix `google_sheets4::Result` (for API calls) and `anyhow::Result` (for `write_page`, which also does CSV IO).
- `write_page` builds two hubs (one for clear, one for append) — an optimization candidate if latency matters.
