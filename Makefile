# HamShack Makefile

.PHONY: all backend frontend dev build clean test help

# Default target
all: backend frontend

# Backend targets
backend:
	@echo "🔧 Building Rust backend..."
	cd backend && cargo build --release

backend-dev:
	@echo "🔧 Running Rust backend in development mode..."
	cd backend && cargo run

# Frontend targets
frontend:
	@echo "⚛️  Building React frontend..."
	cd frontend && npm run build

frontend-dev:
	@echo "⚛️  Starting frontend development server..."
	cd frontend && npm run dev

# Development mode (both backend and frontend)
dev: backend-dev frontend-dev

# Build for production
build: backend frontend
	@echo "✅ Production build complete"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cd backend && cargo clean
	cd frontend && rm -rf dist

# Cross-compilation for Raspberry Pi
build-pi:
	@echo "🍓 Building for Raspberry Pi..."
	cd backend && \
	RUSTFLAGS="-C target-cpu=cortex-a72" \
	cargo build --release --target armv7-unknown-linux-gnueabihf
	cd frontend && npm run build

# Install dependencies
install:
	@echo "🔥 Installing HamShack dependencies..."
	@echo "🔍 Checking for Rust..."
	@if command -v cargo >/dev/null 2>&1; then \
		echo "✅ Rust found: $(cargo --version)"; \
	else \
		echo "🦀 Installing Rust..."; \
		if command -v curl >/dev/null 2>&1; then \
			curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
			export PATH="$$HOME/.cargo/bin:$$PATH"; \
		else \
			echo "❌ Please install Rust manually: https://www.rust-lang.org/tools/install"; \
			exit 1; \
		fi; \
	fi
	@echo "🔍 Checking for Node.js..."
	@if command -v npm >/dev/null 2>&1; then \
		echo "✅ Node.js found: $(node --version)"; \
		echo "✅ npm found: $(npm --version)"; \
	else \
		echo "📦 Installing Node.js..."; \
		if command -v curl >/dev/null 2>&1 && command -v sudo >/dev/null 2>&1; then \
			curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -; \
			sudo apt-get install -y nodejs; \
		elif command -v brew >/dev/null 2>&1; then \
			brew install node; \
		elif command -v winget >/dev/null 2>&1; then \
			winget install OpenJS.NodeJS.LTS; \
		else \
			echo "❌ Please install Node.js manually: https://nodejs.org/"; \
			sudo apt-get update && sudo apt-get install -y nodejs npm || true; \
		fi; \
	fi
	@echo "📦 Installing Rust dependencies..."
	@export PATH="$$HOME/.cargo/bin:$$PATH"; \
	cd backend && cargo fetch
	@echo "📦 Installing Node.js dependencies..."
	cd frontend && npm install
	@echo "✅ All dependencies installed successfully!"

# Run tests
test:
	@echo "🧪 Running tests..."
	cd backend && cargo test
	cd frontend && npm test

# Cross-compilation for Raspberry Pi 4 (ARMv7)
build-pi4:
	@echo "🍓 Building for Raspberry Pi 4 (ARMv7)..."
	cd backend && \
	RUSTFLAGS="-C target-cpu=cortex-a72" \
	cargo build --release --target armv7-unknown-linux-gnueabihf
	cd frontend && npm run build

# Cross-compilation for Raspberry Pi 5 (ARM64)
build-pi5:
	@echo "🍓 Building for Raspberry Pi 5 (ARM64)..."
	cd backend && \
	RUSTFLAGS="-C target-cpu=cortex-a76" \
	cargo build --release --target aarch64-unknown-linux-gnu
	cd frontend && npm run build

# Cross-compilation for Raspberry Pi 3 (ARMv7)
build-pi3:
	@echo "🍓 Building for Raspberry Pi 3 (ARMv7)..."
	cd backend && \
	RUSTFLAGS="-C target-cpu=cortex-a53" \
	cargo build --release --target armv7-unknown-linux-gnueabihf
	cd frontend && npm run build

# Install cross-compilation toolchain
install-pi-toolchain:
	@echo "🔧 Installing Raspberry Pi cross-compilation toolchain..."
	rustup target add armv7-unknown-linux-gnueabihf
	rustup target add aarch64-unknown-linux-gnu

# Create Pi deployment package
package-pi:
	@echo "📦 Creating Raspberry Pi deployment package..."
	mkdir -p dist/pi
	cp backend/target/armv7-unknown-linux-gnueabihf/release/hamshack dist/pi/
	cp -r frontend/dist dist/pi/frontend
	cp .env.example dist/pi/.env
	cp scripts/setup-pi.sh dist/pi/
	chmod +x dist/pi/setup-pi.sh
	@echo "✅ Pi package created in dist/pi/"

# Help
help:
	@echo "HamShack Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  all               - Build both backend and frontend"
	@echo "  backend           - Build Rust backend"
	@echo "  frontend          - Build React frontend"
	@echo "  dev               - Run both in development mode"
	@echo "  build             - Build for production"
	@echo "  build-pi4         - Cross-compile for Raspberry Pi 4"
	@echo "  build-pi5         - Cross-compile for Raspberry Pi 5"
	@echo "  build-pi3         - Cross-compile for Raspberry Pi 3"
	@echo "  install-pi-toolchain - Install Pi cross-compilation tools"
	@echo "  package-pi        - Create Pi deployment package"
	@echo "  clean             - Clean build artifacts"
	@echo "  install           - Install dependencies"
	@echo "  test              - Run tests"
	@echo "  help              - Show this help"