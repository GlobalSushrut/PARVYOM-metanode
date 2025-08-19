# Metanode / BPI Mesh - Development Makefile
# Stage 1.3: Development Environment

.PHONY: help install build test lint format clean dev docker proto wasm napi

# Default target
help:
	@echo "Metanode / BPI Mesh - Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  install     Install all dependencies (Rust + Node.js)"
	@echo "  bootstrap   Complete development setup"
	@echo ""
	@echo "Build:"
	@echo "  build       Build all Rust crates and TypeScript packages"
	@echo "  build-rust  Build Rust workspace only"
	@echo "  build-ts    Build TypeScript packages only"
	@echo "  wasm        Build WASM bindings for browser"
	@echo "  napi        Build Node-API bindings"
	@echo ""
	@echo "Test:"
	@echo "  test        Run all tests (Rust + TypeScript)"
	@echo "  test-rust   Run Rust tests only"
	@echo "  test-ts     Run TypeScript tests only"
	@echo "  test-stage1 Test Stage 1 completion"
	@echo ""
	@echo "Quality:"
	@echo "  lint        Run all linters"
	@echo "  format      Format all code"
	@echo "  clippy      Run Rust clippy"
	@echo ""
	@echo "Development:"
	@echo "  dev         Start development servers"
	@echo "  clean       Clean all build artifacts"
	@echo "  docker      Build development Docker image"
	@echo ""
	@echo "Protobuf:"
	@echo "  proto       Generate protobuf code"

# Stage 1.3.1: Installation
install:
	@echo "🔧 Installing Rust toolchain..."
	rustup update stable
	rustup component add rustfmt clippy
	@echo "🔧 Installing Node.js dependencies..."
	npm install -g pnpm@8.12.0
	pnpm install
	@echo "🔧 Installing pre-commit hooks..."
	pip install pre-commit
	pre-commit install
	@echo "✅ Installation complete"

bootstrap: install
	@echo "🚀 Bootstrapping development environment..."
	cargo build --workspace
	pnpm build
	@echo "🧪 Running Stage 1 tests..."
	$(MAKE) test-stage1
	@echo "✅ Bootstrap complete - ready for development!"

# Stage 1.2.1: Build targets
build: build-rust build-ts

build-rust:
	@echo "🦀 Building Rust workspace..."
	cargo build --workspace

build-ts:
	@echo "📦 Building TypeScript packages..."
	pnpm build

wasm:
	@echo "🌐 Building WASM bindings..."
	pnpm wasm:build

napi:
	@echo "🔗 Building Node-API bindings..."
	pnpm napi:build

# Stage 1.2.2: Testing
test: test-rust test-ts

test-rust:
	@echo "🦀 Running Rust tests..."
	cargo test --workspace --verbose

test-ts:
	@echo "📦 Running TypeScript tests..."
	pnpm test

# Stage 1 Exit Criteria Test
test-stage1:
	@echo "🧪 Testing Stage 1 Exit Criteria..."
	@echo "  ✓ Testing CI pipeline..."
	@test -f .github/workflows/ci.yml || (echo "❌ CI config missing" && exit 1)
	@echo "  ✓ Testing pre-commit hooks..."
	@test -f .pre-commit-config.yaml || (echo "❌ Pre-commit config missing" && exit 1)
	@echo "  ✓ Testing workspace structure..."
	@test -f Cargo.toml || (echo "❌ Cargo.toml missing" && exit 1)
	@test -f package.json || (echo "❌ package.json missing" && exit 1)
	@echo "  ✓ Testing build system..."
	cargo check --workspace --quiet
	@echo "✅ Stage 1 Exit Criteria: PASSED"

# Stage 1.2.3: Quality checks
lint:
	@echo "🔍 Running linters..."
	cargo clippy --workspace --all-targets -- -D warnings
	pnpm lint

format:
	@echo "🎨 Formatting code..."
	cargo fmt --all
	pnpm format

clippy:
	@echo "📎 Running Rust clippy..."
	cargo clippy --workspace --all-targets -- -D warnings

# Development
dev:
	@echo "🔥 Starting development servers..."
	pnpm dev

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	pnpm clean
	rm -rf target/
	rm -rf node_modules/
	rm -rf ts/bindings/*/pkg/

# Docker development environment
docker:
	@echo "🐳 Building development Docker image..."
	docker build -f Dockerfile.dev -t metanode-dev .

# Protobuf generation
proto:
	@echo "📡 Generating protobuf code..."
	pnpm proto:gen

# Stage progression
stage2: test-stage1
	@echo "🎯 Stage 1 complete, proceeding to Stage 2: Canonical Encoding Library"
	@echo "📋 Next: Implement CBOR fixed-order encoding with domain separation"
