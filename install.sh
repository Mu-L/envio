#!/usr/bin/env bash
#
# Install (or uninstall) the envio CLI on Linux or macOS.
#
# Install:
#   curl -fsSL https://raw.githubusercontent.com/humblepenguinn/envio/main/install.sh | bash
#
# Uninstall:
#   curl -fsSL https://raw.githubusercontent.com/humblepenguinn/envio/main/install.sh | bash -s -- uninstall
#
# Environment overrides:
#   ENVIO_VERSION      tag/version to install - default: latest.
#   ENVIO_INSTALL_DIR  install directory - default: ~/.local/bin.

set -euo pipefail

REPO="humblepenguinn/envio"
INSTALL_DIR="${ENVIO_INSTALL_DIR:-${INSTALL_DIR:-$HOME/.local/bin}}"

COLOR_OFF=''
COLOR_RED=''
COLOR_GREEN=''
COLOR_DIM=''
COLOR_YELLOW=''

if [[ -t 1 ]]; then
    COLOR_OFF='\033[0m'
    COLOR_RED='\033[0;31m'
    COLOR_GREEN='\033[0;32m'
    COLOR_DIM='\033[0;2m'
    COLOR_YELLOW='\033[0;33m'
fi

error() {
    echo -e "${COLOR_RED}error${COLOR_OFF}:" "$@" >&2
    exit 1
}

info() {
    echo -e "${COLOR_DIM}$*${COLOR_OFF}"
}

success() {
    echo -e "${COLOR_GREEN}$*${COLOR_OFF}"
}

warn() {
    echo -e "${COLOR_YELLOW}$*${COLOR_OFF}"
}

uninstall() {
    if [[ -f "${INSTALL_DIR}/envio" ]]; then
        info "Uninstalling envio from ${INSTALL_DIR}..."
        rm -f "${INSTALL_DIR}/envio"
        success "envio uninstalled successfully!"
    else
        info "envio is not installed in ${INSTALL_DIR}."
    fi
}

detect_target() {
    local os arch

    os=$(uname -s)
    arch=$(uname -m)

    case "$os" in
        Linux)
            case "$arch" in
                x86_64)
                    echo "x86_64-unknown-linux-gnu"
                    ;;
                aarch64 | arm64)
                    echo "aarch64-unknown-linux-gnu"
                    ;;
                i686 | i386)
                    echo "i686-unknown-linux-gnu"
                    ;;
                *)
                    error "Unsupported architecture: $arch"
                    ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64)
                    echo "x86_64-apple-darwin"
                    ;;
                arm64)
                    echo "aarch64-apple-darwin"
                    ;;
                *)
                    error "Unsupported architecture: $arch"
                    ;;
            esac
            ;;
        MINGW* | MSYS* | CYGWIN*)
            error "Windows installation is not supported using this script. Please use the MSI installer from the releases page"
            ;;
        *)
            error "Unsupported operating system: $os"
            ;;
    esac
}

get_latest_version() {
    local version

    if command -v curl &>/dev/null; then
        version=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget &>/dev/null; then
        version=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
        error "Neither curl nor wget found. Please install one of them."
    fi

    if [[ -z "$version" ]]; then
        error "Failed to fetch the latest version"
    fi

    echo "$version"
}

cleanup() {
    if [[ -n "${tmpdir:-}" ]]; then
        rm -rf "$tmpdir"
    fi
}

download_and_install() {
    local version="$1"
    local target="$2"
    local filename="envio-${version}-${target}.tar.gz"
    local url="https://github.com/${REPO}/releases/download/${version}/${filename}"

    tmpdir=$(mktemp -d)
    trap cleanup EXIT

    info "Downloading envio ${version} for ${target}..."

    if command -v curl &>/dev/null; then
        curl -fsSL "$url" -o "${tmpdir}/${filename}" || error "Failed to download ${url}"
    elif command -v wget &>/dev/null; then
        wget -q "$url" -O "${tmpdir}/${filename}" || error "Failed to download ${url}"
    fi

    info "Extracting..."
    tar -xzf "${tmpdir}/${filename}" -C "$tmpdir"

    local extract_dir="${tmpdir}/envio-${version}-${target}"

    if [[ ! -d "$extract_dir" ]]; then
        error "Expected directory ${extract_dir} not found after extraction"
    fi

    info "Copying to ${INSTALL_DIR}..."

    cp "${extract_dir}/envio" "$INSTALL_DIR/"
    chmod +x "${INSTALL_DIR}/envio"
}

if [[ "${1:-}" == "uninstall" || "${1:-}" == "--uninstall" ]]; then
    uninstall
    exit 0
fi

mkdir -p "$INSTALL_DIR"

info "Detecting platform..."
target=$(detect_target)
info "Detected target: ${target}"

if command -v gpg &>/dev/null; then
    info "gpg already installed ($(gpg --version | head -1))"
else
    warn "note: gpg was not found on your system. Please install it if you plan to use gpg encryption"
fi

is_upgrade=false

if [[ -f "${INSTALL_DIR}/envio" ]]; then
    is_upgrade=true
    installed_version=$("${INSTALL_DIR}/envio" version 2>/dev/null | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+' || echo "0.0.0")
    info "Existing installation found at ${INSTALL_DIR}/envio (version ${installed_version})"
fi

if [[ -n "${ENVIO_VERSION:-}" ]]; then
    version="$ENVIO_VERSION"
    if [[ ! "$version" =~ ^v ]]; then
        version="v$version"
    fi
    info "Using specified version: ${version}"
else
    info "Fetching latest version..."
    version=$(get_latest_version)
    info "Latest version: ${version}"
fi

if [[ "$is_upgrade" == true && "${installed_version#v}" == "${version#v}" ]]; then
    info "envio is already up to date (version ${version#v})."
    exit 0
fi

if [[ "$is_upgrade" == true ]]; then
    info "Upgrading envio to ${version}..."
else
    info "Installing envio ${version}..."
fi

download_and_install "$version" "$target"

if [[ "$is_upgrade" == true ]]; then
    success "envio upgraded to ${version} successfully!"
else
    success "envio ${version} installed successfully!"
fi

case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *) warn "note: add $INSTALL_DIR to your PATH (e.g. export PATH=\"\$PATH:$INSTALL_DIR\")" ;;
esac

info "Run 'envio --help' to get started."