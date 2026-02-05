#!/bin/bash

# HamShack Raspberry Pi Setup Script
# Run this on your Raspberry Pi for easy deployment

set -e

echo "🔧 Setting up HamShack on Raspberry Pi..."

# Clone repository if not already present
if [ ! -d "HamShack" ]; then
    echo "📥 Cloning HamShack repository..."
    git clone https://github.com/lef-zach/HamShack.git
    cd HamShack
else
    echo "📂 Using existing HamShack directory..."
    cd HamShack
    git pull origin main
fi

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "🦀 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Install Node.js if not present
if ! command -v node &> /dev/null; then
    echo "📦 Installing Node.js..."
    curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
    sudo apt-get install -y nodejs
fi

# Install dependencies
echo "📦 Installing dependencies..."
make install

# Build application
echo "🔨 Building HamShack..."
make build

# Setup configuration
if [ ! -f ".env" ]; then
    echo "⚙️  Creating configuration file..."
    cp .env.example .env
    echo "📝 Please edit .env file with your settings:"
    echo "   - HAMSHACK_CALLSIGN"
    echo "   - HAMSHACK_LOCATOR"
    echo "   - HAMSHACK_SDR_ENABLED (if you have SDR hardware)"
fi

echo "✅ Setup complete!"
echo ""
echo "To start HamShack:"
echo "  cd HamShack"
echo "  cd backend && ./target/release/hamshack"
echo ""
echo "Then access the dashboard at: http://$(hostname -I | awk '{print $1}'):3000"