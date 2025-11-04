# spreadsheet-extractor

spreadsheet-extractor

## Usage

```
$ spreadsheet-extractor --help

spreadsheet-extractor

Usage: spreadsheet-extractor [OPTIONS] <COMMAND>

Commands:
  fetch-sheet  Fetch google spreadsheet using range
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Path to a config file
  -h, --help             Print help
  -V, --version          Print version
```


## Configuration

Configuration can be provided as `TOML`, `YML` or `JSON` file either in the current working directory as `fw.*` or in the users home config directory, which maps to the following directories depending on the OS.
- Linux: `$HOME/.config/spreadsheet-extractor/config.*`
- macOS: `$HOME/Library/Application Support/spreadsheet-extractor/config.*`
- Windows: `%APPDATA%/zekro/spreadsheet-extractor/config.*`

You can also pass a configuration via the `--config` parameter.

Following, you can see an example configuration in `TOML` format.

```toml
[google]
client_secret_path = '~/.config/spreadsheet-extractor/secret.json'
token_storage_path = '~/.config/spreadsheet-extractor/tokencache.json'
```


## Install

You can either download the latest release builds form the [Releases page](https://github.com/aeros281/spreadsheet-extractor/releases) or you can install it using cargo install.
```
cargo install --git https://github.com/aeros281/spreadsheet-extractor
```
