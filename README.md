# HamShack - Next Generation Ham Radio Dashboard

A high-performance, Raspberry Pi-optimized ham radio dashboard built with **Rust** backend and **React** frontend.

## Project Vision

HamShack aims to replace OpenHamClock with:
- **80% lower memory usage** (<50MB baseline)
- **Real-time SDR integration** with waterfall display
- **AI-powered signal analysis** and propagation prediction
- **Superior extensibility** via plugin system

## Architecture

```
HamShack/
├── backend/          # Rust backend (Axum + Tokio)
│   ├── src/
│   │   ├── main.rs   # HTTP/2 + SSE server
│   │   ├── config.rs # Configuration loader
│   │   ├── cache.rs  # Spot cache with ring buffers
│   │   └── sdr.rs    # SDR manager (SoapySDR integration)
│   └── Cargo.toml
├── frontend/         # React frontend (Vite)
│   ├── src/
│   │   ├── App.jsx   # Main dashboard UI
│   │   ├── hooks/    # Custom hooks (useSSE)
│   │   └── components/
│   └── package.json
└── scripts/          # Build/deployment scripts
```

## Quick Start

### Prerequisites
- Rust 1.70+
- Node.js 18+

### Development
```bash
# Backend (Rust)
cd backend && cargo run
# Frontend (React) 
cd frontend && npm run dev
```

### Production Build
```bash
make build
```

### Raspberry Pi Cross-compilation
```bash
make build-pi
```

### Getting Started (Git Clone)

#### Automatic Installation (Recommended)
```bash
# Clone and auto-install dependencies
git clone https://github.com/lef-zach/HamShack.git
cd HamShack
./scripts/install-deps.sh
make install
make build
make dev
```

#### Manual Installation
```bash
# Clone repository
git clone https://github.com/lef-zach/HamShack.git
cd HamShack

# Install dependencies (Makefile handles missing tools)
make install

# Build and run
make build
make dev
```

Makefile will automatically detect and help install missing Rust/Node.js dependencies.

## Features

### Implemented
- **Rust backend** with HTTP/2 + SSE streaming
- **React frontend** with real-time updates
- **Spot caching** with memory limits
- **SDR integration** framework

### In Development
- **SDR waterfall** visualization
- **AI signal classification**
- **Plugin system** for extensibility
- **Raspberry Pi optimizations**

## Configuration

Copy `.env.example` to `.env` and customize:
```env
HAMSHACK_PORT=3000
HAMSHACK_HOST=0.0.0.0
HAMSHACK_CALLSIGN=N0CALL
HAMSHACK_LOCATOR=FN31
HAMSHACK_SDR_ENABLED=false
HAMSHACK_SDR_DEVICE=rtlsdr
```

## Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| Memory Usage | <50MB | Initial build ~10MB |
| CPU Usage | <10% idle | Rust efficiency |
| Latency | <100ms | SSE streaming |
| Boot Time | <15s | Fast startup |

## Development Roadmap

### Phase 1: Foundation (Complete)
- [x] Rust backend with SSE
- [x] React frontend
- [x] Basic spot caching

### Phase 2: SDR Integration
- [ ] SoapySDR Rust bindings
- [ ] Real-time waterfall display
- [ ] Multi-VFO support

### Phase 3: AI Features  
- [ ] TensorFlow Lite integration
- [ ] Signal classification
- [ ] Propagation prediction

### Phase 4: Production
- [ ] Raspberry Pi optimizations
- [ ] Plugin marketplace
- [ ] Performance benchmarks

## Contributing

HamShack is designed for community development:
- **Rust expertise** welcome for backend optimization
- **React/TypeScript** skills for UI improvements
- **SDR knowledge** for hardware integration
- **AI/ML experience** for signal processing

## License

MIT License - See LICENSE file for details.

---

**Built for the ham radio community**

*73 de HamShack!*

## Raspberry Pi Deployment

### Fully Autonomous Deployment
Simply run the setup script - it handles everything automatically:

```bash
# On Raspberry Pi - one command deployment
curl -fsSL https://raw.githubusercontent.com/lef-zach/HamShack/main/scripts/setup-pi.sh | bash
```

This will:
- Install Rust and Node.js automatically
- Build both backend and frontend
- Configure as system service
- Start HamShack on boot

### Manual Deployment
```bash
# Clone and run setup
git clone https://github.com/lef-zach/HamShack.git
cd HamShack/bash scripts/setup-pi.sh

# Or run individual commands
make install    # Auto-installs missing dependencies
make build     # Builds the application

# Run manually
cd backend && ./target/release/hamshack
```

### Cross-Compilation Deployment
```bash
# On development machine
make build-pi4  # For Pi 3/4
# or make build-pi5 for Pi 5
make package-pi

# Transfer to Pi and run
scp -r dist/pi/ pi@raspberrypi.local:HamShack/
ssh pi@raspberrypi.local "cd HamShack && ./hamshack"
```

### Pi Model Support
- **Raspberry Pi 3**: `make build-pi3` (ARMv7)
- **Raspberry Pi 4**: `make build-pi4` (ARMv7) 
- **Raspberry Pi 5**: `make build-pi5` (ARM64)
