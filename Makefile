# Zaxiom Makefile
# Cross-platform build commands for development

.PHONY: build release run clean check fmt lint test help

# Default target
all: build

# Debug build
build:
	cargo build

# Release build (optimized)
release:
	cargo build --release

# Run debug build
run:
	cargo run

# Run release build
run-release:
	cargo run --release

# Clean build artifacts
clean:
	cargo clean

# Type check without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Check formatting without modifying
fmt-check:
	cargo fmt --check

# Run clippy lints
lint:
	cargo clippy -- -D warnings

# Run tests
test:
	cargo test

# Run all checks (useful before committing)
ci: fmt-check lint check test
	@echo "All checks passed!"

# Install to system (Windows only, delegates to PowerShell script)
install:
ifeq ($(OS),Windows_NT)
	powershell -ExecutionPolicy Bypass -File run/install.ps1
else
	@echo "Error: Zaxiom only runs on Windows (requires ConPTY)"
	@echo "You can still build and contribute from any platform:"
	@echo "  make build    - compile the project"
	@echo "  make check    - type check"
	@echo "  make lint     - run clippy"
	@exit 1
endif

# Uninstall (Windows only)
uninstall:
ifeq ($(OS),Windows_NT)
	powershell -ExecutionPolicy Bypass -File run/install.ps1 -Uninstall
else
	@echo "Error: Nothing to uninstall on non-Windows platforms"
	@exit 1
endif

# Quick update (Windows only) - rebuild and copy to install location
update:
ifeq ($(OS),Windows_NT)
	powershell -ExecutionPolicy Bypass -File run/update.ps1
else
	@echo "Error: Zaxiom only runs on Windows"
	@exit 1
endif

# Show help
help:
	@echo "Zaxiom Build Commands"
	@echo "====================="
	@echo ""
	@echo "Development:"
	@echo "  make build       - Debug build"
	@echo "  make release     - Optimized release build"
	@echo "  make run         - Build and run (debug)"
	@echo "  make run-release - Build and run (release)"
	@echo "  make clean       - Remove build artifacts"
	@echo ""
	@echo "Quality:"
	@echo "  make check       - Type check without building"
	@echo "  make fmt         - Format code"
	@echo "  make fmt-check   - Check formatting"
	@echo "  make lint        - Run clippy lints"
	@echo "  make test        - Run tests"
	@echo "  make ci          - Run all checks (pre-commit)"
	@echo ""
	@echo "Installation (Windows only):"
	@echo "  make install     - Build and install to system"
	@echo "  make update      - Rebuild and update installed version"
	@echo "  make uninstall   - Remove from system"
