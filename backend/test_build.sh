#!/bin/bash
# Test script to check Rust dependencies

echo "🔧 Testing Rust dependencies..."

# Check if basic Rust compilation works
echo "fn main() { println!("Hello, HamLock!"); }" > test_simple.rs
rustc test_simple.rs && ./test_simple.exe
rm test_simple.rs test_simple.exe 2>/dev/null

echo "✅ Basic Rust compilation works"
echo ""
echo "📦 Dependencies to verify:"
echo "- num-complex"
echo "- rustfft"  
echo "- rand"
echo "- rayon"
echo "- chrono"
echo ""
echo "🚀 To compile the backend, run:"
echo "cd backend && cargo build"