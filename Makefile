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

# Ansi colours
RED    := \\033[1;31m
GREEN  := \\033[1;32m
YELLOW := \\033[1;33m
RESET  := \\033[0m

BINARY_NAME := ce
INSTALL_PATH := ~/.cargo/bin/$(BINARY_NAME)
SETUP_SCRIPT := scripts/libmagic.sh
SOURCE_MAP_GENERATOR_SCRIPT := scripts/generate_source_map.py

# Default target
default: build

# Run the setup script (scripts/libmagic.sh) and source map generator (scripts/generate_source_map.py)
setup:
	@# Ensure the script is executable
	@test -x $(SETUP_SCRIPT) || chmod +x $(SETUP_SCRIPT)
	@echo -e "     $(GREEN)Running$(RESET) $(SETUP_SCRIPT)"
	@./$(SETUP_SCRIPT)
	@echo -e "     $(GREEN)Running$(RESET) $(SOURCE_MAP_GENERATOR_SCRIPT)"
	@python3 $(SOURCE_MAP_GENERATOR_SCRIPT)

# Build target — depends on setup
build: setup
	@cargo build --release

# Run the cerium (after ensuring dependencies are installed)
run: setup
	@cargo run -- $(ARGS)

# Clean build artifacts
clean:
	@cargo clean

# Format code
fmt:
	@cargo fmt --all

# Lint code with Clippy
lint:
	@cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
	@cargo test

# Install the binary globally
install: setup
	@echo -e "     $(GREEN)Running$(RESET) cargo install"
	@cargo install --path . --force --all-features
	@echo -e "   $(GREEN)Installed$(RESET) $(BINARY_NAME) to $(INSTALL_PATH)"

# Rebuild from scratch
rebuild: clean build

# Phony targets (non-file targets)
.PHONY: setup build run clean fmt lint test install rebuild
