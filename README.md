# spreadsheet-extractor

A CLI tool to read from and write to Google Sheets ranges.

## Usage

```
$ spreadsheet-extractor --help

Usage: spreadsheet-extractor [OPTIONS] <COMMAND>

Commands:
  show-config  Show the current configuration
  fetch-sheet  Fetch a Google Sheets range and print it
  push-sheet   Push a CSV file to a Google Sheets tab
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>        Path to a config file
  -l, --log-level <LOG_LEVEL>  [default: warn]
  -h, --help                   Print help
  -V, --version                Print version
```

### fetch-sheet

Fetch a range from a Google Sheets tab and print it as JSON, CSV, or a formatted table.

```
$ spreadsheet-extractor fetch-sheet --help

Usage: spreadsheet-extractor fetch-sheet [OPTIONS] <SHEET_ID> <GID> <RANGE>

Arguments:
  <SHEET_ID>  The ID of the spreadsheet (from the URL)
  <GID>       The numeric GID of the sheet tab (visible in the URL as &gid=...)
  <RANGE>     The cell range, e.g. A1:D10

Options:
  -f, --format <FORMAT>                  Output format [default: json] [possible values: json, csv, table]
      --repeat-columns <REPEAT_COLUMNS>  Columns whose empty cells inherit the previous row's value (e.g. A,C)
      --headers <HEADERS>                Column-to-header mapping, e.g. "A:Name,C:Age". Only listed columns
                                         are included. When set, omit the header row from the range.
  -h, --help                             Print help
```

**Examples**

Fetch cells `A1:D10` from tab with GID `0` in spreadsheet `1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgVE2upms`, print as JSON:

```bash
spreadsheet-extractor fetch-sheet \
  1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgVE2upms \
  0 \
  A1:D10
```

Same fetch but output as CSV:

```bash
spreadsheet-extractor fetch-sheet \
  1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgVE2upms \
  0 \
  A1:D10 \
  --format csv
```

Same fetch but rendered as a human-readable table (first row is used as the header):

```bash
spreadsheet-extractor fetch-sheet \
  1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgVE2upms \
  0 \
  A1:D10 \
  --format table
```

Fetch a range where column A contains merged/grouped labels (empty cells should repeat the value from the row above):

```bash
spreadsheet-extractor fetch-sheet \
  1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgVE2upms \
  812345678 \
  B2:F50 \
  --repeat-columns A,B
```

Fetch only specific columns with explicit header names — the range starts at row 2 (no header row) and only columns B and D are kept:

```bash
spreadsheet-extractor fetch-sheet \
  1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgVE2upms \
  812345678 \
  B2:F50 \
  --headers "B:ProductName,D:Price"
```

Column letters in `--headers` are absolute spreadsheet column letters (matching the range). Only the listed columns appear in the output; any other columns in the range are discarded.

> **Finding the spreadsheet ID and GID**
> Open the sheet in your browser. The URL looks like:
> `https://docs.google.com/spreadsheets/d/1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgVE2upms/edit#gid=812345678`
> - Spreadsheet ID: `1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgVE2upms`
> - GID: `812345678`

### push-sheet

Clear a Google Sheets tab and append rows from a local CSV file.

```
$ spreadsheet-extractor push-sheet --help

Usage: spreadsheet-extractor push-sheet [OPTIONS] <INPUT_FILE> <SHEET_ID> <GID>

Arguments:
  <INPUT_FILE>  Path to the CSV file to upload
  <SHEET_ID>    The ID of the spreadsheet (from the URL)
  <GID>         The numeric GID of the sheet tab (visible in the URL as &gid=...)

Options:
  -f, --format <FORMAT>  Input format [default: csv] [possible values: json, csv, table]
  -h, --help             Print help
```

**Example**

Upload `report.csv` to the tab with GID `812345678`:

```bash
spreadsheet-extractor push-sheet \
  report.csv \
  1BxiMVs0XRA5nFMdKvBdBZjgmUUqptlbs74OgVE2upms \
  812345678
```

The tab is cleared before the new rows are appended.

## Configuration

Configuration can be provided as a `TOML`, `YAML`, or `JSON` file either in the current working directory as `spreadsheet-extractor.*` or in the user's home config directory:

- Linux / macOS: `$HOME/.config/spreadsheet-extractor/config.*`
- Windows: `C:\Users\<username>\.config\spreadsheet-extractor\config.*`

You can also pass a config file explicitly with `--config <path>`.

```toml
[google]
client_secret_path = '~/.config/spreadsheet-extractor/secret.json'
token_storage_path = '~/.config/spreadsheet-extractor/tokencache.json'
```

`client_secret_path` must point to a Google OAuth 2.0 **Desktop app** client secret JSON downloaded from the Google Cloud Console. On first run the tool will open a browser to complete the OAuth flow; the token is then cached at `token_storage_path` for subsequent runs.

### Creating a Google OAuth 2.0 Client ID

1. Go to the [Google Cloud Console](https://console.cloud.google.com/) and create or select a project.
2. Navigate to **APIs & Services → Library**, search for **Google Sheets API**, and click **Enable**.
3. Navigate to **APIs & Services → OAuth consent screen**:
   - Choose **External** (or **Internal** if using a Google Workspace org).
   - Fill in the required app name and support email fields, then save.
   - Under **Data Access**, add the following scopes:
     - `https://www.googleapis.com/auth/drive`
     - `https://www.googleapis.com/auth/drive.metadata.readonly`
     - `https://www.googleapis.com/auth/documents`
   - Under **Audience → Test users**, add the Google account you will authenticate with (required while the app is in *Testing* status).
4. Navigate to **APIs & Services → Credentials**, click **+ Create Credentials → OAuth client ID**:
   - Application type: **Desktop app**
   - Give it a name (e.g. `spreadsheet-extractor`) and click **Create**.
5. Click **Download JSON** on the newly created credential. Save the file somewhere permanent (e.g. `~/.config/spreadsheet-extractor/secret.json`).
6. Point `client_secret_path` in your config file at that downloaded JSON.

## Install

Download the latest release from the [Releases page](https://github.com/aeros281/spreadsheet-extractor/releases), or install from source with Cargo:

```
cargo install --git https://github.com/aeros281/spreadsheet-extractor
```
