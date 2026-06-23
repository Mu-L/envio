<div align="center">
  <img src="assets/icon.svg" width="200px">
  <h1>envio</h1>
</div>

<div align="center">
  <h2>A secure command-line tool for managing environment variables</h2>
  <a href="https://github.com/humblepenguinn/envio/workflows/CICD.yml"><img src="https://github.com/humblepenguinn/envio/actions/workflows/CICD.yml/badge.svg" alt="CICD"></a>
  <a href="https://crates.io/crates/envio"><img src="https://img.shields.io/crates/v/envio.svg" alt="Version info"></a>
  <a href="https://www.pcrf.net/">
    <img src="https://img.shields.io/badge/Support-Palestine-00b03f.svg?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz48c3ZnIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgd2lkdGg9IjEyMDAiIGhlaWdodD0iNjAwIiB2aWV3Qm94PSIwIDAgNiAzIj48cmVjdCBmaWxsPSIjMDA5NjM5IiB3aWR0aD0iNiIgaGVpZ2h0PSIzIi8+PHJlY3QgZmlsbD0iI0ZGRiIgd2lkdGg9IjYiIGhlaWdodD0iMiIvPjxyZWN0IHdpZHRoPSI2IiBoZWlnaHQ9IjEiLz48cGF0aCBmaWxsPSIjRUQyRTM4IiBkPSJNMCwwbDIsMS41TDAsM1oiLz48L3N2Zz4%3D" />
  </a>
</div>

<div align="center" style="margin-top: 24px; margin-bottom: 24px;">
  <img alt="Demo" src="assets/demo.svg" width="80%">
</div>

## About

`envio` is a command-line tool for securely managing environment variables. It allows users to create encrypted profiles containing environment variables for a specific project or use case. The tool provides various operations to manage these profiles, such as loading them into terminal sessions or running programs with the specified environment variables.

Some key features of `envio` include:

- **Encrypt** profiles using different encryption methods
- **Start** new shell sessions with profile environment variables injected
- **Run** programs with your profiles
- And more!

## Installation

Pre-built binaries for Linux, macOS, and Windows are available on the [releases page](https://github.com/humblepenguinn/envio/releases).

### Install Script

This script only supports Linux and macOS. 

```bash
curl -fsSL https://raw.githubusercontent.com/humblepenguinn/envio/main/install.sh | bash
```
Set a custom install directory with `ENVIO_INSTALL_DIR` (default: `~/.local/bin`):
```bash
curl -fsSL https://raw.githubusercontent.com/humblepenguinn/envio/main/install.sh | ENVIO_INSTALL_DIR=/usr/local/bin bash
```
Install a specific version with `ENVIO_VERSION` (default: latest):
```bash
curl -fsSL https://raw.githubusercontent.com/humblepenguinn/envio/main/install.sh | ENVIO_VERSION=v0.0.0 bash
```
Uninstall:
```bash
curl -fsSL https://raw.githubusercontent.com/humblepenguinn/envio/main/install.sh | bash -s -- uninstall
```
### Cargo
```bash
cargo install envio
```

### Linux
#### Arch Linux
Use your favorite AUR helper:
```bash
paru -S envio      # or envio-bin for pre-built binary
```

#### Debian/Ubuntu
A `.deb` package is also published on the [releases page](https://github.com/envio-cli/envio/releases):
```bash
sudo dpkg -i envio_<version>_<arch>.deb
```

### macOS
```bash
brew install envio
```

### Windows
Download the MSI installer or zip archive from the [releases page](https://github.com/envio-cli/envio/releases).

## Usage

See the [Usage Guide](docs/usage.md) for detailed instructions on how to use the tool.

## Contributing

Take a look at the [Contributing Guide](CONTRIBUTING.md) for more information.

## License

`envio` is available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files for license details.
