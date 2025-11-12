# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**instagres** is a Rust-based project for instant PostgreSQL instance provisioning (name suggests "instant Postgres").

## Build and Development Commands

### Building
```bash
cargo build          # Debug build
cargo build --release # Release build
```

### Running
```bash
cargo run           # Run debug build
cargo run --release # Run release build
```

### Testing
```bash
cargo test                    # Run all tests
cargo test <test_name>        # Run specific test
cargo test -- --nocapture     # Run tests with stdout output
cargo test -- --test-threads=1 # Run tests sequentially
```

### Code Quality
```bash
cargo clippy                  # Run linter
cargo clippy -- -D warnings   # Run linter, treating warnings as errors
cargo fmt                     # Format code
cargo fmt -- --check          # Check formatting without modifying
```

### Other Useful Commands
```bash
cargo check        # Fast compilation check without generating binary
cargo clean        # Clean build artifacts
cargo doc --open   # Generate and open documentation
```

## Architecture Notes

This is an early-stage project. Architecture details will be added as the codebase develops.
