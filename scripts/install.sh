#!/usr/bin/env bash

# MIT License

# Copyright (c) 2025 Ritchie Mwewa

# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:

# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

set -euo pipefail

GITHUB_REPO="${CERIUM_REPO:-rly0nheart/cerium}"
INSTALL_DIR="${CERIUM_INSTALL_DIR:-/usr/local/bin}"
BIN_NAME="ce"
NIGHTLY=false
FEATURES=""

usage() {
    echo "Usage: install.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --nightly            Install the latest nightly build instead of stable"
    echo "  --dir <path>         Installation directory (default: /usr/local/bin)"
    echo "  --features <value>   Feature variant to install: checksum, magic, all"
    echo "  -h, --help           Show this help message"
}

while [ $# -gt 0 ]; do
    case "$1" in
        --nightly)    NIGHTLY=true ;;
        --features)   shift; FEATURES="${1:-}" ;;
        --dir)        shift; INSTALL_DIR="${1:-$INSTALL_DIR}" ;;
        -h|--help)    usage; exit 0 ;;
    esac
    shift
done

# Validate features value
if [ -n "$FEATURES" ]; then
    case "$FEATURES" in
        checksum|magic|all) ;;
        *)
            echo "error: invalid --features value: ${FEATURES}"
            echo "valid values: checksum, magic, all"
            exit 1
            ;;
    esac
fi

# --- Detect platform ---

detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        *)
            echo "error: unsupported OS: $(uname -s)" >&2
            exit 1
            ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "x86_64" ;;
        aarch64|arm64)   echo "aarch64" ;;
        *)
            echo "error: unsupported architecture: $(uname -m)" >&2
            exit 1
            ;;
    esac
}

# --- Install libmagic runtime library ---

install_libmagic() {
    echo "~ checking libmagic installation..."

    # Check if libmagic is already available
    if ldconfig -p 2>/dev/null | grep -q libmagic; then
        echo "found libmagic"
        return 0
    fi

    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
    else
        echo "warning: unable to detect distro, skipping libmagic"
        return 1
    fi

    echo "~ installing libmagic for: ${DISTRO}"

    case "$DISTRO" in
        ubuntu|debian)
            sudo apt-get update -qq
            sudo apt-get install -y -qq libmagic1 || return 1
            ;;
        fedora|rhel|centos)
            sudo dnf install -y -q file-libs || return 1
            ;;
        arch)
            sudo pacman -Sy --noconfirm --quiet file || return 1
            ;;
        alpine)
            sudo apk add --no-progress file || return 1
            ;;
        *)
            echo "error: unsupported distro for libmagic: ${DISTRO}"
            return 1
            ;;
    esac

    echo "* libmagic installed successfully"
}

# --- Main ---

main() {
    echo ""
    echo "cerium installer"
    echo ""

    # Check for curl
    if ! command -v curl >/dev/null 2>&1; then
        echo "error: 'curl' is required but not found"
        exit 1
    fi

    OS=$(detect_os)
    ARCH=$(detect_arch)

    echo "platform: ${OS}-${ARCH}"

    # Fetch release info from GitHub (mirror)
    if [ "$NIGHTLY" = true ]; then
        echo "~ fetching latest nightly release..."
        RELEASES_URL="https://api.github.com/repos/${GITHUB_REPO}/releases"
        RELEASE_JSON=$(curl -fsSL "$RELEASES_URL") || {
            echo "error: failed to fetch releases from GitHub"
            exit 1
        }

        # Find the first prerelease (nightly) entry
        RELEASE_JSON=$(echo "$RELEASE_JSON" | python3 -c "
import sys, json
releases = json.load(sys.stdin)
for r in releases:
    if r.get('prerelease'):
        json.dump(r, sys.stdout)
        sys.exit(0)
print('{}')
sys.exit(1)
" 2>/dev/null) || {
            echo "error: no nightly release found"
            exit 1
        }
    else
        echo "~ fetching latest stable release..."
        RELEASE_URL="https://api.github.com/repos/${GITHUB_REPO}/releases/latest"
        RELEASE_JSON=$(curl -fsSL "$RELEASE_URL") || {
            echo "error: failed to fetch latest release from GitHub"
            exit 1
        }
    fi

    # Extract tag name
    TAG=$(echo "$RELEASE_JSON" | grep -o '"tag_name": *"[^"]*"' | head -1 | sed 's/.*"\([^"]*\)"$/\1/')

    if [ -z "${TAG:-}" ]; then
        echo "error: could not determine release version"
        exit 1
    fi

    echo "latest release: ${TAG}"

    # Build asset name based on features
    if [ -n "$FEATURES" ]; then
        ASSET_SUFFIX="-${FEATURES}"
    else
        ASSET_SUFFIX=""
    fi

    # Look for a matching binary asset: try ce-os-arch first, then cerium-os-arch
    DOWNLOAD_URL=""
    for prefix in "$BIN_NAME" "cerium"; do
        ASSET_PATTERN="${prefix}-${OS}-${ARCH}${ASSET_SUFFIX}"
        # Use exact match (end of URL) to avoid e.g. ce-linux-x86_64 matching ce-linux-x86_64-checksum
        DOWNLOAD_URL=$(echo "$RELEASE_JSON" | grep -o "\"browser_download_url\": *\"[^\"]*/${ASSET_PATTERN}\"" | head -1 | grep -o 'https://[^"]*') || true

        if [ -n "$DOWNLOAD_URL" ]; then
            break
        fi
    done

    if [ -z "$DOWNLOAD_URL" ]; then
        echo "error: no binary found for ${OS}-${ARCH} in release ${TAG}"
        echo "available assets:"
        echo "$RELEASE_JSON" | grep -o '"browser_download_url": *"[^"]*"' | sed 's/.*"\(https:[^"]*\)"/  - \1/' || true
        exit 1
    fi

    # Download binary to a temp directory
    echo "~ downloading ${DOWNLOAD_URL##*/}..."

    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT

    curl -fsSL -o "${TMPDIR}/${BIN_NAME}" "$DOWNLOAD_URL" || {
        echo "error: failed to download binary"
        exit 1
    }
    chmod +x "${TMPDIR}/${BIN_NAME}"

    echo "* download complete"

    # Install libmagic runtime when needed (non-fatal)
    if [ "$FEATURES" = "magic" ] || [ "$FEATURES" = "all" ]; then
        install_libmagic || echo "warning: libmagic not installed; file type detection (--magic) will not work"
    fi

    # Install binary
    echo "~ installing ${BIN_NAME} to ${INSTALL_DIR}..."

    mkdir -p "$INSTALL_DIR" 2>/dev/null || sudo mkdir -p "$INSTALL_DIR"

    if [ -w "$INSTALL_DIR" ]; then
        cp "${TMPDIR}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
    else
        sudo cp "${TMPDIR}/${BIN_NAME}" "${INSTALL_DIR}/${BIN_NAME}"
    fi

    echo "* installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}"

    # Verify
    if command -v "$BIN_NAME" >/dev/null 2>&1; then
        VERSION_OUTPUT=$("${INSTALL_DIR}/${BIN_NAME}" --version 2>/dev/null || true)
        echo ""
        echo "* cerium ${VERSION_OUTPUT:+(${VERSION_OUTPUT})} is ready"
        echo "run '${BIN_NAME} --help' to get started"
    else
        echo ""
        echo "warning: ${INSTALL_DIR} may not be in your PATH"
        echo "add it with: export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi

    echo ""
}

main "$@"
