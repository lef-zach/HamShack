#!/bin/bash

# HamShack Raspberry Pi Autonomous Setup Script
# Fully automated deployment for Raspberry Pi

set -e
set -x  # Enable debug mode to see what's happening

echo "🔥 HamShack Raspberry Pi Autonomous Setup"
echo "=========================================="

echo "🔧 Starting automatic setup..."

# Update system first
echo "📦 Updating system packages..."
# Update system first
echo "📦 Updating system packages..."
if command -v apt-get >/dev/null 2>&1; then
    sudo apt-get update
    sudo apt-get upgrade -y
    
    # Install essential dependencies
    echo "📦 Installing essential tools..."
    sudo apt-get install -y curl git build-essential pkg-config libssl-dev make
elif command -v pacman >/dev/null 2>&1; then
    sudo pacman -Syu --noconfirm
    sudo pacman -S --noconfirm curl git base-devel pkgconf openssl make
else
    echo "⚠️  Package manager not detected - assuming tools are available"
fi

# Clone repository
echo "📥 Cloning HamShack repository..."
git clone https://github.com/lef-zach/HamShack.git
cd HamShack

# Run autonomous dependency installer
echo "📦 Running autonomous dependency installer..."
if [ -f "scripts/install-deps.sh" ]; then
    chmod +x scripts/install-deps.sh
    bash scripts/install-deps.sh
else
    echo "❌ install-deps.sh not found - running manual dependency installation"
    # Manual dependency installation fallback
    if ! command -v cargo >/dev/null 2>&1; then
        echo "🦀 Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    if ! command -v node >/dev/null 2>&1 || ! command -v npm >/dev/null 2>&1; then
        echo "📦 Installing Node.js..."
        curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
        sudo apt-get install -y nodejs
    fi
fi

# Install project dependencies
echo "📦 Installing project dependencies..."
PATH="$HOME/.cargo/bin:$PATH" make install

# Build application
echo "🔨 Building HamShack..."
PATH="$HOME/.cargo/bin:$PATH" make build

# Ensure static directory exists for frontend
echo "📁 Setting up static file directory..."
mkdir -p backend/static
if [ -d "frontend/dist" ]; then
    cp -r frontend/dist/* backend/static/
fi

# Setup configuration
if [ ! -f ".env" ]; then
    echo "⚙️  Creating configuration file..."
    cp .env.example .env
    
    # Auto-detect Raspberry Pi settings
    IP_ADDRESS=$(hostname -I | awk '{print $1}')
    echo "# Auto-generated configuration for Raspberry Pi" >> .env
    echo "HAMSHACK_HOST=$IP_ADDRESS" >> .env
    echo "HAMSHACK_PORT=3000" >> .env
    echo "HAMSHACK_SDR_ENABLED=true" >> .env
    
    echo "📝 Configuration file created with auto-detected settings:"
    echo "   Host: $IP_ADDRESS"
    echo "   Port: 3000"
    echo "   SDR Enabled: true"
    echo ""
    echo "💡 Edit .env file to customize callsign and other settings"
fi

# Create startup service
echo "🚀 Creating startup service..."
sudo tee /etc/systemd/system/hamshack.service > /dev/null <<EOF
[Unit]
Description=HamShack Ham Radio Dashboard
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$PWD/backend
ExecStart=$PWD/backend/target/release/hamshack
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
echo "🔧 Enabling HamShack service..."
sudo systemctl daemon-reload
sudo systemctl enable hamshack.service
sudo systemctl start hamshack.service

echo "🎉 Setup complete!"
echo ""
echo "📋 HamShack is now running as a system service"
echo "🌐 Access the dashboard at: http://$IP_ADDRESS:3000"
echo ""
echo "🔧 Service management commands:"
echo "   sudo systemctl status hamshack"
echo "   sudo systemctl stop hamshack"
echo "   sudo systemctl start hamshack"
echo "   sudo systemctl restart hamshack"
echo "   sudo journalctl -u hamshack -f"

# Display initial status
echo ""
echo "🏁 Initial service status:"
sudo systemctl status hamshack --no-pager