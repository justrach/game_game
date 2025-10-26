#!/bin/bash
set -e

echo "🔨 Building On-Chain RPG..."

# Build the program
cargo build-bpf --manifest-path=Cargo.toml

echo "✅ Build complete!"
echo "📦 Program binary: target/deploy/onchain_rpg.so"
