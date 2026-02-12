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

RED="\033[1;31m"
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
CYAN="\033[1;36m"
RESET="\033[0m"

set -euo pipefail

echo -e "    ${CYAN}Checking${RESET} libmagic and pkg-config installations"

# Check if pkg-config and libmagic.pc actually work
if command -v pkg-config >/dev/null 2>&1 && pkg-config --exists libmagic 2>/dev/null; then
    echo -e "       ${GREEN}Found${RESET} libmagic (with pkg-config)"
    exit 0
fi

# Detect distro
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
else
    echo -e "      ${YELLOW}Unable${RESET} to detect distro: ${DISTRO}"
    exit 1
fi

echo -e "  ${GREEN}Installing${RESET} libmagic and pkg-config for: ${DISTRO}"

case "$DISTRO" in
    ubuntu|debian)
        sudo apt-get update -qq
        sudo apt-get install -y -qq libmagic1 libmagic-dev pkg-config || {
            echo -e "      ${RED}Failed${RESET} to install via apt, on: ${DISTRO}"
            exit 1
        }
        ;;
    fedora|rhel|centos)
        sudo dnf install -y -q file-libs file-devel pkgconf-pkg-config || {
            echo -e "      ${RED}Failed${RESET} to install via dnf, on: ${DISTRO}"
            exit 1
        }
        ;;
    arch)
        sudo pacman -Sy --noconfirm --quiet file pkgconf || {
            echo -e "      ${RED}Failed${RESET} to install via pacman, on: ${DISTRO}"
            exit 1
        }
        ;;
    alpine)
        sudo apk add --no-progress file file-dev pkgconf || {
            echo -e "      ${RED}Failed${RESET} to install via apk, on: ${DISTRO}"
            exit 1
        }
        ;;
    *)
        echo -e " ${RED}Unsupported${RESET} distro: ${DISTRO}"
        exit 1
        ;;
esac

# Final sanity check
if pkg-config --exists libmagic 2>/dev/null; then
    echo -e "    ${GREEN}libmagic${RESET} successfully installed and detected via pkg-config, on: ${DISTRO}"
else
    echo -e "${YELLOW}Installation${RESET} completed but libmagic.pc not found, on: ${DISTRO}"
    pkg-config --list-all | grep -i magic || true
    exit 1
fi
