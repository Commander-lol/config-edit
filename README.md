# config-edit

A command-line tool for performing simple edits to configuration files in various formats (TOML, YAML, JSON, Plist).

## Features

- **Multiple Format Support**: Works with TOML, YAML, JSON, and Plist files
- **Format Conversion**: Read from one format and write to another
- **In-place Editing**: Modify files directly or output to a different file
- **JSON Pointer Support**: Use [RFC6901 JSON Pointer](https://tools.ietf.org/html/rfc6901) syntax to reference nested values

## Actions

### Get

Retrieve a value at a specified path:

```bash
config-edit -i config.toml get  # Get the entire document (omit the path)
config-edit -i config.toml get ""  # Also gets the entire document
config-edit -i config.toml get "/server/port"  # Get a specific value
```

### Set

Set a value at a specified path (overwrites existing values):

```bash
config-edit -i config.toml -w set "/server/port" "8080"
config-edit -i config.toml -w set "/server/host" "\"localhost\""  # For string values, use escaped quotes
config-edit -i config.toml -w set "/server/enabled" "true"  # Boolean values
```

### Append

Append a value to an array at a specified path:

```bash
config-edit -i config.toml -w append "/server/allowed_ips" "\"192.168.1.1\""
```

## Options

- `-i, --input <FILE>`: Path to the input file (required)
- `-o, --output <FILE>`: Write output to a different file
- `-w, --write`: Modify the input file in place
- `--if <FORMAT>`: Override input file format detection
- `--of <FORMAT>`: Override output file format detection

Supported formats: `toml`, `yaml`, `json`, `plist`

## Examples

### Convert between formats

```bash
# Convert TOML to JSON
config-edit -i config.toml -o config.json get

# Convert JSON to YAML
config-edit -i config.json -o config.yaml --of yaml get
```

### Modify a configuration value

```bash
# Change the server port in a TOML file
config-edit -i config.toml -w set "/server/port" "9000"
```

### Extract a section of a configuration file

```bash
# Get only the server section from a config file
config-edit -i config.toml get "/server"
```

## Installation

```bash
cargo install config-edit
```

## Building from source

```bash
git clone https://github.com/yourusername/config-edit.git
cd config-edit
cargo build --release
```

The binary will be available at `target/release/config-edit`.

## Releases

Releases are automatically built for Linux, macOS, and Windows using GitHub Actions when a semantic version tag is pushed. To create a new release:

1. Update the version in `Cargo.toml`
2. Commit the changes
3. Tag the commit with a semantic version (e.g., `v1.0.0`)
4. Push the tag to GitHub

```bash
git tag v1.0.0
git push origin v1.0.0
```

This will trigger the GitHub Actions workflow to build the binaries and create a release with the binaries attached.

## License

[Apache License 2.0](LICENSE)
