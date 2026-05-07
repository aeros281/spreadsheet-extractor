# Google Sheets Integration

OAuth setup and Sheets API calls.

## Files
- [src/sheet_utils.rs](../src/sheet_utils.rs)

## Auth

> For instructions on creating the OAuth 2.0 Desktop app client ID and downloading the secret JSON, see the **Creating a Google OAuth 2.0 Client ID** section in [README.md](../README.md).

`build_hub(&Config)`:
- Reads `config.google.client_secret_path` (installed-app OAuth client secret JSON).
- Uses `yup_oauth2::InstalledFlowAuthenticator` with `InstalledFlowReturnMethod::Interactive` — the user completes the OAuth flow in a browser on first run.
- Persists tokens to `config.google.token_storage_path`.
- Both paths go through `simple_expand_tilde::expand_tilde` (so `~/...` works cross-platform).
- HTTPS client: `hyper_util` + `hyper_rustls` with native roots, http1 only.
- `rustls::crypto::ring::default_provider().install_default()` must be called once at startup — currently done in [src/main.rs](../src/main.rs).

## Operations
- `resolve_gid_to_name(config, spreadsheet_id, gid) -> anyhow::Result<String>` — calls `spreadsheets.get` and finds the sheet whose `properties.sheet_id` matches the numeric GID, returning its title. Used by both `fetch-sheet` and `push-sheet` so callers can pass a GID instead of a tab name.
- `a1_range(sheet_name, range) -> String` — builds a properly quoted A1-notation string (`'Sheet Name'!A1:B2`), escaping internal single-quotes.
- `get_sheet_data(config, sheet_id, range) -> (Response, ValueRange)` — `spreadsheets.values.get`. URL-encodes `/` in the range string before calling the API.
- `clear_tab(config, sheet_id, tab_name) -> ClearValuesResponse` — `spreadsheets.values.clear` with an empty `ClearValuesRequest`. URL-encodes `/` in tab name.
- `write_page(config, sheet_id, tab_name, csv_path)` — clears the tab, reads the CSV (no header) with `csv::ReaderBuilder`, then delegates to `upload_rows`.
- `write_page_json(config, sheet_id, tab_name, json_path)` — clears the tab, parses the file as a JSON array of objects (`Vec<serde_json::Map<String, Value>>`), derives the header row from the first object's keys, then delegates to `upload_rows`. Null values become empty strings; nested objects/arrays are serialised with `serde_json::to_string`.
- `upload_rows(config, sheet_id, tab_name, rows)` *(private)* — sends `spreadsheets.values.append` with `valueInputOption=USER_ENTERED`. URL-encodes `/` in the tab name.

## Notes
- Return types mix `google_sheets4::Result` (for API calls) and `anyhow::Result` (for write helpers and `resolve_gid_to_name`, which also do IO or wrap API errors).
- `/` in sheet names or ranges must be percent-encoded as `%2F` before being passed to the API — the helpers do this automatically.
