#!/bin/bash

# HamShack Autonomous Dependency Installer
# Handles all dependencies automatically

set -e

echo "🔥 HamShack Autonomous Dependency Installation"
echo "=============================================="

# Detect platform and set variables
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="mac"
elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    PLATFORM="windows"
else
    PLATFORM="unknown"
fi

# Windows Git Bash specific detection
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    echo "⚠️  Running on Windows Git Bash - skipping system package installation"
    echo "📝 Please ensure you have Rust and Node.js installed manually"
fi

echo "🔧 Platform detected: $PLATFORM"

# Function to check command availability
check_command() {
    if command -v "$1" >/dev/null 2>&1; then
        echo "✅ $1 found: $(which $1)"
        return 0
    else
        echo "❌ $1 not found"
        return 1
    fi
}

# Install Rust if missing
if ! check_command cargo; then
    echo "🦀 Installing Rust..."
    if check_command curl; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        echo "✅ Rust installed successfully"
    else
        echo "❌ Cannot install Rust (curl not available)"
        echo "📝 Please install Rust manually: https://rustup.rs/"
        exit 1
    fi
else
    echo "✅ Rust already installed: $(cargo --version)"
fi

# Install Node.js if missing
if ! check_command node || ! check_command npm; then
    echo "📦 Installing Node.js..."
    
    case "$PLATFORM" in
        "linux")
            if check_command apt-get; then
                # Debian/Ubuntu
                curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
                sudo apt-get install -y nodejs
            elif check_command yum; then
                # RedHat/CentOS
                curl -fsSL https://rpm.nodesource.com/setup_lts.x | sudo bash -
                sudo yum install -y nodejs
            elif check_command pacman; then
                # Arch Linux
                sudo pacman -S nodejs npm
            else
                # Try package manager independent installation
                curl -fsSL https://fnm.vercel.app/install | bash
                eval "$(fnm env --use-on-cd)"
                fnm install --lts
                fnm use --lts
            fi
            ;;
        "mac")
            if check_command brew; then
                brew install node
            else
                # Install Homebrew then Node.js
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
                eval "$(/opt/homebrew/bin/brew shellenv)"
                brew install node
            fi
            ;;
        "windows")
            if check_command winget; then
                winget install OpenJS.NodeJS.LTS
            else
                echo "📝 Please download and install Node.js from: https://nodejs.org/"
                echo "📝 Or install winget and try again"
                exit 1
            fi
            ;;
        *)
            echo "⚠️  Unknown platform - attempting generic Node.js installation"
            curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
            export NVM_DIR="$HOME/.nvm"
            [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
            nvm install --lts
            nvm use --lts
            ;;
    esac
    
    echo "✅ Node.js installation completed"
else
    echo "✅ Node.js already installed: $(node --version)"
    echo "✅ npm already installed: $(npm --version)"
fi

# Verify installations
echo "🔍 Verifying installations..."

if check_command cargo && check_command node && check_command npm; then
    echo ""
    echo "🎉 All dependencies installed successfully!"
    echo ""
    echo "📋 Installation Summary:"
    echo "   Rust: $(cargo --version)"
    echo "   Node.js: $(node --version)"
    echo "   npm: $(npm --version)"
    echo ""
    echo "✅ HamShack is ready for installation!"
    echo "💡 Run 'make install' to install project dependencies"
else
    echo ""
    echo "❌ Some dependencies failed to install"
    echo "📝 Please install missing dependencies manually:"
    echo "   Rust: https://rustup.rs/"
    echo "   Node.js: https://nodejs.org/"
    exit 1
fi